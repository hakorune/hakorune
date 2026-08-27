//! Selected normal/default root and callable-catalog lifecycle.
//!
//! This owner consumes one isolated Builder session and preserves the legacy
//! root ordering without exposing mutable Builder access to the compiler.

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use super::main_expansion::VerifiedRawRootExpansionV1;
use super::normal_default_program_root::{
    NormalDefaultProgramRootConsumptionV1, PreparedNormalDefaultProgramRootV1,
    RejectedNormalDefaultRootOwnerV1,
};
use super::normal_default_root_catalog_post_install::finish_normal_default_root_after_pre_effect_bind;
use super::normal_script_direct_static_lookup::ScriptDirectStaticCallLookupIssuerV1;
use super::normal_script_neutral_window::PreparedCanonicalScriptNeutralProgramWindowV1;
use super::normal_script_pre_effect_source_observation::{
    issue_into_c_transport, NormalScriptPreEffectSourceObservationIssuerV1,
};
use super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::program_root_lowering::NormalCallableSemanticPackageMode;
use super::program_root_work_plan::{
    PreparedProgramRootWorkPlanV1, ProgramRootWorkPlanAdmissionV1,
};
use super::raw_source_projection::OwnedRawRootProjectionV1;
use super::{
    AdmittedNormalRootExecutionModeV1, CallableMainMaterializationPolicyV1, MirModule,
    ModuleBuilderInvocationSessionV1, NormalEntryMaterializationSourceReceiptV1,
    NormalRuntimeInputSnapshotV1, RawEntryMaterializationSourceReceiptV1,
};
use super::{BuilderInstallConsumerV1, BuilderPrivateInstalledCallablePackageBundleV1};
use crate::ast::ASTNode;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_v1;
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
    _source: Option<RejectedNormalDefaultRootOwnerV1>,
    error: NormalDefaultRootCatalogLifecycleErrorV1,
}

impl RejectedNormalDefaultRootCatalogLifecycleV1 {
    pub(in crate::mir) fn stage(&self) -> NormalDefaultRootCatalogLifecycleStageV1 {
        self.error.stage()
    }

    pub(in crate::mir) fn error(&self) -> &NormalDefaultRootCatalogLifecycleErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        let Self {
            session,
            _source,
            error,
        } = self;
        if let Some(source) = _source {
            source.discard_at_named_lifecycle_terminal();
        }
        drop((session, error));
    }
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
        let (mut callable_source, compatibility_source, preflight_is_app_mode) = match source
            .consume_source_backed_root_once()
        {
            NormalDefaultProgramRootConsumptionV1::SourceBacked(consumption) => match consumption {
                Ok(source) => {
                    let is_app = source.mode() == AdmittedNormalRootExecutionModeV1::App;
                    (Some(source.into_consumed_source()), None, is_app)
                }
                Err(rejected) => {
                    let error = format!("{:?}", rejected.error());
                    return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                        session: self,
                        _source: Some(RejectedNormalDefaultRootOwnerV1::RootExecution(rejected)),
                        error: NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                            format!("[mir/normal-root/consume] {error}").into(),
                        ),
                    });
                }
            },
            NormalDefaultProgramRootConsumptionV1::Compatibility(compatibility) => {
                let compatibility = RejectedNormalDefaultRootOwnerV1::Compatibility(compatibility);
                let expansion =
                    match VerifiedRawRootExpansionV1::from_program(compatibility.source_ast()) {
                        Ok(expansion) => expansion,
                        Err(error) => {
                            return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                                session: self,
                                _source: Some(compatibility),
                                error: NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                                    format!(
                                        "[mir/main-expansion/compatibility-preflight] {error:?}"
                                    )
                                    .into(),
                                ),
                            })
                        }
                    };
                let is_app = expansion.is_app_mode();
                drop(expansion);
                (None, Some(compatibility), is_app)
            }
        };

        let source_ast = callable_source
            .as_ref()
            .map(|source| source.source().ast())
            .or_else(|| {
                compatibility_source
                    .as_ref()
                    .map(|source| source.source_ast())
            })
            .expect("one normal/default source route remains owned");
        let declaration_facts = match PreparedNormalProgramDeclarationFactsV1::collect(source_ast) {
            Ok(facts) => facts,
            Err(error) => {
                return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                    session: self,
                    _source: compatibility_source,
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
                    _source: compatibility_source,
                    error: NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                        format!("[mir/callable-semantic/owner] {error:?}").into(),
                    ),
                })
            }
        };
        let mut semantic_package = match callable_source.take() {
            Some(callable) => {
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
                Some(package)
            }
            None => None,
        };
        let source_backed_root_execution = match semantic_package.as_mut() {
            Some(package) => {
                let root = match package.take_root_execution() {
                    Ok(root) => root,
                    Err(()) => {
                        return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                            session: self,
                            _source: None,
                            error: NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                                "[mir/normal-root/pre-effect-projection] already consumed".into(),
                            ),
                        })
                    }
                };
                if root.is_app_mode() != preflight_is_app_mode {
                    return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                        session: self,
                        _source: None,
                        error: NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                            "[mir/normal-root/pre-effect-projection] admitted mode drift".into(),
                        ),
                    });
                }
                Some(root)
            }
            None => None,
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
                }
            },
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
        let (mut pre_effect_script_source, constructor_source_cohort) = match (
            semantic_package.as_ref(),
            neutral_window.take(),
        ) {
            (Some(package), Some(window)) => {
                let (source_window, post_install) = window.split_for_pre_effect();
                let (_instance_transfers, constructor_source_cohort) = post_install.into_parts();
                let observation = if preflight_is_app_mode {
                    None
                } else {
                    let Some(lookup) = script_lookup.take() else {
                        return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                            session: self,
                            _source: None,
                            error: NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                "[mir/script-pre-effect/source] missing selected-normal lookup"
                                    .into(),
                            ),
                        });
                    };
                    match NormalScriptPreEffectSourceObservationIssuerV1::issue(
                        package,
                        source_window,
                        lookup,
                        &declaration_facts,
                        &mut resolver,
                    ) {
                        Ok(observation) => match issue_into_c_transport(observation) {
                            Ok(transport) => Some(transport),
                            Err(error) => {
                                return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                                    session: self,
                                    _source: None,
                                    error:
                                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                            format!("[mir/script-a/capability] {error:?}").into(),
                                        ),
                                })
                            }
                        },
                        Err(error) => {
                            return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                                session: self,
                                _source: None,
                                error: NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                    format!("[mir/script-pre-effect/source] {error:?}").into(),
                                ),
                            })
                        }
                    }
                };
                (observation, Some(constructor_source_cohort))
            }
            (None, None) => (None, None),
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
        let result = self
            .with_builder_and_pinned_text_invocation_binding_and_callable_loop_scope(
                |builder, binding, callable_loop_root_scope| {
            (|| {
                builder
                    .prepare_normal_default_module(runtime_inputs.entry_safepoint_enabled())
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::PrepareModule(error.into())
                    })?;

                let installed_package: Option<BuilderPrivateInstalledCallablePackageBundleV1> =
                    match semantic_package.take() {
                        Some(package) => Some(
                            package.with_normal_callable_install_once(
                                &mut builder.comp_ctx,
                                BuilderInstallConsumerV1::new(),
                            )
                                .map_err(|error| {
                                    NormalDefaultRootCatalogLifecycleErrorV1::CatalogInstall(
                                        format!(
                                            "[mir/callable-semantic-package/install] {error:?}"
                                        )
                                        .into(),
                                    )
                                })
                                ?,
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
                let lower_with_expansion =
                    |source_ast: &ASTNode, expansion: VerifiedRawRootExpansionV1<'_>| {
                        let receipt = NormalEntryMaterializationSourceReceiptV1::seal(
                            &expansion,
                            materialization_policy,
                        );
                        let raw_materialization = if installed_package.is_none() {
                            let projection = OwnedRawRootProjectionV1::from_verified(
                                source_ast,
                                &expansion,
                            )
                            .map_err(|error| {
                                NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                                    format!("[mir/raw-root/projection] {error:?}").into(),
                                )
                            })?;
                            RawEntryMaterializationSourceReceiptV1::seal(
                                &projection,
                                materialization_policy,
                            )
                        } else {
                            None
                        };
                        let lowering_statements = match source_ast.clone() {
                            ASTNode::Program { statements, .. } => statements,
                            _ => unreachable!("root expansion retained a Program"),
                        };
                        let declarations = builder
                            .comp_ctx
                            .callable_declaration_catalog()
                            .map_err(|error| {
                                NormalDefaultRootCatalogLifecycleErrorV1::CallableSemanticSeal(
                                    format!("[mir/static-result-owner/catalog] {error}").into(),
                                )
                            })?;
                        let (script_root_admission, mut pre_effect_script_source) =
                            match pre_effect_script_source.take() {
                                Some(observation) => {
                                    let (admission, observation) =
                                        observation.split_for_work_plan();
                                    (Some(admission), Some(observation))
                                }
                                None => (None, None),
                            };
                        let (work_plan_admission, selected_callable_sources, constructor_sources,
                            script_root_admission) = match installed_package.as_ref() {
                            Some(_) => (
                                ProgramRootWorkPlanAdmissionV1::SelectedNormal,
                                Some(declarations.selected_source_inventory()),
                                constructor_source_cohort.as_ref(),
                                script_root_admission,
                            ),
                            None => (
                                ProgramRootWorkPlanAdmissionV1::RawCompatibility,
                                None,
                                None,
                                None,
                            ),
                        };
                        let work = PreparedProgramRootWorkPlanV1::prepare_with_script_root_admission_and_constructor_sources(
                            lowering_statements,
                            preflight_is_app_mode,
                            work_plan_admission,
                            selected_callable_sources,
                            constructor_sources,
                            script_root_admission,
                        )
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into())
                        })?
                        .into_parts();
                        match (installed_package.as_ref(), pre_effect_script_source.take()) {
                            (Some(package), Some(observation)) => package
                                .with_normal_program_source_loan(|loan| {
                                    observation
                                        .bind_source_loan(loan, |source| {
                                            finish_normal_default_root_after_pre_effect_bind(
                                                builder,
                                                work,
                                                source_ast,
                                                &expansion,
                                                &receipt,
                                                &runtime_inputs,
                                                brand,
                                                declaration_facts,
                                                NormalCallableSemanticPackageMode::Installed(
                                                    package,
                                                ),
                                                Some(source),
                                                &mut preflight_static_result_publication_owner,
                                                &import_rows,
                                                binding,
                                                callable_loop_root_scope,
                                            )
                                        })
                                        .map_err(|error| {
                                            NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                                format!("[mir/script-pre-effect/rebind] {error:?}").into(),
                                            )
                                        })
                                        .and_then(|result| result)
                                })
                                .map_err(|error| {
                                    NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                        format!("[mir/script-pre-effect/source-loan] {error:?}").into(),
                                    )
                                })
                                .and_then(|result| result),
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
                                &mut preflight_static_result_publication_owner,
                                &import_rows,
                                binding,
                                callable_loop_root_scope,
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
                                NormalCallableSemanticPackageMode::Compatibility(
                                    raw_materialization,
                                ),
                                None,
                                &mut preflight_static_result_publication_owner,
                                &import_rows,
                                binding,
                                callable_loop_root_scope,
                            ),
                            _ => Err(
                                NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                                    "[mir/script-pre-effect/rebind] source package/observation mismatch"
                                        .into(),
                                ),
                            ),
                        }
                };
                match installed_package.as_ref() {
                    Some(package) => package
                        .with_normal_program_source_loan(|loan| {
                            source_backed_root_execution
                                .expect("source-backed package retained its pre-effect root projection")
                                .consume_lowering_view_once(|expansion| {
                                    lower_with_expansion(loan.program(), expansion)
                                })
                        })
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                                format!("[mir/main-expansion/source-loan] {error:?}").into(),
                            )
                        })
                        .and_then(|result| result),
                    None => {
                        let source_ast = compatibility_source
                            .as_ref()
                            .expect("compatibility branch retained its source")
                            .source_ast();
                        let expansion = VerifiedRawRootExpansionV1::from_program(source_ast)
                            .map_err(|error| {
                                NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                                    format!("[mir/main-expansion/compatibility-retained] {error:?}")
                                        .into(),
                                )
                            })?;
                        if expansion.is_app_mode() != preflight_is_app_mode {
                            return Err(
                                NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                                    "[mir/main-expansion/compatibility-retained] disposition drift"
                                        .into(),
                                ),
                            );
                        }
                        lower_with_expansion(source_ast, expansion)
                    }
                }
            })()
                },
            );

        match result {
            Ok(module) => {
                if let Some(source) = compatibility_source {
                    source.discard_at_named_lifecycle_terminal();
                }
                Ok(CompletedNormalDefaultRootCatalogLifecycleV1 {
                    session: self,
                    module,
                })
            }
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
