//! Disconnected located carrier adapter for the first strict Parts root.
//!
//! This module contains no recipe policy. A Builder-free preflight seal must
//! be consumed before the adapter can delegate exact PATH0 carriers to the
//! existing statement primitives and shared Parts state owners.

use std::collections::BTreeMap;
use std::convert::Infallible;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::{
    LocatedLoopPlanExpressionPortV1, LocatedLoopPlanStmtInputV1, LoopPlanExpressionPortV1,
};
use crate::mir::builder::control_flow::plan::normalizer::{
    cond_lowering_if_plan_port::lower_cond_expr_to_if_plans_input,
    loop_body_lowering_associated_input::{
        lower_assignment_inputs, lower_local_statement_input, lower_return_statement_input,
    },
};
use crate::mir::builder::control_flow::plan::parts::dispatch::if_exit_only::{
    lower_exit_if_state_core, ExitIfBranchV1, ExitIfStatePolicyV1,
};
use crate::mir::builder::control_flow::plan::parts::dispatch::if_join::{
    lower_if_join_state_core, JoinIfBranchV1,
};
use crate::mir::builder::control_flow::plan::parts::var_map_scope::{
    publish_declared_binding, publish_defined_binding, reseal_branch_bindings,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfContractKind, IfMode};
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{CoreIfJoin, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::ValueId;

use super::block_driver::lower_verified_parts_associated_block;
use super::dispatch::{PartsAssociatedBlockModeV1, PartsAssociatedLoweringHooksV1};
use super::located_preflight::VerifiedLocatedGenericLoopPartsPreflightV1;
use super::{
    LocatedPartsAssociatedSourceV1, PartsAssociatedSourceErrorV1,
    VerifiedLocatedRecipeBlockLoweringViewV1, VerifiedStmtWrappedJoinIfLoweringViewV1,
};

#[derive(Clone, Copy)]
enum LocatedBlockPolicyV1 {
    ExitOnly,
    ExitAllowed,
    NoExit,
}

pub(in crate::mir::builder::control_flow::plan::parts) fn lower_preflighted_located_parts_root_v1<
    'seal,
    'view,
    'plan,
>(
    preflight: VerifiedLocatedGenericLoopPartsPreflightV1<'seal, 'view, 'plan>,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    preflight.lower_with_parts_adapter(|lowering| {
        let crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { root } = lowering.mode()
        else {
            return Err(format!(
                "[freeze:contract][located-parts] preflight_mode_drift: ctx={error_prefix}"
            ));
        };
        lower_located_block(
            &root,
            LocatedBlockPolicyV1::ExitAllowed,
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            error_prefix,
        )
    })
}

#[allow(clippy::too_many_arguments)]
fn lower_located_block<'view, 'plan>(
    root: &VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
    policy: LocatedBlockPolicyV1,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let source = LocatedPartsAssociatedSourceV1::new(root);
    let mut hooks = LocatedPartsAssociatedLoweringHooksV1 {
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        error_prefix,
        policy,
    };
    lower_verified_parts_associated_block::<LocatedPartsAssociatedSourceV1<'_, '_>, _, _, _>(
        &source,
        root,
        policy.block_mode(),
        &mut hooks,
        error_prefix,
        |error| render_source_error(error, error_prefix),
        super::super::dispatch::plans_exit_on_all_paths,
    )
}

impl LocatedBlockPolicyV1 {
    const fn block_mode(self) -> PartsAssociatedBlockModeV1 {
        match self {
            Self::ExitOnly => PartsAssociatedBlockModeV1::ExitOnly,
            Self::ExitAllowed => PartsAssociatedBlockModeV1::ExitAllowed,
            Self::NoExit => PartsAssociatedBlockModeV1::NoExit,
        }
    }
}

fn render_source_error(error: PartsAssociatedSourceErrorV1, error_prefix: &str) -> String {
    format!("[located-parts/source] {error:?}: ctx={error_prefix}")
}

struct LocatedPartsAssociatedLoweringHooksV1<'context> {
    builder: &'context mut MirBuilder,
    current_bindings: &'context mut BTreeMap<String, ValueId>,
    carrier_step_phis: &'context BTreeMap<String, ValueId>,
    break_phi_dsts: &'context BTreeMap<String, ValueId>,
    error_prefix: &'context str,
    policy: LocatedBlockPolicyV1,
}

impl<'view, 'plan: 'view>
    PartsAssociatedLoweringHooksV1<LocatedPartsAssociatedSourceV1<'view, 'plan>>
    for LocatedPartsAssociatedLoweringHooksV1<'_>
{
    type Output = Vec<LoweredRecipe>;

    fn lower_opaque_stmt(
        &mut self,
        port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
        source: LocatedLoopPlanStmtInputV1<'plan, 'view>,
    ) -> Result<Self::Output, String> {
        reseal_branch_bindings(self.builder, self.current_bindings);
        match port.stmt_syntax(&source) {
            ASTNode::Local { .. } if matches!(self.policy, LocatedBlockPolicyV1::ExitAllowed) => {
                let (inits, effects) = lower_local_statement_input(
                    port,
                    source,
                    self.builder,
                    self.current_bindings,
                    self.error_prefix,
                )?;
                let plans = effects_to_plans(effects);
                for (name, value) in inits {
                    publish_declared_binding(self.builder, self.current_bindings, name, value)?;
                }
                Ok(plans)
            }
            ASTNode::Assignment { .. } if matches!(self.policy, LocatedBlockPolicyV1::NoExit) => {
                let target = port
                    .child_expr_from_stmt(&source, ExprChildRoleV1::AssignmentTarget)
                    .map_err(|error| error.render())?;
                let value = port
                    .child_expr_from_stmt(&source, ExprChildRoleV1::AssignmentValue)
                    .map_err(|error| error.render())?;
                let (binding, effects) = lower_assignment_inputs(
                    port,
                    target,
                    value,
                    self.builder,
                    self.current_bindings,
                    self.error_prefix,
                )?;
                let plans = effects_to_plans(effects);
                if let Some((name, value)) = binding {
                    publish_defined_binding(self.builder, self.current_bindings, name, value);
                }
                Ok(plans)
            }
            _ => Err(format!(
                "[freeze:contract][located-parts] unsupported_opaque_statement: ctx={}",
                self.error_prefix
            )),
        }
    }

    fn lower_opaque_exit(
        &mut self,
        port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
        source: LocatedLoopPlanStmtInputV1<'plan, 'view>,
        kind: ExitKind,
    ) -> Result<Self::Output, String> {
        reseal_branch_bindings(self.builder, self.current_bindings);
        if !matches!(self.policy, LocatedBlockPolicyV1::ExitOnly)
            || !matches!(kind, ExitKind::Return)
        {
            return Err(format!(
                "[freeze:contract][located-parts] unsupported_exit: ctx={}",
                self.error_prefix
            ));
        }
        lower_return_statement_input(
            port,
            source,
            self.builder,
            self.current_bindings,
            self.error_prefix,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_explicit_if(
        &mut self,
        port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
        _source: LocatedLoopPlanStmtInputV1<'plan, 'view>,
        condition: crate::mir::builder::control_flow::plan::expression_port::LocatedLoopPlanExprInputV1<'plan, 'view>,
        _then_body: crate::mir::builder::control_flow::plan::expression_port::LocatedLoopPlanBodyInputV1<'plan, 'view>,
        _else_body: Option<
            crate::mir::builder::control_flow::plan::expression_port::LocatedLoopPlanBodyInputV1<
                'plan,
                'view,
            >,
        >,
        contract: IfContractKind,
        then_block: VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
        else_block: Option<VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>>,
    ) -> Result<Self::Output, String> {
        let IfContractKind::ExitOnly { mode } = contract else {
            return Err(format!(
                "[freeze:contract][located-parts] unsupported_explicit_if: ctx={}",
                self.error_prefix
            ));
        };
        if !matches!(self.policy, LocatedBlockPolicyV1::ExitAllowed)
            || !matches!(mode, IfMode::ExitIf)
        {
            return Err(format!(
                "[freeze:contract][located-parts] explicit_if_policy_drift: ctx={}",
                self.error_prefix
            ));
        }
        let mut condition = Some(condition);
        let mut lower_branch =
            |branch, builder: &mut MirBuilder, bindings: &mut BTreeMap<String, ValueId>| {
                let block = match branch {
                    ExitIfBranchV1::Then => &then_block,
                    ExitIfBranchV1::Else => else_block.as_ref().ok_or_else(|| {
                        format!(
                            "[freeze:contract][located-parts] unexpected_else_request: ctx={}",
                            self.error_prefix
                        )
                    })?,
                };
                lower_located_block(
                    block,
                    LocatedBlockPolicyV1::ExitOnly,
                    builder,
                    bindings,
                    self.carrier_step_phis,
                    self.break_phi_dsts,
                    self.error_prefix,
                )
            };
        let mut lower_condition = |builder: &mut MirBuilder,
                                   bindings: &mut BTreeMap<String, ValueId>,
                                   then_plans,
                                   else_plans| {
            lower_cond_expr_to_if_plans_input(
                port,
                condition.take().ok_or_else(|| {
                    format!(
                        "[freeze:contract][located-parts] condition_reused: ctx={}",
                        self.error_prefix
                    )
                })?,
                builder,
                bindings,
                then_plans,
                else_plans,
                Vec::new(),
                self.error_prefix,
            )
        };
        lower_exit_if_state_core(
            self.builder,
            self.current_bindings,
            ExitIfStatePolicyV1::ExitOnly(mode),
            else_block.is_some(),
            self.error_prefix,
            &mut lower_branch,
            &mut lower_condition,
        )
    }

    fn lower_stmt_wrapped_join_if(
        &mut self,
        port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
        bridge: VerifiedStmtWrappedJoinIfLoweringViewV1<'view, 'plan>,
    ) -> Result<Self::Output, String> {
        if !matches!(self.policy, LocatedBlockPolicyV1::ExitAllowed) {
            return Err(format!(
                "[freeze:contract][located-parts] wrapped_join_policy_drift: ctx={}",
                self.error_prefix
            ));
        }
        let root = bridge.singleton_root();
        let then_block = root.then_block();
        let else_block = root.else_block();
        let mut condition = Some(bridge.condition());
        let mut lower_branch =
            |branch, builder: &mut MirBuilder, bindings: &mut BTreeMap<String, ValueId>| {
                let block = match branch {
                    JoinIfBranchV1::Then => &then_block,
                    JoinIfBranchV1::Else => else_block.as_ref().ok_or_else(|| {
                        format!(
                            "[freeze:contract][located-parts] wrapped_join_else_missing: ctx={}",
                            self.error_prefix
                        )
                    })?,
                };
                lower_located_block(
                    block,
                    LocatedBlockPolicyV1::NoExit,
                    builder,
                    bindings,
                    self.carrier_step_phis,
                    self.break_phi_dsts,
                    self.error_prefix,
                )
            };
        let normalize_branch_maps =
            |pre: &BTreeMap<_, _>, then_map: &BTreeMap<_, _>, else_map: &BTreeMap<_, _>| {
                let locals = super::super::join_scope::collect_branch_local_vars_from_maps(
                    pre, then_map, else_map,
                );
                super::super::join_scope::filter_branch_locals_from_maps(
                    pre, then_map, else_map, &locals,
                )
            };
        let mut lower_condition = |builder: &mut MirBuilder,
                                   bindings: &mut BTreeMap<String, ValueId>,
                                   then_plans,
                                   else_plans,
                                   joins: Vec<CoreIfJoin>| {
            lower_cond_expr_to_if_plans_input(
                port,
                condition.take().ok_or_else(|| {
                    format!(
                        "[freeze:contract][located-parts] condition_reused: ctx={}",
                        self.error_prefix
                    )
                })?,
                builder,
                bindings,
                then_plans,
                else_plans,
                joins,
                self.error_prefix,
            )
        };
        let should_update =
            |name: &str, bindings: &BTreeMap<String, ValueId>| bindings.contains_key(name);
        lower_if_join_state_core(
            self.builder,
            self.current_bindings,
            else_block.is_some(),
            &mut lower_branch,
            normalize_branch_maps,
            &mut lower_condition,
            &should_update,
        )
    }

    fn lower_raw_loop_v0(
        &mut self,
        _port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
        loop_input: Infallible,
    ) -> Result<Self::Output, String> {
        match loop_input {}
    }
}
