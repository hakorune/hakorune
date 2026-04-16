use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::recipes::loop_scan_phi_vars_v0::NestedLoopRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::nested_loop_recipe_handoff::lower_loop_scan_phi_vars_nested_loop_recipe;

pub(in crate::mir::builder) fn lower_loop_scan_phi_vars_nested_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_loop_scan_phi_vars_nested_loop_recipe(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        nested,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::recipes::RecipeBody;

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
    fn loop_scan_phi_vars_nested_segment_propagates_fastpath_reject() {
        let mut builder = MirBuilder::new();
        let mut current_bindings = BTreeMap::new();
        let carrier_step_phis = BTreeMap::new();
        let break_phi_dsts = BTreeMap::new();
        let condition = less_cond();
        let body = vec![ASTNode::Assignment {
            target: Box::new(var("x")),
            value: Box::new(int(1)),
            span: span(),
        }];
        let nested = NestedLoopRecipe {
            cond_view: CondBlockView::from_expr(&condition),
            loop_stmt: ASTNode::Loop {
                condition: Box::new(condition.clone()),
                body: body.clone(),
                span: span(),
            },
            body: RecipeBody::new(body),
            body_stmt_only: None,
        };

        let err = lower_loop_scan_phi_vars_nested_segment(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &nested,
        )
        .expect_err("unsupported nested segment should reject");

        assert!(err.contains("nested loop fastpath rejected"));
    }
}
