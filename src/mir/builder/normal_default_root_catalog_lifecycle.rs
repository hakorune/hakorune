//! Selected normal/default root and callable-catalog lifecycle.
//!
//! This owner consumes one isolated Builder session and preserves the legacy
//! root ordering without exposing mutable Builder access to the compiler.

use crate::ast::ASTNode;

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use super::main_expansion::VerifiedRawRootExpansionV1;
use super::program_root_work_plan::{
    PreparedProgramRootWorkPlanV1, ProgramRootWorkPlanAdmissionV1,
};
use super::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use super::program_root_lowering::NormalScriptRootLoweringMode;
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
use super::{
    CallableMainMaterializationPolicyV1, MirModule, ModuleBuilderInvocationSessionV1,
    NormalEntryMaterializationSourceReceiptV1, NormalRuntimeInputSnapshotV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalDefaultRootCatalogLifecycleStageV1 {
    RootExpansion,
    PrepareModule,
    CatalogSeal,
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
    ast: ASTNode,
    _seal: PreparedNormalDefaultProgramRootSealV1,
}

#[derive(Debug)]
struct PreparedNormalDefaultProgramRootSealV1;

impl PreparedNormalDefaultProgramRootV1 {
    pub(in crate::mir) fn seal(ast: ASTNode) -> Result<Self, ASTNode> {
        if !matches!(ast, ASTNode::Program { .. }) {
            return Err(ast);
        }
        Ok(Self {
            ast,
            _seal: PreparedNormalDefaultProgramRootSealV1,
        })
    }

    pub(super) fn source_ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(super) fn clone_lowering_statements(&self) -> Vec<ASTNode> {
        match self.ast.clone() {
            ASTNode::Program { statements, .. } => statements,
            _ => unreachable!("sealed normal/default root must remain Program"),
        }
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
    _source: PreparedNormalDefaultProgramRootV1,
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
        mut self,
        source: PreparedNormalDefaultProgramRootV1,
        materialization_policy: CallableMainMaterializationPolicyV1,
        runtime_inputs: NormalRuntimeInputSnapshotV1,
    ) -> Result<
        CompletedNormalDefaultRootCatalogLifecycleV1,
        RejectedNormalDefaultRootCatalogLifecycleV1,
    > {
        let brand = self.brand();
        let result = {
            let builder = self.builder_mut();
            (|| {
                let expansion = VerifiedRawRootExpansionV1::from_program(source.source_ast())
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                            format!("[mir/main-expansion/preflight] {error:?}").into(),
                        )
                    })?;
                let receipt = NormalEntryMaterializationSourceReceiptV1::seal(
                    &expansion,
                    materialization_policy,
                );
                builder
                    .prepare_normal_default_module(runtime_inputs.entry_safepoint_enabled())
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::PrepareModule(error.into())
                    })?;

                let lowering_statements = source.clone_lowering_statements();
                let catalog =
                    VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(source.source_ast())
                        .map_err(|error| {
                            NormalDefaultRootCatalogLifecycleErrorV1::CatalogSeal(
                                format!("[mir/callable-catalog/seal] {error:?}").into(),
                            )
                        })?;
                let work = PreparedProgramRootWorkPlanV1::prepare(
                    lowering_statements,
                    expansion.is_app_mode(),
                    ProgramRootWorkPlanAdmissionV1::SelectedNormal,
                );
                let work = work.into_parts();
                let script_source = if work.runtime.is_literal_only() {
                    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                            format!("[mir/script-semantic/owner] {error:?}").into(),
                        )
                    })?;
                    let owner = issuer.issue().map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(
                            format!("[mir/script-semantic/owner] {error:?}").into(),
                        )
                    })?;
                    Some(VerifiedScriptSemanticSourceV1::seal(
                        &source,
                        owner,
                        work.runtime.literal_source_indices(),
                    ).map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::ScriptSemanticSeal(error.into())
                    })?)
                } else {
                    None
                };
                builder
                    .comp_ctx
                    .install_callable_declaration_catalog(catalog)
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::CatalogInstall(
                            error.to_string().into(),
                        )
                    })?;
                let result_value = builder
                    .lower_normal_default_program_root_after_catalog_install_v1(
                        work,
                        &source,
                        &expansion,
                        &receipt,
                        &runtime_inputs,
                        brand,
                        match script_source.as_ref() {
                            Some(source) => NormalScriptRootLoweringMode::Complete(source),
                            None => NormalScriptRootLoweringMode::Deferred,
                        },
                    )
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into())
                    })?;
                builder.finalize_module(result_value).map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::FinalizeModule(error.into())
                })
            })()
        };

        match result {
            Ok(module) => Ok(CompletedNormalDefaultRootCatalogLifecycleV1 {
                session: self,
                module,
            }),
            Err(error) => Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                session: self,
                _source: source,
                error,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mir::builder::{
        BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1, MirBuilder,
        ModuleBuilderInvocationSessionV1, NormalDefaultRootCatalogLifecycleStageV1,
        NormalRuntimeInputSnapshotV1, PreparedNormalDefaultProgramRootV1,
    };
    use crate::parser::NyashParser;

    fn session() -> ModuleBuilderInvocationSessionV1 {
        let current = MirBuilder::new();
        let config = BuilderInvocationConfigV1::snapshot_for_raw(&current, None);
        ModuleBuilderInvocationSessionV1::open(&current, config)
    }

    #[test]
    fn verified_expansion_disposition_reaches_script_and_app_root_lowering() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        for (source, expected_app_mode) in [
            ("42", false),
            ("static box Main { main() { return 0 } }", true),
        ] {
            let source = NyashParser::parse_from_string(source).expect("route source");
            let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
            let completed = session()
                .complete_normal_default_program_root_catalog_lifecycle(
                    source,
                    CallableMainMaterializationPolicyV1::Omitted,
                    NormalRuntimeInputSnapshotV1::empty(),
                )
                .expect("verified route must lower");
            let (session, _) = completed.into_parts();

            assert_eq!(session.builder().root_is_app_mode, Some(expected_app_mode));
        }
    }

    #[test]
    fn root_expansion_failure_precedes_prepare_and_retains_source() {
        let source = NyashParser::parse_from_string(
            r#"
                static box Main { main() { return 0 } }
                static box Main { main() { return 1 } }
            "#,
        )
        .expect("duplicate Main source");
        let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
        let rejected = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect_err("duplicate Main must reject before prepare");

        assert_eq!(
            rejected.stage(),
            NormalDefaultRootCatalogLifecycleStageV1::RootExpansion
        );
        assert!(rejected.session.builder().current_module.is_none());
        assert!(matches!(
            rejected._source.ast,
            crate::ast::ASTNode::Program { .. }
        ));
    }

    #[test]
    fn catalog_failure_follows_prepare_and_retains_source() {
        let source = NyashParser::parse_from_string(
            r#"
                box Duplicate { first() { return 0 } }
                box Duplicate { second() { return 1 } }
            "#,
        )
        .expect("duplicate Box source");
        let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
        let rejected = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect_err("duplicate Box owner must reject during catalog seal");

        assert_eq!(
            rejected.stage(),
            NormalDefaultRootCatalogLifecycleStageV1::CatalogSeal
        );
        assert!(rejected.session.builder().current_module.is_some());
        assert!(matches!(
            rejected._source.ast,
            crate::ast::ASTNode::Program { .. }
        ));
    }
}
