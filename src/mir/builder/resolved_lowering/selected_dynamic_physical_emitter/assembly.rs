//! Selected Dynamic assembly facade.
//!
//! The physical emitter owns the session; this small facade only connects the
//! existing demand, activation, DraftSeal, and collector owners. It does not
//! publish a second candidate or create a second physical route.

use crate::mir::builder::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
use crate::mir::builder::pinned_text_invocation_binding::PinnedTextCompileInvocationBindingRefV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter::{
    profile_close, DynamicV2PhysicalEmissionSessionV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::a_prime_i64_physical_capability::issue_selected_a_prime_i64_physical_demand_from_parts;
use crate::mir::normal_callable_semantic_package::{
    SelectedCallableLoweringInputRefV1, SelectedCatalogedCallableLoweringInputV1,
};

/// Assemble one unpublished selected Dynamic W6 candidate through the existing
/// package loan, invocation brand, physical session, DraftSeal, and collector
/// terminal. The adapter is the route owner; root publication remains owned by
/// the existing candidate/external-commit lifecycle.
pub(in crate::mir::builder) fn assemble_unpublished_selected_dynamic_w6<'program, 'builder>(
    builder: &'builder mut MirBuilder,
    module_port: &mut ModuleLoweringPortV1<'_>,
    binding: &PinnedTextCompileInvocationBindingRefV1<'_>,
    input: SelectedCatalogedCallableLoweringInputV1<'program>,
    inspect: impl FnOnce(
        &mut DynamicV2PhysicalEmissionSessionV1<'program, 'builder>,
        &profile_close::DynamicV2PhysicalProfileCloseV1,
    ) -> Result<(), String>,
) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, String> {
    let (input, admission, physical_header) = input.into_lowering_and_admission();
    assemble_unpublished_selected_dynamic_w6_from_parts(
        builder,
        module_port,
        binding,
        &input,
        admission,
        physical_header,
        inspect,
    )
}

pub(in crate::mir::builder) fn assemble_unpublished_selected_dynamic_w6_from_parts<
    'program,
    'builder,
>(
    builder: &'builder mut MirBuilder,
    module_port: &mut ModuleLoweringPortV1<'_>,
    binding: &PinnedTextCompileInvocationBindingRefV1<'_>,
    input: &SelectedCallableLoweringInputRefV1<'program>,
    admission: crate::mir::builder::NormalCatalogedBoxMethodDraftAdmissionV1,
    physical_header: Option<crate::mir::builder::CatalogedBoxMethodPhysicalHeaderProjectionV1>,
    inspect: impl FnOnce(
        &mut DynamicV2PhysicalEmissionSessionV1<'program, 'builder>,
        &profile_close::DynamicV2PhysicalProfileCloseV1,
    ) -> Result<(), String>,
) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, String> {
    let target_layout = binding
        .target_capability()
        .project_a_prime_i64_target_storage_layout();
    let demand =
        issue_selected_a_prime_i64_physical_demand_from_parts(input, admission, physical_header)
            .map_err(|error| format!("A-prime demand rejected: {error:?}"))?;
    let plan =
        crate::mir::builder::resolved_lowering::issue_selected_dynamic_v2_emission_plan(demand)
            .map_err(|error| format!("physical plan rejected: {error:?}"))?;
    let binding_brand = binding.brand();
    let brand = module_port
        .with_invocation_brand(|brand| {
            if brand == binding_brand {
                Ok(brand)
            } else {
                Err("collector brand and target binding diverged".to_owned())
            }
        })
        .map_err(|error| format!("collector brand unavailable: {error:?}"))?
        .map_err(|error| format!("target binding rejected: {error}"))?;
    let capability = crate::mir::builder::resolved_lowering::
        issue_selected_dynamic_v2_physical_capability_admission_from_brand(plan, brand)
        .map_err(|error| format!("physical capability rejected: {error:?}"))?;
    if capability.plan_stamp() != brand {
        return Err("collector brand and admission PlanStamp diverged".to_owned());
    }
    let activation = capability
        .prepare_aot_activation()
        .map_err(|error| format!("AOT activation rejected: {error:?}"))?;
    let session = DynamicV2PhysicalEmissionSessionV1::begin(builder, activation, target_layout)
        .map_err(|error| format!("physical session rejected: {error:?}"))?;
    let completed = session
        .finish_unpublished_draft(inspect)
        .map_err(|error| format!("DraftSeal rejected: {error:?}"))?;
    module_port
        .commit_cataloged_box_method_completed(completed)
        .map_err(|error| format!("collector admission rejected: {error}"))
}
