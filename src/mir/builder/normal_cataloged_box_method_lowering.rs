//! Cataloged method terminals with a caller-selected root source authority.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::module_lowering_invocation::ModuleLoweringPortChildErrorV1;
use super::normal_cataloged_box_method_admission::NormalCatalogedBoxMethodDraftAdmissionV1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceTransportV1, RawSourceTransportPortV1,
};
use super::recursive_child_lowering::{
    normalize_instance_box_method_input_v1, RawInvocationChildPortV1,
};
use super::MirBuilder;

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
}
