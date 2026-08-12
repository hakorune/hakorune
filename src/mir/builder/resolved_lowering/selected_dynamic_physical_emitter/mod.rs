//! Family-native V2 physical emitter boundary for the selected Dynamic cohort.
//!
//! This module is a canary-only handoff. It consumes one admitted activation,
//! installs its exact site plans, opens the canonical unpublished owners inside
//! its scoped entry, and never opens a second Builder/CFG owner or activates
//! the production capability gate.

mod i64_const;
mod formal_header;
mod operation_cursor;
mod targets;
mod value_ledger;

use std::sync::Arc;

use crate::box_callable::provider_admission::PreparedAotExecutableAdmissionV1;
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::{
    DynamicV2I8EvidenceV1, DynamicV2NativePreflightLedgerV1, DynamicV2PhysicalBlockTargetV1,
    DynamicV2PhysicalScheduleRowV1, PreparedSelectedDynamicV2EmissionPlanV1,
};
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::{
    DynamicV2CompareI64CapabilityDemandV1, DynamicV2PhysicalCapabilityDispositionV1,
    DynamicV2TemporaryDischargeRowV1, PreparedSelectedDynamicV2AotActivationV1,
};
use crate::mir::builder::resolved_lowering::DynamicV2PhysicalScheduleSegmentV1;
use crate::mir::builder::MirBuilder;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::checked_callout::{CheckedCallOutPlanTableV1, CheckedCallOutSitePlanPairV1};
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::BasicBlockId;
use targets::{DynamicV2PhysicalTargetRoleV1, DynamicV2PhysicalTargetSetV1};

pub(in crate::mir) use i64_const::DynamicV2I64ProducerReceiptV1;
use value_ledger::{
    DynamicV2PhysicalValueLedgerRejectV1, DynamicV2PhysicalValueLedgerV1,
    DynamicV2PhysicalValueViewV1,
};
use formal_header::DynamicV2OpenedFormalHeaderV1;

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
    FormalHeader(String),
    PhysicalValueLedger(String),
    CheckedCallOutSitePlan(String),
    RecipeOperationCursor(String),
}

#[derive(Debug)]
struct DynamicV2PhysicalSessionBrandV1(Arc<()>);

/// Consuming, unpublished physical session for one selected V2 plan.
pub(in crate::mir) struct DynamicV2PhysicalEmissionSessionV1<'program, 'builder> {
    outer: Option<CanonicalFunctionLoweringSessionV1<'builder>>,
    canonical: Option<CanonicalSsaFunctionSessionV2<'program>>,
    demand: VerifiedAPrimeI64PhysicalDemandV1<'program>,
    schedule: Box<[DynamicV2PhysicalScheduleRowV1]>,
    ledger: DynamicV2NativePreflightLedgerV1,
    brand: DynamicV2PhysicalSessionBrandV1,
    targets: DynamicV2PhysicalTargetSetV1,
    formal_header: DynamicV2OpenedFormalHeaderV1,
    values: DynamicV2PhysicalValueLedgerV1,
    i8_evidence: Option<DynamicV2I8EvidenceV1>,
    compare_i64: DynamicV2CompareI64CapabilityDemandV1,
    cleanup: [DynamicV2TemporaryDischargeRowV1; 4],
    aot: PreparedAotExecutableAdmissionV1,
    disposition: DynamicV2PhysicalCapabilityDispositionV1,
}

/// Validate semantic authority and the canary evidence before opening the
/// Builder-owned function session. This private phase only co-seals existing
/// receipts; it does not issue a new semantic product.
fn validate_pre_session_authority<'program>(
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'program>,
    schedule: &[DynamicV2PhysicalScheduleRowV1],
    ledger: &mut DynamicV2NativePreflightLedgerV1,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'program>,
) -> Result<
    (
        CanonicalSsaFunctionSessionV2<'program>,
        DynamicV2I8EvidenceV1,
    ),
    DynamicV2I8EmitterRejectV1,
> {
    let canonical = demand
        .with_canonical_session_authority(|authority| {
            CanonicalSsaFunctionSessionV2::new_selected_dynamic(input, authority)
        })
        .map_err(DynamicV2I8EmitterRejectV1::SessionOpen)?;
    if canonical.owner() != demand.identity().owner() {
        return Err(DynamicV2I8EmitterRejectV1::OwnerMismatch);
    }
    let evidence = ledger
        .take_i8_evidence()
        .ok_or(DynamicV2I8EmitterRejectV1::MissingI8Evidence)?;
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
    Ok((canonical, evidence))
}

fn install_unpublished_function_header(
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    physical_header: &crate::mir::compiler::a_prime_i64_physical_capability::
        APrimePhysicalFunctionHeaderV1,
    function_name: String,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let declared_param_decls = physical_header.params().to_vec();
    let draft_builder = outer.builder_view_mut_for_lowering();
    draft_builder
        .function_state
        .resolved_binding_state
        .install(input.function())
        .map_err(|error| DynamicV2I8EmitterRejectV1::SessionOpen(error.to_string()))?;
    draft_builder
        .create_resolved_function_skeleton(
            function_name,
            &declared_param_decls,
            physical_header.return_type_name(),
            physical_header.effects(),
        )
        .map_err(DynamicV2I8EmitterRejectV1::SessionOpen)?;
    draft_builder.set_current_function_declared_signature(
        declared_param_decls,
        physical_header.return_type_name().map(str::to_owned),
    );
    draft_builder.set_current_function_runes(physical_header.attrs());
    draft_builder.set_current_function_declared_capability_uses(physical_header.uses());
    let function = draft_builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| {
            DynamicV2I8EmitterRejectV1::SessionOpen("selected function skeleton missing".to_owned())
        })?;
    CanonicalDirectStaticCallCapabilityV1::install_for_function(
        &mut function.metadata.canonical_direct_static_call_capabilities,
        true,
    )
    .map_err(|error| DynamicV2I8EmitterRejectV1::SessionOpen(error.to_string()))
}

fn install_checked_callout_site_plans(
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    site_plans: CheckedCallOutSitePlanPairV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let table = site_plans
        .consume(|i6, i7| {
            let mut table = CheckedCallOutPlanTableV1::default();
            table
                .admit(i6)
                .and_then(|()| table.admit(i7))
                .map(|()| table)
        })
        .map_err(|error| {
            DynamicV2I8EmitterRejectV1::CheckedCallOutSitePlan(format!("{error:?}"))
        })?;
    let builder = outer.builder_view_mut_for_lowering();
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| {
            DynamicV2I8EmitterRejectV1::CheckedCallOutSitePlan(
                "selected function skeleton missing while installing site plans".to_owned(),
            )
        })?;
    function.metadata.install_checked_callout_plan_table(table);
    Ok(())
}

fn open_unpublished_outer<'builder>(
    builder: &'builder mut MirBuilder,
    function_name: &str,
) -> CanonicalFunctionLoweringSessionV1<'builder> {
    builder.open_resolved_function_draft_seal_session_v1(function_name)
}

fn issue_targets_and_formal_header(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    schedule: &[DynamicV2PhysicalScheduleRowV1],
    outer_tail_target: DynamicV2PhysicalBlockTargetV1,
    source_relation: &crate::mir::compiler::dynamic_full_body_recipe::
        DynamicAPrimeI64SourceRelationViewV1<'_>,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<
    (DynamicV2PhysicalTargetSetV1, DynamicV2OpenedFormalHeaderV1),
    DynamicV2I8EmitterRejectV1,
> {
    let targets = DynamicV2PhysicalTargetSetV1::issue(
        canonical,
        outer.builder_view_mut_for_lowering(),
        brand,
        schedule,
        outer_tail_target,
    )
    .map_err(DynamicV2I8EmitterRejectV1::BlockAllocation)?;
    let formal_header = formal_header::open(
        canonical,
        outer.builder_view_mut_for_lowering(),
        source_relation,
        &targets,
        brand,
    )
    .map_err(DynamicV2I8EmitterRejectV1::FormalHeader)?;
    Ok((targets, formal_header))
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
        activation: PreparedSelectedDynamicV2AotActivationV1<'program>,
    ) -> Result<Self, DynamicV2I8EmitterRejectV1> {
        activation.consume_for_session(
            |plan, compare_i64, cleanup, aot, site_plans, disposition| {
                Self::begin_from_parts(
                    builder,
                    plan,
                    compare_i64,
                    cleanup,
                    aot,
                    site_plans,
                    disposition,
                )
            },
        )
    }

    fn begin_from_parts(
        builder: &'builder mut MirBuilder,
        plan: PreparedSelectedDynamicV2EmissionPlanV1<'program>,
        compare_i64: DynamicV2CompareI64CapabilityDemandV1,
        cleanup: [DynamicV2TemporaryDischargeRowV1; 4],
        aot: PreparedAotExecutableAdmissionV1,
        site_plans: CheckedCallOutSitePlanPairV1,
        disposition: DynamicV2PhysicalCapabilityDispositionV1,
    ) -> Result<Self, DynamicV2I8EmitterRejectV1> {
        let (demand, schedule, mut ledger) = plan.into_emitter_parts();
        let input = demand.input();
        let physical_header = demand.physical_function_header();
        let function_name = physical_header.catalog().physical_symbol().to_owned();
        operation_cursor::validate(&demand)
            .map_err(|error| {
                DynamicV2I8EmitterRejectV1::RecipeOperationCursor(format!("{error:?}"))
            })?;
        let (mut canonical, evidence) =
            validate_pre_session_authority(&demand, &schedule, &mut ledger, input)?;

        let mut outer = open_unpublished_outer(builder, &function_name);
        if let Err(error) = install_unpublished_function_header(
            &mut outer,
            input,
            physical_header,
            function_name.clone(),
        ) {
            return Self::reject_begin(outer, error);
        }
        if let Err(error) = install_checked_callout_site_plans(&mut outer, site_plans) {
            return Self::reject_begin(outer, error);
        }
        let brand = DynamicV2PhysicalSessionBrandV1(Arc::new(()));
        let (targets, formal_header) = match issue_targets_and_formal_header(
            &mut canonical,
            &mut outer,
            &schedule,
            ledger.outer_tail_target(),
            demand.source_relation(),
            &brand,
        ) {
            Ok(parts) => parts,
            Err(error) => return Self::reject_begin(outer, error),
        };
        let values = DynamicV2PhysicalValueLedgerV1::new(&brand);
        Ok(Self {
            outer: Some(outer),
            canonical: Some(canonical),
            demand,
            schedule,
            ledger,
            brand,
            targets,
            formal_header,
            values,
            i8_evidence: Some(evidence),
            compare_i64,
            cleanup,
            aot,
            disposition,
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
        let target = self
            .targets
            .with_role(DynamicV2PhysicalTargetRoleV1::BodyPrelude, |target| target);
        if !target.matches(&self.brand)
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
            &target,
            evidence,
            &self.brand,
            &mut self.values,
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
    pub(super) fn with_physical_value_for_test<R>(
        &self,
        result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
        callback: impl for<'a> FnOnce(&'a DynamicV2PhysicalValueViewV1) -> R,
    ) -> Result<R, DynamicV2PhysicalValueLedgerRejectV1> {
        self.values.with_value(
            result,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
            callback,
        )
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

    #[cfg(test)]
    pub(super) fn target_blocks_for_test(&self) -> [BasicBlockId; 6] {
        self.targets.blocks_for_test()
    }
}

#[cfg(test)]
mod tests;
