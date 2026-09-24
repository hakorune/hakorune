//! Thin Builder adapter over the installed semantic-package port.
//!
//! Selection, exact source pairing, and exactly-once consumption stay in the
//! package. This adapter owns the selected Dynamic handoff into the canonical
//! unpublished emitter and the ordinary scoped raw lineage used for
//! compatibility lowering.

use std::{cell::RefCell, rc::Rc};

use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, OwnedExprSiteV1, SourceExprSiteV1, SourcePathSegmentV1,
};
use crate::mir::{MirBuilder, ValueId};

use super::callable_declaration_catalog::{
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
};
use super::main_expansion::VerifiedMainStaticChildV1;
use super::map_read_physical_consumer::MapReadPhysicalConsumerV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::normal_cataloged_box_method_admission::NormalCatalogedBoxMethodDraftAdmissionV1;
use super::normal_instance_constructor_demand_loan::InstanceConstructorDemandConsumptionV1;
use super::normal_instance_constructor_semantic_scope::with_constructor_semantic_scope;
use super::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1;
use super::pinned_text_invocation_binding::PinnedTextCompileInvocationBindingRefV1;
use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawFunctionHeaderLookupPortV1, RawInvocationChildPortV1,
    RawOrdinaryNewClaimPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::normal_callable_semantic_package::{
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackagePortV1,
    ResolvedCallablePhysicalSignatureLoanV1, SelectedCallableLoweringInputRefV1,
};

#[path = "normal_callable_semantic_loan_port/canonical_route.rs"]
mod canonical_route;
#[path = "normal_callable_semantic_loan_port/cataloged_instance_scope.rs"]
mod cataloged_instance_scope;
#[path = "normal_callable_semantic_loan_port/generic_g0.rs"]
mod generic_g0;
#[path = "normal_callable_semantic_loan_port/main0_root.rs"]
mod main0_root;
#[path = "normal_callable_semantic_loan_port/main_root.rs"]
mod main_root;
#[path = "normal_callable_semantic_loan_port/ordinary_new.rs"]
mod ordinary_new;
#[path = "normal_callable_semantic_loan_port/source_scope.rs"]
mod source_scope;
use source_scope::with_selected_source_scope;

pub(super) use canonical_route::{
    classify_canonical_callable_route, try_prepare_callable_single_loop_program_v1,
    CanonicalCallableRouteV1,
};

pub(super) struct NormalCallableSemanticPackagePortAdapterV1<
    'package,
    'loan,
    'port,
    'collector,
    'target,
> {
    inner: &'loan mut RawInvocationChildPortV1<'port, 'collector>,
    package: NormalCallableSemanticPackagePortV1<'package>,
    target_binding: Option<PinnedTextCompileInvocationBindingRefV1<'target>>,
    constructor_demand: InstanceConstructorDemandConsumptionV1,
    map_read_consumer: Option<Rc<RefCell<MapReadPhysicalConsumerV1>>>,
    qualified_main_relation: Option<
        crate::mir::normal_callable_semantic_package::VerifiedQualifiedReceiverCatalogRelationV1,
    >,
}

impl<'package, 'loan, 'port, 'collector, 'target>
    NormalCallableSemanticPackagePortAdapterV1<'package, 'loan, 'port, 'collector, 'target>
{
    pub(super) fn new(
        inner: &'loan mut RawInvocationChildPortV1<'port, 'collector>,
        package: NormalCallableSemanticPackagePortV1<'package>,
        target_binding: Option<PinnedTextCompileInvocationBindingRefV1<'target>>,
        constructor_manifest: Option<super::normal_instance_constructor_admission::VerifiedInstanceConstructorPhysicalDemandManifestV1>,
    ) -> Result<Self, String> {
        let map_read_consumer = {
            let facts = package.map_read_facts_snapshot();
            if facts.is_empty() {
                None
            } else {
                let consumer = Rc::new(RefCell::new(MapReadPhysicalConsumerV1::new(facts)));
                inner.install_map_read_consumer(Rc::clone(&consumer))?;
                Some(consumer)
            }
        };
        Ok(Self {
            inner,
            package,
            target_binding,
            constructor_demand: InstanceConstructorDemandConsumptionV1::new(constructor_manifest),
            map_read_consumer,
            qualified_main_relation: None,
        })
    }

    pub(super) fn install_app_main_qualified_receiver_relation(&mut self) -> Result<(), String> {
        self.qualified_main_relation = self
            .package
            .take_app_main_qualified_receiver_catalog()
            .map_err(|error| {
                format!("[freeze:contract][mir/main-qualified-static-target/{error}]")
            })?;
        Ok(())
    }

    pub(super) fn complete(mut self) -> Result<(), String> {
        if let Some(consumer) = self.map_read_consumer.take() {
            consumer.borrow().finish()?;
            self.inner.clear_map_read_consumer();
        }
        self.constructor_demand
            .complete()
            .map_err(|error| error.to_string())?;
        if let Some(relation) = self.qualified_main_relation.as_ref() {
            relation.finish_empty().map_err(|error| error.to_string())?;
        }
        self.package.complete().map_err(package_issue)
    }

    fn with_callable_source_scope<R>(
        &mut self,
        key: SelectedNormalCallableKeyV1,
        execute: impl FnOnce(
            &mut RawInvocationChildPortV1<'_, 'collector>,
            super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
        ) -> Result<R, String>,
    ) -> Result<R, String> {
        let lineage = match &key {
            SelectedNormalCallableKeyV1::TopLevel(key) => {
                super::raw_invocation_source_transport::RawInvocationRootLineageV1::TopLevel(
                    key.clone(),
                )
            }
            SelectedNormalCallableKeyV1::Cataloged(key) => {
                super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                    key.clone(),
                )
            }
        };
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
        let named_array_emissions = self.package.named_array_emission_collector();
        Ok(self
            .package
            .with_selected_lowering_input_and_core_methods(
                &key,
                |input, core_method_calls, loop_break_take| {
                    with_selected_source_scope(
                        inner,
                        lineage,
                        input,
                        core_method_calls,
                        Rc::clone(&named_array_emissions),
                        Rc::clone(&ordinary_new_claim_ledger),
                        Some(loop_break_take),
                        execute,
                    )
                },
            )
            .map_err(package_issue)?)
    }
}

fn package_issue(error: NormalCallableSemanticPackageInstallIssueV1) -> String {
    format!("[freeze:contract][mir/callable-semantic-package/port] {error:?}")
}

fn validate_selected_cataloged_input(
    input: &crate::mir::normal_callable_semantic_package::SelectedCatalogedCallableLoweringInputV1<
        '_,
    >,
) -> Result<(), String> {
    input.with_selected_and_admission(|selected, admitted| {
        let expected = SelectedNormalCallableKeyV1::Cataloged(admitted.source_key().clone());
        if selected.selected_key() == &expected {
            Ok(())
        } else {
            Err(package_issue(
                NormalCallableSemanticPackageInstallIssueV1::CatalogedAdmissionMismatch,
            ))
        }
    })
}

fn validate_selected_signature_loan(
    input: &crate::mir::normal_callable_semantic_package::SelectedCatalogedCallableLoweringInputV1<
        '_,
    >,
    signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
) -> Result<(), String> {
    input.with_selected_and_admission(|selected, admission| {
        validate_selected_signature_loan_parts(selected, admission, signature)
    })
}

fn validate_selected_signature_loan_parts(
    selected: &SelectedCallableLoweringInputRefV1<'_>,
    admission: &NormalCatalogedBoxMethodDraftAdmissionV1,
    signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
) -> Result<(), String> {
    let key = admission.source_key();
    let expected_receiver_lane_count = match key.namespace() {
        SameModuleCallableNamespaceV1::FreeFunction => 0,
        SameModuleCallableNamespaceV1::StaticBoxMethod => 0,
        SameModuleCallableNamespaceV1::InstanceBoxMethod
        | SameModuleCallableNamespaceV1::BirthConstructor => 1,
    };
    if signature.owner() != selected.source().owner()
        || !signature
            .identity()
            .same_as(selected.source_identity().identity())
        || signature.source_logical_arity() != key.arity()
        || signature.receiver_lane_count() != expected_receiver_lane_count
    {
        return Err(package_issue(
            NormalCallableSemanticPackageInstallIssueV1::PhysicalSignatureMismatch,
        ));
    }
    Ok(())
}

impl RecursiveChildLoweringPortV1
    for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>
{
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn cleanup_exit_policy_v1(
        &self,
    ) -> crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1 {
        self.inner.cleanup_exit_policy_v1()
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        self.inner.lower_body(builder, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        self.inner.lower_statement(builder, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        self.inner.lower_expression(builder, input)
    }

    fn lower_me_expression_v1(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        self.inner.lower_me_expression_v1(builder)
    }

    fn prepare_expression_child_source_v1(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        self.inner.prepare_expression_child_source_v1(parent, role)
    }

    fn prepare_body_child_source_v1(
        &self,
        parent: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        self.inner.prepare_body_child_source_v1(parent, role)
    }

    fn prepare_body_statement_source_v1(
        &self,
        statement: &ASTNode,
        index: usize,
    ) -> Result<PreparedRawChildSourceV1, String> {
        self.inner
            .prepare_body_statement_source_v1(statement, index)
    }

    fn with_prepared_child_source_v1<R>(
        &mut self,
        prepared: PreparedRawChildSourceV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        match prepared {
            PreparedRawChildSourceV1::Preserve => execute(self),
            PreparedRawChildSourceV1::Exact(source) => {
                let parent = self.inner.active_source.replace(source);
                let result = execute(self);
                self.inner.active_source = parent;
                result
            }
        }
    }

    fn try_lower_map_read_method_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        receiver: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
        receiver_source: PreparedRawChildSourceV1,
    ) -> Result<Option<ValueId>, String> {
        self.inner.try_lower_map_read_method_call_v1(
            builder,
            receiver,
            method,
            arguments,
            receiver_source,
        )
    }
}

impl RawBoxMethodChildPortV1 for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_> {
    fn lower_static_main_box(
        &mut self,
        builder: &mut MirBuilder,
        box_name: String,
        methods: BoxMethodInventoryV1,
    ) -> Result<ValueId, String> {
        self.inner.lower_static_main_box(builder, box_name, methods)
    }

    fn lower_nested_box_method(
        &mut self,
        builder: &mut MirBuilder,
        input: super::nested_box_method_source::NestedBoxMethodLoweringInputV1,
    ) -> Result<(), String> {
        self.inner.lower_nested_box_method(builder, input)
    }
}

impl RawFunctionHeaderLookupPortV1
    for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>
{
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(
            Option<&'headers dyn super::function_signature_lookup::FunctionSignatureLookupV1>,
        ) -> R,
    ) -> R {
        self.inner.with_function_headers(observe)
    }
}

impl RootCallableCapturePortV1 for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_> {
    fn lower_app_main_root_body_v1(
        &mut self,
        builder: &mut MirBuilder,
        expected_identity: &crate::parser::CallableDeclarationIdentityV1,
        body: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        main_root::lower_app_main_root_body_v1(self, builder, expected_identity, body)
    }

    fn lower_app_main0_continue_root_v1(
        &mut self,
        builder: &mut MirBuilder,
        expected_identity: &crate::parser::CallableDeclarationIdentityV1,
        product: crate::mir::compiler::main0_continue_recipe_coseal::VerifiedMain0ContinueRecipeProductV1,
    ) -> Result<(), String> {
        main0_root::lower_app_main0_continue_root_v1(self, builder, expected_identity, product)
    }

    fn lower_app_main0_in_body_step_root_v1(
        &mut self,
        builder: &mut MirBuilder,
        expected_identity: &crate::parser::CallableDeclarationIdentityV1,
        product: crate::mir::compiler::main0_in_body_step_recipe_coseal::VerifiedMain0InBodyStepRecipeProductV1,
    ) -> Result<(), String> {
        main0_root::lower_app_main0_in_body_step_root_v1(self, builder, expected_identity, product)
    }

    fn lower_app_main0_derived_predicate_root_v1(
        &mut self,
        builder: &mut MirBuilder,
        expected_identity: &crate::parser::CallableDeclarationIdentityV1,
        product: crate::mir::compiler::main0_derived_predicate_recipe_coseal::VerifiedMain0DerivedPredicateRecipeProductV1,
    ) -> Result<(), String> {
        main0_root::lower_app_main0_derived_predicate_root_v1(
            self,
            builder,
            expected_identity,
            product,
        )
    }

    fn lower_app_main_static_child(
        &mut self,
        builder: &mut MirBuilder,
        child: &VerifiedMainStaticChildV1<'_>,
    ) -> Result<(), String> {
        let (_symbol, params, param_decls, return_type_name, body, uses, attrs, declaration) =
            child.to_owned_lowering().into_parts();
        let target_binding = self.target_binding.as_ref();
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
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
                let target_capability = target_binding.map(|binding| binding.target_capability());
                let lineage =
                    super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                        admission.source_key().clone(),
                    );
                // This bounded production edge owns only the existing
                // CallableSingleLoop consumer. Other static-child shapes keep
                // their prior source lowering until their own consumer is
                // selected, so this adapter does not reclassify or open an
                // unrelated canonical session.
                if let Some(program) = try_prepare_callable_single_loop_program_v1(selected.source())?
                {
                    // The issued canonical program is the bypass evidence:
                    // this exact owner's loan rows stay untouched on purpose
                    // and close through the mark, never silently.
                    if let Some(loans) = inner.direct_call_loans.as_deref_mut() {
                        loans.mark_canonical_route_bypass(selected.source().owner());
                    }
                    return inner
                        .lower_normal_cataloged_static_box_method_with_callable_single_loop_program_v1(
                            builder,
                            admission,
                            signature,
                            program,
                            target_capability,
                        )
                        .map_err(|error| error.to_string());
                }
                with_selected_source_scope(
                    inner,
                    lineage,
                    selected,
                    core_method_calls,
                    Rc::clone(&named_array_emissions),
                        Rc::clone(&ordinary_new_claim_ledger),
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
    fn lower_normal_instance_constructor(
        &mut self,
        builder: &mut MirBuilder,
        source_key: &super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        declaration: Option<ASTNode>,
    ) -> Result<(), String> {
        self.inner
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
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_normal_instance_constructor_with_demand(
        &mut self,
        builder: &mut MirBuilder,
        source_key: &super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1,
        ticket: super::normal_instance_constructor_admission::InstanceConstructorDemandTicketV1,
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
        self.package.with_instance_constructor_lowering_input(
            &source_id,
            |input, kind, construction| {
                with_constructor_semantic_scope(
                    inner,
                    input,
                    &source_id,
                    kind,
                    construction,
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
    fn lower_normal_top_level_function(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalTopLevelFunctionDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        declaration: Option<ASTNode>,
    ) -> Result<(), String> {
        generic_g0::lower_normal_top_level_function(
            self,
            builder,
            admission,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            declaration,
        )
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
        let target_binding = self.target_binding.as_ref();
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
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
                if matches!(
                    input.selected().semantic(),
                    crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Dynamic { .. }
                ) {
                    let (selected, admission, physical_header) =
                        input.into_lowering_and_admission();
                    let target_binding = target_binding.ok_or_else(|| {
                        "[freeze:contract][mir/selected-dynamic/target-binding] missing"
                            .to_owned()
                    })?;
                    let _collector_receipt =
                        crate::mir::builder::resolved_lowering::assemble_unpublished_selected_dynamic_w6_from_parts(
                            builder,
                            inner.module_port,
                            target_binding,
                            &selected,
                            admission,
                            physical_header,
                            |session, profile| {
                                let mut state =
                                    CallableSemanticLoweringState::from_exact_source_with_dynamic_source(
                                        selected.source(),
                                        Some(Rc::clone(session.dynamic_source())),
                                    )?;
                                session.observe_body_state(&mut state, profile)?;
                                state.finish()
                            },
                        )
                        .map_err(|error| {
                            format!(
                                "[freeze:contract][mir/selected-dynamic/production-handoff] {error}"
                            )
                        })?;
                    return Ok(());
                }
                let (selected, admission, _physical_header) = input.into_lowering_and_admission();
                let canonical_route = classify_canonical_callable_route(
                    selected.source(),
                    builder.comp_ctx.emit_debug_policy().generic_g0_policy_mode_v1(),
                )?;
                let target_capability = target_binding.map(|binding| binding.target_capability());
                let lineage =
                    super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                        admission.source_key().clone(),
                    );
                match canonical_route {
                    CanonicalCallableRouteV1::Ready(plan) => inner
                        .lower_normal_cataloged_static_box_method_with_canonical_trivial_plan_v1(
                            builder,
                            admission,
                            signature,
                            plan,
                            target_capability,
                        )
                        .map_err(|error| error.to_string()),
                    CanonicalCallableRouteV1::DirectAccum(plan) => inner
                        .lower_normal_cataloged_static_box_method_with_canonical_direct_accum_plan_v1(
                            builder,
                            admission,
                            signature,
                            plan,
                            target_capability,
                        )
                        .map_err(|error| error.to_string()),
                    CanonicalCallableRouteV1::CallableSingleLoop(program) => inner
                        .lower_normal_cataloged_static_box_method_with_callable_single_loop_program_v1(
                            builder,
                            admission,
                            signature,
                            program,
                            target_capability,
                        )
                        .map_err(|error| error.to_string()),
                    CanonicalCallableRouteV1::GenericG0(_) => Err(
                        "[freeze:contract][mir/callable-generic-g0/cataloged-method-outside-i0]"
                            .to_owned(),
                    ),
                    CanonicalCallableRouteV1::Outside => {
                        with_selected_source_scope(
                            inner,
                            lineage,
                            selected,
                            core_method_calls,
                            Rc::clone(&named_array_emissions),
                        Rc::clone(&ordinary_new_claim_ledger),
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
                    }
                }
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

#[cfg(test)]
#[path = "normal_callable_semantic_loan_port/map_dependency_tests.rs"]
mod map_dependency_tests;

#[cfg(test)]
#[path = "normal_callable_semantic_loan_port/canonical_route_tests.rs"]
mod canonical_route_tests;
