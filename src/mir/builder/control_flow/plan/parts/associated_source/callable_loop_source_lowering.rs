//! Located callable-loop lowering hooks for the neutral associated-source dispatcher.
//!
//! These hooks execute already-issued recipe items through the ported owners:
//! `lower_simple_effect_stmt_input` for opaque statements,
//! `lower_loop_cond_exit_source_input` for opaque exits, the shared exit-if /
//! join-if state cores for explicit `IfV2` items, and the same block driver
//! recursively for branch recipes. `RecipeItem::LoopV0` reuses the shared
//! `loop_v0` frame/carrier core with a ported header condition and a located
//! body block; a `BlockExpr` (prelude) condition stays a named reject until a
//! ported prelude lowering exists.

use std::collections::BTreeMap;
use std::convert::Infallible;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_util::lower_simple_effect_stmt_input;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_if_plan_port::lower_cond_expr_to_if_plans_input;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_loop_header::lower_loop_header_cond_with_port;
use crate::mir::builder::control_flow::plan::parts::dispatch::if_exit_only::{
    lower_exit_if_state_core, ExitIfBranchV1, ExitIfStatePolicyV1,
};
use crate::mir::builder::control_flow::plan::parts::dispatch::if_join::{
    lower_if_join_state_core, JoinIfBranchV1,
};
use crate::mir::builder::control_flow::plan::parts::exit as parts_exit;
use crate::mir::builder::control_flow::plan::parts::join_scope::{
    collect_branch_local_vars_from_maps, filter_branch_locals_from_maps,
};
use crate::mir::builder::control_flow::plan::parts::var_map_scope::reseal_branch_bindings;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, IfMode, RecipeItem,
};
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{CoreIfJoin, LoweredRecipe};
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceBodyInputV1, CallableLoopSourceExprInputV1,
    CallableLoopSourceExpressionPortV1, CallableLoopSourceStmtInputV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::block_driver::lower_verified_parts_associated_block;
use super::callable_loop_source::{
    CallableLoopSourcePartsAssociatedSourceV1, CallableLoopSourcePartsBlockV1,
    CallableLoopSourcePartsLoopV0V1,
};
use super::dispatch::{PartsAssociatedBlockModeV1, PartsAssociatedLoweringHooksV1};
use super::PartsAssociatedSourceErrorV1;

const SOURCE_PARTS_ERR: &str = "[freeze:contract][callable-loop/parts]";

fn render_source_error(error: PartsAssociatedSourceErrorV1, error_prefix: &str) -> String {
    format!("{SOURCE_PARTS_ERR}/source {error:?}: ctx={error_prefix}")
}

/// Drive one co-sealed recipe block through the located source provider.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder::control_flow::plan::parts) fn lower_callable_loop_source_parts_block<
    'view,
    'ledger,
>(
    port: CallableLoopSourceExpressionPortV1<'ledger>,
    block: &CallableLoopSourcePartsBlockV1<'view>,
    mode: PartsAssociatedBlockModeV1,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
    carrier_updates: &mut BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String>
where
    'ledger: 'view,
{
    let source = CallableLoopSourcePartsAssociatedSourceV1::for_block(block, port);
    let mut hooks = CallableLoopSourcePartsLoweringHooksV1 {
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        break_phi_dsts,
        carrier_updates,
        error_prefix,
    };
    lower_verified_parts_associated_block::<
        CallableLoopSourcePartsAssociatedSourceV1<'view, 'ledger>,
        _,
        _,
        _,
    >(
        &source,
        block,
        mode,
        &mut hooks,
        error_prefix,
        |error| render_source_error(error, error_prefix),
        super::super::dispatch::plans_exit_on_all_paths,
    )
}

/// Hook bundle threaded through the neutral dispatcher. `pub(super)` so the
/// sibling test module can drive a single hook (e.g. the LoopV0 boundary)
/// without reconstructing a whole recipe block.
pub(super) struct CallableLoopSourcePartsLoweringHooksV1<'context> {
    pub(super) builder: &'context mut MirBuilder,
    pub(super) current_bindings: &'context mut BTreeMap<String, ValueId>,
    pub(super) carrier_phis: &'context BTreeMap<String, ValueId>,
    pub(super) carrier_step_phis: &'context BTreeMap<String, ValueId>,
    pub(super) break_phi_dsts: &'context BTreeMap<String, ValueId>,
    pub(super) carrier_updates: &'context mut BTreeMap<String, ValueId>,
    pub(super) error_prefix: &'context str,
}

impl<'view, 'ledger: 'view>
    PartsAssociatedLoweringHooksV1<CallableLoopSourcePartsAssociatedSourceV1<'view, 'ledger>>
    for CallableLoopSourcePartsLoweringHooksV1<'_>
{
    type Output = Vec<LoweredRecipe>;

    fn lower_opaque_stmt(
        &mut self,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
        source: CallableLoopSourceStmtInputV1<'view>,
    ) -> Result<Self::Output, String> {
        reseal_branch_bindings(self.builder, self.current_bindings);
        match port.stmt_syntax(&source) {
            ASTNode::If { .. } => self.lower_opaque_if_source(port, source),
            ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Call { .. }
            | ASTNode::Print { .. } => lower_simple_effect_stmt_input(
                &port,
                source,
                self.builder,
                self.current_bindings,
                self.carrier_phis,
                self.carrier_updates,
                self.error_prefix,
            )?
            .ok_or_else(|| {
                format!(
                    "{SOURCE_PARTS_ERR} simple-stmt-owner-drift: ctx={}",
                    self.error_prefix
                )
            }),
            // `Program`/`ScopeBox`/`Loop` opaque statements need flattened
            // container recipes, which break located body alignment; they stay
            // a named reject until their own slice co-seals the projection.
            _ => Err(format!(
                "{SOURCE_PARTS_ERR} opaque-stmt-container-unlocated: ctx={}",
                self.error_prefix
            )),
        }
    }

    fn lower_opaque_exit(
        &mut self,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
        source: CallableLoopSourceStmtInputV1<'view>,
        kind: crate::mir::builder::control_flow::plan::recipe_tree::ExitKind,
    ) -> Result<Self::Output, String> {
        reseal_branch_bindings(self.builder, self.current_bindings);
        parts_exit::lower_loop_cond_exit_source_input(
            &port,
            self.builder,
            self.current_bindings,
            self.carrier_step_phis,
            self.break_phi_dsts,
            source,
            kind,
            self.error_prefix,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_explicit_if(
        &mut self,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
        _source: CallableLoopSourceStmtInputV1<'view>,
        condition: CallableLoopSourceExprInputV1<'view>,
        _then_body: CallableLoopSourceBodyInputV1<'view>,
        _else_body: Option<CallableLoopSourceBodyInputV1<'view>>,
        contract: IfContractKind,
        then_block: CallableLoopSourcePartsBlockV1<'view>,
        else_block: Option<CallableLoopSourcePartsBlockV1<'view>>,
    ) -> Result<Self::Output, String> {
        reseal_branch_bindings(self.builder, self.current_bindings);
        if matches!(contract, IfContractKind::Join) {
            return self.lower_join_if_source(port, condition, then_block, else_block);
        }
        let (policy, then_mode, else_mode) = match contract {
            IfContractKind::ExitOnly {
                mode: mode @ (IfMode::ExitIf | IfMode::ExitAll),
            } => (
                ExitIfStatePolicyV1::ExitOnly(mode),
                PartsAssociatedBlockModeV1::ExitOnly,
                PartsAssociatedBlockModeV1::ExitOnly,
            ),
            IfContractKind::ExitAllowed {
                mode: IfMode::ElseOnlyExit,
            } => (
                ExitIfStatePolicyV1::ElseOnlyExit,
                PartsAssociatedBlockModeV1::ExitAllowed,
                PartsAssociatedBlockModeV1::ExitOnly,
            ),
            IfContractKind::ExitAllowed {
                mode: IfMode::ThenOnlyExit,
            } => (
                ExitIfStatePolicyV1::ThenOnlyExit,
                PartsAssociatedBlockModeV1::ExitOnly,
                PartsAssociatedBlockModeV1::ExitAllowed,
            ),
            _ => {
                return Err(format!(
                    "{SOURCE_PARTS_ERR} explicit-if-contract-unsupported: ctx={}",
                    self.error_prefix
                ))
            }
        };
        let mut condition = Some(condition);
        let mut lower_branch =
            |branch: ExitIfBranchV1,
             builder: &mut MirBuilder,
             bindings: &mut BTreeMap<String, ValueId>| {
                let (block, mode) = match branch {
                    ExitIfBranchV1::Then => (&then_block, then_mode),
                    ExitIfBranchV1::Else => (
                        else_block.as_ref().ok_or_else(|| {
                            format!(
                                "{SOURCE_PARTS_ERR} exit-if-else-missing: ctx={}",
                                self.error_prefix
                            )
                        })?,
                        else_mode,
                    ),
                };
                lower_callable_loop_source_parts_block(
                    port,
                    block,
                    mode,
                    builder,
                    bindings,
                    self.carrier_phis,
                    self.carrier_step_phis,
                    self.break_phi_dsts,
                    self.carrier_updates,
                    self.error_prefix,
                )
            };
        let mut lower_condition = |builder: &mut MirBuilder,
                                   bindings: &mut BTreeMap<String, ValueId>,
                                   then_plans,
                                   else_plans| {
            lower_cond_expr_to_if_plans_input(
                &port,
                condition.take().ok_or_else(|| {
                    format!(
                        "{SOURCE_PARTS_ERR} condition-carrier-reused: ctx={}",
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
            policy,
            else_block.is_some(),
            self.error_prefix,
            &mut lower_branch,
            &mut lower_condition,
        )
    }

    fn lower_stmt_wrapped_join_if(
        &mut self,
        _port: CallableLoopSourceExpressionPortV1<'ledger>,
        bridge: Infallible,
    ) -> Result<Self::Output, String> {
        match bridge {}
    }

    /// Nested `LoopV0` reuses the shared `loop_v0` frame/carrier owner: the
    /// header condition enters through `lower_loop_header_cond_with_port` and
    /// the co-sealed body block recurses through this same provider. The raw
    /// `CondBlockView` prelude has no ported lowering yet, so a BlockExpr
    /// condition is a named reject before any Builder effect.
    fn lower_raw_loop_v0(
        &mut self,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
        loop_input: CallableLoopSourcePartsLoopV0V1<'view>,
    ) -> Result<Self::Output, String> {
        if matches!(
            port.expr_syntax(&loop_input.condition),
            ASTNode::BlockExpr { .. }
        ) {
            return Err(format!(
                "{SOURCE_PARTS_ERR} loop-v0-cond-prelude-unlocated: ctx={}",
                self.error_prefix
            ));
        }
        reseal_branch_bindings(self.builder, self.current_bindings);
        let mode = match loop_input.body_contract {
            BlockContractKind::StmtOnly => PartsAssociatedBlockModeV1::StmtOnly,
            BlockContractKind::NoExit => PartsAssociatedBlockModeV1::NoExit,
            BlockContractKind::ExitAllowed => PartsAssociatedBlockModeV1::ExitAllowed,
            BlockContractKind::ExitOnly => PartsAssociatedBlockModeV1::ExitOnly,
        };
        let body_recipe = loop_input
            .body_block
            .recipe_body()
            .map_err(|error| render_source_error(error, self.error_prefix))?;
        let cond_tail = port.expr_syntax(&loop_input.condition);
        let condition = loop_input.condition;
        let body_block = loop_input.body_block;
        let error_prefix = self.error_prefix;
        let plan = super::super::loop_::lower_loop_v0_core(
            self.builder,
            self.current_bindings,
            &body_recipe.body,
            cond_tail,
            |builder, loop_bindings, frame| {
                lower_loop_header_cond_with_port(
                    builder,
                    loop_bindings,
                    &port,
                    condition,
                    frame.header_bb,
                    frame.body_bb,
                    frame.after_bb,
                    empty_carriers_args(),
                    empty_carriers_args(),
                    SOURCE_PARTS_ERR,
                )
            },
            |builder, body_bindings, carrier_step_phis, break_phi_dsts, _pre_bindings| {
                // The inner body's carrier updates are consumed by the loop_v0
                // backedge, not by the enclosing block's update map — matching
                // the raw path, which threads no carrier_updates through a
                // nested loop body.
                let mut inner_updates = BTreeMap::new();
                lower_callable_loop_source_parts_block(
                    port,
                    &body_block,
                    mode,
                    builder,
                    body_bindings,
                    carrier_step_phis,
                    carrier_step_phis,
                    break_phi_dsts,
                    &mut inner_updates,
                    error_prefix,
                )
            },
            error_prefix,
        )?;
        Ok(vec![plan])
    }
}

impl CallableLoopSourcePartsLoweringHooksV1<'_> {
    /// Container recipe authority for an `OpaqueStmt` that wraps `ASTNode::If`.
    ///
    /// Mirrors the raw return-prelude If arm ordering: a no-exit singleton
    /// recipe first, then the planner-required exit-allowed singleton; a
    /// degenerate `[Stmt]` recipe and the raw join/exit-if fallback owners
    /// stay rejected on the located spine.
    fn lower_opaque_if_source<'view, 'ledger: 'view>(
        &mut self,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
        source: CallableLoopSourceStmtInputV1<'view>,
    ) -> Result<Vec<LoweredRecipe>, String> {
        let stmt_node = port.stmt_syntax(&source);
        if let Some(recipe) = try_build_no_exit_block_recipe(std::slice::from_ref(stmt_node), true)
        {
            let block = CallableLoopSourcePartsBlockV1::singleton(
                &recipe.arena,
                &recipe.block,
                source,
                &port,
            )
            .map_err(|error| render_source_error(error, self.error_prefix))?;
            return lower_callable_loop_source_parts_block(
                port,
                &block,
                PartsAssociatedBlockModeV1::NoExit,
                self.builder,
                self.current_bindings,
                self.carrier_phis,
                self.carrier_step_phis,
                self.break_phi_dsts,
                self.carrier_updates,
                self.error_prefix,
            );
        }
        if let Some(recipe) =
            try_build_exit_allowed_block_recipe(std::slice::from_ref(stmt_node), true)
        {
            if matches!(recipe.block.items.as_slice(), [RecipeItem::Stmt(_)]) {
                return Err(format!(
                    "{SOURCE_PARTS_ERR} opaque-if-recipe-degenerate: ctx={}",
                    self.error_prefix
                ));
            }
            let block = CallableLoopSourcePartsBlockV1::singleton(
                &recipe.arena,
                &recipe.block,
                source,
                &port,
            )
            .map_err(|error| render_source_error(error, self.error_prefix))?;
            return lower_callable_loop_source_parts_block(
                port,
                &block,
                PartsAssociatedBlockModeV1::ExitAllowed,
                self.builder,
                self.current_bindings,
                self.carrier_phis,
                self.carrier_step_phis,
                self.break_phi_dsts,
                self.carrier_updates,
                self.error_prefix,
            );
        }
        Err(format!(
            "{SOURCE_PARTS_ERR} opaque-if-unlocatable: ctx={}",
            self.error_prefix
        ))
    }

    /// Join-bearing `IfV2` inside a `NoExit` block: branch snapshots and join
    /// materialization stay in `lower_if_join_state_core`; the located branch
    /// blocks recurse through this same provider in `NoExit` mode.
    fn lower_join_if_source<'view, 'ledger: 'view>(
        &mut self,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
        condition: CallableLoopSourceExprInputV1<'view>,
        then_block: CallableLoopSourcePartsBlockV1<'view>,
        else_block: Option<CallableLoopSourcePartsBlockV1<'view>>,
    ) -> Result<Vec<LoweredRecipe>, String> {
        let mut condition = Some(condition);
        let mut lower_branch =
            |branch: JoinIfBranchV1,
             builder: &mut MirBuilder,
             bindings: &mut BTreeMap<String, ValueId>| {
                let block = match branch {
                    JoinIfBranchV1::Then => &then_block,
                    JoinIfBranchV1::Else => else_block.as_ref().ok_or_else(|| {
                        format!(
                            "{SOURCE_PARTS_ERR} join-if-else-missing: ctx={}",
                            self.error_prefix
                        )
                    })?,
                };
                lower_callable_loop_source_parts_block(
                    port,
                    block,
                    PartsAssociatedBlockModeV1::NoExit,
                    builder,
                    bindings,
                    self.carrier_phis,
                    self.carrier_step_phis,
                    self.break_phi_dsts,
                    self.carrier_updates,
                    self.error_prefix,
                )
            };
        let normalize_branch_maps =
            |pre: &BTreeMap<String, ValueId>,
             then_map: &BTreeMap<String, ValueId>,
             else_map: &BTreeMap<String, ValueId>| {
                let locals = collect_branch_local_vars_from_maps(pre, then_map, else_map);
                filter_branch_locals_from_maps(pre, then_map, else_map, &locals)
            };
        let mut lower_condition = |builder: &mut MirBuilder,
                                   bindings: &mut BTreeMap<String, ValueId>,
                                   then_plans,
                                   else_plans,
                                   joins: Vec<CoreIfJoin>| {
            lower_cond_expr_to_if_plans_input(
                &port,
                condition.take().ok_or_else(|| {
                    format!(
                        "{SOURCE_PARTS_ERR} condition-carrier-reused: ctx={}",
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
}
