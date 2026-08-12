//! Exact-once V2 Recipe operation cursor.
//!
//! This cursor is a physical preflight boundary.  The verified V2 operation
//! rows remain the only order authority; the cursor does not issue semantic
//! facts, choose blocks, or create a second value/SSA owner.  Its current
//! bounded responsibility is to prove that the complete row array can be
//! consumed in dependency order before the unpublished Builder session is
//! opened.  CallOut emission and typed physical receipts remain the next
//! corridor step.

use std::collections::BTreeSet;

use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicAPrimeI64SourceRelationViewV1, DynamicFullLoopOperationPhysicalRefV2,
    DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2,
};
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{CoreMethodEffectV1, CoreMethodResultKindV1};
use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopOperationExecutionClassV2, LoopOperationV2, LoopValueKeyV1,
};

const V0: LoopValueKeyV1 = LoopValueKeyV1::new(0);
const V1: LoopValueKeyV1 = LoopValueKeyV1::new(1);
const V2: LoopValueKeyV1 = LoopValueKeyV1::new(2);
const V3: LoopValueKeyV1 = LoopValueKeyV1::new(3);
const V4: LoopValueKeyV1 = LoopValueKeyV1::new(4);
const V5: LoopValueKeyV1 = LoopValueKeyV1::new(5);
const V6: LoopValueKeyV1 = LoopValueKeyV1::new(6);
const V7: LoopValueKeyV1 = LoopValueKeyV1::new(7);
const V8: LoopValueKeyV1 = LoopValueKeyV1::new(8);
const V9: LoopValueKeyV1 = LoopValueKeyV1::new(9);
const V10: LoopValueKeyV1 = LoopValueKeyV1::new(10);
const V11: LoopValueKeyV1 = LoopValueKeyV1::new(11);
const V12: LoopValueKeyV1 = LoopValueKeyV1::new(12);
const V13: LoopValueKeyV1 = LoopValueKeyV1::new(13);
const V14: LoopValueKeyV1 = LoopValueKeyV1::new(14);
const V15: LoopValueKeyV1 = LoopValueKeyV1::new(15);
const V16: LoopValueKeyV1 = LoopValueKeyV1::new(16);
const V17: LoopValueKeyV1 = LoopValueKeyV1::new(17);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicV2RecipeOperationCursorRejectV1 {
    RowCount,
    DuplicateItem,
    UseBeforeProduce(LoopValueKeyV1),
    DuplicateResult(LoopValueKeyV1),
    BindingDrift,
    OperationShape,
    CallShape,
    CoreMethodShape,
    MissingCallRole,
    MissingRequiredShape,
}

#[derive(Debug, Default)]
struct ProducedValuesV1 {
    values: BTreeSet<LoopValueKeyV1>,
}

impl ProducedValuesV1 {
    fn seeded() -> Self {
        Self {
            values: [V0, V1, V2, V3].into_iter().collect(),
        }
    }

    fn require(&self, value: LoopValueKeyV1) -> Result<(), DynamicV2RecipeOperationCursorRejectV1> {
        self.values.contains(&value).then_some(()).ok_or(
            DynamicV2RecipeOperationCursorRejectV1::UseBeforeProduce(value),
        )
    }

    fn publish(
        &mut self,
        value: LoopValueKeyV1,
    ) -> Result<(), DynamicV2RecipeOperationCursorRejectV1> {
        if !self.values.insert(value) {
            return Err(DynamicV2RecipeOperationCursorRejectV1::DuplicateResult(
                value,
            ));
        }
        Ok(())
    }
}

/// Move-only cursor over the complete, already verified V2 operation array.
/// No caller can supply a physical value id or split the row array into another cursor.
#[derive(Debug)]
pub(super) struct DynamicV2RecipeOperationCursorV1<'program> {
    rows: &'program [DynamicFullLoopOperationPhysicalRefV2<'program>;
                  DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
    next: usize,
    seen_items: BTreeSet<crate::mir::loop_recipe_contract::LoopItemKeyV1>,
    produced: ProducedValuesV1,
    read_count: usize,
    const_one_count: usize,
    const_zero_count: usize,
    binary_count: usize,
    compare_count: usize,
    call_count: usize,
    write_count: usize,
    saw_substring: bool,
    saw_index_of: bool,
    saw_write: bool,
}

pub(super) fn validate(
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
) -> Result<(), DynamicV2RecipeOperationCursorRejectV1> {
    demand.with_operation_program(|program| {
        DynamicV2RecipeOperationCursorV1::new(program.operation_rows())
            .consume_all(demand.source_relation())
    })
}

impl<'program> DynamicV2RecipeOperationCursorV1<'program> {
    pub(super) fn new(
        rows: &'program [DynamicFullLoopOperationPhysicalRefV2<'program>;
                      DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
    ) -> Self {
        Self {
            rows,
            next: 0,
            seen_items: BTreeSet::new(),
            produced: ProducedValuesV1::seeded(),
            read_count: 0,
            const_one_count: 0,
            const_zero_count: 0,
            binary_count: 0,
            compare_count: 0,
            call_count: 0,
            write_count: 0,
            saw_substring: false,
            saw_index_of: false,
            saw_write: false,
        }
    }

    /// Consume every row once.  The returned unit is deliberately not a
    /// durable semantic receipt; the session will later issue physical
    /// physical value ids through the canonical owner after the CallOut corridor is
    /// closed.
    pub(super) fn consume_all(
        mut self,
        relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
    ) -> Result<(), DynamicV2RecipeOperationCursorRejectV1> {
        if self.rows.len() != DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2 {
            return Err(DynamicV2RecipeOperationCursorRejectV1::RowCount);
        }
        let formal = relation.formal_rows();
        let src_value = formal[0].recipe_value();
        let pred_chars_value = formal[3].recipe_value();
        let induction_key = relation.induction_key();

        while self.next < self.rows.len() {
            let row = &self.rows[self.next];
            self.next += 1;
            if !self.seen_items.insert(row.item()) {
                return Err(DynamicV2RecipeOperationCursorRejectV1::DuplicateItem);
            }
            self.consume_row(row, src_value, pred_chars_value, induction_key)?;
        }

        if self.read_count != 5
            || self.const_one_count != 2
            || self.const_zero_count != 1
            || self.binary_count != 2
            || self.compare_count != 2
            || self.call_count != 2
            || self.write_count != 1
            || !self.saw_substring
            || !self.saw_index_of
            || !self.saw_write
            || self.produced.values
                != [
                    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17,
                ]
                .into_iter()
                .collect()
        {
            return Err(DynamicV2RecipeOperationCursorRejectV1::MissingRequiredShape);
        }
        Ok(())
    }

    fn consume_row(
        &mut self,
        row: &DynamicFullLoopOperationPhysicalRefV2<'_>,
        src_value: LoopValueKeyV1,
        pred_chars_value: LoopValueKeyV1,
        induction_key: LoopBindingKeyV1,
    ) -> Result<(), DynamicV2RecipeOperationCursorRejectV1> {
        match row.operation() {
            LoopOperationV2::ReadBinding { binding, result } => {
                if *binding != induction_key {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::BindingDrift);
                }
                self.read_count += 1;
                self.produced.publish(*result)?;
            }
            LoopOperationV2::ConstI64 { result, value } => {
                match *value {
                    0 => self.const_zero_count += 1,
                    1 => self.const_one_count += 1,
                    _ => return Err(DynamicV2RecipeOperationCursorRejectV1::OperationShape),
                }
                self.produced.publish(*result)?;
            }
            LoopOperationV2::BinaryI64 {
                op,
                left,
                right,
                result,
            } => {
                if *op != crate::mir::loop_recipe_contract::LoopBinaryI64OpV2::Add {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::OperationShape);
                }
                self.produced.require(*left)?;
                self.produced.require(*right)?;
                self.binary_count += 1;
                self.produced.publish(*result)?;
            }
            LoopOperationV2::CompareI64 {
                op,
                left,
                right,
                result,
            } => {
                if *op != crate::mir::loop_recipe_contract::LoopCompareI64OpV2::Less {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::OperationShape);
                }
                self.produced.require(*left)?;
                self.produced.require(*right)?;
                self.compare_count += 1;
                self.produced.publish(*result)?;
            }
            LoopOperationV2::CallSlot {
                receiver,
                args,
                result,
            } => {
                let Some(receiver) = receiver else {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::CallShape);
                };
                let Some(result) = result else {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::CallShape);
                };
                self.produced.require(*receiver)?;
                for arg in args {
                    self.produced.require(*arg)?;
                }
                self.verify_call(row, *receiver, args, *result, src_value, pred_chars_value)?;
                self.call_count += 1;
                self.produced.publish(*result)?;
            }
            LoopOperationV2::WriteBinding { binding, value } => {
                if *binding != induction_key {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::BindingDrift);
                }
                self.produced.require(*value)?;
                self.write_count += 1;
                self.saw_write = true;
            }
            LoopOperationV2::DynamicAdd { .. }
            | LoopOperationV2::DynamicLess { .. }
            | LoopOperationV2::TextEq { .. } => {
                return Err(DynamicV2RecipeOperationCursorRejectV1::OperationShape)
            }
        }
        Ok(())
    }

    fn verify_call(
        &mut self,
        row: &DynamicFullLoopOperationPhysicalRefV2<'_>,
        receiver: LoopValueKeyV1,
        args: &[LoopValueKeyV1],
        result: LoopValueKeyV1,
        src_value: LoopValueKeyV1,
        pred_chars_value: LoopValueKeyV1,
    ) -> Result<(), DynamicV2RecipeOperationCursorRejectV1> {
        let Some(role) = row.call_role() else {
            return Err(DynamicV2RecipeOperationCursorRejectV1::MissingCallRole);
        };
        let Some(core) = row.core_method() else {
            return Err(DynamicV2RecipeOperationCursorRejectV1::CoreMethodShape);
        };
        if core.effect != CoreMethodEffectV1::PureRead {
            return Err(DynamicV2RecipeOperationCursorRejectV1::CoreMethodShape);
        }
        match role {
            DynamicFullBodySourceRoleV1::SubstringCall => {
                if self.saw_substring
                    || receiver != src_value
                    || args.len() != 2
                    || result != V10
                    || core.op != CoreMethodOp::StringSubstring
                    || core.result_kind != CoreMethodResultKindV1::StringValue
                    || row.execution()
                        != (LoopOperationExecutionClassV2::ExternallyBoundOutcome {
                            normal_result: Some(V10),
                        })
                {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::CallShape);
                }
                self.saw_substring = true;
            }
            DynamicFullBodySourceRoleV1::IndexOfCall => {
                if self.saw_index_of
                    || receiver != pred_chars_value
                    || args.as_ref() != [V10]
                    || result != V11
                    || core.op != CoreMethodOp::StringIndexOf
                    || core.result_kind != CoreMethodResultKindV1::I64Value
                    || row.execution()
                        != (LoopOperationExecutionClassV2::ExternallyBoundOutcome {
                            normal_result: Some(V11),
                        })
                {
                    return Err(DynamicV2RecipeOperationCursorRejectV1::CallShape);
                }
                self.saw_index_of = true;
            }
            _ => return Err(DynamicV2RecipeOperationCursorRejectV1::CallShape),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DynamicV2RecipeOperationCursorRejectV1, ProducedValuesV1};
    use crate::mir::loop_recipe_contract::LoopValueKeyV1;

    #[test]
    fn produced_value_cursor_rejects_use_before_definition() {
        let values = ProducedValuesV1::seeded();
        assert_eq!(
            values.require(LoopValueKeyV1::new(17)),
            Err(DynamicV2RecipeOperationCursorRejectV1::UseBeforeProduce(
                LoopValueKeyV1::new(17)
            ))
        );
    }

    #[test]
    fn produced_value_cursor_rejects_duplicate_definition() {
        let mut values = ProducedValuesV1::seeded();
        values
            .publish(LoopValueKeyV1::new(4))
            .expect("first definition");
        assert_eq!(
            values.publish(LoopValueKeyV1::new(4)),
            Err(DynamicV2RecipeOperationCursorRejectV1::DuplicateResult(
                LoopValueKeyV1::new(4)
            ))
        );
    }
}
