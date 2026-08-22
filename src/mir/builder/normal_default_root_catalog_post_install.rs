//! Post-install root lowering adapter for the pre-effect Script handoff.
//!
//! This child keeps the lifecycle orchestrator small.  It consumes an already
//! bound Script source wrapper and retains the existing Bundle/Recipe/Join and
//! physical lowering order; it does not issue a new source authority.

use crate::ast::ASTNode;
use crate::mir::builder::main_expansion::VerifiedRawRootExpansionV1;
use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::builder::normal_default_root_catalog_lifecycle::NormalDefaultRootCatalogLifecycleErrorV1;
use crate::mir::builder::normal_script_pre_effect_source_observation::
    CanonicalScriptCBoundSourceV1;
use crate::mir::builder::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use crate::mir::builder::program_root_lowering::{
    NormalCallableSemanticPackageMode, NormalScriptRootLoweringMode,
};
use crate::mir::builder::program_root_work_plan::PreparedProgramRootWorkPlanPartsV1;
use crate::mir::builder::{
    MirBuilder, MirModule, NormalEntryMaterializationSourceReceiptV1, NormalRuntimeInputSnapshotV1,
};
use crate::mir::callable_result_representation::{
    VerifiedSameModuleCallableResultCatalogV1, VerifiedStaticCallResultPublicationOwnerV1,
};
use crate::mir::source_call_target::{
    VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
};

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
    import_rows: &[(String, String)],
    target_capability: Option<
        &crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1,
    >,
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
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                            error.into(),
                        )
                    })?,
            )
        }
        None => None,
    };

    let static_result_publication_owner = match preflight_static_result_publication_owner.take() {
        Some(owner) => owner,
        None => {
            let declarations =
                builder
                    .comp_ctx
                    .callable_declaration_catalog()
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                            format!("[mir/static-result-owner/catalog] {error}").into(),
                        )
                    })?;
            let imports =
                VerifiedStaticImportAliasViewV1::seal(declarations, import_rows.iter().cloned())
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                            format!("[mir/static-result-owner/imports] {error:?}").into(),
                        )
                    })?;
            let inventory =
                VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, &imports)
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                            format!("[mir/static-result-owner/targets] {error:?}").into(),
                        )
                    })?;
            let targets = inventory.into_targets();
            let results = VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
                .map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                        format!("[mir/static-result-owner/results] {error:?}").into(),
                    )
                })?;
            VerifiedStaticCallResultPublicationOwnerV1::issue(declarations, &targets, &results)
                .map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                        format!("[mir/static-result-owner/issue] {error:?}").into(),
                    )
                })?
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
            target_capability,
        )
        .map_err(|error| NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into()))?;
    builder
        .finalize_module(result_value)
        .map_err(|error| NormalDefaultRootCatalogLifecycleErrorV1::FinalizeModule(error.into()))
}
