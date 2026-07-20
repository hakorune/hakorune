use crate::ast::CheckItem;

use super::{MirInstruction, MirType, ValueId};

mod select_type;

impl super::MirBuilder {
    pub(super) fn build_check_expression(
        &mut self,
        items: Vec<CheckItem>,
    ) -> Result<ValueId, String> {
        let one = crate::mir::builder::emission::constant::emit_integer(self, 1)?;
        let zero = crate::mir::builder::emission::constant::emit_integer(self, 0)?;
        let mut ok = one;

        for item in items {
            let condition = self.build_expression_impl(item.expression)?;
            let dst = self.next_value_id();
            self.emit_instruction(MirInstruction::Select {
                dst,
                cond: condition,
                then_val: ok,
                else_val: zero,
            })?;
            self.function_state
                .type_ctx
                .value_types
                .insert(dst, MirType::Integer);
            ok = dst;
        }

        Ok(ok)
    }
}

#[cfg(test)]
mod tests {
    use super::super::MirBuilder;
    use super::super::{MirInstruction, MirType, ValueId};
    use crate::ast::{ASTNode, CheckItem, LiteralValue, Span};

    fn boolean_item(value: bool) -> CheckItem {
        CheckItem {
            label: None,
            expression: ASTNode::Literal {
                value: LiteralValue::Bool(value),
                span: Span::unknown(),
            },
        }
    }

    fn select_destinations(builder: &MirBuilder) -> Vec<ValueId> {
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|instruction| match instruction {
                MirInstruction::Select { dst, .. } => Some(*dst),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn empty_check_returns_the_existing_const_integer_without_a_select() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("check_empty/0".to_string());

        let result = builder.build_check_expression(vec![]).unwrap();

        assert_eq!(
            builder.function_state.type_ctx.get_type(result),
            Some(&MirType::Integer)
        );
        assert!(select_destinations(&builder).is_empty());
    }

    #[test]
    fn every_successful_check_select_keeps_the_integer_accumulator_induction() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("check_multiple/0".to_string());

        let result = builder
            .build_check_expression(vec![boolean_item(true), boolean_item(false)])
            .unwrap();
        let selects = select_destinations(&builder);

        assert_eq!(selects.len(), 2);
        assert_eq!(selects.last(), Some(&result));
        for destination in selects {
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                Some(&MirType::Integer)
            );
        }
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .metadata
                .value_types
                .get(&result),
            None,
            "CheckExpr lowering must not use finalized metadata as a live type authority"
        );
    }

    #[test]
    fn failed_select_emission_leaves_its_destination_without_a_type_fact() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("check_select_failure/0".to_string());
        let destination = builder.alloc_value_for_test();
        builder.function_state.current_block = None;

        assert!(builder
            .emit_for_test(MirInstruction::Select {
                dst: destination,
                cond: ValueId::new(1),
                then_val: ValueId::new(2),
                else_val: ValueId::new(3),
            })
            .is_err());
        assert_eq!(builder.function_state.type_ctx.get_type(destination), None);
    }

    #[test]
    fn normal_finalization_snapshots_the_transient_check_select_fact() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("check_finalize/0".to_string());
        let result = builder
            .build_check_expression(vec![boolean_item(true)])
            .unwrap();

        let finalized = builder.finalize_function_draft(false).unwrap();
        assert_eq!(
            finalized.metadata.value_types.get(&result),
            Some(&MirType::Integer)
        );
    }
}
