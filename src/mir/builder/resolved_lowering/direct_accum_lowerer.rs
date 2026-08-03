//! Candidate-only DirectAccum consumer over the canonical function session.
//!
//! This facade consumes one sealed plan, publishes the exact two-entry prefix
//! into the function-owned identity/SSA session, and delegates loop emission
//! to the existing role-aware physicalizer.  It owns no route selection and
//! no second CFG, SSA, or PHI transaction.

use crate::mir::builder::control_flow::plan::loop_accum_physicalizer::physicalize_direct_accum_v1_with_port;
use crate::mir::builder::control_flow::plan::loop_physical_input::{
    direct_accum_physical_input, LoopPhysicalRoleV1, VerifiedLoopBindingProjectionV1,
    VerifiedLoopInputProjectionV1, VerifiedLoopPhysicalRolePlanV1,
};
use crate::mir::builder::emission::constant::emit_integer;
use crate::mir::compiler::direct_accum_profile::CanonicalDirectAccumPlanV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_recipe_contract::{LoopBindingKeyV1, LoopValueKeyV1};
use crate::mir::loop_structural_facts::VerifiedDirectAccumBindingEffectPlanV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::BindingKindV1;
use crate::mir::{BasicBlockId, MirBuilder};

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;
use super::completion_consumption::ReadyFunctionCompletionV1;
use super::direct_accum_adapter::CanonicalDirectAccumBindingPort;

pub(in crate::mir::builder::resolved_lowering) struct CanonicalDirectAccumSsaLowererV1<
    'builder,
    'source,
> {
    builder: &'builder mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'source>,
    prefix: crate::mir::compiler::direct_accum_prefix::VerifiedDirectAccumPrefixInputV1,
    effect_plan: VerifiedDirectAccumBindingEffectPlanV1,
    recipe: Option<crate::mir::loop_recipe_contract::VerifiedDirectAccumRecipeProductV1>,
    session: CanonicalSsaFunctionSessionV2<'source>,
}

impl<'builder, 'source> CanonicalDirectAccumSsaLowererV1<'builder, 'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        builder: &'builder mut MirBuilder,
        plan: CanonicalDirectAccumPlanV1<'source>,
    ) -> Result<Self, String> {
        let (input, _loop_stmt, receipt, prefix, recipe, effect_plan, completion) =
            plan.into_parts();
        if input.owner() != effect_plan.owner()
            || prefix.owner() != input.owner()
            || receipt.frame_key() != effect_plan.frame_key()
        {
            return Err("[freeze:contract][direct_accum/plan_owner_or_frame_mismatch]".into());
        }
        if !builder
            .function_state
            .resolved_binding_state
            .is_installed_for(input.owner())
        {
            return Err("[freeze:contract][canonical_binding_ssa/authority_not_installed]".into());
        }
        let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input)?;
        let session = CanonicalSsaFunctionSessionV2::new(input, if_control, completion, 0)?;
        Ok(Self {
            builder,
            input,
            prefix,
            effect_plan,
            recipe: Some(recipe),
            session,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) fn lower(
        mut self,
    ) -> Result<ReadyFunctionCompletionV1, String> {
        let (bindings, inputs) = self.publish_prefix()?;
        let preheader = self.current_block()?;
        let roles = self.allocate_roles(preheader)?;
        let recipe = self
            .recipe
            .take()
            .ok_or_else(|| "[freeze:contract][direct_accum/recipe_reconsumed]".to_string())?;
        let physical_input = direct_accum_physical_input(recipe);
        let mut port = CanonicalDirectAccumBindingPort::new(
            &mut self.session.identity,
            &self.effect_plan,
            self.input.owner(),
            self.effect_plan.frame_key(),
        )?;
        let continuation = physicalize_direct_accum_v1_with_port(
            self.builder,
            physical_input,
            bindings,
            inputs,
            roles,
            &mut self.session.cfg,
            &mut port,
            &mut self.session.phis,
        )
        .map_err(|error| format!("[freeze:contract][direct_accum/physicalizer] {error:?}"))?;
        port.finish_effect_claims()?;
        self.finish_after(continuation.continuation_block)?;

        let body = self
            .input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        let body_end = u32::try_from(body.statements().len())
            .map_err(|_| "[freeze:contract][direct_accum/body_length_overflow]".to_string())?;
        let target_function = self.session.semantics.function_region();
        self.session.semantics.finish()?;
        self.session
            .if_control
            .finish()
            .map_err(|error| format!("[freeze:contract][if_control/finish] {error:?}"))?;
        self.session.identity.finish()?;
        self.session
            .phis
            .commit(self.builder)
            .map_err(|error| error.to_string())?;
        self.builder
            .function_state
            .resolved_binding_state
            .finish(self.input.owner())?;
        self.session
            .completion
            .finish(body.site(), body_end, target_function)
    }

    fn publish_prefix(
        &mut self,
    ) -> Result<
        (
            VerifiedLoopBindingProjectionV1,
            VerifiedLoopInputProjectionV1,
        ),
        String,
    > {
        let block = self.current_block()?;
        let mut binding_rows = Vec::with_capacity(2);
        let mut input_rows = Vec::with_capacity(2);
        for local in self.prefix.locals() {
            let ordinal = match local.kind() {
                BindingKindV1::Local { ordinal } => ordinal,
                _ => return Err("[freeze:contract][direct_accum/prefix_kind]".into()),
            };
            let value = emit_integer(self.builder, local.initial())?;
            let binding_key = LoopBindingKeyV1::new(ordinal);
            let binding = self.session.identity.publish_declaration(
                local.site(),
                local.kind(),
                local.name(),
                block,
                value,
            )?;
            let expected = self
                .effect_plan
                .entries()
                .iter()
                .find(|entry| entry.recipe_binding() == binding_key)
                .map(|entry| entry.binding())
                .ok_or_else(|| {
                    format!(
                        "[freeze:contract][direct_accum/input_binding_missing] key={binding_key:?}"
                    )
                })?;
            if binding != expected {
                return Err(format!(
                    "[freeze:contract][direct_accum/input_binding_mismatch] key={binding_key:?}"
                ));
            }
            binding_rows.push((binding_key, binding));
            input_rows.push((LoopValueKeyV1::new(ordinal), binding_key, value));
        }
        let bindings =
            VerifiedLoopBindingProjectionV1::try_new(self.input.owner(), binding_rows)
                .map_err(|error| format!("[freeze:contract][direct_accum/bindings] {error:?}"))?;
        let inputs = VerifiedLoopInputProjectionV1::try_new(block, input_rows)
            .map_err(|error| format!("[freeze:contract][direct_accum/inputs] {error:?}"))?;
        Ok((bindings, inputs))
    }

    fn allocate_roles(
        &mut self,
        preheader: BasicBlockId,
    ) -> Result<VerifiedLoopPhysicalRolePlanV1, String> {
        let rows = [
            (LoopPhysicalRoleV1::Preheader, preheader),
            (LoopPhysicalRoleV1::Header, self.builder.next_block_id()),
            (LoopPhysicalRoleV1::Body, self.builder.next_block_id()),
            (LoopPhysicalRoleV1::Step, self.builder.next_block_id()),
            (LoopPhysicalRoleV1::After, self.builder.next_block_id()),
        ];
        VerifiedLoopPhysicalRolePlanV1::try_new(rows.into_iter().collect())
            .map_err(|error| format!("[freeze:contract][direct_accum/roles] {error:?}"))
    }

    fn finish_after(&mut self, after: BasicBlockId) -> Result<(), String> {
        if self.builder.function_state.current_block != Some(after) {
            return Err("[freeze:contract][direct_accum/after_not_current]".into());
        }
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][direct_accum/function_missing]".to_string())?;
        let witness = self
            .session
            .cfg
            .seal_block(function, after)
            .map_err(|error| error.to_string())?;
        self.session
            .identity
            .seal_block(self.builder, &mut self.session.phis, after, &witness)
    }

    fn current_block(&self) -> Result<BasicBlockId, String> {
        self.builder
            .function_state
            .current_block
            .ok_or_else(|| "[freeze:contract][direct_accum/current_block_missing]".to_string())
    }
}
