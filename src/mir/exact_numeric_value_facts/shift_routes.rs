use super::{
    exact_numeric_type_for_value_fact, ExactNumericValueFact, ExactNumericValueFactSource,
};
use crate::mir::numeric_substrate::NumericSignedness;
use crate::mir::{BasicBlockId, BinaryOp, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericShiftRouteFact {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub dst: ValueId,
    pub op: BinaryOp,
    pub lhs: ValueId,
    pub rhs: ValueId,
    pub declared_type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactNumericShiftRouteRejectionKind {
    SignedLogicalShift { source_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericShiftRouteRejection {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub dst: ValueId,
    pub op: BinaryOp,
    pub lhs: ValueId,
    pub rhs: ValueId,
    pub kind: ExactNumericShiftRouteRejectionKind,
}

pub(super) fn try_publish_logical_shift_fact(
    facts: &mut BTreeMap<ValueId, ExactNumericValueFact>,
    dst: ValueId,
    op: BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
) -> bool {
    if op != BinaryOp::Shr || facts.contains_key(&dst) {
        return false;
    }
    let Some(ty) = facts.get(&lhs).and_then(exact_numeric_type_for_value_fact) else {
        return false;
    };
    if ty.kind.signedness != NumericSignedness::Unsigned {
        return false;
    }

    facts.insert(
        dst,
        ExactNumericValueFact {
            declared_type_name: ty.source_name,
            source: ExactNumericValueFactSource::BinaryOp { op, lhs, rhs },
        },
    );
    true
}

pub(super) fn collect_shift_route_facts(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Vec<ExactNumericShiftRouteFact> {
    let mut route_facts = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Shr,
                lhs,
                rhs,
            } = spanned.inst
            else {
                continue;
            };
            let Some(fact) = facts.get(dst) else {
                continue;
            };
            if !matches!(
                &fact.source,
                ExactNumericValueFactSource::BinaryOp {
                    op: BinaryOp::Shr,
                    lhs: source_lhs,
                    rhs: source_rhs,
                } if *source_lhs == *lhs && *source_rhs == *rhs
            ) {
                continue;
            }
            route_facts.push(ExactNumericShiftRouteFact {
                block: block_id,
                instruction_index,
                dst: *dst,
                op: BinaryOp::Shr,
                lhs: *lhs,
                rhs: *rhs,
                declared_type_name: fact.declared_type_name.clone(),
            });
        }
    }
    route_facts
}

pub(super) fn collect_shift_route_rejections(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Vec<ExactNumericShiftRouteRejection> {
    let mut rejections = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Shr,
                lhs,
                rhs,
            } = spanned.inst
            else {
                continue;
            };
            let Some(ty) = facts.get(lhs).and_then(exact_numeric_type_for_value_fact) else {
                continue;
            };
            if ty.kind.signedness == NumericSignedness::Unsigned {
                continue;
            }
            rejections.push(ExactNumericShiftRouteRejection {
                block: block_id,
                instruction_index,
                dst: *dst,
                op: BinaryOp::Shr,
                lhs: *lhs,
                rhs: *rhs,
                kind: ExactNumericShiftRouteRejectionKind::SignedLogicalShift {
                    source_name: ty.source_name,
                },
            });
        }
    }
    rejections
}
