//! Thin Builder adapter over the installed semantic-package port.
//!
//! Selection, exact source pairing, and exactly-once consumption stay in the
//! package. This adapter owns the selected Dynamic handoff into the canonical
//! unpublished emitter and the ordinary scoped raw lineage used for
//! compatibility lowering.

use std::{cell::RefCell, rc::Rc};

use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, OwnedExprSiteV1, SourceExprSiteV1, SourcePathSegmentV1,
};
use crate::mir::{MirBuilder, ValueId};
use crate::parser::CallableMethodSourceObservationV1;

use super::callable_declaration_catalog::{
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
};
use super::main_expansion::VerifiedMainStaticChildV1;
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
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::CanonicalLoweringErrorV1;
use crate::mir::normal_callable_semantic_package::{
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackagePortV1,
    ResolvedCallablePhysicalSignatureLoanV1, SelectedCallableLoweringInputRefV1,
};

#[path = "normal_callable_semantic_loan_port/cataloged_instance_scope.rs"]
mod cataloged_instance_scope;
#[path = "normal_callable_semantic_loan_port/main_root.rs"]
mod main_root;

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
}

impl<'package, 'loan, 'port, 'collector, 'target>
    NormalCallableSemanticPackagePortAdapterV1<'package, 'loan, 'port, 'collector, 'target>
{
    pub(super) fn new(
        inner: &'loan mut RawInvocationChildPortV1<'port, 'collector>,
        package: NormalCallableSemanticPackagePortV1<'package>,
        target_binding: Option<PinnedTextCompileInvocationBindingRefV1<'target>>,
        constructor_manifest: Option<super::normal_instance_constructor_admission::VerifiedInstanceConstructorPhysicalDemandManifestV1>,
    ) -> Self {
        Self {
            inner,
            package,
            target_binding,
            constructor_demand: InstanceConstructorDemandConsumptionV1::new(constructor_manifest),
        }
    }

    pub(super) fn complete(self) -> Result<(), String> {
        self.constructor_demand
            .complete()
            .map_err(|error| error.to_string())?;
        self.package.complete().map_err(package_issue)
    }

    fn with_callable_source_scope<R>(
        &mut self,
        key: SelectedNormalCallableKeyV1,
        execute: impl FnOnce(
            &mut RawInvocationChildPortV1<'port, 'collector>,
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
        self.package
            .with_selected_lowering_input(&key, |input| {
                with_selected_source_scope(
                    inner,
                    lineage,
                    input,
                    Rc::clone(&ordinary_new_claim_ledger),
                    execute,
                )
            })
            .map_err(package_issue)?
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
    })
}

enum CanonicalTrivialRouteV1<'source> {
    Ready(crate::mir::compiler::capability::CanonicalTrivialBindingSsaPlanV1<'source>),
    Outside,
}

fn classify_canonical_trivial_route(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
) -> Result<CanonicalTrivialRouteV1<'_>, String> {
    match CanonicalLoweringPreflightV1::verify_function(input) {
        Ok(CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan)) => {
            Ok(CanonicalTrivialRouteV1::Ready(plan))
        }
        Ok(_) => Ok(CanonicalTrivialRouteV1::Outside),
        Err(error) if is_canonical_shape_outside(&error) => Ok(CanonicalTrivialRouteV1::Outside),
        Err(error) => Err(format!(
            "[freeze:contract][mir/callable-canonical-preflight] {error:?}"
        )),
    }
}

fn is_canonical_shape_outside(error: &CanonicalLoweringErrorV1) -> bool {
    matches!(
        error,
        CanonicalLoweringErrorV1::UnsupportedCanonicalOwnerKind
            | CanonicalLoweringErrorV1::UnsupportedCanonicalSyntaxKind
            | CanonicalLoweringErrorV1::UnsupportedCanonicalControlRoute
            | CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape { .. }
    )
}

fn with_selected_source_scope<'port, 'collector, R>(
    inner: &mut RawInvocationChildPortV1<'port, 'collector>,
    lineage: super::raw_invocation_source_transport::RawInvocationRootLineageV1,
    input: SelectedCallableLoweringInputRefV1<'_>,
    ordinary_new_claim_ledger: Rc<
        crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    >,
    execute: impl FnOnce(
        &mut RawInvocationChildPortV1<'port, 'collector>,
        super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let dynamic_source = match input.semantic() {
        crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Dynamic {
            source,
            ..
        } => Some(std::rc::Rc::clone(source)),
        crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Ordinary => {
            None
        }
    };
    with_callable_source_scope(
        inner,
        lineage,
        input.source(),
        dynamic_source,
        input.method_source_observation().cloned(),
        ordinary_new_claim_ledger,
        execute,
    )
}

fn with_callable_source_scope<'port, 'collector, R>(
    inner: &mut RawInvocationChildPortV1<'port, 'collector>,
    lineage: super::raw_invocation_source_transport::RawInvocationRootLineageV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    dynamic_source: Option<Rc<crate::mir::builder::VerifiedSourceBackedDynamicCallableV1>>,
    observation: Option<CallableMethodSourceObservationV1>,
    ordinary_new_claim_ledger: Rc<
        crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    >,
    execute: impl FnOnce(
        &mut RawInvocationChildPortV1<'port, 'collector>,
        super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let transport =
        super::raw_invocation_source_transport::RawInvocationSourceTransportV1::root((), lineage);
    let state = super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source_with_dynamic_source(
        input,
        dynamic_source,
    )?;
    let state = Rc::new(RefCell::new(state));
    let script_ledger = inner.semantic_ledger.take();
    let parent_callable = inner.callable_ledger.replace(state.clone());
    let parent_ordinary_new_claim_ledger = inner
        .ordinary_new_claim_ledger
        .replace(ordinary_new_claim_ledger);
    let result = inner
        .with_callable_method_source_observation(observation, |inner| execute(inner, transport));
    inner.callable_ledger = parent_callable;
    inner.ordinary_new_claim_ledger = parent_ordinary_new_claim_ledger;
    inner.semantic_ledger = script_ledger;
    match result {
        Ok(value) => {
            Rc::try_unwrap(state)
                .map_err(|_| "[freeze:contract][mir/callable-semantic/ledger-loan]".to_owned())?
                .into_inner()
                .finish()?;
            Ok(value)
        }
        Err(error) => Err(error),
    }
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

impl RawOrdinaryNewClaimPortV1 for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_> {
    fn complete_ordinary_new_expression(&mut self, class: &str, value: ValueId)
        -> Result<(), String> {
        let owner = self.inner.callable_owner_v1().ok_or_else(||
            "[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned())?;
        let site = self.inner.current_source_site_v1().ok_or_else(||
            "[freeze:contract][raw-ordinary-new/claim-site-missing]".to_owned())?;
        if !matches!(site.segments(), [SourcePathSegmentV1::Body(_), SourcePathSegmentV1::Initializer(_)])
            || !self.package.ordinary_box_is_covered(class) { return Ok(()); }
        self.package.ordinary_new_claim_ledger().complete_new_expression(
            &OwnedExprSiteV1::new(owner, SourceExprSiteV1::from_node(site)), class, value)
    }
    fn try_take_ordinary_new_claim(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1>,
        String,
    > {
        let Some(owner) = self.inner.callable_owner_v1() else {
            return Err("[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned());
        };
        let Some(site) = self.inner.current_source_site_v1() else {
            return Err("[freeze:contract][raw-ordinary-new/claim-site-missing]".to_owned());
        };
        if !matches!(
            site.segments(),
            [
                SourcePathSegmentV1::Body(_),
                SourcePathSegmentV1::Initializer(_)
            ]
        ) || !self.package.ordinary_box_is_covered(class)
        {
            return Ok(None);
        }
        let site = OwnedExprSiteV1::new(owner, SourceExprSiteV1::from_node(site));
        self.package
            .take_ordinary_new_claim(&site, class, argument_count)
            .map(Some)
            .map_err(package_issue)
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

    fn lower_app_main_static_child(
        &mut self,
        builder: &mut MirBuilder,
        child: &VerifiedMainStaticChildV1<'_>,
    ) -> Result<(), String> {
        let (_symbol, params, param_decls, return_type_name, body, uses, attrs) =
            child.to_owned_lowering().into_parts();
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
        self.package
            .with_main_static_child_lowering_input(child, |input| {
                let (selected, admission) = input.into_lowering_and_admission();
                if !matches!(
                    selected.semantic(),
                    crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Ordinary
                ) {
                    return Err(package_issue(
                        NormalCallableSemanticPackageInstallIssueV1::MainChildRoleMismatch,
                    ));
                }
                let lineage =
                    super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                        admission.source_key().clone(),
                    );
                with_selected_source_scope(
                    inner,
                    lineage,
                    selected,
                    Rc::clone(&ordinary_new_claim_ledger),
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
                        )
                        .map_err(|error| error.to_string())
                    },
                )
            })
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
        self.package
            .with_instance_constructor_lowering_input(&source_id, |input| {
                with_constructor_semantic_scope(inner, input, |inner| {
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
                        )
                        .map_err(|error| error.to_string())
                })
            })?
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
    ) -> Result<(), String> {
        let key = SelectedNormalCallableKeyV1::TopLevel(admission.source_key().clone());
        self.with_callable_source_scope(key, |inner, transport| {
            inner
                .lower_normal_top_level_function_with_source_v1(
                    builder,
                    admission,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    transport,
                )
                .map_err(|error| error.to_string())
        })
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
    ) -> Result<(), String> {
        let target_binding = self.target_binding.as_ref();
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
        self.package
            .with_selected_cataloged_lowering_input_and_signature(admission, |input, signature| {
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
                let canonical_route = classify_canonical_trivial_route(selected.source())?;
                let target_capability = target_binding.map(|binding| binding.target_capability());
                let lineage =
                    super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                        admission.source_key().clone(),
                    );
                match canonical_route {
                    CanonicalTrivialRouteV1::Ready(plan) => inner
                        .lower_normal_cataloged_static_box_method_with_canonical_trivial_plan_v1(
                            builder,
                            admission,
                            signature,
                            plan,
                            target_capability,
                        )
                        .map_err(|error| error.to_string()),
                    CanonicalTrivialRouteV1::Outside => {
                        with_selected_source_scope(
                            inner,
                            lineage,
                            selected,
                            Rc::clone(&ordinary_new_claim_ledger),
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
                                )
                                .map_err(|error| error.to_string())
                            },
                        )
                    }
                }
            })
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
                            target_capability,
                        )
                        .map_err(|error| error.to_string())
                })
            },
        )
    }
}
