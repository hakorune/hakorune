//! Selected normal/default root and callable-catalog lifecycle.
//!
//! This owner consumes one isolated Builder session and preserves the legacy
//! root ordering without exposing mutable Builder access to the compiler.

use crate::ast::ASTNode;

use super::main_expansion::VerifiedRawRootExpansionV1;
use super::{MirModule, ModuleBuilderInvocationSessionV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalDefaultRootCatalogLifecycleStageV1 {
    RootExpansion,
    PrepareModule,
    CatalogSeal,
    CatalogInstall,
    RootLower,
    FinalizeModule,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum NormalDefaultRootCatalogLifecycleErrorV1 {
    RootExpansion(Box<str>),
    PrepareModule(Box<str>),
    CatalogSeal(Box<str>),
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
    ) -> Result<
        CompletedNormalDefaultRootCatalogLifecycleV1,
        RejectedNormalDefaultRootCatalogLifecycleV1,
    > {
        let result = {
            let builder = self.builder_mut();
            (|| {
                let expansion = VerifiedRawRootExpansionV1::from_program(source.source_ast())
                    .map_err(|error| {
                        NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                            format!("[mir/main-expansion/preflight] {error:?}").into(),
                        )
                    })?;
                builder.prepare_module().map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::PrepareModule(error.into())
                })?;

                let result_value =
                    builder.lower_normal_default_program_root_catalog_v1(&source, &expansion)?;
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
        BuilderInvocationConfigV1, MirBuilder, ModuleBuilderInvocationSessionV1,
        NormalDefaultRootCatalogLifecycleStageV1, PreparedNormalDefaultProgramRootV1,
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
                .complete_normal_default_program_root_catalog_lifecycle(source)
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
            .complete_normal_default_program_root_catalog_lifecycle(source)
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
            .complete_normal_default_program_root_catalog_lifecycle(source)
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
