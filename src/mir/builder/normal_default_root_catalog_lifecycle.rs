//! Selected normal/default root and callable-catalog lifecycle.
//!
//! This owner consumes one isolated Builder session and preserves the legacy
//! root ordering without exposing mutable Builder access to the compiler.

use crate::ast::ASTNode;

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NormalDefaultRootPartitionV1 {
    Program,
    NonProgramCompatibility,
}

#[derive(Debug)]
struct PreparedNormalDefaultRootSourceV1 {
    ast: ASTNode,
    _partition: NormalDefaultRootPartitionV1,
}

impl PreparedNormalDefaultRootSourceV1 {
    fn seal(ast: ASTNode) -> Result<Self, (ASTNode, NormalDefaultRootCatalogLifecycleErrorV1)> {
        let partition = if matches!(ast, ASTNode::Program { .. }) {
            if let Err(error) = VerifiedRawRootExpansionV1::from_program(&ast) {
                return Err((
                    ast,
                    NormalDefaultRootCatalogLifecycleErrorV1::RootExpansion(
                        format!("[mir/main-expansion/preflight] {error:?}").into(),
                    ),
                ));
            }
            NormalDefaultRootPartitionV1::Program
        } else {
            NormalDefaultRootPartitionV1::NonProgramCompatibility
        };
        Ok(Self {
            ast,
            _partition: partition,
        })
    }
}

#[derive(Debug)]
enum RetainedNormalDefaultRootSourceV1 {
    BeforeExpansion {
        _ast: ASTNode,
    },
    Prepared {
        _source: PreparedNormalDefaultRootSourceV1,
    },
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
    _source: RetainedNormalDefaultRootSourceV1,
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
    pub(in crate::mir) fn complete_normal_default_root_catalog_lifecycle(
        mut self,
        ast: ASTNode,
    ) -> Result<
        CompletedNormalDefaultRootCatalogLifecycleV1,
        RejectedNormalDefaultRootCatalogLifecycleV1,
    > {
        let source = match PreparedNormalDefaultRootSourceV1::seal(ast) {
            Ok(source) => source,
            Err((ast, error)) => {
                return Err(RejectedNormalDefaultRootCatalogLifecycleV1 {
                    session: self,
                    _source: RetainedNormalDefaultRootSourceV1::BeforeExpansion { _ast: ast },
                    error,
                });
            }
        };

        let result = {
            let builder = self.builder_mut();
            (|| {
                builder.prepare_module().map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::PrepareModule(error.into())
                })?;

                let lowering_ast = source.ast.clone();
                let catalog =
                    VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&source.ast)
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

                let result_value = builder
                    .lower_root_after_callable_catalog_install_v1(lowering_ast, &source.ast)
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
                _source: RetainedNormalDefaultRootSourceV1::Prepared { _source: source },
                error,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::{
        BuilderInvocationConfigV1, MirBuilder, ModuleBuilderInvocationSessionV1,
        NormalDefaultRootCatalogLifecycleStageV1,
    };
    use crate::parser::NyashParser;

    use super::RetainedNormalDefaultRootSourceV1;

    fn session() -> ModuleBuilderInvocationSessionV1 {
        let current = MirBuilder::new();
        let config = BuilderInvocationConfigV1::snapshot_for_raw(&current, None);
        ModuleBuilderInvocationSessionV1::open(&current, config)
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
        let rejected = session()
            .complete_normal_default_root_catalog_lifecycle(source)
            .expect_err("duplicate Main must reject before prepare");

        assert_eq!(
            rejected.stage(),
            NormalDefaultRootCatalogLifecycleStageV1::RootExpansion
        );
        assert!(rejected.session.builder().current_module.is_none());
        assert!(matches!(
            rejected._source,
            RetainedNormalDefaultRootSourceV1::BeforeExpansion { .. }
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
        let rejected = session()
            .complete_normal_default_root_catalog_lifecycle(source)
            .expect_err("duplicate Box owner must reject during catalog seal");

        assert_eq!(
            rejected.stage(),
            NormalDefaultRootCatalogLifecycleStageV1::CatalogSeal
        );
        assert!(rejected.session.builder().current_module.is_some());
        assert!(matches!(
            rejected._source,
            RetainedNormalDefaultRootSourceV1::Prepared { .. }
        ));
    }

    #[test]
    fn non_program_root_lower_failure_is_typed_and_retained() {
        let source = ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::unknown(),
        };
        let rejected = session()
            .complete_normal_default_root_catalog_lifecycle(source)
            .expect_err("undefined non-Program root must reject during lowering");

        assert_eq!(
            rejected.stage(),
            NormalDefaultRootCatalogLifecycleStageV1::RootLower
        );
        assert!(rejected.error().to_string().contains("Undefined variable"));
        assert!(matches!(
            rejected._source,
            RetainedNormalDefaultRootSourceV1::Prepared { .. }
        ));
    }
}
