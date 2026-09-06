//! Instance draft capture has one lowering/finalization implementation.
//! Constructor capture additionally moves its existing validation state with
//! the draft; ordinary method capture does not acquire that authority.

use super::*;
use crate::mir::builder::calls::{
    CanonicalFunctionSessionErrorV1, LegacyFunctionPayloadPendingSessionV1,
    LegacyFunctionPayloadSessionErrorV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::construction::RetainedConstructionValidation;
use crate::mir::MirFunction;

impl RawInvocationChildPortV1<'_, '_> {
    #[allow(clippy::too_many_arguments)]
    fn lower_instance_capture_draft(
        &mut self,
        builder: &mut MirBuilder,
        function_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<MirFunction, String> {
        let prepared = builder.build_instance_method_draft_with_port_v1(
            self,
            function_name,
            box_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )?;
        let function = self.with_headers(|headers| {
            builder.finalize_function_draft_with_headers(prepared, headers)
        })?;
        if let Some(ledger) = &self.callable_ledger {
            let ledger = ledger.borrow();
            ledger.validate_finalized_construction_stores(&function)?;
            if let Some(news) = &self.ordinary_new_claim_ledger {
                news.validate_new_emissions(ledger.owner(), &function)?;
            }
        }
        Ok(function)
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn capture_normalized_instance_box_method_pending_v1<'builder>(
        &mut self,
        builder: &'builder mut MirBuilder,
        function_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<LegacyFunctionPendingSessionV1<'builder>, ModuleLoweringPortChildErrorV1> {
        let body_snapshot = body.clone();
        let session_name = function_name.clone();
        let mut child = self.reborrow();
        builder
            .capture_legacy_function_pending_session_v1(
                &session_name,
                body_snapshot,
                move |builder| {
                    child.lower_instance_capture_draft(
                        builder,
                        function_name,
                        box_name,
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                    )
                },
            )
            .map_err(ModuleLoweringPortChildErrorV1::Session)
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn capture_normalized_constructor_pending_v1<'builder>(
        &mut self,
        builder: &'builder mut MirBuilder,
        function_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<
        LegacyFunctionPayloadPendingSessionV1<'builder, Option<RetainedConstructionValidation>>,
        ModuleLoweringPortChildErrorV1,
    > {
        let body_snapshot = body.clone();
        let session_name = function_name.clone();
        let mut child = self.reborrow();
        builder
            .capture_legacy_function_payload_pending_session_v1(
                &session_name,
                body_snapshot,
                move |builder| {
                    let function = child.lower_instance_capture_draft(
                        builder,
                        function_name,
                        box_name,
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                    )?;
                    let validation = match &child.callable_ledger {
                        Some(ledger) => ledger
                            .borrow_mut()
                            .take_finalized_construction_validation(&function)?,
                        None => None,
                    };
                    Ok::<_, String>((function, validation))
                },
            )
            .map_err(|error| {
                // Payload is discarded only on failed capture; nothing is published.
                let error = match error {
                    LegacyFunctionPayloadSessionErrorV1::Primary(primary) => {
                        CanonicalFunctionSessionErrorV1::Primary(primary)
                    }
                    LegacyFunctionPayloadSessionErrorV1::CleanupAfterSuccess {
                        payload: _,
                        detail,
                    } => CanonicalFunctionSessionErrorV1::Cleanup(detail.into_string()),
                    LegacyFunctionPayloadSessionErrorV1::DuringCleanup { primary, detail } => {
                        CanonicalFunctionSessionErrorV1::DuringCleanup {
                            primary,
                            cleanup: detail.into_string(),
                        }
                    }
                };
                ModuleLoweringPortChildErrorV1::Session(error)
            })
    }
}
