//! Cataloged method terminals with a caller-selected root source authority.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::calls::PendingFunctionSessionCloseV1;
use super::module_lowering_invocation::ModuleLoweringPortChildErrorV1;
use super::module_lowering_invocation::ResolvedChildDraftAdmissionV1;
use super::normal_cataloged_box_method_admission::NormalCatalogedBoxMethodDraftAdmissionV1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceTransportV1, RawSourceTransportPortV1,
};
use super::recursive_child_lowering::{
    normalize_instance_box_method_input_v1, RawInvocationChildPortV1,
};
use super::MirBuilder;
use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;

impl RawInvocationChildPortV1<'_, '_> {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_cataloged_static_box_method_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let lineage = RawInvocationRootLineageV1::Cataloged(admission.source_key().clone());
        self.lower_normal_cataloged_static_box_method_with_source_v1(
            builder,
            admission,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            RawInvocationSourceTransportV1::root((), lineage),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_cataloged_static_box_method_with_source_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        source: RawInvocationSourceTransportV1<()>,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let name = admission.physical_symbol().to_owned();
        builder.observe_legacy_method_lowering_v1(&name, &body, None);
        let pending = self.with_source_transport_v1(source, |port, ()| {
            port.capture_static_box_method_pending_v1(
                builder,
                name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )
        })?;
        self.module_port
            .commit_normal_cataloged_box_method_pending(pending, admission)
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_cataloged_instance_box_method_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let lineage = RawInvocationRootLineageV1::Cataloged(admission.source_key().clone());
        self.lower_normal_cataloged_instance_box_method_with_source_v1(
            builder,
            admission,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            RawInvocationSourceTransportV1::root((), lineage),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_cataloged_instance_box_method_with_source_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        source: RawInvocationSourceTransportV1<()>,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let name = admission.physical_symbol().to_owned();
        let box_name = admission.source_key().owner().to_owned();
        let (params, param_decls) =
            normalize_instance_box_method_input_v1(&name, params, param_decls);
        builder.observe_legacy_method_lowering_v1(&name, &body, Some(&box_name));
        let pending = self.with_source_transport_v1(source, |port, ()| {
            port.capture_normalized_instance_box_method_pending_v1(
                builder,
                name,
                box_name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )
        })?;
        self.module_port
            .commit_normal_cataloged_box_method_pending(pending, admission)
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_cataloged_static_box_method_with_signature_and_source_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        signature: ResolvedCallablePhysicalSignatureLoanV1<'_>,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        target_capability: Option<
            &crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1,
        >,
        source: RawInvocationSourceTransportV1<()>,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let function_name = admission.physical_symbol().to_owned();
        let session_name = function_name.clone();
        builder.observe_legacy_method_lowering_v1(&function_name, &body, None);
        let resolved = ResolvedChildDraftAdmissionV1::canonical_resolved_owner(
            signature.owner(),
            function_name.clone(),
            admission.physical_arity(),
        );
        let pending: PendingFunctionSessionCloseV1<'_> = {
            let mut child_port = self.reborrow();
            child_port.with_source_transport_v1(source, |child_port, ()| {
                builder
                    .capture_resolved_function_pending_session_v1(&session_name, move |builder| {
                        let prepared = builder.build_static_method_draft_with_port_v1(
                            child_port,
                            function_name,
                            params,
                            param_decls,
                            return_type_name,
                            body,
                            uses,
                            attrs,
                        )?;
                        child_port.with_headers(|headers| {
                            builder.finalize_function_draft_with_headers(prepared, headers)
                        })
                    })
                    .map_err(ModuleLoweringPortChildErrorV1::Session)
            })?
        };
        self.module_port.complete_resolved_child_with_physical_loan(
            pending,
            resolved,
            signature,
            target_capability,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_cataloged_instance_box_method_with_signature_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        signature: ResolvedCallablePhysicalSignatureLoanV1<'_>,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        target_capability: Option<
            &crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1,
        >,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let function_name = admission.physical_symbol().to_owned();
        let session_name = function_name.clone();
        let box_name = admission.source_key().owner().to_owned();
        builder.observe_legacy_method_lowering_v1(&function_name, &body, Some(&box_name));
        let (params, param_decls) =
            normalize_instance_box_method_input_v1(&function_name, params, param_decls);
        let resolved = ResolvedChildDraftAdmissionV1::canonical_resolved_owner(
            signature.owner(),
            function_name.clone(),
            admission.physical_arity(),
        );
        let pending: PendingFunctionSessionCloseV1<'_> = {
            let mut child_port = self.reborrow();
            builder
                .capture_resolved_function_pending_session_v1(&session_name, move |builder| {
                    let prepared = builder.build_instance_method_draft_with_port_v1(
                        &mut child_port,
                        function_name,
                        box_name,
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                    )?;
                    child_port.with_headers(|headers| {
                        builder.finalize_function_draft_with_headers(prepared, headers)
                    })
                })
                .map_err(ModuleLoweringPortChildErrorV1::Session)?
        };
        self.module_port.complete_resolved_child_with_physical_loan(
            pending,
            resolved,
            signature,
            target_capability,
        )
    }
}
