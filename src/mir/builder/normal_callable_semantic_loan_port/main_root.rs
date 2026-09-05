use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::normal_callable_binding_materialization_port::{
    CallableBindingMaterializationPortV1, CallableEntryShapeV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawSourceTransportPortV1;
use crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::{MirBuilder, ValueId};
use crate::parser::CallableDeclarationIdentityV1;

use super::super::raw_invocation_source_transport::RawInvocationRootLineageV1;
use super::super::recursive_child_lowering::{
    AppMainDirectCallDispositionPortV1, RecursiveChildLoweringPortV1,
};
use super::NormalCallableSemanticPackagePortAdapterV1;

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
    let inner = &mut *adapter.inner;
    let ordinary_new_claim_ledger = adapter.package.ordinary_new_claim_ledger();
    adapter
        .package
        .with_app_main_root_lowering_input(&catalog_key, expected_identity, |input, identity| {
            let lineage = RawInvocationRootLineageV1::Cataloged(catalog_key.clone());
            let expected_lineage = lineage.clone();
            super::with_callable_source_scope(
                inner,
                lineage,
                input,
                None,
                identity.method_source_observation().cloned(),
                Rc::clone(&ordinary_new_claim_ledger),
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
                        inner.callable_ledger.as_ref().ok_or_else(|| {
                            "[freeze:contract][mir/callable-main/ledger-missing]".to_owned()
                        })?.borrow_mut().select_root_fault_frame()?;
                        ordinary_new_claim_ledger.register_new_root(identity.owner())?;
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
                        let value = inner.lower_body(builder, body)?;
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

impl AppMainDirectCallDispositionPortV1
    for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>
{
    fn take_app_main_direct_call_disposition_v1(
        &mut self,
    ) -> Result<AppMainDirectCallDispositionRowV1, String> {
        self.inner.take_app_main_direct_call_disposition_v1()
    }

    fn validate_current_call_argument_site_v1(
        &self,
        expected: &SourceExprSiteV1,
    ) -> Result<(), String> {
        self.inner.validate_current_call_argument_site_v1(expected)
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
