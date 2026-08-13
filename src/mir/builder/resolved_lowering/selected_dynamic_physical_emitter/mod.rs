//! Family-native V2 physical emitter boundary for the selected Dynamic cohort.
//!
//! This module is a canary-only handoff. It consumes one admitted activation,
//! installs its exact site plans, opens the canonical unpublished owners inside
//! its scoped entry, and never opens a second Builder/CFG owner or activates
//! the production capability gate.

mod a_prime_receipt;
mod callout_corridor;
mod continuation_backedge;
mod fault_terminals;
mod formal_header;
mod i64_const;
mod i8_i9_control;
mod inner_return_then;
mod lifecycle_terminal;
mod operation_cursor;
mod profile_close;
mod targets;
mod value_ledger;

use std::sync::Arc;

use crate::box_callable::provider_admission::PreparedAotExecutableAdmissionV1;
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
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
use crate::mir::compiler::a_prime_i64_physical_capability::issue_selected_a_prime_i64_physical_demand;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::normal_callable_semantic_package::SelectedCatalogedCallableLoweringInputV1;
#[cfg(test)]
use crate::mir::BasicBlockId;
use targets::DynamicV2PhysicalTargetSetV1;

use formal_header::DynamicV2OpenedFormalHeaderV1;
use value_ledger::{
    DynamicV2PhysicalValueLedgerRejectV1, DynamicV2PhysicalValueLedgerV1,
    DynamicV2PhysicalValueViewV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2I8EmitterRejectV1 {
    MissingI8Evidence,
    OwnerMismatch,
    TargetMismatch,
    BlockAllocation(String),
    ConstantEmission(String),
    SessionOpen(String),
    PhysicalHeader(String),
    FormalHeader(String),
    PhysicalValueLedger(String),
    CheckedCallOutSitePlan(String),
    RecipeOperationCursor(String),
    PhysicalCorridor(String),
    LifecycleTerminal(String),
    InnerReturn(String),
    ProfileClose(String),
    DraftSeal(String),
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
    cleanup_cursor: Option<lifecycle_terminal::DynamicV2PhysicalCleanupCursorV1>,
    operation_census: operation_cursor::DynamicV2PhysicalOperationCensusV1,
    aot: PreparedAotExecutableAdmissionV1,
    disposition: DynamicV2PhysicalCapabilityDispositionV1,
    lifecycle: lifecycle_terminal::DynamicV2PhysicalLifecycleTerminalPlanV1,
    callout_corridor: callout_corridor::DynamicV2CallOutCorridorV1,
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
) -> Result<callout_corridor::DynamicV2InstalledCallOutSitesV1, DynamicV2I8EmitterRejectV1> {
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
    let i6 = table
        .get(crate::mir::checked_callout::CheckedCallOutSiteIdV1(0))
        .ok_or_else(|| {
            DynamicV2I8EmitterRejectV1::CheckedCallOutSitePlan(
                "missing installed I6 site plan".to_owned(),
            )
        })?;
    let i7 = table
        .get(crate::mir::checked_callout::CheckedCallOutSiteIdV1(1))
        .ok_or_else(|| {
            DynamicV2I8EmitterRejectV1::CheckedCallOutSitePlan(
                "missing installed I7 site plan".to_owned(),
            )
        })?;
    let sites = callout_corridor::DynamicV2InstalledCallOutSitesV1::new(
        i6.site_id(),
        i7.site_id(),
        i6.normal_shape(),
        i7.normal_shape(),
    );
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
    Ok(sites)
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
) -> Result<(DynamicV2PhysicalTargetSetV1, DynamicV2OpenedFormalHeaderV1), DynamicV2I8EmitterRejectV1>
{
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
        let lifecycle = lifecycle_terminal::DynamicV2PhysicalLifecycleTerminalPlanV1::issue(
            &site_plans,
            &cleanup,
        )
        .map_err(|error| DynamicV2I8EmitterRejectV1::LifecycleTerminal(format!("{error:?}")))?;
        let mut cleanup_cursor =
            lifecycle_terminal::DynamicV2PhysicalCleanupCursorV1::issue(cleanup);
        let (demand, schedule, mut ledger) = plan.into_emitter_parts();
        let input = demand.input();
        let physical_header = demand.physical_function_header();
        let function_name = physical_header.catalog().physical_symbol().to_owned();
        let mut operation_census = operation_cursor::validate(&demand).map_err(|error| {
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
        let sites = match install_checked_callout_site_plans(&mut outer, site_plans) {
            Ok(sites) => sites,
            Err(error) => return Self::reject_begin(outer, error),
        };
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
        let mut values = DynamicV2PhysicalValueLedgerV1::new(&brand);
        let callout_corridor = match callout_corridor::emit(
            &mut canonical,
            &mut outer,
            &demand,
            &targets,
            &formal_header,
            &mut values,
            &brand,
            sites,
            evidence,
            compare_i64,
            &mut operation_census,
        ) {
            Ok(corridor) => corridor,
            Err(error) => return Self::reject_begin(outer, error),
        };
        if let Err(error) = fault_terminals::emit(
            &mut canonical,
            &mut outer,
            &callout_corridor,
            &lifecycle,
            &mut cleanup_cursor,
            &brand,
        ) {
            return Self::reject_begin(outer, error);
        }
        if let Err(error) = continuation_backedge::emit(
            &mut canonical,
            &mut outer,
            &demand,
            &formal_header,
            &targets,
            &callout_corridor,
            &lifecycle,
            &mut cleanup_cursor,
            &mut operation_census,
            &mut values,
            &brand,
        ) {
            return Self::reject_begin(outer, error);
        }
        if let Err(error) = inner_return_then::emit(
            &mut canonical,
            &mut outer,
            &demand,
            &targets,
            &callout_corridor,
            &lifecycle,
            &mut cleanup_cursor,
            &mut operation_census,
            &brand,
        ) {
            return Self::reject_begin(outer, error);
        }
        let session = Self {
            outer: Some(outer),
            canonical: Some(canonical),
            demand,
            schedule,
            ledger,
            brand,
            targets,
            formal_header,
            values,
            cleanup_cursor: Some(cleanup_cursor),
            operation_census,
            aot,
            disposition,
            lifecycle,
            callout_corridor,
        };
        Ok(session)
    }

    /// Close the unpublished Dynamic profile through the existing canonical
    /// Completion/DraftSeal owners. The returned draft is not collected or
    /// published; this is the final physical-session canary only.
    pub(super) fn finish_unpublished_draft(
        mut self,
    ) -> Result<
        crate::mir::builder::resolved_lowering::CompletedCatalogedBoxCallableDraftV1,
        DynamicV2I8EmitterRejectV1,
    > {
        let mut canonical = self.canonical.take().ok_or_else(|| {
            DynamicV2I8EmitterRejectV1::DraftSeal("canonical session missing".into())
        })?;
        let mut outer = self
            .outer
            .take()
            .ok_or_else(|| DynamicV2I8EmitterRejectV1::DraftSeal("outer session missing".into()))?;
        let cleanup_cursor = match self.cleanup_cursor.take() {
            Some(cursor) => cursor,
            None => {
                outer.discard_unpublished();
                return Err(DynamicV2I8EmitterRejectV1::LifecycleTerminal(
                    "physical cleanup cursor missing".to_owned(),
                ));
            }
        };
        if let Err(error) = cleanup_cursor.close() {
            outer.discard_unpublished();
            return Err(DynamicV2I8EmitterRejectV1::LifecycleTerminal(format!(
                "physical cleanup cursor drift: {error:?}"
            )));
        }
        let operation_census = self.operation_census;
        if let Err(error) = operation_census.close() {
            outer.discard_unpublished();
            return Err(DynamicV2I8EmitterRejectV1::RecipeOperationCursor(format!(
                "physical operation census drift: {error:?}"
            )));
        }
        let profile = match profile_close::emit(
            &mut canonical,
            &mut outer,
            &self.demand,
            &self.targets,
            &self.callout_corridor,
            &self.lifecycle,
            &self.brand,
        ) {
            Ok(profile) => profile,
            Err(error) => {
                outer.discard_unpublished();
                return Err(error);
            }
        };
        let after = self
            .targets
            .with_role(targets::DynamicV2PhysicalTargetRoleV1::After, |target| {
                target.block()
            });
        let profile_close =
            match crate::mir::builder::resolved_lowering::canonical_ssa::finish_profile_close(
                self.demand.identity().owner(),
                after,
                || profile.finish(self.demand.identity().owner(), after),
            ) {
                Ok(close) => close,
                Err(error) => {
                    outer.discard_unpublished();
                    return Err(DynamicV2I8EmitterRejectV1::ProfileClose(error));
                }
            };
        let ready = match canonical
            .finish_for_draft_seal(outer.builder_view_mut_for_lowering(), profile_close)
        {
            Ok(ready) => ready,
            Err(error) => {
                outer.discard_unpublished();
                return Err(DynamicV2I8EmitterRejectV1::DraftSeal(format!("{error:?}")));
            }
        };
        let receipt = match a_prime_receipt::issue(
            &self.demand,
            &self.formal_header,
            &self.callout_corridor,
            &ready,
            &self.brand,
        ) {
            Ok(receipt) => receipt,
            Err(error) => {
                outer.discard_unpublished();
                return Err(error);
            }
        };
        let projection = {
            let function = match outer
                .builder_view_mut_for_lowering()
                .function_state
                .current_function
                .as_ref()
            {
                Some(function) => function,
                None => {
                    outer.discard_unpublished();
                    return Err(DynamicV2I8EmitterRejectV1::DraftSeal(
                        "selected function missing while projecting AOT metadata".to_owned(),
                    ));
                }
            };
            crate::box_callable::provider_admission::project_dynamic_v2_aot_call_metadata(
                &self.aot,
                &receipt,
                function.metadata.checked_callout_site_plan_table(),
            )
            .map_err(|error| {
                DynamicV2I8EmitterRejectV1::DraftSeal(format!(
                    "Dynamic AOT metadata projection rejected: {error:?}"
                ))
            })
        };
        let projection = match projection {
            Ok(projection) => projection,
            Err(error) => {
                outer.discard_unpublished();
                return Err(error);
            }
        };
        let candidate = crate::mir::builder::resolved_lowering::
            SelectedDynamicCandidateMetadataV1::new(receipt, projection);
        let Some(outer_site) = self
            .demand
            .source_relation()
            .completion_sites()
            .get(1)
            .copied()
            .cloned()
        else {
            outer.discard_unpublished();
            return Err(DynamicV2I8EmitterRejectV1::DraftSeal(
                "outer Completion site missing".into(),
            ));
        };
        let open = ready.open(outer);
        let prepared = match open.prepare_exact_two_with_candidate_metadata(&outer_site, candidate) {
            Ok(prepared) => prepared,
            Err(rejected) => {
                let detail = format!("{:?}", rejected.error());
                rejected.discard();
                return Err(DynamicV2I8EmitterRejectV1::DraftSeal(format!(
                    "exact-two DraftSeal preparation rejected: {detail}"
                )));
            }
        };
        let completed = prepared.commit();
        Ok(crate::mir::builder::resolved_lowering::CompletedCatalogedBoxCallableDraftV1::from_admission(
            completed,
            self.demand.physical_function_header().catalog(),
        ))
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
    pub(super) fn with_physical_value_for_test_as<R>(
        &self,
        result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
        representation: DynamicV2PhysicalRepresentationV1,
        callback: impl FnOnce(&DynamicV2PhysicalValueViewV1) -> R,
    ) -> Result<R, DynamicV2PhysicalValueLedgerRejectV1> {
        self.values
            .with_value_for_test(result, representation, callback)
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

    #[cfg(test)]
    pub(super) fn i7_normal_block_for_test(&self) -> BasicBlockId {
        self.callout_corridor
            .with_i7_normal(|target| target.block())
    }

    #[cfg(test)]
    pub(super) fn i6_fault_block_for_test(&self) -> BasicBlockId {
        self.callout_corridor.with_i6_fault(|target| target.block())
    }

    #[cfg(test)]
    pub(super) fn i7_fault_block_for_test(&self) -> BasicBlockId {
        self.callout_corridor.with_i7_fault(|target| target.block())
    }
}

/// Assemble one unpublished selected Dynamic W6 canary through the existing
/// package loan, invocation brand, physical session, DraftSeal, and collector
/// terminal. This is an orchestration seam only: no production caller or
/// module publication is opened here.
pub(super) fn assemble_unpublished_selected_dynamic_w6<'program, 'builder>(
    builder: &'builder mut MirBuilder,
    module_port: &mut ModuleLoweringPortV1<'_>,
    input: SelectedCatalogedCallableLoweringInputV1<'program>,
    inspect: impl FnOnce(
        &mut DynamicV2PhysicalEmissionSessionV1<'program, 'builder>,
    ) -> Result<(), String>,
) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, String> {
    let demand = issue_selected_a_prime_i64_physical_demand(input)
        .map_err(|error| format!("A-prime demand rejected: {error:?}"))?;
    let plan =
        crate::mir::builder::resolved_lowering::issue_selected_dynamic_v2_emission_plan(demand)
            .map_err(|error| format!("physical plan rejected: {error:?}"))?;
    let (capability, brand) = module_port
        .with_invocation_brand(|brand| {
            (
                crate::mir::builder::resolved_lowering::
                    issue_selected_dynamic_v2_physical_capability_admission_from_brand(
                        plan, brand,
                    ),
                brand,
            )
        })
        .map_err(|error| format!("collector brand unavailable: {error:?}"))?;
    let capability =
        capability.map_err(|error| format!("physical capability rejected: {error:?}"))?;
    if capability.plan_stamp() != brand {
        return Err("collector brand and admission PlanStamp diverged".to_owned());
    }
    let activation = capability
        .prepare_aot_activation()
        .map_err(|error| format!("AOT activation rejected: {error:?}"))?;
    let mut session = DynamicV2PhysicalEmissionSessionV1::begin(builder, activation)
        .map_err(|error| format!("physical session rejected: {error:?}"))?;
    if let Err(error) = inspect(&mut session) {
        session.discard_unpublished();
        return Err(error);
    }
    let completed = session
        .finish_unpublished_draft()
        .map_err(|error| format!("DraftSeal rejected: {error:?}"))?;
    module_port
        .commit_cataloged_box_method_completed(completed)
        .map_err(|error| format!("collector admission rejected: {error}"))
}

#[cfg(test)]
mod tests;
