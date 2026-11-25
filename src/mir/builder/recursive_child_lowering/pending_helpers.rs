//! Pending-draft and direct-call helper methods for the raw invocation port.
//!
//! This module is a BoxShape-only extraction.  The methods below forward
//! already-issued pending records or consume the existing direct-call loan;
//! they do not select routes, issue targets, or add fallback policy.

use super::*;
use crate::mir::builder::calls::LegacyFunctionPayloadPendingSessionV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::construction::RetainedConstructionValidation;

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
        pending: LegacyFunctionPayloadPendingSessionV1<'_, Option<RetainedConstructionValidation>>,
        admission: super::super::normal_instance_constructor_admission::NormalInstanceConstructorDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.module_port
            .commit_normal_instance_constructor_pending(pending, admission)
    }


    pub(in crate::mir::builder) fn is_direct_call_scope_v1(&self) -> bool {
        let Some(loans) = self.direct_call_loans.as_deref() else {
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
        loans.get(owner).is_some()
    }

    pub(in crate::mir::builder) fn take_direct_call_disposition_inner_v1(
        &mut self,
    ) -> Result<crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1, String>
    {
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][direct-call/owner-missing]".to_owned())?;
        let site = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][direct-call/site-missing]".to_owned())?;
        let loan = self
            .direct_call_loans
            .as_deref_mut()
            .and_then(|loans| loans.get_mut(owner))
            .ok_or_else(|| "[freeze:contract][direct-call/loan-unavailable]".to_owned())?;
        loan.take_once(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        )
        .map_err(|error| format!("[freeze:contract][direct-call/{error:?}]"))
    }
}
