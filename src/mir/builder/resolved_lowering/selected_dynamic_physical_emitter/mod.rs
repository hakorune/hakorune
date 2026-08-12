//! Family-native V2 physical emitter boundary for the selected Dynamic cohort.
//!
//! This module is a canary-only handoff. It consumes a preflight plan, opens
//! the canonical unpublished owners inside its scoped entry, and never opens a
//! second Builder/CFG owner or activates the production capability gate.

mod i64_const;

use std::sync::Arc;

use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::{
    DynamicV2I8EvidenceV1, DynamicV2NativePreflightLedgerV1, DynamicV2PhysicalBlockTargetV1,
    DynamicV2PhysicalScheduleRowV1, PreparedSelectedDynamicV2EmissionPlanV1,
};
use crate::mir::builder::resolved_lowering::DynamicV2PhysicalScheduleSegmentV1;
use crate::mir::builder::MirBuilder;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::BasicBlockId;

pub(in crate::mir) use i64_const::DynamicV2I64ProducerReceiptV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2I8EmitterRejectV1 {
    MissingI8Evidence,
    OwnerMismatch,
    TargetMismatch,
    DuplicateI8Emission,
    BlockAllocation(String),
    ConstantEmission(String),
    SessionOpen(String),
    PhysicalHeader(String),
}

#[derive(Debug)]
struct DynamicV2PhysicalSessionBrandV1(Arc<()>);

#[derive(Debug)]
struct DynamicV2OpaqueBodyPreludeTargetV1 {
    brand: Arc<()>,
    block: BasicBlockId,
}

impl DynamicV2OpaqueBodyPreludeTargetV1 {
    fn matches(&self, brand: &DynamicV2PhysicalSessionBrandV1) -> bool {
        Arc::ptr_eq(&self.brand, &brand.0)
    }
}

/// Consuming, unpublished physical session for one selected V2 plan.
pub(in crate::mir) struct DynamicV2PhysicalEmissionSessionV1<'program, 'builder> {
    outer: Option<CanonicalFunctionLoweringSessionV1<'builder>>,
    canonical: Option<CanonicalSsaFunctionSessionV2<'program>>,
    demand: VerifiedAPrimeI64PhysicalDemandV1<'program>,
    schedule: Box<[DynamicV2PhysicalScheduleRowV1]>,
    ledger: DynamicV2NativePreflightLedgerV1,
    brand: DynamicV2PhysicalSessionBrandV1,
    body_prelude_target: DynamicV2OpaqueBodyPreludeTargetV1,
    i8_evidence: Option<DynamicV2I8EvidenceV1>,
}

impl<'program, 'builder> DynamicV2PhysicalEmissionSessionV1<'program, 'builder> {
    fn reject_begin(
        outer: CanonicalFunctionLoweringSessionV1<'builder>,
        error: DynamicV2I8EmitterRejectV1,
    ) -> Result<Self, DynamicV2I8EmitterRejectV1> {
        outer.discard_unpublished();
        Err(error)
    }

    /// Consume the plan and open the canonical unpublished owners internally.
    /// The final Dynamic program lends only a scoped authority view; the
    /// canonical session snapshots the completion/control expectations before
    /// this method returns, so no semantic borrow escapes the session.
    pub(super) fn begin(
        builder: &'builder mut MirBuilder,
        plan: PreparedSelectedDynamicV2EmissionPlanV1<'program>,
    ) -> Result<Self, DynamicV2I8EmitterRejectV1> {
        let (demand, schedule, mut ledger) = plan.into_emitter_parts();
        let input = demand.input();
        // The A-prime demand owns the single catalog-backed physical-header
        // admission.  This emitter only borrows its checked projection; it
        // never re-seals the selected key or reconstructs a raw AST header.
        let physical_header = demand.physical_function_header();
        let function_name = physical_header.catalog().physical_symbol().to_owned();
        let declared_param_decls = physical_header.params().to_vec();

        // Validate all borrowed semantic/control authority and the canary
        // evidence before opening any Builder-owned session or skeleton.
        let mut canonical = match demand.with_canonical_session_authority(|authority| {
            CanonicalSsaFunctionSessionV2::new_selected_dynamic(input, authority)
        }) {
            Ok(canonical) => canonical,
            Err(error) => {
                return Err(DynamicV2I8EmitterRejectV1::SessionOpen(error));
            }
        };
        if canonical.owner() != demand.identity().owner() {
            return Err(DynamicV2I8EmitterRejectV1::OwnerMismatch);
        }
        let evidence = match ledger.take_i8_evidence() {
            Some(evidence) => evidence,
            None => return Err(DynamicV2I8EmitterRejectV1::MissingI8Evidence),
        };
        if schedule
            .iter()
            .filter(|row| row.item() == evidence.item())
            .count()
            != 1
            || evidence.segment() != DynamicV2PhysicalScheduleSegmentV1::Prelude
            || evidence.target() != DynamicV2PhysicalBlockTargetV1::BodyPrelude
            || ledger.outer_tail_target() != DynamicV2PhysicalBlockTargetV1::After
        {
            return Err(DynamicV2I8EmitterRejectV1::TargetMismatch);
        }

        let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
        let setup = (|| -> Result<(), DynamicV2I8EmitterRejectV1> {
            let draft_builder = outer.builder_view_mut_for_lowering();
            draft_builder
                .function_state
                .resolved_binding_state
                .install(input.function())
                .map_err(|error| DynamicV2I8EmitterRejectV1::SessionOpen(error.to_string()))?;
            draft_builder
                .create_resolved_function_skeleton(
                    function_name.clone(),
                    &declared_param_decls,
                    physical_header.return_type_name(),
                    physical_header.effects(),
                )
                .map_err(DynamicV2I8EmitterRejectV1::SessionOpen)?;
            draft_builder.set_current_function_declared_signature(
                declared_param_decls.clone(),
                physical_header.return_type_name().map(str::to_owned),
            );
            draft_builder.set_current_function_runes(physical_header.attrs());
            draft_builder
                .set_current_function_declared_capability_uses(physical_header.uses());
            let function = draft_builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    DynamicV2I8EmitterRejectV1::SessionOpen(
                        "selected function skeleton missing".to_owned(),
                    )
                })?;
            CanonicalDirectStaticCallCapabilityV1::install_for_function(
                &mut function.metadata.canonical_direct_static_call_capabilities,
                true,
            )
            .map_err(|error| DynamicV2I8EmitterRejectV1::SessionOpen(error.to_string()))?;
            Ok(())
        })();
        if let Err(error) = setup {
            return Self::reject_begin(outer, error);
        }
        let brand = DynamicV2PhysicalSessionBrandV1(Arc::new(()));
        let prelude_block =
            match canonical.create_unpublished_block(outer.builder_view_mut_for_lowering()) {
                Ok(block) => block,
                Err(error) => {
                    return Self::reject_begin(
                        outer,
                        DynamicV2I8EmitterRejectV1::BlockAllocation(error),
                    )
                }
            };
        let target_brand = Arc::clone(&brand.0);
        Ok(Self {
            outer: Some(outer),
            canonical: Some(canonical),
            demand,
            schedule,
            ledger,
            brand,
            body_prelude_target: DynamicV2OpaqueBodyPreludeTargetV1 {
                brand: target_brand,
                block: prelude_block,
            },
            i8_evidence: Some(evidence),
        })
    }

    /// Emit exactly one I8 leaf. A failure consumes the evidence and cannot be
    /// retried; the caller must discard the unpublished session.
    pub(super) fn emit_i8_const(
        &mut self,
    ) -> Result<DynamicV2I64ProducerReceiptV1<'_>, DynamicV2I8EmitterRejectV1> {
        let evidence = self
            .i8_evidence
            .take()
            .ok_or(DynamicV2I8EmitterRejectV1::DuplicateI8Emission)?;
        if !self.body_prelude_target.matches(&self.brand)
            || self
                .schedule
                .iter()
                .filter(|row| row.item() == evidence.item())
                .count()
                != 1
        {
            return Err(DynamicV2I8EmitterRejectV1::TargetMismatch);
        }
        let outer = self
            .outer
            .as_mut()
            .ok_or(DynamicV2I8EmitterRejectV1::TargetMismatch)?;
        i64_const::emit(
            outer.builder_view_mut_for_lowering(),
            &self.body_prelude_target,
            evidence,
            &self.brand,
        )
    }

    /// Explicit terminal for the unpublished canary.
    pub(super) fn discard_unpublished(mut self) {
        self.canonical.take();
        self.outer
            .take()
            .expect("unpublished emitter must retain outer session")
            .discard_unpublished();
    }

    #[cfg(test)]
    pub(super) fn current_instruction_count(&self) -> usize {
        self.outer
            .as_ref()
            .expect("canary session open")
            .builder_view()
            .current_function_instructions()
            .len()
    }
}

#[cfg(test)]
mod tests;
