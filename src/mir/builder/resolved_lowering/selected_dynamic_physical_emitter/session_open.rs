use super::callout_corridor;
use super::formal_header;
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::{
    DynamicV2I8EvidenceV1, DynamicV2NativePreflightLedgerV1, DynamicV2PhysicalBlockTargetV1,
    DynamicV2PhysicalScheduleRowV1,
};
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2APrimeFormalRepresentationPairV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter::targets::DynamicV2PhysicalTargetSetV1;
use crate::mir::builder::resolved_lowering::DynamicV2PhysicalScheduleSegmentV1;
use crate::mir::builder::MirBuilder;
use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::checked_callout::{CheckedCallOutPlanTableV1, CheckedCallOutSitePlanPairV1};
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;

/// Validate semantic authority and selected activation evidence before opening the
/// Builder-owned function session. This private phase only co-seals existing
/// receipts; it does not issue a new semantic product.
pub(super) fn validate_pre_session_authority<'program>(
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

pub(super) fn install_unpublished_function_header(
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

pub(super) fn install_checked_callout_site_plans(
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    site_plans: CheckedCallOutSitePlanPairV1,
) -> Result<callout_corridor::DynamicV2InstalledCallOutSitesV1, DynamicV2I8EmitterRejectV1> {
    let (table, i6_site, i7_site) = site_plans
        .consume(|i6, i7| {
            let i6_site = i6.site_id();
            let i7_site = i7.site_id();
            let mut table = CheckedCallOutPlanTableV1::default();
            table
                .admit(i6)
                .and_then(|()| table.admit(i7))
                .map(|()| (table, i6_site, i7_site))
        })
        .map_err(|error| {
            DynamicV2I8EmitterRejectV1::CheckedCallOutSitePlan(format!("{error:?}"))
        })?;
    let i6 = table.get(i6_site).ok_or_else(|| {
        DynamicV2I8EmitterRejectV1::CheckedCallOutSitePlan(
            "missing installed I6 site plan".to_owned(),
        )
    })?;
    let i7 = table.get(i7_site).ok_or_else(|| {
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

pub(super) fn open_unpublished_outer<'builder>(
    builder: &'builder mut MirBuilder,
    function_name: &str,
) -> CanonicalFunctionLoweringSessionV1<'builder> {
    builder.open_resolved_function_draft_seal_session_v1(function_name)
}

pub(super) fn issue_targets_and_formal_header(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    schedule: &[DynamicV2PhysicalScheduleRowV1],
    outer_tail_target: DynamicV2PhysicalBlockTargetV1,
    source_relation: &crate::mir::compiler::dynamic_full_body_recipe::
        DynamicAPrimeI64SourceRelationViewV1<'_>,
    brand: &DynamicV2PhysicalSessionBrandV1,
    formal_representation: DynamicV2APrimeFormalRepresentationPairV1,
) -> Result<
    (
        DynamicV2PhysicalTargetSetV1,
        super::formal_header::DynamicV2OpenedFormalHeaderV1,
    ),
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
        formal_representation,
    )
    .map_err(DynamicV2I8EmitterRejectV1::FormalHeader)?;
    Ok((targets, formal_header))
}
