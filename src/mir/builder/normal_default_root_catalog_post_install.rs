//! Post-install root lowering adapter for the pre-effect Script handoff.
//!
//! This child keeps the lifecycle orchestrator small.  It consumes an already
//! bound Script source wrapper and retains the existing Bundle/Recipe/Join and
//! physical lowering order; it does not issue a new source authority.

use crate::ast::ASTNode;
use crate::mir::builder::main_expansion::VerifiedRawRootExpansionV1;
use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::builder::normal_default_root_catalog_lifecycle::NormalDefaultRootCatalogLifecycleErrorV1;
use crate::mir::builder::normal_script_pre_effect_source_observation::CanonicalScriptCBoundSourceV1;
use crate::mir::builder::pinned_text_invocation_binding::PinnedTextCompileInvocationBindingRefV1;
use crate::mir::builder::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use crate::mir::builder::program_root_lowering::{
    NormalCallableSemanticPackageMode, NormalScriptRootLoweringMode,
};
use crate::mir::builder::program_root_work_plan::PreparedProgramRootWorkPlanPartsV1;
use crate::mir::builder::{
    MirBuilder, MirModule, NormalEntryMaterializationSourceReceiptV1, NormalRuntimeInputSnapshotV1,
    UnpublishedCallableLoopRootScopeV1,
};
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationOwnerV1;

pub(super) fn finish_normal_default_root_after_pre_effect_bind<'source, 'package>(
    builder: &mut MirBuilder,
    work: PreparedProgramRootWorkPlanPartsV1,
    source_ast: &'source ASTNode,
    expansion: &VerifiedRawRootExpansionV1<'_>,
    materialization: &NormalEntryMaterializationSourceReceiptV1,
    runtime_inputs: &NormalRuntimeInputSnapshotV1,
    brand: ModuleInvocationBrandV1,
    declaration_facts: PreparedNormalProgramDeclarationFactsV1,
    callable_mode: NormalCallableSemanticPackageMode<'package>,
    script_source: Option<CanonicalScriptCBoundSourceV1<'source>>,
    preflight_static_result_publication_owner: &mut Option<
        VerifiedStaticCallResultPublicationOwnerV1,
    >,
    _import_rows: &[(String, String)],
    target_binding: Option<PinnedTextCompileInvocationBindingRefV1<'_>>,
    callable_loop_root_scope: &mut UnpublishedCallableLoopRootScopeV1,
) -> Result<MirModule, NormalDefaultRootCatalogLifecycleErrorV1> {
    let script_source = match script_source {
        Some(bound) => {
            let admission = work.script_root_admission.as_ref().ok_or_else(|| {
                NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                    "[mir/script-a-c/consumer] missing Script admission".into(),
                )
            })?;
            Some(
                bound
                    .consume_into_lowering_source(admission)
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(error.into())
                    })?,
            )
        }
        None => None,
    };

    let root_new_ledger = match &callable_mode {
        NormalCallableSemanticPackageMode::Installed(port) => Some(port.ordinary_new_claim_ledger()),
        NormalCallableSemanticPackageMode::Compatibility(_) => None,
    };
    let source_backed = matches!(
        &callable_mode,
        NormalCallableSemanticPackageMode::Installed(_)
    );
    let static_result_publication_owner = match (
        source_backed,
        preflight_static_result_publication_owner.take(),
    ) {
        (true, Some(owner)) => Some(owner),
        (true, None) => {
            return Err(
                NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                    "[mir/static-result-owner/source-backed/missing]".into(),
                ),
            );
        }
        (false, None) => None,
        (false, Some(_)) => {
            return Err(
                NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                    "[mir/static-result-owner/compatibility/drift]".into(),
                ),
            );
        }
    };

    let result_value = builder
        .lower_normal_default_program_root_after_catalog_install_v1(
            work,
            source_ast,
            expansion,
            materialization,
            runtime_inputs,
            brand,
            declaration_facts,
            callable_mode,
            match script_source {
                Some(source) => NormalScriptRootLoweringMode::Complete(source),
                None => NormalScriptRootLoweringMode::Unavailable,
            },
            static_result_publication_owner,
            target_binding,
            callable_loop_root_scope,
        )
        .map_err(|error| NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into()))?;
    builder
        .finalize_module_with_root_validation(result_value, |function| {
            match root_new_ledger {
                Some(ledger) => ledger.validate_finalized_new_root(function),
                None => Ok(()),
            }
        })
        .map_err(|error| NormalDefaultRootCatalogLifecycleErrorV1::FinalizeModule(error.into()))
}
