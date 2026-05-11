use super::{exact_numeric_type_for_value_fact, ExactNumericValueFact};
use crate::mir::exact_numeric_unification::{
    unify_exact_numeric_inputs, ExactNumericMergeSite, ExactNumericUnificationError,
};
use crate::mir::{BasicBlockId, CompareOp, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericCompareRouteFact {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub dst: ValueId,
    pub op: CompareOp,
    pub lhs: ValueId,
    pub rhs: ValueId,
    pub declared_type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactNumericCompareRouteRejectionKind {
    MixedExactAndDynamic {
        exact_source_name: String,
    },
    TypeMismatch {
        left_source_name: String,
        right_source_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericCompareRouteRejection {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub dst: ValueId,
    pub op: CompareOp,
    pub lhs: ValueId,
    pub rhs: ValueId,
    pub kind: ExactNumericCompareRouteRejectionKind,
}

pub(super) fn collect_compare_route_facts(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Vec<ExactNumericCompareRouteFact> {
    let mut route_facts = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::Compare { dst, op, lhs, rhs } = spanned.inst else {
                continue;
            };
            let incoming = [
                facts.get(lhs).and_then(exact_numeric_type_for_value_fact),
                facts.get(rhs).and_then(exact_numeric_type_for_value_fact),
            ];
            let Ok(Some(ty)) =
                unify_exact_numeric_inputs(ExactNumericMergeSite::Compare, &incoming)
            else {
                continue;
            };
            route_facts.push(ExactNumericCompareRouteFact {
                block: block_id,
                instruction_index,
                dst: *dst,
                op: *op,
                lhs: *lhs,
                rhs: *rhs,
                declared_type_name: ty.source_name,
            });
        }
    }
    route_facts
}

pub(super) fn collect_compare_route_rejections(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Vec<ExactNumericCompareRouteRejection> {
    let mut rejections = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::Compare { dst, op, lhs, rhs } = spanned.inst else {
                continue;
            };
            let incoming = [
                facts.get(lhs).and_then(exact_numeric_type_for_value_fact),
                facts.get(rhs).and_then(exact_numeric_type_for_value_fact),
            ];
            let Some(error) =
                unify_exact_numeric_inputs(ExactNumericMergeSite::Compare, &incoming).err()
            else {
                continue;
            };
            rejections.push(ExactNumericCompareRouteRejection {
                block: block_id,
                instruction_index,
                dst: *dst,
                op: *op,
                lhs: *lhs,
                rhs: *rhs,
                kind: compare_route_rejection_kind(error),
            });
        }
    }
    rejections
}

fn compare_route_rejection_kind(
    error: ExactNumericUnificationError,
) -> ExactNumericCompareRouteRejectionKind {
    match error {
        ExactNumericUnificationError::MixedExactAndDynamic {
            exact_source_name, ..
        } => ExactNumericCompareRouteRejectionKind::MixedExactAndDynamic { exact_source_name },
        ExactNumericUnificationError::TypeMismatch {
            left_source_name,
            right_source_name,
            ..
        } => ExactNumericCompareRouteRejectionKind::TypeMismatch {
            left_source_name,
            right_source_name,
        },
    }
}
