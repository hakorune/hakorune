//! Source-backed, effect-free admission for the common-V2 Substring site.
//!
//! This is the one bridge between the landed S6C target plan and the neutral
//! CheckedCallOut vocabulary.  It issues no MIR instruction, runtime handle,
//! lease token, or V9 `ValueId`; the canonical session remains the physical
//! value owner.

use crate::mir::checked_callout::{
    CheckedCallOutAdmittedSiteInputV1, CheckedCallOutEntryIdV1, CheckedCallOutLeaseSlotIdV1,
    CheckedCallOutNormalShapeV1, CheckedCallOutSingleSitePlanRejectV1, CheckedCallOutSitePlanV1,
};
use crate::mir::loop_recipe_contract::PreparedLoopV2SubstringCallTargetPlanV1;
use crate::mir::loop_recipe_contract::SubstringCallTargetPlanRejectV1;
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::normal_callable_semantic_package::VerifiedS6CPhysicalFunctionEffectsV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::EffectMask;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2SubstringCallOutAdmissionRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    PhysicalEffectsMismatch,
    ProviderShapeMismatch,
    TargetShapeMismatch,
    Target(SubstringCallTargetPlanRejectV1),
    SitePlan(CheckedCallOutSingleSitePlanRejectV1),
    Callback(String),
}

/// Opaque lifecycle identity for one admitted Substring normal result.
///
/// It carries no runtime handle/token and cannot consume the runtime lease.
/// The future materializer gets only this callback-scoped identity; the
/// enclosing unpublished function transaction remains the compiler rollback
/// owner.
#[derive(Debug)]
pub(in crate::mir::builder) struct CommonV2SubstringEndObligationV1 {
    owner: FunctionOwnerIdV1,
    site: crate::mir::checked_callout::CheckedCallOutSiteIdV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2SubstringEndConsumerRefV1 {
    owner: FunctionOwnerIdV1,
    site: crate::mir::checked_callout::CheckedCallOutSiteIdV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

impl CommonV2SubstringEndConsumerRefV1 {
    pub(in crate::mir::builder) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn site(
        self,
    ) -> crate::mir::checked_callout::CheckedCallOutSiteIdV1 {
        self.site
    }

    pub(in crate::mir::builder) const fn result(
        self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.result
    }
}

impl CommonV2SubstringEndObligationV1 {
    fn with_consumer<R>(self, callback: impl FnOnce(CommonV2SubstringEndConsumerRefV1) -> R) -> R {
        callback(CommonV2SubstringEndConsumerRefV1 {
            owner: self.owner,
            site: self.site,
            result: self.result,
        })
    }
}

/// One move-only provider/site-plan/lifecycle admission.  The target plan is
/// retained as the source-backed provider relation; only a scoped borrow is
/// lent to the eventual materializer consumer.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCommonV2SubstringCallOutAdmissionV1 {
    target: PreparedLoopV2SubstringCallTargetPlanV1,
    site_plan: CheckedCallOutSitePlanV1,
    end: CommonV2SubstringEndObligationV1,
    invocation_brand: ModuleInvocationBrandV1,
}

impl PreparedCommonV2SubstringCallOutAdmissionV1 {
    pub(in crate::mir::builder) const fn invocation_brand(&self) -> ModuleInvocationBrandV1 {
        self.invocation_brand
    }

    pub(in crate::mir::builder) fn site_plan(&self) -> &CheckedCallOutSitePlanV1 {
        &self.site_plan
    }

    /// Consume the admission once.  The target and lifecycle identity cannot
    /// be split into independently reusable parts or escape as raw handles.
    pub(in crate::mir::builder) fn consume<R>(
        self,
        callback: impl FnOnce(
            &PreparedLoopV2SubstringCallTargetPlanV1,
            &CheckedCallOutSitePlanV1,
            CommonV2SubstringEndConsumerRefV1,
        ) -> R,
    ) -> R {
        let Self {
            target,
            site_plan,
            end,
            invocation_brand: _,
        } = self;
        end.with_consumer(|end_ref| callback(&target, &site_plan, end_ref))
    }
}

pub(in crate::mir::builder) fn issue_common_v2_s6c_substring_callout_admission_v1(
    target: PreparedLoopV2SubstringCallTargetPlanV1,
    physical_effects: &VerifiedS6CPhysicalFunctionEffectsV1,
    invocation_brand: ModuleInvocationBrandV1,
) -> Result<PreparedCommonV2SubstringCallOutAdmissionV1, CommonV2SubstringCallOutAdmissionRejectV1>
{
    if target.owner() != physical_effects.owner() {
        return Err(CommonV2SubstringCallOutAdmissionRejectV1::OwnerMismatch);
    }
    if physical_effects.effect_mask() != EffectMask::READ
        || physical_effects.external_call_count() != 2
    {
        return Err(CommonV2SubstringCallOutAdmissionRejectV1::PhysicalEffectsMismatch);
    }
    if target.item().raw() != 6 || target.block().raw() != 1 || target.result().raw() != 9 {
        return Err(CommonV2SubstringCallOutAdmissionRejectV1::TargetShapeMismatch);
    }
    let provider = target.provider();
    if provider.entry != crate::abi::text_scan_aot_export_facts::TextScanAotEntryIdV1::Substring
        || provider.symbol != crate::abi::text_scan_aot_export_facts::TEXT_SCAN_SYMBOL_SUBSTRING_V1
        || provider.arity != 2
        || provider.result_lane
            != crate::abi::text_scan_aot_export_facts::TextScanValueLaneV1::HostHandle
        || provider.lease
            != crate::abi::text_scan_aot_export_facts::TextScanLeaseCapabilityV1::EndAuthorized
        || provider.call_abi.abi_revision
            != crate::abi::text_scan_aot_export_facts::TEXT_SCAN_CALL_ABI_REVISION_V1
        || provider.call_abi.out_wire_revision
            != crate::abi::text_scan_aot_export_facts::TEXT_SCAN_CALL_OUT_WIRE_REVISION_V2
    {
        return Err(CommonV2SubstringCallOutAdmissionRejectV1::ProviderShapeMismatch);
    }
    let site_plan = CheckedCallOutSitePlanV1::from_admitted_single(
        CheckedCallOutAdmittedSiteInputV1 {
            entry: CheckedCallOutEntryIdV1::from_admitted(provider.entry as u32),
            call_abi_revision: provider.call_abi.abi_revision,
            wire_revision: provider.call_abi.out_wire_revision,
            normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                lease_slot: CheckedCallOutLeaseSlotIdV1::from_admitted(0),
            },
            effects: physical_effects.effect_mask(),
        },
        invocation_brand,
    )
    .map_err(CommonV2SubstringCallOutAdmissionRejectV1::SitePlan)?;
    let end = CommonV2SubstringEndObligationV1 {
        owner: target.owner(),
        site: site_plan.site_id(),
        result: target.result(),
    };
    Ok(PreparedCommonV2SubstringCallOutAdmissionV1 {
        target,
        site_plan,
        end,
        invocation_brand,
    })
}

#[cfg(test)]
mod tests {
    use super::issue_common_v2_s6c_substring_callout_admission_v1;
    use super::CommonV2SubstringEndObligationV1;
    use crate::mir::builder::CompilationContext;
    use crate::mir::checked_callout::CheckedCallOutSiteIdV1;
    use crate::mir::loop_recipe_contract::issue_s6c_v2_substring_call_target_plan_v1;
    use crate::mir::loop_recipe_contract::LoopValueKeyV1;
    use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
    use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::parser::{NyashParser, ParserBuildConfig};

    fn final_source() -> crate::parser::VerifiedFinalCallableProgramSourceV1 {
        let source = include_str!("../../../../apps/tests/scan_with_init_typed_ok_min.hako");
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            ParserBuildConfig::default(),
        )
        .expect("physical entry source");
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("source-backed transform");
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                transformed
            else {
                panic!("fixture must remain source-backed")
            };
            source
        })
    }

    #[test]
    fn end_obligation_has_one_callback_scoped_consumer() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = issuer.issue().expect("owner");
        let obligation = CommonV2SubstringEndObligationV1 {
            owner,
            site: CheckedCallOutSiteIdV1::from_test(0),
            result: LoopValueKeyV1::new(9),
        };
        let observed =
            obligation.with_consumer(|end| (end.owner(), end.site().as_u32(), end.result().raw()));
        assert_eq!(observed, (owner, 0, 9));
    }

    #[test]
    fn admission_projects_one_checked_substring_site_without_effect() {
        let mut resolver = FunctionSemanticResolverSessionV1::new(9912).expect("resolver");
        let package = issue_normal_callable_semantic_package_v1(&mut resolver, final_source())
            .expect("same-cohort package");
        let mut context = CompilationContext::new();
        let installed = package
            .prepare_install(&mut context)
            .expect("vacant catalog")
            .commit();
        let mut port = installed.begin_lowering(&context).expect("same catalog");

        port.with_s6c_common_v2_pre_session(|loan| {
            let owner = loan.callable().owner();
            let target = issue_s6c_v2_substring_call_target_plan_v1(loan.envelope(), owner)
                .expect("source-backed target");
            let admission = issue_common_v2_s6c_substring_callout_admission_v1(
                target,
                loan.callable().physical_effects(),
                ModuleInvocationBrandV1::legacy_test(),
            )
            .expect("checked single-site admission");
            assert_eq!(admission.site_plan().site_id().as_u32(), 0);
            assert_eq!(
                admission.invocation_brand(),
                ModuleInvocationBrandV1::legacy_test()
            );
            admission.consume(|target, site_plan, end| {
                assert_eq!(target.owner(), owner);
                assert_eq!(target.result().raw(), 9);
                assert_eq!(
                    site_plan.plan_stamp(),
                    ModuleInvocationBrandV1::legacy_test()
                );
                assert_eq!(end.owner(), owner);
                assert_eq!(end.result().raw(), 9);
            });
            Ok::<(), String>(())
        })
        .expect("one installed S6C callback");
        port.complete().expect("selected child coverage");
    }
}
