//! Selected normal/default root and callable-catalog lifecycle.
//!
//! This owner consumes one isolated Builder session and preserves the legacy
//! root ordering without exposing mutable Builder access to the compiler.

use crate::ast::ASTNode;
use crate::parser::VerifiedFinalCallableProgramSourceV1;

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use super::main_expansion::VerifiedRawRootExpansionV1;
use super::normal_instance_constructor_admission::VerifiedInstanceConstructorPhysicalSourceCohortV1;
use super::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeV1;
use super::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use super::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationOwnerV1;
use super::normal_script_instance_box_transfer::VerifiedScriptInstanceBoxTransferCohortV1;
use super::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::program_root_lowering::{
    NormalCallableSemanticPackageMode, NormalScriptRootLoweringMode,
};
use super::program_root_work_plan::{
    PreparedProgramRootWorkPlanV1, ProgramRootWorkPlanAdmissionV1,
};
use super::{
    CallableMainMaterializationPolicyV1, MirModule, ModuleBuilderInvocationSessionV1,
    NormalEntryMaterializationSourceReceiptV1, NormalRuntimeInputSnapshotV1,
};
use crate::mir::callable_result_representation::{
    VerifiedSameModuleCallableResultCatalogV1, VerifiedStaticCallResultPublicationOwnerV1,
};
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    InstalledNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptForestOutcomeV1, ScriptSyntaxViewV1,
};
use crate::mir::source_call_target::{
    VerifiedScriptDirectStaticCallTargetInventoryV1, VerifiedStaticImportAliasViewV1,
    VerifiedWholeSourceStaticCallTargetInventoryV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalDefaultRootCatalogLifecycleStageV1 {
    RootExpansion,
    PrepareModule,
    CatalogSeal,
    CallableSemanticSeal,
    ScriptSemanticSeal,
    CatalogInstall,
    RootLower,
    FinalizeModule,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum NormalDefaultRootCatalogLifecycleErrorV1 {
    RootExpansion(Box<str>),
    PrepareModule(Box<str>),
    CatalogSeal(Box<str>),
    CallableSemanticSeal(Box<str>),
    ScriptSemanticSeal(Box<str>),
    CatalogInstall(Box<str>),
    RootLower(Box<str>),
    FinalizeModule(Box<str>),
}

impl NormalDefaultRootCatalogLifecycleErrorV1 {
    pub(in crate::mir) const fn stage(&self) -> NormalDefaultRootCatalogLifecycleStageV1 {
        match self {
            Self::RootExpansion(_) => NormalDefaultRootCatalogLifecycleStageV1::RootExpansion,
            Self::PrepareModule(_) => NormalDefaultRootCatalogLifecycleStageV1::PrepareModule,
            Self::CatalogSeal(_) => NormalDefaultRootCatalogLifecycleStageV1::CatalogSeal,
            Self::CallableSemanticSeal(_) => {
                NormalDefaultRootCatalogLifecycleStageV1::CallableSemanticSeal
            }
            Self::ScriptSemanticSeal(_) => {
                NormalDefaultRootCatalogLifecycleStageV1::ScriptSemanticSeal
            }
            Self::CatalogInstall(_) => NormalDefaultRootCatalogLifecycleStageV1::CatalogInstall,
            Self::RootLower(_) => NormalDefaultRootCatalogLifecycleStageV1::RootLower,
            Self::FinalizeModule(_) => NormalDefaultRootCatalogLifecycleStageV1::FinalizeModule,
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::RootExpansion(message)
            | Self::PrepareModule(message)
            | Self::CatalogSeal(message)
            | Self::CallableSemanticSeal(message)
            | Self::ScriptSemanticSeal(message)
            | Self::CatalogInstall(message)
            | Self::RootLower(message)
            | Self::FinalizeModule(message) => message,
        }
    }
}

impl std::fmt::Display for NormalDefaultRootCatalogLifecycleErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.message())
    }
}

impl std::error::Error for NormalDefaultRootCatalogLifecycleErrorV1 {}

#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalDefaultProgramRootV1 {
    source: PreparedNormalDefaultProgramSourceV1,
    _seal: PreparedNormalDefaultProgramRootSealV1,
}

#[derive(Debug)]
struct PreparedNormalDefaultProgramRootSealV1;

#[derive(Debug)]
enum PreparedNormalDefaultProgramSourceV1 {
    Callable(VerifiedFinalCallableProgramSourceV1),
    Compatibility(ASTNode),
}

impl PreparedNormalDefaultProgramRootV1 {
    pub(in crate::mir) fn seal(ast: ASTNode) -> Result<Self, ASTNode> {
        if !matches!(ast, ASTNode::Program { .. }) {
            return Err(ast);
        }
        Ok(Self {
            source: PreparedNormalDefaultProgramSourceV1::Compatibility(ast),
            _seal: PreparedNormalDefaultProgramRootSealV1,
        })
    }

    pub(in crate::mir) fn from_callable_source(
        source: VerifiedFinalCallableProgramSourceV1,
    ) -> Self {
        Self {
            source: PreparedNormalDefaultProgramSourceV1::Callable(source),
            _seal: PreparedNormalDefaultProgramRootSealV1,
        }
    }

    pub(super) fn source_ast(&self) -> &ASTNode {
        match &self.source {
            PreparedNormalDefaultProgramSourceV1::Callable(source) => source.ast(),
            PreparedNormalDefaultProgramSourceV1::Compatibility(ast) => ast,
        }
    }

    pub(super) fn clone_lowering_statements(&self) -> Vec<ASTNode> {
        match self.source_ast().clone() {
            ASTNode::Program { statements, .. } => statements,
            _ => unreachable!("sealed normal/default root must remain Program"),
        }
    }

    pub(in crate::mir) fn is_callable_source_backed(&self) -> bool {
        matches!(
            &self.source,
            PreparedNormalDefaultProgramSourceV1::Callable(_)
        )
    }
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalDefaultRootCatalogLifecycleV1 {
    session: ModuleBuilderInvocationSessionV1,
    module: MirModule,
}

impl CompletedNormalDefaultRootCatalogLifecycleV1 {
    pub(in crate::mir) fn into_parts(self) -> (ModuleBuilderInvocationSessionV1, MirModule) {
        (self.session, self.module)
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalDefaultRootCatalogLifecycleV1 {
    session: ModuleBuilderInvocationSessionV1,
    _source: Option<PreparedNormalDefaultProgramRootV1>,
    error: NormalDefaultRootCatalogLifecycleErrorV1,
}

impl RejectedNormalDefaultRootCatalogLifecycleV1 {
    pub(in crate::mir) fn stage(&self) -> NormalDefaultRootCatalogLifecycleStageV1 {
        self.error.stage()
    }

    pub(in crate::mir) fn error(&self) -> &NormalDefaultRootCatalogLifecycleErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl ModuleBuilderInvocationSessionV1 {
    pub(in crate::mir) fn complete_normal_default_program_root_catalog_lifecycle(
        self,
        source: PreparedNormalDefaultProgramRootV1,
        materialization_policy: CallableMainMaterializationPolicyV1,
        runtime_inputs: NormalRuntimeInputSnapshotV1,
    ) -> Result<
        CompletedNormalDefaultRootCatalogLifecycleV1,
        RejectedNormalDefaultRootCatalogLifecycleV1,
    > {
        self.complete_normal_default_program_root_catalog_lifecycle_with_target(
            source,
            materialization_policy,
            runtime_inputs,
            None,
        )
    }

    pub(in crate::mir) fn complete_normal_default_program_root_catalog_lifecycle_with_target(
        mut self,
        source: PreparedNormalDefaultProgramRootV1,
        materialization_policy: CallableMainMaterializationPolicyV1,
        runtime_inputs: NormalRuntimeInputSnapshotV1,
        target_capability: Option<
            crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1,
        >,
    ) -> Result<
        CompletedNormalDefaultRootCatalogLifecycleV1,
        RejectedNormalDefaultRootCatalogLifecycleV1,
    > {
        if let Err(error) = self.install_pinned_text_target_capability(target_capability) {
            return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                session: self,
                _source: Some(source),
                error: NormalDefaultRootCatalogLifecycleErrorV1::RootLower(
                    format!("[freeze:contract][pinned-text/invocation-binding] {error:?}").into(),
                ),
            });
        }
        let preflight_expansion =
            match VerifiedRawRootExpansionV1::from_program(source.source_ast()) {
                Ok(expansion) => expansion,
                Err(error) => {
                    return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                        session: self,
                        _source: Some(source),
                        error: NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                            format!("[mir/main-expansion/preflight] {error:?}").into(),
                        ),
                    })
                }
            };
        let preflight_is_app_mode = preflight_expansion.is_app_mode();
        drop(preflight_expansion);

        let declaration_facts =
            match PreparedNormalProgramDeclarationFactsV1::collect(source.source_ast()) {
                Ok(facts) => facts,
                Err(error) => {
                    return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                        session: self,
                        _source: Some(source),
                        error: NormalDefaultRootCatalogLifecycleErrorV1::CatalogSeal(
                            error.to_string().into(),
                        ),
                    })
                }
            };

        let mut resolver = match FunctionSemanticResolverSessionV1::new(0) {
            Ok(resolver) => resolver,
            Err(error) => {
                return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                    session: self,
                    _source: Some(source),
                    error: NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                        format!("[mir/callable-semantic/owner] {error:?}").into(),
                    ),
                })
            }
        };
        let (mut semantic_package, compatibility_source) = match source {
            PreparedNormalDefaultProgramRootV1 {
                source: PreparedNormalDefaultProgramSourceV1::Callable(callable),
                ..
            } => {
                let package = match declaration_facts.with_brand_catalog(|catalog| {
                    issue_normal_callable_semantic_package_with_brand_catalog_v1(
                        &mut resolver,
                        callable,
                        Some(catalog),
                    )
                }) {
                    Ok(package) => package,
                    Err(error) => {
                        return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                            session: self,
                            _source: None,
                            error: NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                                format!("[mir/callable-semantic-package/issue] {error:?}").into(),
                            ),
                        })
                    }
                };
                (Some(package), None)
            }
            compatibility => (None, Some(compatibility)),
        };
        let instance_box_transfers = match (preflight_is_app_mode, semantic_package.as_ref()) {
            (true, _) => None,
            (false, Some(package)) => Some(match VerifiedScriptInstanceBoxTransferCohortV1::issue(
                package.source_ast(),
                package,
            ) {
                Ok(cohort) => cohort,
                Err(error) => {
                    return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                        session: self,
                        _source: None,
                        error: NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                            format!("[mir/script-instance-box-transfer/issue] {error:?}").into(),
                        ),
                    })
                }
            }),
            (false, None) => None,
        };
        let constructor_source_cohort = if let Some(package) = semantic_package.as_ref() {
            match VerifiedInstanceConstructorPhysicalSourceCohortV1::issue(
                package.source_ast(),
                package,
            ) {
                Ok(cohort) => Some(cohort),
                Err(error) => {
                    return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                        session: self,
                        _source: None,
                        error: NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                            format!("[mir/instance-constructor-source/issue] {error:?}").into(),
                        ),
                    })
                }
            }
        } else {
            None
        };
        let brand = self.brand();
        let import_rows = self
            .config()
            .using_import_boxes()
            .iter()
            .map(|(alias, owner)| (alias.clone(), owner.clone()))
            .collect::<Vec<_>>();
        let result = self.with_builder_and_pinned_text_invocation_binding(|builder, binding| {
            let target_capability = binding
                .as_ref()
                .map(|binding| binding.target_capability());
            (|| {
                builder
                    .prepare_normal_default_module(runtime_inputs.entry_safepoint_enabled())
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::PrepareModule(error.into())
                    })?;

                let installed_package: Option<InstalledNormalCallableSemanticPackageV1> =
                    match semantic_package.take() {
                        Some(package) => Some(
                            package
                                .prepare_install(&mut builder.comp_ctx)
                                .map_err(|_package| {
                                    NormalDefaultRootCatalogLifecycleErrorV1::CatalogInstall(
                                        "[mir/callable-semantic-package/install] catalog slot occupied"
                                            .into(),
                                    )
                                })?
                                .commit(),
                        ),
                        None => {
                            let source = compatibility_source.as_ref().ok_or_else(|| {
                                NormalDefaultRootCatalogLifecycleErrorV1::CatalogSeal(
                                    "[mir/callable-catalog/source] compatibility source missing"
                                        .into(),
                                )
                            })?;
                            let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(
                                source.source_ast(),
                            )
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::CatalogSeal(
                                format!("[mir/callable-catalog/seal] {error:?}").into(),
                            )
                        })?;
                            builder
                                .comp_ctx
                                .install_callable_declaration_catalog(catalog)
                                .map_err(|error| {
                                    NormalDefaultRootCatalogLifecycleErrorV1::CatalogInstall(
                                        error.to_string().into(),
                                    )
                                })?;
                            None
                        }
                    };
                let source_ast = match installed_package.as_ref() {
                    Some(package) => package.source_ast(),
                    None => compatibility_source
                        .as_ref()
                        .expect("compatibility branch retained its source")
                        .source_ast(),
                };
                let expansion =
                    VerifiedRawRootExpansionV1::from_program(source_ast).map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                            format!("[mir/main-expansion/retained-source] {error:?}").into(),
                        )
                    })?;
                if expansion.is_app_mode() != preflight_is_app_mode {
                    return Err(NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                        "[mir/main-expansion/retained-source] disposition drift".into(),
                    ));
                }
                let receipt = NormalEntryMaterializationSourceReceiptV1::seal(
                    &expansion,
                    materialization_policy,
                );
                let lowering_statements = match source_ast.clone() {
                    ASTNode::Program { statements, .. } => statements,
                    _ => unreachable!("root expansion retained a Program"),
                };
                let declarations =
                    builder
                        .comp_ctx
                        .callable_declaration_catalog()
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                                format!("[mir/static-result-owner/catalog] {error}").into(),
                            )
                        })?;
                let work = PreparedProgramRootWorkPlanV1::prepare_with_instance_box_transfers_and_constructor_sources(
                    lowering_statements,
                    expansion.is_app_mode(),
                    ProgramRootWorkPlanAdmissionV1::SelectedNormal,
                    Some(declarations.selected_source_inventory()),
                    instance_box_transfers.as_ref(),
                    constructor_source_cohort.as_ref(),
                )
                .map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into())
                })?;
                let mut work = work.into_parts();
                let imports = VerifiedStaticImportAliasViewV1::seal(declarations, import_rows)
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                            format!("[mir/static-result-owner/imports] {error:?}").into(),
                        )
                    })?;
                if let Some(admission) = work.script_root_admission.as_mut() {
                    let inventory = VerifiedScriptDirectStaticCallTargetInventoryV1::issue(
                        source_ast,
                        admission.window(),
                        declarations,
                        &imports,
                    )
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                            format!("[mir/script-static-target/issue] {error:?}").into(),
                        )
                    })?;
                    admission
                        .attach_script_direct_static_targets(inventory)
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-static-target/attach] {error:?}").into(),
                            )
                        })?;
                }
                let mut script_source = match work.script_root_admission.as_ref() {
                    None => None,
                    Some(admission) => {
                        let window = admission.window();
                        let view =
                            ScriptSyntaxViewV1::from_program(source_ast).ok_or_else(|| {
                                NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                    "[mir/script-semantic/source-root] expected Program".into(),
                                )
                            })?;
                        let outcome =
                            declaration_facts.with_record_schema_demand_view(|record_schemas| {
                                declaration_facts.with_enum_variant_demand_view(|enum_variants| {
                                    declaration_facts.with_enum_match_demand_view(|enum_matches| {
                                        declaration_facts.with_brand_catalog(|brand_catalog| {
                                            resolver.resolve_script_forest_with_declaration_views(
                                                view,
                                                window,
                                                record_schemas,
                                                enum_variants,
                                                enum_matches,
                                                brand_catalog,
                                            )
                                        })
                                    })
                                })
                            });
                        match outcome.map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-semantic/seal] {error:?}").into(),
                            )
                        })? {
                            ResolveScriptForestOutcomeV1::Complete(forest) => Some(
                                VerifiedScriptSemanticSourceV1::seal_ast_with_forest(
                                    source_ast, forest, window,
                                )
                                .map_err(|error| {
                                    NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                        error.into(),
                                    )
                                })?,
                            ),
                            ResolveScriptForestOutcomeV1::Deferred => None,
                        }
                    }
                };
                let inventory =
                    VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, &imports)
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                                format!("[mir/static-result-owner/targets] {error:?}").into(),
                            )
                        })?;
                let targets = inventory.into_targets();
                let callable_mode = match installed_package.as_ref() {
                    Some(package) => NormalCallableSemanticPackageMode::Installed(package),
                    None => NormalCallableSemanticPackageMode::Compatibility,
                };
                let results =
                    VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                                format!("[mir/static-result-owner/results] {error:?}").into(),
                            )
                        })?;
                if let Some(source) = script_source.as_mut() {
                    let bundle = work
                        .script_root_admission
                        .as_mut()
                        .ok_or_else(|| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                "[mir/script-static-result/bundle] missing Script admission".into(),
                            )
                        })?
                        .with_taken_script_direct_static_targets(|window, target_inventory| {
                            VerifiedScriptDirectStaticResultBundleV1::issue(
                                source,
                                window,
                                &target_inventory,
                                declarations,
                                &imports,
                                &results,
                            )
                        })
                        .ok_or_else(|| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                "[mir/script-static-result/bundle] missing target inventory".into(),
                            )
                        })?
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-static-result/bundle] {error:?}").into(),
                            )
                        })?;
                    let publication_owner =
                        VerifiedScriptDirectStaticResultPublicationOwnerV1::issue(
                            source,
                            &bundle,
                            source.continuation(),
                        )
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-static-result/owner] {error:?}").into(),
                            )
                        })?;
                    let recipe = VerifiedScriptDirectStaticRecipeV1::issue(
                        &publication_owner,
                        work.script_root_admission
                            .as_ref()
                            .expect("Script admission remains attached")
                            .window(),
                    )
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                            format!("[mir/script-static-result/recipe] {error:?}").into(),
                        )
                    })?;
                    source
                        .attach_direct_static_result_bundle(bundle)
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-static-result/attach] {error}").into(),
                            )
                        })?;
                    source
                        .attach_direct_static_result_publication_owner(publication_owner)
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-static-result/owner-attach] {error}").into(),
                            )
                        })?;
                    source.attach_direct_static_recipe(recipe).map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                            format!("[mir/script-static-result/recipe-attach] {error}").into(),
                        )
                    })?;
                }
                let static_result_publication_owner =
                    VerifiedStaticCallResultPublicationOwnerV1::issue(
                        declarations,
                        &targets,
                        &results,
                    )
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                            format!("[mir/static-result-owner/issue] {error:?}").into(),
                        )
                    })?;
                let result_value = builder
                    .lower_normal_default_program_root_after_catalog_install_v1(
                        work,
                        source_ast,
                        &expansion,
                        &receipt,
                        &runtime_inputs,
                        brand,
                        declaration_facts,
                        callable_mode,
                        match script_source {
                            Some(source) => NormalScriptRootLoweringMode::Complete(source),
                            None => NormalScriptRootLoweringMode::Deferred,
                        },
                        static_result_publication_owner,
                        target_capability,
                    )
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into())
                    })?;
                builder.finalize_module(result_value).map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::FinalizeModule(error.into())
                })
            })()
        });

        match result {
            Ok(module) => Ok(CompletedNormalDefaultRootCatalogLifecycleV1 {
                session: self,
                module,
            }),
            Err(error) => Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                session: self,
                _source: compatibility_source,
                error,
            }),
        }
    }
}

#[cfg(test)]
#[path = "normal_default_root_catalog_lifecycle_tests.rs"]
mod normal_default_root_catalog_lifecycle_tests;
