//! Root callable captures keep their source-scope and canonical route selection.
use super::*;

impl RootCallableCapturePortV1 for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_> {
    fn lower_app_main_root_body_v1(
        &mut self,
        builder: &mut MirBuilder,
        expected_identity: &crate::parser::CallableDeclarationIdentityV1,
        parameters: &[String],
        runtime_inputs: Option<&crate::mir::builder::NormalRuntimeInputSnapshotV1>,
        body: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        main_root::lower_app_main_root_body_v1(
            self,
            builder,
            expected_identity,
            parameters,
            runtime_inputs,
            body,
        )
    }

    fn lower_app_main_static_child(
        &mut self,
        builder: &mut MirBuilder,
        child: &VerifiedMainStaticChildV1<'_>,
    ) -> Result<(), String> {
        let (_symbol, params, param_decls, return_type_name, body, uses, attrs, declaration) =
            child.to_owned_lowering().into_parts();
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
        let owned_array_source = self.package.owned_array_lifecycle_source_v1();
        let named_array_emissions = self.package.named_array_emission_collector();
        self.package
            .with_main_static_child_lowering_input(
                child,
                |input, core_method_calls, loop_break_take| {
                let (selected, admission, signature) = input.into_lowering_and_admission();
                if !matches!(
                    selected.semantic(),
                    crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Ordinary
                ) {
                    return Err(package_issue(
                        NormalCallableSemanticPackageInstallIssueV1::MainChildRoleMismatch,
                    ));
                }
                validate_selected_signature_loan_parts(&selected, &admission, &signature)?;
                let lineage =
                    super::super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                        admission.source_key().clone(),
                    );
                    with_selected_source_scope(
                        inner,
                        lineage,
                        selected,
                        core_method_calls,
                        Rc::clone(&named_array_emissions),
                        Rc::clone(&ordinary_new_claim_ledger),
                        Some(owned_array_source),
                        Some(loop_break_take),
                        |inner, transport| {
                            inner
                                .lower_normal_cataloged_static_box_method_with_source_v1(
                                    builder,
                                    admission,
                                    params,
                                    param_decls,
                                    return_type_name,
                                    body,
                                    uses,
                                    attrs,
                                    transport,
                                    Some(declaration),
                                )
                                .map_err(|error| error.to_string())
                        },
                    )
                },
            )
            .map_err(package_issue)?
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_normal_instance_constructor_with_demand(
        &mut self,
        builder: &mut MirBuilder,
        source_key: &super::super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1,
        ticket: super::super::normal_instance_constructor_admission::InstanceConstructorDemandTicketV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        declaration: Option<ASTNode>,
    ) -> Result<(), String> {
        if !ticket.source_id().same_as(source_key.source_id()) {
            return Err(
                "[freeze:contract][mir/instance-constructor-demand/source-id-drift]".to_owned(),
            );
        }
        let source_id = ticket.source_id().clone();
        self.constructor_demand
            .consume(ticket)
            .map_err(|error| error.to_string())?;
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
        let named_array_field_providers = self.package.take_named_array_field_providers(&source_id);
        self.package.with_instance_constructor_lowering_input(
            &source_id,
            |input, kind, construction| {
                with_constructor_semantic_scope(
                    inner,
                    input,
                    &source_id,
                    kind,
                    construction,
                    ordinary_new_claim_ledger,
                    named_array_field_providers,
                    |inner| {
                        inner
                            .lower_normal_instance_constructor_v1(
                                builder,
                                source_key,
                                params,
                                param_decls,
                                return_type_name,
                                body,
                                uses,
                                attrs,
                                declaration,
                            )
                            .map_err(|error| error.to_string())
                    },
                )
            },
        )?
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_cataloged_static_box_method(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        declaration: Option<ASTNode>,
    ) -> Result<(), String> {
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
        let owned_array_source = self.package.owned_array_lifecycle_source_v1();
        let named_array_emissions = self.package.named_array_emission_collector();
        let core_method_calls =
            self.package
                .take_source_core_method_calls(&SelectedNormalCallableKeyV1::Cataloged(
                    admission.source_key().clone(),
                ));
        self.package
            .with_selected_cataloged_lowering_input_signature_and_loop_break(
                admission,
                |input, signature, loop_break_take| {
                validate_selected_cataloged_input(&input)?;
                validate_selected_signature_loan(&input, &signature)?;
                let (selected, admission, _physical_header) = input.into_lowering_and_admission();
                let lineage =
                    super::super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                        admission.source_key().clone(),
                    );
                    with_selected_source_scope(
                        inner,
                        lineage,
                        selected,
                        core_method_calls,
                        Rc::clone(&named_array_emissions),
                        Rc::clone(&ordinary_new_claim_ledger),
                        Some(owned_array_source),
                        Some(loop_break_take),
                        |inner, transport| {
                        inner
                            .lower_normal_cataloged_static_box_method_with_source_v1(
                                builder,
                                admission,
                                params,
                                param_decls,
                                return_type_name,
                                body,
                                uses,
                                attrs,
                                transport,
                                declaration,
                            )
                            .map_err(|error| error.to_string())
                        },
                    )
                },
            )
            .map_err(package_issue)?
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_cataloged_instance_box_method(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        declaration: Option<ASTNode>,
    ) -> Result<(), String> {
        let target_capability = self
            .target_binding
            .as_ref()
            .map(|binding| binding.target_capability());
        self.with_cataloged_callable_source_scope(
            admission,
            |inner, transport, admission, signature| {
                inner.with_source_transport_v1(transport, |inner, ()| {
                    inner
                        .lower_normal_cataloged_instance_box_method_with_signature_v1(
                            builder,
                            admission,
                            signature,
                            params,
                            param_decls,
                            return_type_name,
                            body,
                            uses,
                            attrs,
                            declaration,
                            target_capability,
                        )
                        .map_err(|error| error.to_string())
                })
            },
        )
    }
}
