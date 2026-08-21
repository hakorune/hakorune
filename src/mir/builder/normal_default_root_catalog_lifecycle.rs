//! Selected normal/default root and callable-catalog lifecycle.
//!
//! This owner consumes one isolated Builder session and preserves the legacy
//! root ordering without exposing mutable Builder access to the compiler.

use crate::ast::ASTNode;
use crate::parser::VerifiedFinalCallableProgramSourceV1;

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use super::main_expansion::VerifiedRawRootExpansionV1;
use super::normal_default_root_catalog_post_install::
    finish_normal_default_root_after_pre_effect_bind;
use super::normal_script_direct_static_lookup::ScriptDirectStaticCallLookupIssuerV1;
use super::normal_script_neutral_window::PreparedCanonicalScriptNeutralProgramWindowV1;
use super::normal_script_pre_effect_source_observation::{
    NormalScriptPreEffectSourceObservationIssuerV1, PreEffectCompleteSourceObservationV1,
};
use super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::program_root_lowering::NormalCallableSemanticPackageMode;
use super::program_root_work_plan::{
    PreparedProgramRootWorkPlanV1, ProgramRootWorkPlanAdmissionV1,
};
use super::{
    CallableMainMaterializationPolicyV1, MirModule, ModuleBuilderInvocationSessionV1,
    NormalEntryMaterializationSourceReceiptV1, NormalRuntimeInputSnapshotV1,
};
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    InstalledNormalCallableSemanticPackageV1,
};
use crate::mir::normal_source_plan::NormalCallableCompatibilityOriginV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;

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
    Callable {
        source: VerifiedFinalCallableProgramSourceV1,
    },
    TypedCompatibility(NormalCallableCompatibilityOriginV1),
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
            source: PreparedNormalDefaultProgramSourceV1::Callable { source },
            _seal: PreparedNormalDefaultProgramRootSealV1,
        }
    }

    pub(in crate::mir) fn from_compatibility_origin(
        origin: NormalCallableCompatibilityOriginV1,
    ) -> Self {
        Self {
            source: PreparedNormalDefaultProgramSourceV1::TypedCompatibility(origin),
            _seal: PreparedNormalDefaultProgramRootSealV1,
        }
    }

    pub(super) fn source_ast(&self) -> &ASTNode {
        match &self.source {
            PreparedNormalDefaultProgramSourceV1::Callable { source, .. } => source.ast(),
            PreparedNormalDefaultProgramSourceV1::TypedCompatibility(origin) => origin.ast(),
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
            PreparedNormalDefaultProgramSourceV1::Callable { .. }
        )
    }

    pub(in crate::mir) fn is_typed_compatibility(&self) -> bool {
        matches!(
            &self.source,
            PreparedNormalDefaultProgramSourceV1::TypedCompatibility(_)
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
                source: PreparedNormalDefaultProgramSourceV1::Callable { source: callable },
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
        let mut neutral_window = match semantic_package.as_ref() {
            None => None,
            Some(package) => match PreparedCanonicalScriptNeutralProgramWindowV1::issue(package) {
                Ok(window) => Some(window),
                Err(error) => {
                    return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                        session: self,
                        _source: None,
                        error: NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                            format!("[mir/script-neutral-window/issue] {error:?}").into(),
                        ),
                    })
                },
            }
        };
        let import_rows = self
            .config()
            .using_import_boxes()
            .iter()
            .map(|(alias, owner)| (alias.clone(), owner.clone()))
            .collect::<Vec<_>>();
        let lookup_window = if preflight_is_app_mode {
            None
        } else {
            neutral_window.as_ref()
        };
        let (mut script_lookup, mut preflight_static_result_publication_owner) =
            match semantic_package.as_ref() {
                None => (None, None),
                Some(package) => match ScriptDirectStaticCallLookupIssuerV1::issue(
                    package,
                    lookup_window,
                    &import_rows,
                ) {
                    Ok((lookup, publication_owner)) => (lookup, Some(publication_owner)),
                    Err(error) => {
                        return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                            session: self,
                            _source: None,
                            error: NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-static-lookup/preflight] {error:?}").into(),
                            ),
                        })
                    }
                },
            };
        let mut pre_effect_script_source: Option<PreEffectCompleteSourceObservationV1> =
            if preflight_is_app_mode {
                None
            } else {
                match (
                    semantic_package.as_ref(),
                    neutral_window.as_ref(),
                    script_lookup.take(),
                ) {
                    (Some(package), Some(window), Some(lookup)) => {
                        match NormalScriptPreEffectSourceObservationIssuerV1::issue(
                            package,
                            window,
                            lookup,
                            &declaration_facts,
                            &mut resolver,
                        ) {
                            Ok(observation) => Some(observation),
                            Err(error) => {
                                return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                                    session: self,
                                    _source: None,
                                    error:
                                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                            format!("[mir/script-pre-effect/source] {error:?}")
                                                .into(),
                                        ),
                                })
                            }
                        }
                    }
                    (None, None, None) => None,
                    _ => {
                        return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                            session: self,
                            _source: None,
                            error: NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                "[mir/script-pre-effect/source] incomplete selected-normal source inputs"
                                    .into(),
                            ),
                        })
                    }
                }
            };
        if let Err(error) = self.install_pinned_text_target_capability(target_capability) {
            return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                session: self,
                _source: None,
                error: NormalDefaultRootCatalogLifecycleErrorV1::RootLower(
                    format!("[freeze:contract][pinned-text/invocation-binding] {error:?}").into(),
                ),
            });
        }
        let brand = self.brand();
        let (script_root_admission, constructor_source_cohort) = match neutral_window.take() {
            Some(window) => {
                let (admission, _instance_transfers, constructor_source_cohort) =
                    window.into_parts();
                (
                    (!preflight_is_app_mode).then_some(admission),
                    Some(constructor_source_cohort),
                )
            }
            None => (None, None),
        };
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
                let work = PreparedProgramRootWorkPlanV1::prepare_with_script_root_admission_and_constructor_sources(
                    lowering_statements,
                    expansion.is_app_mode(),
                    ProgramRootWorkPlanAdmissionV1::SelectedNormal,
                    Some(declarations.selected_source_inventory()),
                    constructor_source_cohort.as_ref(),
                    script_root_admission,
                )
                .map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into())
                })?;
                let work = work.into_parts();
                let result = match (installed_package.as_ref(), pre_effect_script_source.take()) {
                    (Some(package), Some(observation)) => observation
                        .with_bound_source(package, |source, lookup| {
                            finish_normal_default_root_after_pre_effect_bind(
                                builder,
                                work,
                                source_ast,
                                &expansion,
                                &receipt,
                                &runtime_inputs,
                                brand,
                                declaration_facts,
                                NormalCallableSemanticPackageMode::Installed(package),
                                Some(source),
                                Some(lookup),
                                &mut preflight_static_result_publication_owner,
                                &import_rows,
                                target_capability,
                            )
                        })
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                format!("[mir/script-pre-effect/rebind] {error:?}").into(),
                            )
                        })?,
                    (Some(package), None) => finish_normal_default_root_after_pre_effect_bind(
                        builder,
                        work,
                        source_ast,
                        &expansion,
                        &receipt,
                        &runtime_inputs,
                        brand,
                        declaration_facts,
                        NormalCallableSemanticPackageMode::Installed(package),
                        None,
                        script_lookup.take(),
                        &mut preflight_static_result_publication_owner,
                        &import_rows,
                        target_capability,
                    ),
                    (None, None) => finish_normal_default_root_after_pre_effect_bind(
                        builder,
                        work,
                        source_ast,
                        &expansion,
                        &receipt,
                        &runtime_inputs,
                        brand,
                        declaration_facts,
                        NormalCallableSemanticPackageMode::Compatibility,
                        None,
                        script_lookup.take(),
                        &mut preflight_static_result_publication_owner,
                        &import_rows,
                        target_capability,
                    ),
                    _ => Err(NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                        "[mir/script-pre-effect/rebind] source package/observation mismatch".into(),
                    )),
                };
                result
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
