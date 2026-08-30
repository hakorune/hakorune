//! Pending-draft and direct-call helper methods for the raw invocation port.
//!
//! This module is a BoxShape-only extraction.  The methods below forward
//! already-issued pending records or consume the existing App Main loan; they
//! do not select routes, issue targets, or add fallback policy.

use super::*;

impl RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn commit_normal_top_level_function_pending_v1(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: super::super::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.module_port
            .commit_normal_top_level_function_pending(pending, admission)
    }

    pub(in crate::mir::builder) fn commit_normal_instance_constructor_pending_v1(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: super::super::normal_instance_constructor_admission::NormalInstanceConstructorDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.module_port
            .commit_normal_instance_constructor_pending(pending, admission)
    }

    pub(in crate::mir::builder) fn complete_raw_root_static_child_branded(
        &mut self,
        builder: &mut MirBuilder,
        prepared: super::super::PreparedRawRootStaticChildDraftV1,
    ) -> Result<
        super::super::module_invocation_owner_chain::InvocationBranded<
            super::super::module_draft_collector::CollectedDraftAdmissionReceiptV1,
        >,
        ModuleLoweringPortChildErrorV1,
    > {
        let (admission, lowering) = prepared.into_parts();
        let source_root = RawInvocationRootLineageV1::Main(admission.source_locator().clone());
        let pending = self.with_source_transport_v1(
            RawInvocationSourceTransportV1::root((), source_root),
            |port, ()| {
                port.capture_static_box_method_pending_v1(
                    builder,
                    lowering.function_name,
                    lowering.params,
                    lowering.param_decls,
                    lowering.return_type_name,
                    lowering.body,
                    lowering.uses,
                    lowering.attrs,
                )
            },
        )?;
        self.module_port
            .commit_legacy_symbol_pending_branded(pending, admission.into_collector_parts())
    }

    pub(in crate::mir::builder) fn is_app_main_direct_call_scope_v1(&self) -> bool {
        let Some(loan) = self.direct_call_loan.as_deref() else {
            return false;
        };
        let Some(owner) = self.callable_owner_v1() else {
            return false;
        };
        let Some(RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(_),
            ..
        }) = self.active_source.as_ref()
        else {
            return false;
        };
        owner == loan.owner()
    }

    pub(in crate::mir::builder) fn take_app_main_direct_call_disposition_inner_v1(
        &mut self,
    ) -> Result<
        crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1,
        String,
    > {
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][app-main-direct-call/owner-missing]".to_owned())?;
        let site = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][app-main-direct-call/site-missing]".to_owned())?;
        let loan = self
            .direct_call_loan
            .as_deref_mut()
            .ok_or_else(|| "[freeze:contract][app-main-direct-call/loan-unavailable]".to_owned())?;
        loan.take_once(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        )
        .map_err(|error| format!("[freeze:contract][app-main-direct-call/{error:?}]"))
    }
}
