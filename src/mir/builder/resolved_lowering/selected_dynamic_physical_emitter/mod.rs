//! Family-native V2 physical emitter boundary for the selected Dynamic cohort.
//!
//! This module is the selected Dynamic handoff. It consumes one admitted
//! activation, installs its exact site plans, opens the canonical unpublished
//! owners inside its scoped entry, and never opens a second Builder/CFG owner
//! or publication path.

mod a_prime_callable_storage_layout;
mod a_prime_receipt;
mod assembly;
mod body_state_bridge;
mod callout_corridor;
mod continuation_backedge;
mod fault_terminals;
mod formal_header;
mod i64_const;
mod i8_i9_control;
mod inner_return_then;
mod lifecycle_terminal;
#[cfg(test)]
mod observation_fixture;
mod operation_cursor;
mod profile_close;
mod session_open;
mod targets;
mod value_ledger;

use std::sync::Arc;

use crate::box_callable::provider_admission::PreparedAotExecutableAdmissionV1;
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
#[cfg(test)]
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::DynamicV2PhysicalBlockTargetV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::{
    DynamicV2NativePreflightLedgerV1, DynamicV2PhysicalScheduleRowV1,
    PreparedSelectedDynamicV2EmissionPlanV1,
};
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::{
    DynamicV2CompareI64CapabilityDemandV1, DynamicV2TemporaryDischargeRowV1,
    DynamicV2UnpublishedSessionReadinessV1, PreparedSelectedDynamicV2AotActivationV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::checked_callout::CheckedCallOutSitePlanPairV1;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::target_capability::APrimeI64TargetStorageLayoutV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
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
    CallableStorageLayout(String),
}

#[derive(Debug)]
struct DynamicV2PhysicalSessionBrandV1(Arc<()>, FunctionOwnerIdV1);

impl DynamicV2PhysicalSessionBrandV1 {
    fn for_owner(owner: FunctionOwnerIdV1) -> Self {
        Self(Arc::new(()), owner)
    }

    fn owner(&self) -> FunctionOwnerIdV1 {
        self.1
    }

    fn matches(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) && self.1 == other.1
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
    targets: DynamicV2PhysicalTargetSetV1,
    formal_header: DynamicV2OpenedFormalHeaderV1,
    values: DynamicV2PhysicalValueLedgerV1,
    cleanup_cursor: Option<lifecycle_terminal::DynamicV2PhysicalCleanupCursorV1>,
    operation_census: operation_cursor::DynamicV2PhysicalOperationCensusV1,
    aot: PreparedAotExecutableAdmissionV1,
    lifecycle: lifecycle_terminal::DynamicV2PhysicalLifecycleTerminalPlanV1,
    callout_corridor: callout_corridor::DynamicV2CallOutCorridorV1,
    target_layout: APrimeI64TargetStorageLayoutV1,
}

impl<'program, 'builder> DynamicV2PhysicalEmissionSessionV1<'program, 'builder> {
    pub(in crate::mir::builder) fn dynamic_source(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::VerifiedSourceBackedDynamicCallableV1> {
        self.demand.dynamic_source()
    }

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
        target_layout: APrimeI64TargetStorageLayoutV1,
    ) -> Result<Self, DynamicV2I8EmitterRejectV1> {
        activation.consume_for_session(
            |plan, compare_i64, cleanup, formal_representation, aot, site_plans, readiness| {
                Self::begin_from_parts(
                    builder,
                    plan,
                    compare_i64,
                    cleanup,
                    formal_representation,
                    aot,
                    site_plans,
                    readiness,
                    target_layout,
                )
            },
        )
    }

    fn begin_from_parts(
        builder: &'builder mut MirBuilder,
        plan: PreparedSelectedDynamicV2EmissionPlanV1<'program>,
        compare_i64: DynamicV2CompareI64CapabilityDemandV1,
        cleanup: [DynamicV2TemporaryDischargeRowV1; 4],
        formal_representation:
            crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::
                DynamicV2APrimeFormalRepresentationPairV1,
        aot: PreparedAotExecutableAdmissionV1,
        site_plans: CheckedCallOutSitePlanPairV1,
        readiness: DynamicV2UnpublishedSessionReadinessV1,
        target_layout: APrimeI64TargetStorageLayoutV1,
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
            session_open::validate_pre_session_authority(&demand, &schedule, &mut ledger, input)?;
        readiness.consume_before_open();
        let mut outer = session_open::open_unpublished_outer(builder, &function_name);
        if let Err(error) = session_open::install_unpublished_function_header(
            &mut outer,
            input,
            physical_header,
            function_name.clone(),
        ) {
            return Self::reject_begin(outer, error);
        }
        let sites = match session_open::install_checked_callout_site_plans(&mut outer, site_plans) {
            Ok(sites) => sites,
            Err(error) => return Self::reject_begin(outer, error),
        };
        let brand = DynamicV2PhysicalSessionBrandV1::for_owner(demand.identity().owner());
        let (targets, formal_header) = match session_open::issue_targets_and_formal_header(
            &mut canonical,
            &mut outer,
            &schedule,
            ledger.outer_tail_target(),
            demand.source_relation(),
            &brand,
            formal_representation,
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
            &mut values,
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
            lifecycle,
            callout_corridor,
            target_layout,
        };
        Ok(session)
    }

    /// Close the unpublished Dynamic profile through the existing canonical
    /// Completion/DraftSeal owners. The returned draft is handed to the
    /// existing root collector; publication remains outside this emitter.
    pub(super) fn finish_unpublished_draft(
        mut self,
        inspect: impl FnOnce(
            &mut Self,
            &profile_close::DynamicV2PhysicalProfileCloseV1,
        ) -> Result<(), String>,
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
        if let Err(error) = self.operation_census.check_closed() {
            outer.discard_unpublished();
            return Err(DynamicV2I8EmitterRejectV1::RecipeOperationCursor(format!(
                "physical operation census drift: {error:?}"
            )));
        }
        let profile = match profile_close::emit(
            &mut canonical,
            &mut outer,
            &self.demand,
            &self.formal_header,
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
        self.canonical = Some(canonical);
        self.outer = Some(outer);
        if let Err(error) = inspect(&mut self, &profile) {
            self.canonical.take();
            self.outer
                .take()
                .expect("unpublished emitter must retain outer session")
                .discard_unpublished();
            return Err(DynamicV2I8EmitterRejectV1::DraftSeal(format!(
                "selected Dynamic body-state bridge rejected: {error}"
            )));
        }
        let canonical = self
            .canonical
            .take()
            .expect("unpublished emitter must retain canonical session");
        let mut outer = self
            .outer
            .take()
            .expect("unpublished emitter must retain outer session");
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
        let mut ready = match canonical
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
        let callable_storage_layout = match a_prime_callable_storage_layout::issue(
            &self.demand,
            &self.formal_header,
            &self.values,
            &receipt,
            &self.brand,
            &self.target_layout,
        ) {
            Ok(layout) => layout,
            Err(error) => {
                outer.discard_unpublished();
                return Err(DynamicV2I8EmitterRejectV1::CallableStorageLayout(format!(
                    "{error:?}"
                )));
            }
        };
        let projection = {
            let formal_parameters = self.formal_header.transport_rows();
            let expected_effects = self.demand.function_effects();
            let census = match ready.take_checked_callout_census() {
                Some(census) => census,
                None => {
                    outer.discard_unpublished();
                    return Err(DynamicV2I8EmitterRejectV1::DraftSeal(
                        "selected function missing canonical CheckedCallOut census".to_owned(),
                    ));
                }
            };
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
                function,
                formal_parameters,
                self.formal_header.physical_representation(),
                callable_storage_layout,
                self.target_layout,
                expected_effects,
                &census,
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
        let candidate =
            crate::mir::builder::resolved_lowering::SelectedDynamicCandidateMetadataV1::new(
                receipt, projection,
            );
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
        let prepared = match open.prepare_exact_two_with_candidate_metadata(&outer_site, candidate)
        {
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

    pub(in crate::mir::builder) fn observe_body_state(
        &self,
        state: &mut crate::mir::builder::normal_callable_semantic_lowering_state::
            CallableSemanticLoweringState,
        profile: &profile_close::DynamicV2PhysicalProfileCloseV1,
    ) -> Result<(), String> {
        body_state_bridge::observe(
            state,
            &self.demand,
            &self.formal_header,
            &self.targets,
            &self.callout_corridor,
            &self.values,
            profile,
            self.lifecycle.lease_slot(),
            &self.brand,
        )
    }

    /// Explicit terminal for an unpublished selected-Dynamic candidate.
    pub(super) fn discard_unpublished(mut self) {
        self.canonical.take();
        self.outer
            .take()
            .expect("unpublished emitter must retain outer session")
            .discard_unpublished();
    }

    #[cfg(test)]
    fn with_physical_value_for_test<R>(
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
    fn with_physical_value_for_test_as<R>(
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
    pub(super) fn i6_normal_block_for_test(&self) -> BasicBlockId {
        self.callout_corridor
            .with_i6_normal(|target| target.block())
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

pub(in crate::mir::builder) use assembly::{
    assemble_unpublished_selected_dynamic_w6, assemble_unpublished_selected_dynamic_w6_from_parts,
};

#[cfg(test)]
mod tests;
