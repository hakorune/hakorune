//! Selected Main0 Continue canonical-root handoff.
//!
//! The pre-wrapper selection already proved the exact profile and owns the
//! verified recipe product.  This sibling consumes the one-shot App Main
//! root loan exactly once, issues the prepared operation program inside the
//! same loan scope, lowers the whole function through the canonical
//! draft-seal session, and admits the sealed `main` draft to the invocation
//! collector.  It never opens the legacy wrapper and never emits a raw
//! Return on the draft's behalf.

use crate::mir::builder::normal_main0_continue_prepared_operation::PreparedMain0ContinueLoopIngressV1;
use crate::mir::builder::resolved_lowering::{
    commit_callable_single_loop_ready_to_pending_v1, lower_main0_continue_function_draft_v1,
};
use crate::mir::compiler::main0_continue_recipe_coseal::VerifiedMain0ContinueRecipeProductV1;
use crate::mir::MirBuilder;
use crate::parser::CallableDeclarationIdentityV1;

use super::NormalCallableSemanticPackagePortAdapterV1;

pub(super) fn lower_app_main0_continue_root_v1(
    adapter: &mut NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>,
    builder: &mut MirBuilder,
    expected_identity: &CallableDeclarationIdentityV1,
    product: VerifiedMain0ContinueRecipeProductV1,
) -> Result<(), String> {
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
    let ordinary_new_claim_ledger = adapter.package.ordinary_new_claim_ledger();
    adapter
        .package
        .with_app_main_root_lowering_input(&catalog_key, expected_identity, |input, identity| {
            ordinary_new_claim_ledger
                .register_app_main_root(identity.owner(), expected_identity)
                .map_err(|error| {
                    format!("[freeze:contract][mir/main0-root/ordinary-new-claim] {error}")
                })?;
            // The canonical draft bypasses direct-call disposition rows for
            // this exact owner on purpose; the mark keeps the loan ledger
            // honest instead of leaving an unexamined row behind.
            if let Some(loans) = inner.direct_call_loans.as_deref_mut() {
                loans.mark_canonical_route_bypass(input.owner());
            }
            let ingress = PreparedMain0ContinueLoopIngressV1::issue(input, product)
                .map_err(|reject| {
                    format!("[freeze:contract][mir/main0-root/ingress] {reject:?}")
                })?;
            let program = ingress.prepare_full_demand().map_err(|reject| {
                format!("[freeze:contract][mir/main0-root/demand] {reject:?}")
            })?;
            let mut session = builder.open_resolved_function_draft_seal_session_v1("main");
            let ready =
                lower_main0_continue_function_draft_v1(&mut session, program, "main".to_owned())
                    .map_err(|error| format!("[freeze:contract][mir/main0-root/lower] {error}"))?;
            let pending = commit_callable_single_loop_ready_to_pending_v1(session, ready)
                .map_err(|error| format!("[freeze:contract][mir/main0-root/draft-seal] {error}"))?;
            inner
                .module_port
                .complete_main0_continue_root_draft_v1(pending)
                .map_err(|error| format!("[freeze:contract][mir/main0-root/admission] {error}"))
        })
        .map_err(super::package_issue)?
}
