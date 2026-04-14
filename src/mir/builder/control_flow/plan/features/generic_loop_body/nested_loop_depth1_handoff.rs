use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(super) fn try_lower_generic_nested_loop_depth1_fastpath(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    error_prefix: &str,
) -> Option<LoweredRecipe> {
    lower_nested_loop_depth1_any(builder, condition, body, error_prefix).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn less_cond() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var("i")),
            right: Box::new(int(10)),
            span: span(),
        }
    }

    #[test]
    fn generic_nested_loop_depth1_fastpath_skips_when_shape_is_unsupported() {
        let mut builder = MirBuilder::new();
        let condition = less_cond();
        let body = vec![ASTNode::Assignment {
            target: Box::new(var("x")),
            value: Box::new(int(1)),
            span: span(),
        }];

        let result = try_lower_generic_nested_loop_depth1_fastpath(
            &mut builder,
            &condition,
            &body,
            "[test] generic nested loop",
        );

        assert!(result.is_none());
    }
}
