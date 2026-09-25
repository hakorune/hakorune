use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::callable_declaration_catalog::CanonicalSameModuleCallableKeyV1;
use crate::mir::builder::normal_callable_binding_materialization_port::{
    CallableBindingMaterializationPortV1, CallableEntryShapeV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawSourceTransportPortV1;
use crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1;
use crate::mir::compiler::capability::CanonicalLoweringPreflightV1;
use crate::mir::compiler::normal_source_plan::VerifiedNormalMainRoleV1;
use crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use crate::mir::{MirBuilder, ValueId};
use crate::parser::CallableDeclarationIdentityV1;

use super::super::raw_invocation_source_transport::RawInvocationRootLineageV1;
use super::super::raw_invocation_source_transport::RawInvocationSourceContextV1;
use super::super::recursive_child_lowering::RawInvocationChildPortV1;
use super::super::recursive_child_lowering::{
    DirectCallDispositionPortV1, RecursiveChildLoweringPortV1,
};
use super::super::recursive_child_lowering_port::{
    QualifiedStaticMethodHandoffIngressV1, QualifiedStaticMethodHandoffPortV1,
};
use super::NormalCallableSemanticPackagePortAdapterV1;

pub(super) struct MainQualifiedMethodRecipePort<'relation, 'port, 'collector> {
    relation: &'relation mut Option<
        crate::mir::normal_callable_semantic_package::VerifiedQualifiedReceiverCatalogRelationV1,
    >,
    context: &'relation mut RawInvocationChildPortV1<'port, 'collector>,
}

impl<'relation, 'port, 'collector> MainQualifiedMethodRecipePort<'relation, 'port, 'collector> {
    pub(super) fn new(
        relation: &'relation mut Option<
            crate::mir::normal_callable_semantic_package::VerifiedQualifiedReceiverCatalogRelationV1,
        >,
        context: &'relation mut RawInvocationChildPortV1<'port, 'collector>,
    ) -> Result<Self, String> {
        let Some(RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(_),
            ..
        }) = context.current_source_context_v1()
        else {
            return Err("[freeze:contract][mir/main-qualified-recipe/context-missing]".to_owned());
        };
        Ok(Self { relation, context })
    }
}

impl crate::mir::builder::resolved_lowering::QualifiedMethodRecipePortV1
    for MainQualifiedMethodRecipePort<'_, '_, '_>
{
    fn take_qualified_method_recipe_v1(
        &mut self,
        builder: &mut MirBuilder,
        site: &SourceExprSiteV1,
        receiver: &str,
        selector: &str,
        argument_count: usize,
    ) -> Result<
        Option<(
            CanonicalSameModuleCallableKeyV1,
            Box<[SourceExprSiteV1]>,
            Option<crate::mir::callable_result_representation::
                VerifiedStaticCallResultPublicationHandoffV1>,
        )>,
        String,
    >{
        let Some(RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(caller),
            ..
        }) = self.context.current_source_context_v1()
        else {
            return Err("[freeze:contract][mir/main-qualified-recipe/context-missing]".to_owned());
        };
        let relation = self.relation.as_mut().ok_or_else(|| {
            "[freeze:contract][mir/main-qualified-recipe/relation-missing]".to_owned()
        })?;
        let take = relation
            .take_for_source(
                &caller,
                site,
                receiver,
                selector,
                u32::try_from(argument_count).map_err(|_| {
                    "[freeze:contract][mir/main-qualified-recipe/arity-overflow]".to_owned()
                })?,
            )
            .map_err(|error| error.to_string())?;
        let Some(take) = take else {
            return Ok(None);
        };
        let declarations = builder
            .comp_ctx
            .callable_declaration_catalog()
            .map_err(|_| {
                "[freeze:contract][mir/main-qualified-recipe/declarations-missing]".to_owned()
            })?;
        let publication = self
            .context
            .module_port
            .take_static_result_publication_handoff(declarations, &caller, site)
            .map_err(|error| {
                format!("[freeze:contract][mir/main-qualified-recipe/publication] {error:?}")
            })?;
        let publication = match publication {
            StaticCallResultPublicationTakeV1::Selected(handoff) => Some(handoff),
            StaticCallResultPublicationTakeV1::TargetOnly(target) => {
                return Err(format!(
                    "[freeze:contract][mir/main-qualified-recipe/target-only/{:?}] {}",
                    target.reason(),
                    target.target().mir_symbol_projection()
                ))
            }
            StaticCallResultPublicationTakeV1::NoExactStaticTarget => {
                return Err(
                    "[freeze:contract][mir/main-qualified-recipe/publication-target-missing]"
                        .to_owned(),
                )
            }
        };
        Ok(Some((
            take.declaration_key().clone(),
            take.argument_sites().to_vec().into_boxed_slice(),
            publication,
        )))
    }
}

pub(super) fn lower_app_main_root_body_v1(
    adapter: &mut NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>,
    builder: &mut MirBuilder,
    expected_identity: &CallableDeclarationIdentityV1,
    body: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let catalog_key = {
        let catalog = builder
            .comp_ctx
            .callable_declaration_catalog()
            .map_err(|_| {
                super::package_issue(
                    super::NormalCallableSemanticPackageInstallIssueV1::ForeignCatalog,
                )
            })?;
        let app_main = catalog.source_backed_app_main().ok_or_else(|| {
            super::package_issue(
                super::NormalCallableSemanticPackageInstallIssueV1::MainRootUnavailable,
            )
        })?;
        if !app_main.parser_identity().same_as(expected_identity) {
            return Err(super::package_issue(
                super::NormalCallableSemanticPackageInstallIssueV1::MainRootRelationMismatch,
            ));
        }
        app_main.catalog_key().clone()
    };
    adapter.install_app_main_qualified_receiver_relation()?;
    let inner = &mut *adapter.inner;
    let qualified_relation = &mut adapter.qualified_main_relation;
    let ordinary_new_claim_ledger = adapter.package.ordinary_new_claim_ledger();
    let named_array_emissions = adapter.package.named_array_emission_collector();
    let core_method_calls = adapter.package.take_source_core_method_calls(
        &crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(catalog_key.clone()),
    );
    adapter
        .package
        .with_app_main_root_lowering_input(&catalog_key, expected_identity, |input, identity| {
            for row in core_method_calls.values() {
                row.require_selected(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(catalog_key.clone()), input.owner())?;
            }
            let lineage = RawInvocationRootLineageV1::Cataloged(catalog_key.clone());
            let expected_lineage = lineage.clone();
            super::source_scope::with_callable_source_scope(
                inner,
                lineage,
                input,
                None,
                core_method_calls,
                identity.method_source_observation().cloned(),
                Some(Rc::clone(&named_array_emissions)),
                Rc::clone(&ordinary_new_claim_ledger),
                None,
                |inner, transport| {
                    inner.with_source_transport_v1(transport, |inner, ()| {
                        verify_raw_callable_owner_v1(identity.owner(), inner.callable_owner_v1())
                            .map_err(|error| {
                                format!("[freeze:contract][mir/callable-main/{error}]")
                            })?;
                        let context = inner.current_source_context_v1().ok_or_else(|| {
                            "[freeze:contract][mir/callable-main/raw-context-missing]".to_owned()
                        })?;
                        if !context.is_exact_function_root(&expected_lineage) {
                            return Err(
                                "[freeze:contract][mir/callable-main/raw-root-mismatch]".to_owned()
                            );
                        }
                        inner
                            .callable_ledger
                            .as_ref()
                            .ok_or_else(|| {
                                "[freeze:contract][mir/callable-main/ledger-missing]".to_owned()
                            })?
                            .borrow_mut()
                            .select_root_fault_frame()?;
                        ordinary_new_claim_ledger
                            .register_app_main_root(identity.owner(), expected_identity)?;
                        let parameter_count = builder
                            .function_state
                            .current_function
                            .as_ref()
                            .map(|function| function.params.len())
                            .ok_or_else(|| {
                                "[freeze:contract][mir/callable-main/current-function-missing]"
                                    .to_owned()
                            })?;
                        inner.adopt_callable_entry_values_v1(
                            builder,
                            CallableEntryShapeV1::Static { parameter_count },
                        )?;
                        // The canonical qualified-methods route only serves
                        // qualified (unbound-receiver) calls; lexical receiver
                        // calls like `pair.sum()` stay on the lifecycle/
                        // ordinary_new owner path below.
                        let value = if input.function().method_calls().any(|(_, call)| {
                            call.receiver()
                                == ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
                        }) {
                            let canonical_result = (|| {
                                let plan = CanonicalLoweringPreflightV1::
                                    verify_normal_main0_function_with_qualified_methods_v1(
                                        input,
                                        VerifiedNormalMainRoleV1::seal_for_qualified_methods(),
                                    )
                                    .map_err(|error| {
                                        format!(
                                            "[freeze:contract][mir/callable-main/qualified-preflight] {error:?}"
                                        )
                                    })?;
                                let mut recipe_port = MainQualifiedMethodRecipePort::new(
                                    qualified_relation,
                                    inner,
                                )?;
                                crate::mir::builder::resolved_lowering::
                                    lower_resolved_trivial_body_with_qualified_method_port_v1(
                                        builder,
                                        plan,
                                        &mut recipe_port,
                                    )
                            })();
                            // The App Main wrapper is still owned by the legacy
                            // outer function session. The canonical inner
                            // session has already finished its authority before
                            // returning, so clear that scoped guard before the
                            // legacy wrapper performs its normal cleanup.
                            builder.function_state.resolved_binding_state = Default::default();
                            canonical_result?
                        } else {
                            inner.lower_body(builder, body)?
                        };
                        inner.complete_construction_stores_v1(builder)?;
                        Ok(value)
                    })
                },
            )
        })
        .map_err(super::package_issue)?
}

fn verify_raw_callable_owner_v1(
    expected: FunctionOwnerIdV1,
    actual: Option<FunctionOwnerIdV1>,
) -> Result<(), &'static str> {
    let Some(actual) = actual else {
        return Err("raw-owner-missing");
    };
    if actual != expected {
        return Err("raw-owner-mismatch");
    }
    Ok(())
}

impl DirectCallDispositionPortV1
    for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>
{
    fn take_direct_call_disposition_v1(&mut self) -> Result<DirectCallDispositionRowV1, String> {
        self.inner.take_direct_call_disposition_v1()
    }

    fn emit_local_lifecycle_call_v1(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        row: DirectCallDispositionRowV1,
        arguments: Vec<crate::mir::ValueId>,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        self.inner
            .emit_local_lifecycle_call_v1(builder, row, arguments)
    }

    fn validate_current_call_argument_site_v1(
        &self,
        expected: &SourceExprSiteV1,
    ) -> Result<(), String> {
        self.inner.validate_current_call_argument_site_v1(expected)
    }
}

impl QualifiedStaticMethodHandoffPortV1
    for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>
{
    fn take_qualified_static_method_handoff_v1(
        &mut self,
        receiver: &str,
        method: &str,
        argument_count: usize,
    ) -> Result<QualifiedStaticMethodHandoffIngressV1, String> {
        let Some(relation) = self.qualified_main_relation.as_mut() else {
            return Ok(QualifiedStaticMethodHandoffIngressV1::Unavailable);
        };
        let Some(crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1::Located {
            root: crate::mir::builder::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(caller),
            site,
            ..
        }) = self.inner.current_source_context_v1() else {
            return Ok(QualifiedStaticMethodHandoffIngressV1::Unavailable);
        };
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site);
        let Some(take) = relation
            .take_for_source(&caller, &site, receiver, method, argument_count as u32)
            .map_err(|error| error.to_string())?
        else {
            return Ok(QualifiedStaticMethodHandoffIngressV1::Unavailable);
        };
        if take.argument_sites().len() != argument_count {
            return Err(
                "[freeze:contract][mir/main-qualified-relation/argument-cardinality]".to_owned(),
            );
        }
        Ok(QualifiedStaticMethodHandoffIngressV1::Ready(take))
    }
}

#[cfg(test)]
mod tests {
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    #[test]
    fn raw_callable_owner_witness_rejects_missing_or_foreign_owner() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("brand");
        let expected = issuer.issue().expect("expected owner");
        let foreign = issuer.issue().expect("foreign owner");

        assert!(super::verify_raw_callable_owner_v1(expected, Some(expected)).is_ok());
        assert_eq!(
            super::verify_raw_callable_owner_v1(expected, None),
            Err("raw-owner-missing")
        );
        assert_eq!(
            super::verify_raw_callable_owner_v1(expected, Some(foreign)),
            Err("raw-owner-mismatch")
        );
    }
}
