//! Proven `Callee::Method { ArrayBox, get }` residuals canonicalize to the
//! sole physical read owner.
//!
//! A canonical `Call` whose receiver class is either `TypeCertainty::Known`
//! or independently proven `ArrayBox` by receiver origin carries the same
//! meaning as `ArrayElementRead` — the producer already proved the
//! receiver, so rewriting is a representation fix, not a new proof.
//! `Union` rows without origin proof keep their dynamic-facade meaning and
//! pass through untouched: this arm is deliberately not a second resolver.
//! `LegacyCallV0` rows are never laundered here — downstream named-stops
//! own their rejection.

use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::generic_method_route_facts::receiver_origin_box_name;
use crate::mir::value_origin::{build_value_def_map, ValueDefMap};
use crate::mir::{ArrayReadSiteId, Callee, MirFunction, MirInstruction, MirType, ValueId};

/// Extract the element index from a `get` argument list.
///
/// The canonical shape is `get(index)`; a legacy surface may redundantly
/// carry the receiver as `args[0]` — accept `get(receiver, index)` only
/// when `args[0]` is literally the receiver value.
fn get_index(args: &[ValueId], receiver: ValueId) -> Option<ValueId> {
    match args {
        [index] => Some(*index),
        [first, index] if *first == receiver => Some(*index),
        _ => None,
    }
}

/// Rewrite proven `Call` + `Callee::Method { ArrayBox, get }` rows to
/// `ArrayElementRead`. Returns the number of rewritten instructions.
pub(super) fn canonicalize_proven_array_method_reads(function: &mut MirFunction) -> usize {
    let mut candidates = Vec::new();
    let mut needs_origin_proof = false;
    for (block_id, block) in &function.blocks {
        for (index, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::Call(call) = inst else {
                continue;
            };
            let Callee::Method {
                box_name,
                method,
                receiver: Some(_),
                certainty,
                ..
            } = &call.callee
            else {
                continue;
            };
            if box_name != "ArrayBox" || method != "get" {
                continue;
            }
            if *certainty == TypeCertainty::Union {
                needs_origin_proof = true;
            }
            candidates.push((*block_id, index));
        }
    }
    if candidates.is_empty() {
        return 0;
    }

    let def_map: Option<ValueDefMap> = if needs_origin_proof {
        Some(build_value_def_map(function))
    } else {
        None
    };

    let mut next_site = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|inst| match inst {
            MirInstruction::ArrayElementRead { site_id, .. } => Some(site_id.0),
            _ => None,
        })
        .max()
        .map_or(0, |max| max + 1);

    let mut rewrites = Vec::new();
    for (block_id, index) in candidates {
        let block = &function.blocks[&block_id];
        let MirInstruction::Call(call) = &block.instructions[index] else {
            continue;
        };
        let Callee::Method {
            receiver: Some(receiver),
            certainty,
            ..
        } = &call.callee
        else {
            continue;
        };
        let proven = *certainty == TypeCertainty::Known
            || def_map.as_ref().map_or(false, |map| {
                receiver_origin_box_name(function, map, *receiver).as_deref() == Some("ArrayBox")
            });
        let Some(element_index) = proven.then(|| get_index(&call.args, *receiver)).flatten() else {
            continue;
        };
        rewrites.push((block_id, index, call.dst, *receiver, element_index));
    }

    let rewritten = rewrites.len();
    for (block_id, index, dst, receiver, element_index) in rewrites {
        let block = function
            .blocks
            .get_mut(&block_id)
            .expect("candidate block exists");
        block.instructions[index] = MirInstruction::ArrayElementRead {
            site_id: ArrayReadSiteId::new(next_site),
            dst,
            receiver,
            index: element_index,
        };
        next_site += 1;
        if let Some(dst) = dst {
            function.metadata.value_types.insert(dst, MirType::Unknown);
        }
    }
    rewritten
}
