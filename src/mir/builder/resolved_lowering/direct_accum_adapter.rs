//! Resolved DirectAccum identity adapter.
//!
//! The adapter is the only bridge from the builder-free source-effect plan to
//! the function-owned identity/SSA session.  It does not create or finish a
//! second SSA/PHI owner; it merely claims the five sealed roles and delegates
//! reaching-value operations to `ResolvedSsaIdentityStateV2`.

use std::collections::BTreeSet;

use crate::mir::builder::control_flow::plan::loop_accum_physicalizer::DirectAccumBindingPortV1;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::LoopBindingKeyV1;
use crate::mir::loop_structural_facts::{
    DirectAccumBindingEffectRoleV1, VerifiedDirectAccumBindingEffectPlanV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1};
use crate::mir::{BasicBlockId, ValueId};

use super::canonical_ssa::ResolvedSsaIdentityStateV2;

pub(in crate::mir::builder::resolved_lowering) struct CanonicalDirectAccumBindingPort<
    'plan,
    'source,
> {
    identity: &'plan mut ResolvedSsaIdentityStateV2<'source>,
    plan: &'plan VerifiedDirectAccumBindingEffectPlanV1,
    claimed: BTreeSet<DirectAccumBindingEffectRoleV1>,
}

impl<'plan, 'source> CanonicalDirectAccumBindingPort<'plan, 'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        identity: &'plan mut ResolvedSsaIdentityStateV2<'source>,
        plan: &'plan VerifiedDirectAccumBindingEffectPlanV1,
        owner: FunctionOwnerIdV1,
        frame_key: &LoopExecutionFrameKeyV1,
    ) -> Result<Self, String> {
        if plan.owner() != owner {
            return Err("[freeze:contract][direct_accum/plan_owner_mismatch]".into());
        }
        if plan.frame_key() != frame_key {
            return Err("[freeze:contract][direct_accum/plan_frame_mismatch]".into());
        }
        Ok(Self {
            identity,
            plan,
            claimed: BTreeSet::new(),
        })
    }

    pub(in crate::mir::builder::resolved_lowering) fn binding_for(
        &self,
        key: LoopBindingKeyV1,
    ) -> Result<BindingRefV1, String> {
        self.plan
            .entries()
            .iter()
            .find(|entry| entry.recipe_binding() == key)
            .map(|entry| entry.binding())
            .ok_or_else(|| format!("[freeze:contract][direct_accum/binding_missing] key={key:?}"))
    }

    pub(in crate::mir::builder::resolved_lowering) fn read_entry_for_key(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        key: LoopBindingKeyV1,
    ) -> Result<ValueId, String> {
        let binding = self.binding_for(key)?;
        self.identity.read_entry(builder, phis, block, binding)
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish_effect_claims(
        self,
    ) -> Result<(), String> {
        for role in DirectAccumBindingEffectRoleV1::ALL {
            if !self.claimed.contains(&role) {
                return Err(format!(
                    "[freeze:contract][direct_accum/effect_unclaimed] role={role:?}"
                ));
            }
        }
        Ok(())
    }

    fn claim_read(
        &mut self,
        role: DirectAccumBindingEffectRoleV1,
        expected_key: LoopBindingKeyV1,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        let site = self
            .check_entry(role, expected_key, binding)?
            .site()
            .clone();
        self.identity.claim_variable_use_binding(&site, binding)?;
        let value = self.identity.read_entry(builder, phis, block, binding)?;
        self.claimed.insert(role);
        Ok(value)
    }

    fn claim_write(
        &mut self,
        role: DirectAccumBindingEffectRoleV1,
        expected_key: LoopBindingKeyV1,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        let site = self
            .check_entry(role, expected_key, binding)?
            .site()
            .clone();
        self.identity
            .define_assignment_exact(&site, binding, block, value)?;
        self.claimed.insert(role);
        Ok(())
    }

    fn check_entry(
        &self,
        role: DirectAccumBindingEffectRoleV1,
        expected_key: LoopBindingKeyV1,
        binding: BindingRefV1,
    ) -> Result<&crate::mir::loop_structural_facts::DirectAccumBindingEffectEntryV1, String> {
        if self.claimed.contains(&role) {
            return Err(format!(
                "[freeze:contract][direct_accum/effect_duplicate] role={role:?}"
            ));
        }
        let entry = self.plan.entry(role);
        if entry.recipe_binding() != expected_key || entry.binding() != binding {
            return Err(format!(
                "[freeze:contract][direct_accum/effect_mismatch] role={role:?}"
            ));
        }
        Ok(entry)
    }
}

impl DirectAccumBindingPortV1 for CanonicalDirectAccumBindingPort<'_, '_> {
    fn seed_input(
        &mut self,
        _builder: &mut MirBuilder,
        _binding: BindingRefV1,
        _block: BasicBlockId,
        _value: ValueId,
    ) -> Result<(), String> {
        // Canonical lowering already published the entry definition into the
        // function-owned identity/SSA session.  Re-defining it here would
        // create a second entry authority.
        Ok(())
    }

    fn read_binding(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        self.identity.read_entry(builder, phis, block, binding)
    }

    fn read_condition_induction(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        self.claim_read(
            DirectAccumBindingEffectRoleV1::ConditionInductionRead,
            LoopBindingKeyV1::new(0),
            builder,
            phis,
            binding,
            block,
        )
    }

    fn read_update_accumulator(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        self.claim_read(
            DirectAccumBindingEffectRoleV1::UpdateAccumulatorRead,
            LoopBindingKeyV1::new(1),
            builder,
            phis,
            binding,
            block,
        )
    }

    fn read_step_induction(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        self.claim_read(
            DirectAccumBindingEffectRoleV1::StepInductionRead,
            LoopBindingKeyV1::new(0),
            builder,
            phis,
            binding,
            block,
        )
    }

    fn write_update_accumulator(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.claim_write(
            DirectAccumBindingEffectRoleV1::UpdateAccumulatorWrite,
            LoopBindingKeyV1::new(1),
            binding,
            block,
            value,
        )
    }

    fn write_step_induction(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.claim_write(
            DirectAccumBindingEffectRoleV1::StepInductionWrite,
            LoopBindingKeyV1::new(0),
            binding,
            block,
            value,
        )
    }

    fn seal(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
    ) -> Result<(), String> {
        self.identity.seal_block(builder, phis, block, witness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::direct_accum_profile::issue_direct_accum_plan_v1;
    use crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_control_flow::verify_function_completion_v1;

    #[test]
    fn adapter_role_contract_is_named_and_keyed() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
            .expect("fixture resolves");
        let input = unit.root_function_input().expect("input");
        let body = input.source().root_body().expect("body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop");
        let completion = verify_function_completion_v1(input).expect("completion");
        let source = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("source");
        let profile = issue_direct_accum_plan_v1(input, loop_stmt, completion).expect("profile");
        let (input, _loop_stmt, _receipt, _prefix, _recipe, plan, _completion) =
            profile.into_parts();
        let mut identity = ResolvedSsaIdentityStateV2::new(input.function());
        let mut port = CanonicalDirectAccumBindingPort::new(
            &mut identity,
            &plan,
            input.owner(),
            &source.frame_key(),
        )
        .expect("adapter");

        for role in DirectAccumBindingEffectRoleV1::ALL {
            let entry = plan.entry(role);
            let expected_key = entry.recipe_binding();
            let expected_binding = entry.binding();
            port.check_entry(role, expected_key, expected_binding)
                .expect("exact role mapping");
        }
        assert_eq!(
            port.binding_for(LoopBindingKeyV1::new(0)),
            Ok(plan
                .entry(DirectAccumBindingEffectRoleV1::ConditionInductionRead)
                .binding())
        );
        assert!(port.finish_effect_claims().is_err());
    }

    #[test]
    fn adapter_rejects_duplicate_role_claims_before_identity_access() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
            .expect("fixture resolves");
        let input = unit.root_function_input().expect("input");
        let body = input.source().root_body().expect("body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop");
        let completion = verify_function_completion_v1(input).expect("completion");
        let source = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("source");
        let profile = issue_direct_accum_plan_v1(input, loop_stmt, completion).expect("profile");
        let (input, _loop_stmt, _receipt, _prefix, _recipe, plan, _completion) =
            profile.into_parts();
        let mut identity = ResolvedSsaIdentityStateV2::new(input.function());
        let mut port = CanonicalDirectAccumBindingPort::new(
            &mut identity,
            &plan,
            input.owner(),
            &source.frame_key(),
        )
        .expect("adapter");
        let role = DirectAccumBindingEffectRoleV1::ConditionInductionRead;
        let entry = plan.entry(role);
        port.claimed.insert(role);
        assert!(port
            .check_entry(role, entry.recipe_binding(), entry.binding())
            .is_err());
    }
}
