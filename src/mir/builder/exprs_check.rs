use crate::ast::{ASTNode, CheckItem};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};

use super::{MirInstruction, ValueId};

mod select_type;

use select_type::PreparedCheckSelectIntegerTypeV1;

impl super::MirBuilder {
    /// Lower check items while retaining the caller's raw child-descent port.
    pub(in crate::mir::builder) fn build_check_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        items: Vec<CheckItem>,
    ) -> Result<ValueId, String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        let one = crate::mir::builder::emission::constant::emit_integer(self, 1)?;
        let zero = crate::mir::builder::emission::constant::emit_integer(self, 0)?;
        let mut ok = one;

        for item in items {
            let condition = drive_legacy_expression_v1(self, port, item.expression)?;
            let dst = self.next_value_id();
            let prepared = PreparedCheckSelectIntegerTypeV1::prepare(
                self.function_state.type_ctx.get_type(dst),
            )
            .map_err(|error| error.to_string())?;
            self.emit_instruction(MirInstruction::Select {
                dst,
                cond: condition,
                then_val: ok,
                else_val: zero,
            })?;
            prepared.commit(dst, &mut self.function_state.type_ctx);
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
    use crate::mir::builder::recursive_child_lowering::{
        RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
    };

    fn boolean_item(value: bool) -> CheckItem {
        CheckItem {
            label: None,
            expression: ASTNode::Literal {
                value: LiteralValue::Bool(value),
                span: Span::unknown(),
            },
        }
    }

    fn integer_item(value: i64) -> CheckItem {
        CheckItem {
            label: Some(format!("item-{value}")),
            expression: ASTNode::Literal {
                value: LiteralValue::Integer(value),
                span: Span::unknown(),
            },
        }
    }

    #[derive(Default)]
    struct RecordingCheckPortV1 {
        seen: Vec<i64>,
        fail_at: Option<i64>,
    }

    impl RecursiveChildLoweringPortV1 for RecordingCheckPortV1 {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            Err("check test port received a body".to_owned())
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            Err("check test port received a statement".to_owned())
        }

        fn lower_expression(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            let ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } = input
            else {
                return Err("check test port expected an Integer marker".to_owned());
            };
            self.seen.push(value);
            if self.fail_at == Some(value) {
                return Err(format!("check item {value} failed"));
            }
            crate::mir::builder::emission::constant::emit_integer(builder, value)
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

    fn lower_legacy_check(
        builder: &mut MirBuilder,
        items: Vec<CheckItem>,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        builder.build_check_expression_with_port_v1(&mut port, items)
    }

    #[test]
    fn empty_check_returns_the_existing_const_integer_without_a_select() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("check_empty/0".to_string());

        let result = lower_legacy_check(&mut builder, vec![]).unwrap();

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

        let result =
            lower_legacy_check(&mut builder, vec![boolean_item(true), boolean_item(false)])
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
        let result = lower_legacy_check(&mut builder, vec![boolean_item(true)]).unwrap();

        let finalized = builder.finalize_function_draft(false).unwrap();
        assert_eq!(
            finalized.metadata.value_types.get(&result),
            Some(&MirType::Integer)
        );
    }

    #[test]
    fn selected_port_consumes_check_items_once_in_source_order() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("check_port_order/0".to_owned());
        let mut port = RecordingCheckPortV1::default();

        let result = builder
            .build_check_expression_with_port_v1(
                &mut port,
                vec![integer_item(1), integer_item(2), integer_item(3)],
            )
            .unwrap();

        assert_eq!(port.seen, vec![1, 2, 3]);
        let selects = select_destinations(&builder);
        assert_eq!(selects.len(), 3);
        assert_eq!(selects.last(), Some(&result));
    }

    #[test]
    fn failed_item_stops_before_later_children_without_route_retry() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("check_port_failure/0".to_owned());
        let mut port = RecordingCheckPortV1 {
            fail_at: Some(2),
            ..RecordingCheckPortV1::default()
        };

        let error = builder
            .build_check_expression_with_port_v1(
                &mut port,
                vec![integer_item(1), integer_item(2), integer_item(3)],
            )
            .unwrap_err();

        assert_eq!(error, "check item 2 failed");
        assert_eq!(port.seen, vec![1, 2]);
        assert_eq!(select_destinations(&builder).len(), 1);
    }
}
