//! Utility functions for item/statement lowering.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering_associated_input::{
    lower_assignment_inputs, lower_function_call_statement_input, lower_local_statement_input,
    lower_method_call_statement_input,
};
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::parts::var_map_scope::publish_emission_cache;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, LoopPlanExpressionPortV1, LoweredRecipe,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{Effect, EffectMask, ValueId};
use std::collections::BTreeMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum DirectExitRejectReason {
    BlockContainsDirectExit,
    ExitMustBeInsideIf,
    BreakMustBeLast,
    ReturnMustBeLast,
}

const DIRECT_EXIT_REJECT_TAG: &str = "[loop_cond/direct_exit]";

fn direct_exit_reason_text(reason: DirectExitRejectReason) -> &'static str {
    match reason {
        DirectExitRejectReason::BlockContainsDirectExit => "block contains direct exit",
        DirectExitRejectReason::ExitMustBeInsideIf => "exit must be inside if",
        DirectExitRejectReason::BreakMustBeLast => "break must be last",
        DirectExitRejectReason::ReturnMustBeLast => "return must be last",
    }
}

pub(in crate::mir::builder) fn direct_exit_reject(
    error_prefix: &str,
    reason: DirectExitRejectReason,
) -> String {
    format!(
        "{error_prefix}: {DIRECT_EXIT_REJECT_TAG} {}",
        direct_exit_reason_text(reason)
    )
}

pub(in crate::mir::builder) fn is_direct_exit_reject(err: &str) -> bool {
    if err.contains(DIRECT_EXIT_REJECT_TAG) {
        return true;
    }
    // Backward-compatible fallback for legacy message shapes.
    err.contains(": block contains direct exit")
        || err.contains(": exit must be inside if")
        || err.contains(": break must be last")
        || err.contains(": return must be last")
}

/// Lower the simple statement subset through a located expression port.
///
/// This is the shared entry for the source-aware LoopCond physical path. The
/// raw facade above uses the same owner with `RawLoopPlanExpressionPortV1`,
/// while block-expression loop preludes stay in the existing specialized
/// owner until their nested source contexts are co-sealed.
pub(in crate::mir::builder) fn lower_simple_effect_stmt_input<'input, P>(
    port: &P,
    statement: P::StmtInput<'input>,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_updates: &mut BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    match port.stmt_syntax(&statement) {
        ASTNode::Assignment { .. } => {
            for (name, value_id) in current_bindings.iter() {
                publish_emission_cache(builder, name.clone(), *value_id);
            }
            let target = port
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentTarget)
                .map_err(|error| error.render())?;
            let value = port
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .map_err(|error| error.render())?;
            let (binding, effects) = lower_assignment_inputs(
                port,
                target,
                value,
                builder,
                current_bindings,
                error_prefix,
            )?;
            if let Some((name, value_id)) = binding {
                if carrier_phis.contains_key(&name) {
                    carrier_updates.insert(name.clone(), value_id);
                }
                if carrier_phis.contains_key(&name) || current_bindings.contains_key(&name) {
                    current_bindings.insert(name.clone(), value_id);
                }
                publish_emission_cache(builder, name, value_id);
            }
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Local { .. } => {
            let (inits, effects) = lower_local_statement_input(
                port,
                &statement,
                builder,
                current_bindings,
                error_prefix,
            )?;
            let values = inits.iter().map(|(_, value)| *value).collect::<Vec<_>>();
            let _ = port.exact_source_local_completion(&statement, &values)?;
            for (name, value_id) in inits {
                current_bindings.insert(name.clone(), value_id);
                publish_emission_cache(builder, name, value_id);
            }
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::MethodCall { .. } => {
            let input = port
                .statement_expr(&statement)
                .map_err(|error| error.render())?;
            let effects = lower_method_call_statement_input(
                port,
                input,
                builder,
                current_bindings,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::FunctionCall { .. } => {
            let input = port
                .statement_expr(&statement)
                .map_err(|error| error.render())?;
            let effects = lower_function_call_statement_input(
                port,
                input,
                builder,
                current_bindings,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Call { .. } => {
            let input = port
                .statement_expr(&statement)
                .map_err(|error| error.render())?;
            let (_value_id, effects) =
                PlanNormalizer::lower_value_input(port, input, builder, current_bindings)?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Print { .. } => {
            let expression = port
                .child_expr_from_stmt(&statement, ExprChildRoleV1::PrintValue)
                .map_err(|error| error.render())?;
            let source = port
                .call_source(&expression)
                .map_err(|error| error.render())?;
            let (value_id, mut effects) =
                PlanNormalizer::lower_value_input(port, expression, builder, current_bindings)?;
            effects.push(CoreEffectPlan::ExternCall {
                source,
                dst: None,
                iface_name: "env.console".to_string(),
                method_name: "log".to_string(),
                args: vec![value_id],
                effects: EffectMask::PURE.add(Effect::Io),
            });
            Ok(Some(effects_to_plans(effects)))
        }
        _ => Ok(None),
    }
}

/// Resolve one recipe statement reference through the already-issued body
/// input, then lower only the simple-statement subset through the same port.
///
/// The source-aware caller supplies the located body input; this helper never
/// scans a `RecipeBody`, reconstructs a source site, or falls back to a raw AST
/// lookup.  The raw loop-cond facade can use `RawLoopPlanExpressionPortV1`
/// when preserving legacy behavior.
pub(super) fn lower_simple_effect_stmt_body_input<'input, P>(
    port: &P,
    body: &P::BodyInput<'input>,
    stmt_ref: StmtRef,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_updates: &mut BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let statement = port
        .body_stmt(body, stmt_ref.index())
        .map_err(|error| error.render())?;
    lower_simple_effect_stmt_input(
        port,
        statement,
        builder,
        current_bindings,
        carrier_phis,
        carrier_updates,
        error_prefix,
    )
}

#[cfg(test)]
mod tests {
    use super::{direct_exit_reject, is_direct_exit_reject, DirectExitRejectReason};
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::recipe_tree::ExitKind;
    use crate::mir::builder::control_flow::plan::RawLoopPlanExpressionPortV1;
    use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
    use crate::mir::builder::control_flow::recipes::refs::StmtRef;
    use crate::mir::builder::MirBuilder;
    use crate::mir::ValueId;
    use std::collections::BTreeMap;

    #[test]
    fn direct_exit_reject_is_tagged_and_detectable() {
        let err = direct_exit_reject(
            "[normalizer] loop_cond_break_continue",
            DirectExitRejectReason::ExitMustBeInsideIf,
        );
        assert!(is_direct_exit_reject(&err));
        assert!(err.contains("[loop_cond/direct_exit]"));
    }

    #[test]
    fn direct_exit_reject_accepts_legacy_strings() {
        let legacy = "[normalizer] loop_cond_break_continue: break must be last";
        assert!(is_direct_exit_reject(legacy));
    }

    #[test]
    fn raw_body_stmt_ref_uses_port_input_without_ast_rescan() {
        let body = vec![ASTNode::FunctionCall {
            name: "Worker.run".to_string(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::Integer(7),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }];
        let port = RawLoopPlanExpressionPortV1::new();
        let body_input = body.as_slice();
        let mut builder = MirBuilder::new();
        let mut bindings = BTreeMap::<String, ValueId>::new();
        let result = super::super::loop_cond_bc_item::lower_loop_cond_item_input(
            &port,
            &body_input,
            &LoopCondBreakContinueItem::Stmt(StmtRef::new(0)),
            &mut builder,
            &mut bindings,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &mut BTreeMap::new(),
            "raw body input",
        )
        .expect("body statement lowering")
        .expect("simple function call");
        assert!(matches!(
            result.as_slice(),
            [
                crate::mir::builder::control_flow::plan::LoweredRecipe::Effect(
                    crate::mir::builder::control_flow::plan::CoreEffectPlan::Const { .. }
                ),
                crate::mir::builder::control_flow::plan::LoweredRecipe::Effect(
                    crate::mir::builder::control_flow::plan::CoreEffectPlan::GlobalCall { func, .. }
                )
            ] if func == "Worker.run"
        ));
    }

    #[test]
    fn raw_exit_leaf_return_uses_port_value_input() {
        let body = vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(9),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }];
        let port = RawLoopPlanExpressionPortV1::new();
        let body_input = body.as_slice();
        let mut builder = MirBuilder::new();
        let mut bindings = BTreeMap::<String, ValueId>::new();
        let result = super::super::loop_cond_bc_item::lower_loop_cond_item_input(
            &port,
            &body_input,
            &LoopCondBreakContinueItem::ExitLeaf {
                kind: ExitKind::Return,
                stmt: StmtRef::new(0),
            },
            &mut builder,
            &mut bindings,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &mut BTreeMap::new(),
            "raw exit input",
        )
        .expect("exit lowering")
        .expect("exit leaf");
        assert!(matches!(
            result.as_slice(),
            [
                crate::mir::builder::control_flow::plan::LoweredRecipe::Effect(
                    crate::mir::builder::control_flow::plan::CoreEffectPlan::Const { .. }
                ),
                crate::mir::builder::control_flow::plan::LoweredRecipe::Exit(
                    crate::mir::builder::control_flow::plan::CoreExitPlan::Return(Some(_))
                )
            ]
        ));
    }
}
