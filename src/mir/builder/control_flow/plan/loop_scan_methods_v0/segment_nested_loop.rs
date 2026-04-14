use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::nested_loop_handoff::lower_loop_scan_methods_nested_loop_fallback;
use super::recipe::NestedLoopRecipe;

const LOOP_SCAN_METHODS_ERR: &str = "[normalizer] loop_scan_methods_v0";

fn apply_loop_final_values_to_bindings(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    plan: &LoweredRecipe,
) {
    let CorePlan::Loop(loop_plan) = plan else {
        return;
    };
    for (name, value_id) in &loop_plan.final_values {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
        if current_bindings.contains_key(name) {
            current_bindings.insert(name.clone(), *value_id);
        }
    }
}

pub(in crate::mir::builder) fn lower_loop_scan_methods_nested_segment(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
    ctx: &LoopRouteContext,
) -> Result<Vec<LoweredRecipe>, String> {
    if let Some(plans) = crate::mir::builder::control_flow::plan::parts::entry::lower_nested_loop_recipe_stmt_only(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        nested,
        LOOP_SCAN_METHODS_ERR,
    )? {
        for plan in &plans {
            apply_loop_final_values_to_bindings(builder, current_bindings, plan);
        }
        return Ok(plans);
    }

    let plan = lower_loop_scan_methods_nested_loop_fallback(
        builder,
        &nested.cond_view.tail_expr,
        &nested.body.body,
        ctx,
        LOOP_SCAN_METHODS_ERR,
    )?;
    apply_loop_final_values_to_bindings(builder, current_bindings, &plan);
    Ok(vec![plan])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

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
    fn loop_scan_methods_nested_segment_rejects_when_stmt_only_and_fallback_miss() {
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
            body: RecipeBody::new(body.clone()),
            body_stmt_only: None,
        };
        let ctx = LoopRouteContext::new(&condition, &body, "test", false, false);

        let err = lower_loop_scan_methods_nested_segment(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &nested,
            &ctx,
        )
        .expect_err("unsupported nested segment should reject");

        assert!(!err.is_empty());
    }
}
