//! Prepared raw BlockExpr admission and ordered child descent.
//!
//! Admission observes the complete prelude before Builder effects. Lowering
//! consumes only the sealed prelude and tail through the caller's child port.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, drive_legacy_statement_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::{MirBuilder, ValueId};

const PRELUDE_EXIT_ERROR: &str =
    "[freeze:contract][blockexpr] exit stmt is forbidden in BlockExpr prelude";

#[derive(Debug)]
pub(super) struct PreparedRawBlockExprV1 {
    prelude: Vec<ASTNode>,
    tail: ASTNode,
}

impl PreparedRawBlockExprV1 {
    pub(super) fn prepare(prelude: Vec<ASTNode>, tail: ASTNode) -> Result<Self, String> {
        if prelude
            .iter()
            .any(ASTNode::contains_non_local_exit_outside_loops)
        {
            return Err(PRELUDE_EXIT_ERROR.to_owned());
        }
        Ok(Self { prelude, tail })
    }
}

pub(super) fn lower_prepared_raw_block_expr_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawBlockExprV1,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<StatementInput = ASTNode, ExpressionInput = ASTNode>,
{
    for statement in prepared.prelude {
        let _ = drive_legacy_statement_v1(builder, port, statement)?;
    }
    drive_legacy_expression_v1(builder, port, prepared.tail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;

    #[derive(Default)]
    struct RecordingBlockExprPortV1 {
        demands: Vec<String>,
        fail_statement: Option<i64>,
        fail_tail: bool,
    }

    impl RecursiveChildLoweringPortV1 for RecordingBlockExprPortV1 {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            Err("BlockExpr test port does not lower bodies".to_owned())
        }

        fn lower_statement(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            let value = integer_value(&input)?;
            self.demands.push(format!("stmt:{value}"));
            if self.fail_statement == Some(value) {
                return Err(format!("statement {value} failed"));
            }
            crate::mir::builder::emission::constant::emit_integer(builder, value)
        }

        fn lower_expression(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            let value = integer_value(&input)?;
            self.demands.push(format!("tail:{value}"));
            if self.fail_tail {
                return Err("tail failed".to_owned());
            }
            crate::mir::builder::emission::constant::emit_integer(builder, value)
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn integer_value(node: &ASTNode) -> Result<i64, String> {
        match node {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } => Ok(*value),
            other => Err(format!(
                "unexpected BlockExpr test node: {}",
                other.node_type()
            )),
        }
    }

    fn break_node() -> ASTNode {
        ASTNode::Break {
            span: Span::unknown(),
        }
    }

    fn continue_node() -> ASTNode {
        ASTNode::Continue {
            span: Span::unknown(),
        }
    }

    #[test]
    fn prepare_rejects_each_escaping_prelude_exit_with_exact_diagnostic() {
        let exits = [
            ASTNode::Return {
                value: Some(Box::new(integer(1))),
                span: Span::unknown(),
            },
            ASTNode::Throw {
                expression: Box::new(integer(1)),
                span: Span::unknown(),
            },
            break_node(),
            continue_node(),
        ];

        for exit in exits {
            assert_eq!(
                PreparedRawBlockExprV1::prepare(vec![exit], integer(9)).unwrap_err(),
                PRELUDE_EXIT_ERROR
            );
        }
    }

    #[test]
    fn prepare_preserves_loop_owned_break_and_continue() {
        let loop_node = ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: vec![continue_node(), break_node()],
            span: Span::unknown(),
        };

        PreparedRawBlockExprV1::prepare(vec![loop_node], integer(9)).unwrap();
    }

    #[test]
    fn lower_consumes_prelude_in_order_then_tail_once() {
        let prepared =
            PreparedRawBlockExprV1::prepare(vec![integer(1), integer(2)], integer(3)).unwrap();
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("prepared_block_expr_order/0".to_owned());
        let mut port = RecordingBlockExprPortV1::default();

        lower_prepared_raw_block_expr_with_port_v1(&mut builder, &mut port, prepared).unwrap();

        assert_eq!(port.demands, ["stmt:1", "stmt:2", "tail:3"]);
    }

    #[test]
    fn lower_stops_after_first_failed_prelude_statement() {
        let prepared =
            PreparedRawBlockExprV1::prepare(vec![integer(1), integer(2)], integer(3)).unwrap();
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("prepared_block_expr_failure/0".to_owned());
        let mut port = RecordingBlockExprPortV1 {
            fail_statement: Some(2),
            ..Default::default()
        };

        assert_eq!(
            lower_prepared_raw_block_expr_with_port_v1(&mut builder, &mut port, prepared)
                .unwrap_err(),
            "statement 2 failed"
        );
        assert_eq!(port.demands, ["stmt:1", "stmt:2"]);
    }

    #[test]
    fn empty_prelude_still_lowers_tail_exactly_once() {
        let prepared = PreparedRawBlockExprV1::prepare(Vec::new(), integer(3)).unwrap();
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("prepared_block_expr_empty/0".to_owned());
        let mut port = RecordingBlockExprPortV1::default();

        lower_prepared_raw_block_expr_with_port_v1(&mut builder, &mut port, prepared).unwrap();

        assert_eq!(port.demands, ["tail:3"]);
    }

    #[test]
    fn tail_failure_occurs_after_the_complete_prelude_once() {
        let prepared =
            PreparedRawBlockExprV1::prepare(vec![integer(1), integer(2)], integer(3)).unwrap();
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("prepared_block_expr_tail_failure/0".to_owned());
        let mut port = RecordingBlockExprPortV1 {
            fail_tail: true,
            ..Default::default()
        };

        assert_eq!(
            lower_prepared_raw_block_expr_with_port_v1(&mut builder, &mut port, prepared)
                .unwrap_err(),
            "tail failed"
        );
        assert_eq!(port.demands, ["stmt:1", "stmt:2", "tail:3"]);
    }
}
