//! Canonical identity adapter for the bounded Nested Predicate physicalizer.
//!
//! This box owns only the nine resolver-issued effect claims. It borrows the
//! function-owned identity/SSA state and never creates a second reaching-value
//! map, PHI transaction, or route decision.

use std::collections::BTreeSet;

use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::nested_predicate_effect_plan::{
    NestedBindingEffectEntryV1, NestedBindingEffectRoleV1, VerifiedNestedBindingEffectPlanV1,
    VerifiedNestedBindingExecutionClaimsV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, ValueId};

pub(in crate::mir::builder::resolved_lowering) struct CanonicalNestedBindingPort<'plan, 'source> {
    identity: &'plan mut ResolvedSsaIdentityStateV2<'source>,
    plan: &'plan VerifiedNestedBindingEffectPlanV1,
    claimed: BTreeSet<NestedBindingEffectRoleV1>,
    child_activated: bool,
}

impl<'plan, 'source> CanonicalNestedBindingPort<'plan, 'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        identity: &'plan mut ResolvedSsaIdentityStateV2<'source>,
        claims: &'plan VerifiedNestedBindingExecutionClaimsV1,
        owner: FunctionOwnerIdV1,
        frame_key: &crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
    ) -> Result<Self, String> {
        let plan = claims.effect_plan();
        if plan.owner() != owner {
            return Err("[freeze:contract][nested_effect/owner_mismatch]".into());
        }
        if plan.frame_key() != frame_key {
            return Err("[freeze:contract][nested_effect/frame_mismatch]".into());
        }
        Ok(Self {
            identity,
            plan,
            claimed: BTreeSet::new(),
            child_activated: false,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) fn publish_prefix(
        &mut self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
        claims: &VerifiedNestedBindingExecutionClaimsV1,
    ) -> Result<(), String> {
        for binding in claims.prefix().initialized() {
            let value = loop_operation::emit_const_i64(builder, binding.initial())?;
            let published = self.identity.publish_declaration(
                binding.declaration_site(),
                binding.kind(),
                binding.name(),
                block,
                value,
            )?;
            if published != binding.binding() {
                return Err("[freeze:contract][nested_effect/prefix_binding_mismatch]".into());
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn activate_child(
        &mut self,
        claims: &VerifiedNestedBindingExecutionClaimsV1,
    ) -> Result<(), String> {
        if self.child_activated {
            return Err("[freeze:contract][nested_effect/child_duplicate]".into());
        }
        let binding = claims.prefix().uninitialized();
        let activated = self.identity.activate_declaration_without_value(
            binding.declaration_site(),
            binding.kind(),
            binding.name(),
        )?;
        if activated != binding.binding() {
            return Err("[freeze:contract][nested_effect/child_binding_mismatch]".into());
        }
        self.child_activated = true;
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn read(
        &mut self,
        role: NestedBindingEffectRoleV1,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        let (site, binding) = match self.plan.entry(role) {
            NestedBindingEffectEntryV1::Read(claim) => (claim.site(), claim.binding()),
            _ => {
                return Err(format!(
                    "[freeze:contract][nested_effect/read_role] role={role:?}"
                ))
            }
        };
        self.claim_once(role)?;
        self.identity.claim_variable_use_binding(site, binding)?;
        let value = self.identity.read_entry(builder, phis, block, binding)?;
        // Binding SSA may return a provisional PHI before its inputs are
        // patched.  This effect plan is i64-only, so publish that semantic
        // type at the operation boundary before arithmetic consumes it.
        loop_operation::publish_i64_value(builder, value)?;
        Ok(value)
    }

    pub(in crate::mir::builder::resolved_lowering) fn write_first(
        &mut self,
        builder: &mut MirBuilder,
        role: NestedBindingEffectRoleV1,
        block: BasicBlockId,
        value: i64,
    ) -> Result<(), String> {
        if role != NestedBindingEffectRoleV1::ChildInitializeWriteJ || !self.child_activated {
            return Err("[freeze:contract][nested_effect/first_assignment_order]".into());
        }
        let (site, binding, expected) = match self.plan.entry(role) {
            NestedBindingEffectEntryV1::FirstAssignment(claim) => {
                (claim.target_site(), claim.binding(), claim.value())
            }
            _ => return Err("[freeze:contract][nested_effect/first_assignment_role]".into()),
        };
        if expected != value {
            return Err("[freeze:contract][nested_effect/first_assignment_value]".into());
        }
        self.claim_once(role)?;
        let value = loop_operation::emit_const_i64(builder, value)?;
        self.identity
            .define_assignment_exact(site, binding, block, value)
    }

    pub(in crate::mir::builder::resolved_lowering) fn write_delta(
        &mut self,
        builder: &mut MirBuilder,
        role: NestedBindingEffectRoleV1,
        block: BasicBlockId,
        current: ValueId,
    ) -> Result<(), String> {
        let (site, binding, delta) = match self.plan.entry(role) {
            NestedBindingEffectEntryV1::Assignment(claim) => {
                (claim.target_site(), claim.binding(), claim.delta())
            }
            _ => {
                return Err(format!(
                    "[freeze:contract][nested_effect/write_role] role={role:?}"
                ))
            }
        };
        self.claim_once(role)?;
        let step = loop_operation::emit_const_i64(builder, delta)?;
        let next = loop_operation::emit_add_i64(builder, current, step)?;
        self.identity
            .define_assignment_exact(site, binding, block, next)
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish_effect_claims(
        &self,
    ) -> Result<(), String> {
        if self.claimed.len() != NestedBindingEffectRoleV1::ALL.len() {
            return Err(format!(
                "[freeze:contract][nested_effect/incomplete] claimed={}",
                self.claimed.len()
            ));
        }
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn seal(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
    ) -> Result<(), String> {
        self.identity.seal_block(builder, phis, block, witness)
    }

    fn claim_once(&mut self, role: NestedBindingEffectRoleV1) -> Result<(), String> {
        if !self.claimed.insert(role) {
            return Err(format!(
                "[freeze:contract][nested_effect/duplicate] role={role:?}"
            ));
        }
        Ok(())
    }
}
