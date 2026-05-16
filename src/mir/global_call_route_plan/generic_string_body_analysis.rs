use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

use super::generic_string_facts::{
    update_generic_pure_string_return_param_values, GenericPureValueClass,
};
use super::generic_string_reject::GenericPureStringReject;
use super::model::{GlobalCallTargetFacts, GlobalCallTargetShapeReason};

mod call_transfer;
mod value_transfer;

pub(super) struct GenericPureStringAnalysisContext<'a> {
    pub(super) function: &'a MirFunction,
    pub(super) block: BasicBlockId,
    pub(super) instruction_index: usize,
    pub(super) targets: &'a BTreeMap<String, GlobalCallTargetFacts>,
    pub(super) values: &'a mut BTreeMap<ValueId, GenericPureValueClass>,
    pub(super) has_string_surface: &'a mut bool,
    pub(super) has_void_sentinel_const: &'a mut bool,
    pub(super) non_void_string_values: &'a BTreeSet<ValueId>,
    pub(super) changed: &'a mut bool,
}

pub(super) fn generic_pure_string_instruction_reject_reason(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    return_param_values: &mut BTreeSet<ValueId>,
    has_string_surface: &mut bool,
    has_void_sentinel_const: &mut bool,
    non_void_string_values: &BTreeSet<ValueId>,
    changed: &mut bool,
) -> Option<GenericPureStringReject> {
    update_generic_pure_string_return_param_values(instruction, return_param_values, changed);
    let mut ctx = GenericPureStringAnalysisContext {
        function,
        block,
        instruction_index,
        targets,
        values,
        has_string_surface,
        has_void_sentinel_const,
        non_void_string_values,
        changed,
    };

    match instruction {
        MirInstruction::Const { .. }
        | MirInstruction::Copy { .. }
        | MirInstruction::NewBox { .. }
        | MirInstruction::BinOp { .. }
        | MirInstruction::Compare { .. }
        | MirInstruction::UnaryOp { .. }
        | MirInstruction::Phi { .. }
        | MirInstruction::Select { .. } => {
            value_transfer::generic_pure_string_value_instruction_reject_reason(
                &mut ctx,
                instruction,
            )
        }
        MirInstruction::Call { .. } => {
            call_transfer::generic_pure_string_call_reject_reason(&mut ctx, instruction)
        }
        MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::ReleaseStrong { .. } => None,
        _ => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
        )),
    }
}
