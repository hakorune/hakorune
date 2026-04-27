use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::helpers::apply_loop_final_values_to_bindings;
use super::recipe::NestedLoopRecipe;

const LOOP_SCAN_ERR: &str = "[normalizer] loop_scan_v0";

fn try_lower_loop_scan_v0_nested_stmt_only_plans(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    parts::entry::lower_nested_loop_recipe_stmt_only(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        nested,
        LOOP_SCAN_ERR,
    )
}

fn apply_loop_scan_v0_nested_stmt_only_plans(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    plans: &[LoweredRecipe],
) {
    for plan in plans {
        apply_loop_final_values_to_bindings(builder, current_bindings, plan);
    }
}

pub(in crate::mir::builder) fn try_lower_loop_scan_v0_nested_stmt_only(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    let Some(plans) = try_lower_loop_scan_v0_nested_stmt_only_plans(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        nested,
    )?
    else {
        return Ok(None);
    };
    apply_loop_scan_v0_nested_stmt_only_plans(builder, current_bindings, &plans);
    Ok(Some(plans))
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
    fn loop_scan_v0_nested_stmt_only_returns_none_without_stmt_only_recipe() {
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

        let result = try_lower_loop_scan_v0_nested_stmt_only_plans(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &nested,
        )
        .expect("missing stmt-only payload should not fail");

        assert!(result.is_none());
    }

    #[test]
    fn loop_scan_v0_nested_stmt_only_wrapper_returns_none_without_stmt_only_recipe() {
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

        let result = try_lower_loop_scan_v0_nested_stmt_only(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &nested,
        )
        .expect("missing stmt-only payload should not fail");

        assert!(result.is_none());
    }
}
