use std::collections::BTreeMap;

use crate::mir::{Callee, MirFunction, MirInstruction, ValueId};

pub(super) fn generic_pure_string_known_receiver_return_blocker(
    function: &MirFunction,
) -> Option<super::super::generic_string_reject::GenericPureStringReject> {
    let mut blockers = BTreeMap::<ValueId, String>::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                update_generic_pure_string_known_receiver_return_blockers(
                    instruction,
                    &mut blockers,
                    &mut changed,
                );
            }
        }
        if !changed {
            break;
        }
    }

    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value: Some(value) } = instruction else {
                continue;
            };
            let Some(symbol) = blockers.get(value) else {
                continue;
            };
            return Some(super::super::generic_string_reject::GenericPureStringReject::with_blocker(
                super::super::model::GlobalCallTargetShapeReason::GenericStringUnsupportedKnownReceiverMethod,
                symbol.clone(),
                Some(super::super::model::GlobalCallTargetShapeReason::GenericStringUnsupportedKnownReceiverMethod),
            ));
        }
    }
    None
}

fn update_generic_pure_string_known_receiver_return_blockers(
    instruction: &MirInstruction,
    blockers: &mut BTreeMap<ValueId, String>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            ..
        } if box_name == "ParserBox" => {
            let symbol = format!("{}.{}", box_name, method);
            if blockers.get(dst) != Some(&symbol) {
                blockers.insert(*dst, symbol);
                *changed = true;
            }
        }
        MirInstruction::Copy { dst, src } => {
            if let Some(symbol) = blockers.get(src).cloned() {
                if blockers.get(dst) != Some(&symbol) {
                    blockers.insert(*dst, symbol);
                    *changed = true;
                }
            }
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            let Some(symbol) = inputs
                .iter()
                .filter_map(|(_, value)| blockers.get(value))
                .next()
                .cloned()
            else {
                return;
            };
            if blockers.get(dst) != Some(&symbol) {
                blockers.insert(*dst, symbol);
                *changed = true;
            }
        }
        _ => {}
    }
}
