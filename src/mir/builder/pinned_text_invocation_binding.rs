//! Session-owned physical identity for the pinned-Text ingress.
//!
//! The module invocation brand and the compile-target capability have
//! different issuers and meanings.  This module relates them only through a
//! live `ModuleBuilderInvocationSessionV1`; it never compares their numeric
//! ordinals or exposes a detached target/brand pair.

use super::module_invocation_session::ModuleBuilderInvocationSessionV1;
use super::InvocationBranded;
use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::compiler::common_v2_physical_function_skeleton::PreparedPhysicalEntrySessionInputV1;
use crate::mir::compiler::pinned_text_backend_frame::{
    issue_pinned_text_backend_frame_contract_v1, PinnedTextBackendFrameBorrowV1,
    PinnedTextBackendFrameContractIssueV1, PinnedTextBackendFrameContractV1,
};
use crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1;
use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::runtime::text_formal_residence::residence_abi_layout_v1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PinnedTextInvocationBindingIssueV1 {
    AlreadyInstalled,
    MissingTarget,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum PinnedTextPhysicalEntryIngressRejectV1 {
    Invocation(PinnedTextInvocationBindingIssueV1),
    ForeignInvocationBrand,
    Admission(String),
    BackendFrame(PinnedTextBackendFrameContractIssueV1),
}

/// A non-copyable, callback-scoped relation between one module session and
/// its explicitly issued pinned-Text compile target.
#[must_use = "a pinned-Text invocation binding must stay within its callback"]
#[derive(Debug)]
pub(in crate::mir) struct PinnedTextCompileInvocationBindingRefV1<'session> {
    pub(in crate::mir) brand: ModuleInvocationBrandV1,
    pub(in crate::mir) target: &'session PinnedTextCompileTargetCapabilityV1,
}

impl<'session> PinnedTextCompileInvocationBindingRefV1<'session> {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) const fn target_capability(
        &self,
    ) -> &'session PinnedTextCompileTargetCapabilityV1 {
        self.target
    }

    /// Consume the prepared S6C physical entry once and issue the existing
    /// backend-frame contract before any DraftSeal handoff.  The signature is
    /// adapted from the same retained S6C loan; no free target/signature pair
    /// can be constructed by the caller.
    pub(in crate::mir) fn prepare_physical_entry_ingress<'loan, 'source, 'join>(
        self,
        prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'loan, 'source, 'join>>,
        plans: &PinnedTextAccessPlanTableV1,
    ) -> Result<
        PreparedPinnedTextPhysicalEntryIngressV1<'session, 'loan, 'source, 'join>,
        PinnedTextPhysicalEntryIngressRejectV1,
    > {
        if prepared.brand() != self.brand {
            return Err(PinnedTextPhysicalEntryIngressRejectV1::ForeignInvocationBrand);
        }
        let brand = prepared.brand();
        let mut payload = prepared.into_payload();
        let frame = payload
            .with_admission(|_, _, _, loan| {
                let signature = ResolvedCallablePhysicalSignatureLoanV1::from_s6c_row(
                    loan.callable().signature(),
                );
                Ok(issue_pinned_text_backend_frame_contract_v1(
                    &signature,
                    plans,
                    residence_abi_layout_v1(),
                    self.target,
                )
                .map_err(PinnedTextPhysicalEntryIngressRejectV1::BackendFrame))
            })
            .map_err(PinnedTextPhysicalEntryIngressRejectV1::Admission)??;
        Ok(PreparedPinnedTextPhysicalEntryIngressV1 {
            prepared: InvocationBranded::from_source(brand, payload),
            binding: self,
            frame,
        })
    }
}

/// Affine, compiler-only ingress product.  It owns the prepared S6C shell and
/// the frame contract while borrowing the session-owned target/brand binding.
/// No runtime frame, pointer, token, or MIR effect is carried here.
#[must_use = "a physical-entry ingress must be consumed by its canonical owner"]
pub(in crate::mir) struct PreparedPinnedTextPhysicalEntryIngressV1<'session, 'loan, 'source, 'join>
{
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'loan, 'source, 'join>>,
    binding: PinnedTextCompileInvocationBindingRefV1<'session>,
    frame: PinnedTextBackendFrameContractV1,
}

impl<'session, 'loan, 'source, 'join>
    PreparedPinnedTextPhysicalEntryIngressV1<'session, 'loan, 'source, 'join>
{
    pub(in crate::mir) const fn invocation_brand(&self) -> ModuleInvocationBrandV1 {
        self.binding.brand()
    }

    pub(in crate::mir) fn frame(&self) -> PinnedTextBackendFrameBorrowV1<'_> {
        self.frame.borrow()
    }

    pub(in crate::mir) fn into_prepared(
        self,
    ) -> InvocationBranded<PreparedPhysicalEntrySessionInputV1<'loan, 'source, 'join>> {
        self.prepared
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::{BuilderInvocationConfigV1, CompilationContext, MirBuilder};
    use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
    use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
    use crate::mir::compiler::target_capability::{
        PinnedTextCompileTargetCapabilityIssuerV1, PinnedTextCompileTargetProfileV1,
    };
    use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
    use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

    fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            ParserBuildConfig::default(),
        )
        .expect("physical ingress source");
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
    fn targetless_session_rejects_selected_binding_and_duplicate_install() {
        let live = MirBuilder::new();
        let config = BuilderInvocationConfigV1::snapshot_for_canonical(&live, None);
        let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
        assert_eq!(
            session
                .with_pinned_text_invocation_binding(|_| ())
                .unwrap_err(),
            PinnedTextInvocationBindingIssueV1::MissingTarget
        );

        let profile = PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1;
        let target = PinnedTextCompileTargetCapabilityIssuerV1::issue(profile).unwrap();
        session
            .install_pinned_text_target_capability(Some(target))
            .expect("first target install");
        let second = PinnedTextCompileTargetCapabilityIssuerV1::issue(profile).unwrap();
        assert_eq!(
            session.install_pinned_text_target_capability(Some(second)),
            Err(PinnedTextInvocationBindingIssueV1::AlreadyInstalled)
        );
        session.with_builder_and_pinned_text_invocation_binding(|_, binding| {
            let binding = binding.expect("installed target binding");
            assert_eq!(binding.brand(), ModuleInvocationBrandV1::legacy_test());
            assert_eq!(binding.target_capability().profile(), profile);
        });
    }

    #[test]
    fn s6c_loan_and_session_target_issue_one_pre_draftseal_frame() {
        let mut resolver = FunctionSemanticResolverSessionV1::new(1_307).expect("resolver");
        let package = issue_normal_callable_semantic_package_v1(
            &mut resolver,
            final_source(include_str!(
                "../../../apps/tests/scan_with_init_typed_ok_min.hako"
            )),
        )
        .expect("same-cohort package");
        let mut context = CompilationContext::new();
        let installed = package
            .prepare_install(&mut context)
            .expect("vacant catalog")
            .commit();
        let mut port = installed.begin_lowering(&context).expect("same catalog");
        let live = MirBuilder::new();
        let config = BuilderInvocationConfigV1::snapshot_for_canonical(&live, None);
        let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
        let profile = PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1;
        let target = PinnedTextCompileTargetCapabilityIssuerV1::issue(profile).unwrap();
        let target_ordinal = target.invocation_ordinal().get();
        session
            .install_pinned_text_target_capability(Some(target))
            .expect("target install");

        port.with_s6c_common_v2_pre_session(|loan| {
            let prepared =
                issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
            let skeleton =
                reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
            let branded = skeleton.into_session_input();
            let plans = PinnedTextAccessPlanTableV1::new(1);
            let ingress = session
                .prepare_pinned_text_physical_entry_ingress(branded, &plans)
                .expect("pre-DraftSeal physical ingress");
            let frame = ingress.frame();
            assert_eq!(frame.invocation_ordinal(), target_ordinal);
            assert_eq!(frame.plan_stamp(), 1);
            assert_eq!(frame.target_profile_id(), profile.profile_id());
            assert_eq!(
                ingress.invocation_brand(),
                ModuleInvocationBrandV1::legacy_test()
            );
            let prepared = ingress.into_prepared();
            assert_eq!(prepared.brand(), ModuleInvocationBrandV1::legacy_test());
            let binding = session
                .pinned_text_invocation_binding()
                .expect("session binding remains scoped");
            let foreign_binding = PinnedTextCompileInvocationBindingRefV1 {
                brand: ModuleInvocationBrandV1::test_with_ordinal(9_991),
                target: binding.target,
            };
            assert!(matches!(
                foreign_binding.prepare_physical_entry_ingress(prepared, &plans),
                Err(PinnedTextPhysicalEntryIngressRejectV1::ForeignInvocationBrand)
            ));
        })
        .expect("one S6C callback");
        port.complete().expect("selected child coverage");
    }
}
