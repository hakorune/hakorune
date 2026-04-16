use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::recipes::loop_scan_phi_vars_v0::NestedLoopRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::helpers::apply_loop_final_values_to_bindings;

const LOOP_SCAN_PHI_VARS_ERR: &str = "[normalizer] loop_scan_phi_vars_v0";

fn try_lower_loop_scan_phi_vars_nested_loop_fastpath_plan(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<LoweredRecipe> {
    lower_nested_loop_depth1_any(builder, condition, body, LOOP_SCAN_PHI_VARS_ERR).ok()
}

pub(in crate::mir::builder) fn try_lower_loop_scan_phi_vars_nested_fastpath(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
) -> Option<Vec<LoweredRecipe>> {
    let plan = try_lower_loop_scan_phi_vars_nested_loop_fastpath_plan(
        builder,
        &nested.cond_view.tail_expr,
        nested.body.as_ref(),
    )?;
    apply_loop_final_values_to_bindings(builder, current_bindings, &plan);
    Some(vec![plan])
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
    fn loop_scan_phi_vars_nested_loop_fastpath_skips_when_shape_is_unsupported() {
        let mut builder = MirBuilder::new();
        let condition = less_cond();
        let body = vec![ASTNode::Assignment {
            target: Box::new(var("x")),
            value: Box::new(int(1)),
            span: span(),
        }];

        let result =
            try_lower_loop_scan_phi_vars_nested_loop_fastpath_plan(&mut builder, &condition, &body);

        assert!(result.is_none());
    }

    #[test]
    fn loop_scan_phi_vars_nested_fastpath_wrapper_returns_none_when_shape_is_unsupported() {
        let mut builder = MirBuilder::new();
        let mut current_bindings = BTreeMap::new();
        let condition = less_cond();
        let body = vec![ASTNode::Assignment {
            target: Box::new(var("x")),
            value: Box::new(int(1)),
            span: span(),
        }];
        let nested = NestedLoopRecipe {
            cond_view: crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView::from_expr(&condition),
            loop_stmt: ASTNode::Loop {
                condition: Box::new(condition.clone()),
                body: body.clone(),
                span: span(),
            },
            body: crate::mir::builder::control_flow::recipes::RecipeBody::new(body),
            body_stmt_only: None,
        };

        let result = try_lower_loop_scan_phi_vars_nested_fastpath(
            &mut builder,
            &mut current_bindings,
            &nested,
        );

        assert!(result.is_none());
    }
}
