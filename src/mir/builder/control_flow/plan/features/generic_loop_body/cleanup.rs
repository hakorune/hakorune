//! Route-local cleanup for `GenericLoopV1`.
//!
//! Scope:
//! - append the synthetic fallthrough `ContinueWithPhiArgs` only when the body
//!   does not already exit on all paths

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

use super::body_plans_exit_on_all_paths;

pub(in crate::mir::builder) fn apply_generic_loop_v1_fallthrough_cleanup(
    builder: &mut MirBuilder,
    body_plans: &mut Vec<LoweredRecipe>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    current_bindings: &BTreeMap<String, ValueId>,
    loop_var: &str,
    loop_increment: &ASTNode,
    error_prefix: &str,
) -> Result<(), String> {
    if body_plans_exit_on_all_paths(body_plans) {
        return Ok(());
    }

    let mut fallthrough_bindings = current_bindings.clone();
    let (loop_var_next, effects) =
        PlanNormalizer::lower_value_ast(loop_increment, builder, &fallthrough_bindings)?;
    body_plans.extend(effects_to_plans(effects));
    fallthrough_bindings.insert(loop_var.to_string(), loop_var_next);

    let exit = crate::mir::builder::control_flow::plan::parts::exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        &fallthrough_bindings,
        error_prefix,
    )?;
    body_plans.push(CorePlan::Exit(exit));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreExitPlan};
    use crate::mir::MirType;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn inc_expr(name: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v(name)),
            right: Box::new(lit(1)),
            span: Span::unknown(),
        }
    }

    #[test]
    fn generic_loop_v1_cleanup_appends_fallthrough_continue() {
        let mut builder = MirBuilder::new();
        let step_phi = builder.alloc_typed(MirType::Integer);
        let current_value = builder.alloc_typed(MirType::Integer);
        let mut body_plans = vec![CorePlan::Effect(CoreEffectPlan::Copy {
            dst: current_value,
            src: current_value,
        })];

        apply_generic_loop_v1_fallthrough_cleanup(
            &mut builder,
            &mut body_plans,
            &BTreeMap::from([("i".to_string(), step_phi)]),
            &BTreeMap::from([("i".to_string(), current_value)]),
            "i",
            &inc_expr("i"),
            "generic_loop_v1",
        )
        .expect("cleanup should append continue");

        assert!(matches!(
            body_plans.last(),
            Some(CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { .. }))
        ));
    }

    #[test]
    fn generic_loop_v1_cleanup_skips_when_body_already_exits() {
        let mut builder = MirBuilder::new();
        let step_phi = builder.alloc_typed(MirType::Integer);
        let current_value = builder.alloc_typed(MirType::Integer);
        let mut body_plans = vec![CorePlan::Exit(CoreExitPlan::Break(1))];

        apply_generic_loop_v1_fallthrough_cleanup(
            &mut builder,
            &mut body_plans,
            &BTreeMap::from([("i".to_string(), step_phi)]),
            &BTreeMap::from([("i".to_string(), current_value)]),
            "i",
            &inc_expr("i"),
            "generic_loop_v1",
        )
        .expect("cleanup should keep terminal body unchanged");

        assert!(matches!(
            body_plans.as_slice(),
            [CorePlan::Exit(CoreExitPlan::Break(1))]
        ));
    }

    #[test]
    fn generic_loop_v1_cleanup_respects_nested_terminality() {
        let mut builder = MirBuilder::new();
        let step_phi = builder.alloc_typed(MirType::Integer);
        let current_value = builder.alloc_typed(MirType::Integer);
        let mut body_plans = vec![CorePlan::Seq(vec![CorePlan::Exit(CoreExitPlan::Return(
            Some(current_value),
        ))])];

        apply_generic_loop_v1_fallthrough_cleanup(
            &mut builder,
            &mut body_plans,
            &BTreeMap::from([("i".to_string(), step_phi)]),
            &BTreeMap::from([("i".to_string(), current_value)]),
            "i",
            &inc_expr("i"),
            "generic_loop_v1",
        )
        .expect("nested terminal body should not append continue");

        assert!(matches!(
            body_plans.as_slice(),
            [CorePlan::Seq(inner)] if matches!(inner.as_slice(), [CorePlan::Exit(CoreExitPlan::Return(Some(_)))])
        ));
    }
}
