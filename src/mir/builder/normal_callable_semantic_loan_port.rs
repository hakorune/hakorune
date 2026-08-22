//! Thin Builder adapter over the installed semantic-package port.
//!
//! Selection, exact source pairing, and exactly-once consumption stay in the
//! package. This adapter owns the selected Dynamic handoff into the canonical
//! unpublished emitter and the ordinary scoped raw lineage used for
//! compatibility lowering.

use std::{cell::RefCell, rc::Rc};

use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::{MirBuilder, ValueId};

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
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawInvocationChildPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1;
use crate::mir::compiler::CanonicalLoweringErrorV1;
use crate::mir::normal_callable_semantic_package::{
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackagePortV1,
    ResolvedCallablePhysicalSignatureLoanV1, SelectedCallableLoweringInputRefV1,
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
    target_capability: Option<&'target PinnedTextCompileTargetCapabilityV1>,
    constructor_demand: InstanceConstructorDemandConsumptionV1,
}

impl<'package, 'loan, 'port, 'collector, 'target>
    NormalCallableSemanticPackagePortAdapterV1<'package, 'loan, 'port, 'collector, 'target>
{
    pub(super) fn new(
        inner: &'loan mut RawInvocationChildPortV1<'port, 'collector>,
        package: NormalCallableSemanticPackagePortV1<'package>,
        target_capability: Option<&'target PinnedTextCompileTargetCapabilityV1>,
        constructor_manifest: Option<super::normal_instance_constructor_admission::VerifiedInstanceConstructorPhysicalDemandManifestV1>,
    ) -> Self {
        Self {
            inner,
            package,
            target_capability,
            constructor_demand: InstanceConstructorDemandConsumptionV1::new(constructor_manifest),
        }
    }

    pub(super) fn complete(self) -> Result<(), String> {
        self.constructor_demand.complete()?;
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
        self.package
            .with_selected_lowering_input(&key, |input| {
                with_selected_source_scope(inner, lineage, input, execute)
            })
            .map_err(package_issue)?
    }

    fn with_cataloged_callable_source_scope<R>(
        &mut self,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        execute: impl FnOnce(
            &mut RawInvocationChildPortV1<'port, 'collector>,
            super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
            NormalCatalogedBoxMethodDraftAdmissionV1,
            ResolvedCallablePhysicalSignatureLoanV1<'_>,
        ) -> Result<R, String>,
    ) -> Result<R, String> {
        let inner = &mut *self.inner;
        self.package
            .with_selected_cataloged_lowering_input_and_signature(admission, |input, signature| {
                validate_selected_cataloged_input(&input)?;
                validate_selected_signature_loan(&input, &signature)?;
                if matches!(
                    input.selected().semantic(),
                    crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Dynamic { .. }
                ) {
                    return Err(
                        "[freeze:contract][mir/callable-semantic-package/dynamic-instance-route]"
                            .to_owned(),
                    );
                }
                let (selected, admission, _physical_header) = input.into_lowering_and_admission();
                let lineage =
                    super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                        admission.source_key().clone(),
                    );
                with_selected_source_scope(inner, lineage, selected, |inner, transport| {
                    execute(inner, transport, admission, signature)
                })
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
            SameModuleCallableNamespaceV1::StaticBoxMethod => 0,
            SameModuleCallableNamespaceV1::InstanceBoxMethod => 1,
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
    execute: impl FnOnce(
        &mut RawInvocationChildPortV1<'port, 'collector>,
        super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let transport =
        super::raw_invocation_source_transport::RawInvocationSourceTransportV1::root((), lineage);
    let dynamic_source = match input.semantic() {
        crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Dynamic {
            source,
            ..
        } => Some(std::rc::Rc::clone(source)),
        crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Ordinary => {
            None
        }
    };
    let state = super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source_with_dynamic_source(
        input.source(),
        dynamic_source,
    )?;
    let state = Rc::new(RefCell::new(state));
    let script_ledger = inner.semantic_ledger.take();
    let parent_callable = inner.callable_ledger.replace(state.clone());
    let observation = input.method_source_observation().cloned();
    let result = inner
        .with_callable_method_source_observation(observation, |inner| execute(inner, transport));
    inner.callable_ledger = parent_callable;
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

impl RootCallableCapturePortV1 for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_> {
    fn lower_app_main_static_child(
        &mut self,
        builder: &mut MirBuilder,
        child: &VerifiedMainStaticChildV1<'_>,
    ) -> Result<(), String> {
        let (_symbol, params, param_decls, return_type_name, body, uses, attrs) =
            child.to_owned_lowering().into_parts();
        let inner = &mut *self.inner;
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
                with_selected_source_scope(inner, lineage, selected, |inner, transport| {
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
                })
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
        self.constructor_demand.consume(ticket)?;
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
        let inner = &mut *self.inner;
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
                    let _collector_receipt =
                        crate::mir::builder::resolved_lowering::assemble_unpublished_selected_dynamic_w6_from_parts(
                            builder,
                            inner.module_port,
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
                            self.target_capability,
                        )
                        .map_err(|error| error.to_string()),
                    CanonicalTrivialRouteV1::Outside => {
                        with_selected_source_scope(inner, lineage, selected, |inner, transport| {
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
                        })
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
        let target_capability = self.target_capability;
        self.with_cataloged_callable_source_scope(
            admission,
            |inner, _transport, admission, signature| {
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
            },
        )
    }
}
