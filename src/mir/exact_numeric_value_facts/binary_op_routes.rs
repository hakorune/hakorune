use super::{
    exact_numeric_type_for_value_fact, ExactNumericValueFact, ExactNumericValueFactSource,
};
use crate::mir::exact_numeric_unification::{
    unify_exact_numeric_inputs, ExactNumericMergeSite, ExactNumericUnificationError,
};
use crate::mir::{BasicBlockId, BinaryOp, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericBinaryOpRouteFact {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub dst: ValueId,
    pub op: BinaryOp,
    pub lhs: ValueId,
    pub rhs: ValueId,
    pub declared_type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactNumericBinaryOpRouteRejectionKind {
    MixedExactAndDynamic {
        exact_source_name: String,
    },
    TypeMismatch {
        left_source_name: String,
        right_source_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericBinaryOpRouteRejection {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub dst: ValueId,
    pub op: BinaryOp,
    pub lhs: ValueId,
    pub rhs: ValueId,
    pub kind: ExactNumericBinaryOpRouteRejectionKind,
}

pub(super) fn try_publish_binary_op_add_fact(
    facts: &mut BTreeMap<ValueId, ExactNumericValueFact>,
    dst: ValueId,
    lhs: ValueId,
    rhs: ValueId,
) -> bool {
    if facts.contains_key(&dst) {
        return false;
    }

    let incoming = [
        facts.get(&lhs).and_then(exact_numeric_type_for_value_fact),
        facts.get(&rhs).and_then(exact_numeric_type_for_value_fact),
    ];
    let Ok(Some(ty)) = unify_exact_numeric_inputs(ExactNumericMergeSite::BinaryOpAdd, &incoming)
    else {
        return false;
    };

    facts.insert(
        dst,
        ExactNumericValueFact {
            declared_type_name: ty.source_name,
            source: ExactNumericValueFactSource::BinaryOpAdd { lhs, rhs },
        },
    );
    true
}

pub(super) fn collect_binary_op_route_facts(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Vec<ExactNumericBinaryOpRouteFact> {
    let mut route_facts = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Add,
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
                ExactNumericValueFactSource::BinaryOpAdd {
                    lhs: source_lhs,
                    rhs: source_rhs,
                } if *source_lhs == *lhs && *source_rhs == *rhs
            ) {
                continue;
            }
            route_facts.push(ExactNumericBinaryOpRouteFact {
                block: block_id,
                instruction_index,
                dst: *dst,
                op: BinaryOp::Add,
                lhs: *lhs,
                rhs: *rhs,
                declared_type_name: fact.declared_type_name.clone(),
            });
        }
    }
    route_facts
}

pub(super) fn collect_binary_op_route_rejections(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Vec<ExactNumericBinaryOpRouteRejection> {
    let mut rejections = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Add,
                lhs,
                rhs,
            } = spanned.inst
            else {
                continue;
            };
            let incoming = [
                facts.get(lhs).and_then(exact_numeric_type_for_value_fact),
                facts.get(rhs).and_then(exact_numeric_type_for_value_fact),
            ];
            let Some(error) =
                unify_exact_numeric_inputs(ExactNumericMergeSite::BinaryOpAdd, &incoming).err()
            else {
                continue;
            };
            rejections.push(ExactNumericBinaryOpRouteRejection {
                block: block_id,
                instruction_index,
                dst: *dst,
                op: BinaryOp::Add,
                lhs: *lhs,
                rhs: *rhs,
                kind: binary_op_rejection_kind(error),
            });
        }
    }
    rejections
}

fn binary_op_rejection_kind(
    error: ExactNumericUnificationError,
) -> ExactNumericBinaryOpRouteRejectionKind {
    match error {
        ExactNumericUnificationError::MixedExactAndDynamic {
            exact_source_name, ..
        } => ExactNumericBinaryOpRouteRejectionKind::MixedExactAndDynamic { exact_source_name },
        ExactNumericUnificationError::TypeMismatch {
            left_source_name,
            right_source_name,
            ..
        } => ExactNumericBinaryOpRouteRejectionKind::TypeMismatch {
            left_source_name,
            right_source_name,
        },
    }
}
