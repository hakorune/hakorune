//! Port-aware adapter for ordinary direct-body join If lowering.
//!
//! The branch snapshot/join state remains owned by `lower_if_join_state_core`.
//! This file only injects exact expression/body carriers and the associated
//! condition/body lowerer used by the disconnected direct-body core.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_if_plan_port::lower_cond_expr_to_if_plans_input;
use crate::mir::builder::control_flow::plan::parts::dispatch::if_join::{
    lower_if_join_state_core, JoinIfBranchV1,
};
use crate::mir::builder::control_flow::plan::parts::join_scope::{
    collect_branch_local_vars_from_maps, filter_branch_locals_from_maps,
};
use crate::mir::builder::control_flow::plan::{LoopPlanExpressionPortV1, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder) fn lower_if_join_input<'input, P, LowerBody>(
    port: &P,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    condition: P::ExprInput<'input>,
    then_body: P::BodyInput<'input>,
    else_body: Option<P::BodyInput<'input>>,
    _carrier_step_phis: &BTreeMap<String, ValueId>,
    _loop_var: &str,
    error_prefix: &str,
    mut lower_body: LowerBody,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
    LowerBody: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, ValueId>,
        &P,
        P::BodyInput<'input>,
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let mut then_body = Some(then_body);
    let mut else_body = else_body;
    let mut condition = Some(condition);
    let has_else = else_body.is_some();

    let mut lower_branch = |branch: JoinIfBranchV1,
                            builder: &mut MirBuilder,
                            bindings: &mut BTreeMap<String, ValueId>| {
        let body = match branch {
            JoinIfBranchV1::Then => then_body.take().ok_or_else(|| {
                format!("[freeze:contract][direct-if] then carrier reused: ctx={error_prefix}")
            })?,
            JoinIfBranchV1::Else => else_body.take().ok_or_else(|| {
                format!("[freeze:contract][direct-if] else carrier reused: ctx={error_prefix}")
            })?,
        };
        lower_body(builder, bindings, port, body)
    };

    let normalize_branch_maps =
        |pre: &BTreeMap<String, ValueId>,
         then_map: &BTreeMap<String, ValueId>,
         else_map: &BTreeMap<String, ValueId>| {
            let branch_locals = collect_branch_local_vars_from_maps(pre, then_map, else_map);
            filter_branch_locals_from_maps(pre, then_map, else_map, &branch_locals)
        };

    let mut lower_condition = |builder: &mut MirBuilder,
                               bindings: &mut BTreeMap<String, ValueId>,
                               then_plans: Vec<LoweredRecipe>,
                               else_plans: Option<Vec<LoweredRecipe>>,
                               joins| {
        let condition = condition.take().ok_or_else(|| {
            format!("[freeze:contract][direct-if] condition carrier reused: ctx={error_prefix}")
        })?;
        lower_cond_expr_to_if_plans_input(
            port,
            condition,
            builder,
            bindings,
            then_plans,
            else_plans,
            joins,
            error_prefix,
        )
    };

    let should_update =
        |name: &str, bindings: &BTreeMap<String, ValueId>| bindings.contains_key(name);
    lower_if_join_state_core(
        builder,
        current_bindings,
        has_else,
        &mut lower_branch,
        normalize_branch_maps,
        &mut lower_condition,
        &should_update,
    )
}
