//! Object Home End verification from the published MIR alone.
//!
//! The source handoff supplies aliases and initial publication only for
//! functions that contain a checked move. Every other function with a Home
//! (`NewBox`) used to be checked only by the source-to-MIR correspondence
//! check. This module derives the same two inputs from the MIR itself — the
//! `NewBox` results, the `Birth` invokes that publish them and the single
//! `Copy` that names the published value — and runs the one path-sensitive
//! verifier over every Normal and Fault exit. It reads no generator proof;
//! the only external input is the declared `take` permission maps, which are
//! the published input's declarations and which the move path already uses.

use super::error::OwnershipSsaErrorV1 as Error;
use super::model::{FunctionResultOwnershipV1, MirOwnershipKindV1, OwnershipFunctionAbiV1, OwnershipFunctionOwnerV1};
use crate::mir::instruction::InvokeOperation;
use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};
use hakorune_mir_defs::{CanonicalObjectIdV1, CanonicalSameModuleCallableKeyV1};
use std::collections::{BTreeMap, BTreeSet};

/// What the independent check established for one function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedHomeVerdictV1 {
    /// The function creates no object Home.
    NoHomes,
    /// Every Home is released exactly once on every Normal and Fault exit.
    Verified,
    /// The function uses a value or instruction the ownership verifier does
    /// not model (handle results, text, owned parameters). Nothing is claimed.
    OutsideModel,
}

pub(crate) fn verify_published_object_homes_v1(
    function: &MirFunction,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    birth_takes: &BTreeMap<CanonicalSameModuleCallableKeyV1, BTreeSet<u32>>,
    call_takes: &BTreeMap<CanonicalSameModuleCallableKeyV1, BTreeMap<u32, CanonicalObjectIdV1>>,
    module: Option<&crate::mir::MirModule>,
) -> Result<PublishedHomeVerdictV1, Error> {
    let mut homes = BTreeSet::new();
    let mut shared = false;
    for (id, block) in &function.blocks {
        match &block.terminator {
            Some(MirInstruction::Invoke { operation: InvokeOperation::NewBox { .. }, normal_landing, .. }) => {
                let Some(landing) = function.blocks.get(normal_landing) else { return Ok(PublishedHomeVerdictV1::OutsideModel); };
                homes.extend(landing.instructions.iter().filter_map(|row| match row {
                    MirInstruction::InvokeNormalResult { invoke_block, dst } if invoke_block == id => Some(*dst),
                    _ => None,
                }));
            }
            Some(MirInstruction::Invoke { operation: InvokeOperation::Shared(_), .. }) => shared = true,
            _ => {}
        }
    }
    if homes.is_empty() { return Ok(PublishedHomeVerdictV1::NoHomes); }
    if shared { return Ok(PublishedHomeVerdictV1::OutsideModel); }
    let Some(parameters) = function.signature.params.iter().map(|ty| match ty {
        MirType::Integer | MirType::Bool => Some(MirOwnershipKindV1::None),
        MirType::Box(_) => Some(MirOwnershipKindV1::Borrowed),
        _ => None,
    }).collect::<Option<Vec<_>>>() else { return Ok(PublishedHomeVerdictV1::OutsideModel); };
    if !matches!(function.signature.return_type, MirType::Integer | MirType::Bool | MirType::Void) {
        return Ok(PublishedHomeVerdictV1::OutsideModel);
    }
    let birth_receivers: BTreeSet<ValueId> = function.blocks.values().filter_map(|block| match &block.terminator {
        Some(MirInstruction::Invoke { operation: InvokeOperation::Birth(row), .. }) => Some(row.receiver),
        _ => None,
    }).collect();
    let mut aliases = BTreeMap::new();
    for home in &homes {
        let mut copies = function.blocks.values().flat_map(|block| &block.instructions)
            .filter_map(|row| match row { MirInstruction::Copy { dst, src } if src == home => Some(*dst), _ => None });
        match (copies.next(), copies.next()) {
            (Some(dst), None) => { aliases.insert(dst, *home); }
            // Several copies (a Home plus LocalSSA read temporaries) cannot be
            // told apart from the MIR alone: make no claim.
            (Some(_), Some(_)) => return Ok(PublishedHomeVerdictV1::OutsideModel),
            _ => {}
        }
    }
    let initially_published = homes.iter().copied().filter(|home| !birth_receivers.contains(home)).collect();
    let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), parameters, FunctionResultOwnershipV1::None)
        .with_source_aliases(aliases)
        .with_initially_published(initially_published)
        .with_source_birth_takes(birth_takes.clone())
        .with_source_call_takes(call_takes.clone());
    match super::verify::verify(function, &abi, module) {
        Ok(_) => Ok(PublishedHomeVerdictV1::Verified),
        Err(Error::UnsupportedManagedInstruction { .. } | Error::InvalidAcquisition { .. }
            | Error::ManagedCallOwnershipUnsupported { .. } | Error::InvalidSourceAlias { .. }
            | Error::UnknownValueKind { .. } | Error::EdgeArgumentsForbidden { .. }) => Ok(PublishedHomeVerdictV1::OutsideModel),
        Err(error) => Err(error),
    }
}
