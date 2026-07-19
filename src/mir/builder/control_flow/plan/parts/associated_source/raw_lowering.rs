//! Raw recipe lowering hooks for the neutral associated-source dispatcher.
//!
//! This is the sole raw semantic consumer of `VerifiedPartsAssociatedItemV1`.
//! The raw provider owns recipe/source pairing; these hooks only execute the
//! already-selected item through the pre-existing Parts lowering owners.

use std::collections::BTreeMap;
use std::convert::Infallible;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::parts::dispatch::block::{
    lower_stmt_dispatch, BlockKindInternal, LowerStmtFn,
};
use crate::mir::builder::control_flow::plan::parts::dispatch::if_exit_only::{
    lower_else_only_exit_if, lower_exit_only_if, lower_then_only_exit_if,
};
use crate::mir::builder::control_flow::plan::parts::dispatch::if_join::lower_if_join_with_stmt_lowerer;
use crate::mir::builder::control_flow::plan::parts::exit as parts_exit;
use crate::mir::builder::control_flow::plan::parts::stmt as parts_stmt;
use crate::mir::builder::control_flow::plan::recipe_tree::{IfContractKind, IfMode, RecipeBodies};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

use super::dispatch::PartsAssociatedLoweringHooksV1;
use super::{RawPartsAssociatedBlockV1, RawPartsAssociatedLoopV0V1, RawPartsAssociatedSourceV1};

pub(in crate::mir::builder::control_flow::plan::parts) struct RawPartsAssociatedLoweringHooksV1<
    'context,
    'policy,
> {
    builder: &'context mut MirBuilder,
    current_bindings: &'context mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &'context BTreeMap<String, crate::mir::ValueId>,
    arena: &'context RecipeBodies,
    error_prefix: &'context str,
    policy: &'context mut BlockKindInternal<'policy>,
    lower_stmt_outer: Option<&'context mut LowerStmtFn<'policy>>,
}

impl<'context, 'policy> RawPartsAssociatedLoweringHooksV1<'context, 'policy> {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder::control_flow::plan::parts) fn new(
        builder: &'context mut MirBuilder,
        current_bindings: &'context mut BTreeMap<String, crate::mir::ValueId>,
        carrier_step_phis: &'context BTreeMap<String, crate::mir::ValueId>,
        arena: &'context RecipeBodies,
        error_prefix: &'context str,
        policy: &'context mut BlockKindInternal<'policy>,
        lower_stmt_outer: Option<&'context mut LowerStmtFn<'policy>>,
    ) -> Self {
        Self {
            builder,
            current_bindings,
            carrier_step_phis,
            arena,
            error_prefix,
            policy,
            lower_stmt_outer,
        }
    }
}

impl<'source> PartsAssociatedLoweringHooksV1<RawPartsAssociatedSourceV1<'source>>
    for RawPartsAssociatedLoweringHooksV1<'_, '_>
{
    type Output = Vec<LoweredRecipe>;

    fn lower_opaque_stmt(
        &mut self,
        _port: crate::mir::builder::control_flow::plan::expression_port::RawLoopPlanExpressionPortV1,
        source: &'source ASTNode,
    ) -> Result<Self::Output, String> {
        match self.policy {
            BlockKindInternal::ExitOnly { break_phi_dsts }
            | BlockKindInternal::ExitAllowed { break_phi_dsts } => {
                parts_stmt::lower_return_prelude_stmt(
                    self.builder,
                    self.current_bindings,
                    self.carrier_step_phis,
                    Some(*break_phi_dsts),
                    source,
                    self.error_prefix,
                )
            }
            BlockKindInternal::StmtOnly {
                break_phi_dsts,
                lower_stmt,
            } => lower_stmt_dispatch(
                self.builder,
                self.current_bindings,
                self.carrier_step_phis,
                *break_phi_dsts,
                source,
                self.error_prefix,
                *lower_stmt,
            ),
            BlockKindInternal::NoExit { break_phi_dsts, .. } => {
                let lower_stmt = self.lower_stmt_outer.as_deref_mut().ok_or_else(|| {
                    format!(
                        "[freeze:contract][recipe] missing_no_exit_stmt_lowerer: ctx={}",
                        self.error_prefix
                    )
                })?;
                lower_stmt_dispatch(
                    self.builder,
                    self.current_bindings,
                    self.carrier_step_phis,
                    *break_phi_dsts,
                    source,
                    self.error_prefix,
                    lower_stmt,
                )
            }
        }
    }

    fn lower_opaque_exit(
        &mut self,
        _port: crate::mir::builder::control_flow::plan::expression_port::RawLoopPlanExpressionPortV1,
        source: &'source ASTNode,
        kind: crate::mir::builder::control_flow::plan::recipe_tree::ExitKind,
    ) -> Result<Self::Output, String> {
        let break_phi_dsts = match &*self.policy {
            BlockKindInternal::ExitOnly { break_phi_dsts }
            | BlockKindInternal::ExitAllowed { break_phi_dsts } => *break_phi_dsts,
            _ => {
                return Err(format!(
                    "[freeze:contract][recipe] exit_missing_break_phi_dsts: ctx={}",
                    self.error_prefix
                ));
            }
        };
        parts_exit::lower_loop_cond_exit_source(
            self.builder,
            self.current_bindings,
            self.carrier_step_phis,
            break_phi_dsts,
            source,
            kind,
            self.error_prefix,
        )
    }

    fn lower_explicit_if(
        &mut self,
        _port: crate::mir::builder::control_flow::plan::expression_port::RawLoopPlanExpressionPortV1,
        _source: &'source ASTNode,
        condition: &'source crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView,
        _then_body: &'source [ASTNode],
        _else_body: Option<&'source [ASTNode]>,
        contract: IfContractKind,
        then_block: RawPartsAssociatedBlockV1<'source>,
        else_block: Option<RawPartsAssociatedBlockV1<'source>>,
    ) -> Result<Self::Output, String> {
        match (&mut *self.policy, contract) {
            (
                BlockKindInternal::ExitOnly { break_phi_dsts }
                | BlockKindInternal::ExitAllowed { break_phi_dsts },
                IfContractKind::ExitOnly { mode },
            ) => lower_exit_only_if(
                self.builder,
                self.current_bindings,
                self.carrier_step_phis,
                break_phi_dsts,
                self.arena,
                condition,
                mode,
                then_block.recipe_block(),
                else_block
                    .as_ref()
                    .map(RawPartsAssociatedBlockV1::recipe_block),
                self.error_prefix,
            ),
            (
                BlockKindInternal::ExitOnly { break_phi_dsts }
                | BlockKindInternal::ExitAllowed { break_phi_dsts },
                IfContractKind::ExitAllowed {
                    mode: IfMode::ElseOnlyExit,
                },
            ) => lower_else_only_exit_if(
                self.builder,
                self.current_bindings,
                self.carrier_step_phis,
                break_phi_dsts,
                self.arena,
                condition,
                then_block.recipe_block(),
                else_block
                    .as_ref()
                    .map(RawPartsAssociatedBlockV1::recipe_block),
                self.error_prefix,
            ),
            (
                BlockKindInternal::ExitOnly { break_phi_dsts }
                | BlockKindInternal::ExitAllowed { break_phi_dsts },
                IfContractKind::ExitAllowed {
                    mode: IfMode::ThenOnlyExit,
                },
            ) => lower_then_only_exit_if(
                self.builder,
                self.current_bindings,
                self.carrier_step_phis,
                break_phi_dsts,
                self.arena,
                condition,
                then_block.recipe_block(),
                else_block
                    .as_ref()
                    .map(RawPartsAssociatedBlockV1::recipe_block),
                self.error_prefix,
            ),
            (
                BlockKindInternal::NoExit {
                    break_phi_dsts,
                    make_lower_stmt,
                    should_update_binding,
                },
                IfContractKind::Join,
            ) => lower_if_join_with_stmt_lowerer(
                self.builder,
                self.current_bindings,
                self.carrier_step_phis,
                *break_phi_dsts,
                self.arena,
                condition,
                then_block.recipe_block(),
                else_block
                    .as_ref()
                    .map(RawPartsAssociatedBlockV1::recipe_block),
                self.error_prefix,
                *make_lower_stmt,
                *should_update_binding,
            ),
            _ => Err(format!(
                "[freeze:contract][recipe] dispatch_saw_unsupported_item: ctx={}",
                self.error_prefix
            )),
        }
    }

    fn lower_stmt_wrapped_join_if(
        &mut self,
        _port: crate::mir::builder::control_flow::plan::expression_port::RawLoopPlanExpressionPortV1,
        bridge: Infallible,
    ) -> Result<Self::Output, String> {
        match bridge {}
    }

    fn lower_raw_loop_v0(
        &mut self,
        _port: crate::mir::builder::control_flow::plan::expression_port::RawLoopPlanExpressionPortV1,
        loop_input: RawPartsAssociatedLoopV0V1<'source>,
    ) -> Result<Self::Output, String> {
        for (name, value_id) in self.current_bindings.iter() {
            self.builder
                .function_state
                .variable_ctx
                .variable_map
                .insert(name.clone(), *value_id);
        }
        let plan = super::super::loop_::lower_loop_v0(
            self.builder,
            self.current_bindings,
            loop_input.condition,
            loop_input.body_contract,
            self.arena,
            loop_input.body_block.recipe_block(),
            self.error_prefix,
        )?;
        Ok(vec![plan])
    }
}
