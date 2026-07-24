//! RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0.
//!
//! This is the last source-only gate before the future Raw physical owner.
//! It consumes no Builder state and does not re-scan source after success.

use super::raw_root_environment_manifest::RawRootEnvironmentManifestV1;
use super::raw_root_manifest_package::ManifestBoundRawRootPackageV1;
use super::raw_root_package::SourceBoundRawRootPackageV1;
use super::raw_root_plan0::RawStaticDataSourceRowV1;
use super::raw_root_source_facts::RawRootSourceFactsErrorV1;
use crate::ast::ASTNode;
use crate::mir::builder::{
    MirBuilder, ModuleBuilderInvocationSessionV1, ModuleLoweringShellErrorV1,
    RawRootPhysicalStateV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEligibilityStageV1 {
    Work,
    Catalog,
    Access,
    Slots,
    Manifest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawEligibleCatalogV1 {
    EmptyScript,
    PlainStaticMain { helper_count: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEligibilityErrorV1 {
    UnsupportedWork { statement_index: usize },
    UnsupportedCatalog,
    MainMustBeArityZero,
    UnsupportedClosureAccess { statement_index: usize },
    UnsupportedStaticDataAuthority { statement_index: usize },
    UnsupportedProcessGlobalSlot { statement_index: usize },
    UnsupportedBodyGrammar { statement_index: usize },
    InvalidCallableRow { statement_index: usize },
    Manifest(RawRootSourceFactsErrorV1),
}

impl std::fmt::Display for RawRootEligibilityErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][raw_root_eligibility] {self:?}"
        )
    }
}

impl std::error::Error for RawRootEligibilityErrorV1 {}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct RawRootEligibilityV1 {
    pub(super) coverage: RawRootCoverageV1,
}

/// Exact first-slice source coverage consumed by the future manifest.
///
/// This witness is deliberately not `Copy`/`Clone`: the accepted route shape
/// must have one source authority rather than a freely duplicated summary.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum RawRootCoverageV1 {
    EmptyScript { statement_count: usize },
    PlainStaticMain { helper_count: usize },
}

impl RawRootEligibilityV1 {
    pub(in crate::mir) fn verify(
        package: &SourceBoundRawRootPackageV1,
    ) -> Result<Self, (RawRootEligibilityStageV1, RawRootEligibilityErrorV1)> {
        let source = package.source();
        let ASTNode::Program { statements, .. } = source.ast() else {
            return Err((
                RawRootEligibilityStageV1::Work,
                RawRootEligibilityErrorV1::UnsupportedWork { statement_index: 0 },
            ));
        };

        let is_script = matches!(
            package.plan().kind(),
            super::raw_root_plan0::RawRootKindV1::Script(_)
        );
        for item in package.plan().environment().work_schedule() {
            let failure = match item.kind() {
                super::raw_root_plan0::RawRootWorkKindV1::UnsupportedClosure => Some((
                    RawRootEligibilityStageV1::Access,
                    RawRootEligibilityErrorV1::UnsupportedClosureAccess {
                        statement_index: item.statement_index(),
                    },
                )),
                super::raw_root_plan0::RawRootWorkKindV1::UnsupportedStaticData => Some((
                    RawRootEligibilityStageV1::Access,
                    RawRootEligibilityErrorV1::UnsupportedStaticDataAuthority {
                        statement_index: item.statement_index(),
                    },
                )),
                super::raw_root_plan0::RawRootWorkKindV1::UnsupportedProcessGlobalSlot => Some((
                    RawRootEligibilityStageV1::Slots,
                    RawRootEligibilityErrorV1::UnsupportedProcessGlobalSlot {
                        statement_index: item.statement_index(),
                    },
                )),
                super::raw_root_plan0::RawRootWorkKindV1::UnsupportedSurface => Some((
                    RawRootEligibilityStageV1::Work,
                    RawRootEligibilityErrorV1::UnsupportedWork {
                        statement_index: item.statement_index(),
                    },
                )),
                super::raw_root_plan0::RawRootWorkKindV1::DeclarationFact
                | super::raw_root_plan0::RawRootWorkKindV1::StaticBox
                | super::raw_root_plan0::RawRootWorkKindV1::InstanceBox
                | super::raw_root_plan0::RawRootWorkKindV1::TopLevelFunction
                | super::raw_root_plan0::RawRootWorkKindV1::MainRoot
                    if is_script =>
                {
                    Some((
                        RawRootEligibilityStageV1::Work,
                        RawRootEligibilityErrorV1::UnsupportedWork {
                            statement_index: item.statement_index(),
                        },
                    ))
                }
                super::raw_root_plan0::RawRootWorkKindV1::StaticData => Some((
                    RawRootEligibilityStageV1::Access,
                    RawRootEligibilityErrorV1::UnsupportedStaticDataAuthority {
                        statement_index: item.statement_index(),
                    },
                )),
                super::raw_root_plan0::RawRootWorkKindV1::ScalarControl if is_script => None,
                _ => None,
            };
            if let Some(failure) = failure {
                return Err(failure);
            }
        }

        let catalog = match package.plan().kind() {
            super::raw_root_plan0::RawRootKindV1::Script(_) => RawEligibleCatalogV1::EmptyScript,
            super::raw_root_plan0::RawRootKindV1::App(_) => verify_plain_static_main(statements)?,
        };

        if package.plan().environment().access().static_data {
            let index = package
                .plan()
                .environment()
                .static_data()
                .rows()
                .first()
                .map(RawStaticDataSourceRowV1::statement_index)
                .unwrap_or(0);
            return Err((
                RawRootEligibilityStageV1::Access,
                RawRootEligibilityErrorV1::UnsupportedStaticDataAuthority {
                    statement_index: index,
                },
            ));
        }

        let coverage = match catalog {
            RawEligibleCatalogV1::EmptyScript => RawRootCoverageV1::EmptyScript {
                statement_count: statements.len(),
            },
            RawEligibleCatalogV1::PlainStaticMain { helper_count } => {
                RawRootCoverageV1::PlainStaticMain { helper_count }
            }
        };
        Ok(Self { coverage })
    }

    pub(in crate::mir) const fn catalog(&self) -> RawEligibleCatalogV1 {
        match self.coverage {
            RawRootCoverageV1::EmptyScript { .. } => RawEligibleCatalogV1::EmptyScript,
            RawRootCoverageV1::PlainStaticMain { helper_count } => {
                RawEligibleCatalogV1::PlainStaticMain { helper_count }
            }
        }
    }

    pub(in crate::mir) const fn coverage(&self) -> &RawRootCoverageV1 {
        &self.coverage
    }
}

fn verify_plain_static_main(
    statements: &[ASTNode],
) -> Result<RawEligibleCatalogV1, (RawRootEligibilityStageV1, RawRootEligibilityErrorV1)> {
    if statements.len() != 1 {
        return Err((
            RawRootEligibilityStageV1::Catalog,
            RawRootEligibilityErrorV1::UnsupportedCatalog,
        ));
    }
    let ASTNode::BoxDeclaration {
        name,
        methods,
        is_static,
        fields,
        field_decls,
        public_fields,
        private_fields,
        constructors,
        init_fields,
        weak_fields,
        delegates,
        invariants,
        transitions,
        is_interface,
        is_sync,
        is_record,
        type_parameters,
        extends,
        implements,
        static_init,
        ..
    } = &statements[0]
    else {
        return Err((
            RawRootEligibilityStageV1::Catalog,
            RawRootEligibilityErrorV1::UnsupportedCatalog,
        ));
    };
    if name != "Main"
        || !*is_static
        || *is_interface
        || *is_sync
        || *is_record
        || !fields.is_empty()
        || !field_decls.is_empty()
        || !public_fields.is_empty()
        || !private_fields.is_empty()
        || !constructors.is_empty()
        || !init_fields.is_empty()
        || !weak_fields.is_empty()
        || !delegates.is_empty()
        || !invariants.is_empty()
        || !transitions.is_empty()
        || !type_parameters.is_empty()
        || !extends.is_empty()
        || !implements.is_empty()
        || static_init.is_some()
    {
        return Err((
            RawRootEligibilityStageV1::Catalog,
            RawRootEligibilityErrorV1::UnsupportedCatalog,
        ));
    }
    let Some(main) = methods.get("main") else {
        return Err((
            RawRootEligibilityStageV1::Catalog,
            RawRootEligibilityErrorV1::UnsupportedCatalog,
        ));
    };
    let mut helper_count = 0;
    for (method_name, method) in methods {
        let ASTNode::FunctionDeclaration {
            name: declared_name,
            params,
            param_decls,
            body: _,
            is_static: method_static,
            is_override,
            contracts,
            ..
        } = method
        else {
            return Err((
                RawRootEligibilityStageV1::Catalog,
                RawRootEligibilityErrorV1::InvalidCallableRow { statement_index: 0 },
            ));
        };
        if method_name == "main" && !params.is_empty() {
            return Err((
                RawRootEligibilityStageV1::Slots,
                RawRootEligibilityErrorV1::MainMustBeArityZero,
            ));
        }
        if method_name != declared_name
            || !*method_static
            || *is_override
            || !contracts.is_empty()
            || param_decls.len() != params.len()
        {
            return Err((
                RawRootEligibilityStageV1::Catalog,
                RawRootEligibilityErrorV1::InvalidCallableRow { statement_index: 0 },
            ));
        }
        if method_name == "main" {
            if !std::ptr::eq(method, main) {
                return Err((
                    RawRootEligibilityStageV1::Catalog,
                    RawRootEligibilityErrorV1::InvalidCallableRow { statement_index: 0 },
                ));
            }
        } else {
            helper_count += 1;
        }
    }
    Ok(RawEligibleCatalogV1::PlainStaticMain { helper_count })
}

pub(in crate::mir) type EligibleSourceBoundRawRootPackageV1 = ManifestBoundRawRootPackageV1;

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootEligibilityV1 {
    owner: SourceBoundRawRootPackageV1,
    stage: RawRootEligibilityStageV1,
    error: RawRootEligibilityErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RawRootPhysicalOpenErrorV1 {
    Shell(ModuleLoweringShellErrorV1),
}

#[derive(Debug)]
pub(super) struct RawRootPhysicalCoreV1 {
    pub(super) token: crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
    pub(super) source: crate::mir::builder::OwnedRawSourceV1,
    pub(super) continuation: super::raw_source_binding::RawRootContinuationV1,
    pub(super) module_name: Box<str>,
    pub(super) plan: super::raw_root_plan0::RawRootPlanV1,
    pub(super) proof: RawRootEligibilityV1,
    pub(super) manifest: super::raw_root_environment_manifest::RawRootPhysicalManifestV1,
    pub(super) session: ModuleBuilderInvocationSessionV1,
    pub(super) physical: RawRootPhysicalStateV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptRootInvocationV1 {
    pub(super) core: RawRootPhysicalCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppRootInvocationV1 {
    pub(super) core: RawRootPhysicalCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RawRootInvocationV1 {
    Script(RawScriptRootInvocationV1),
    App(RawAppRootInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootPhysicalOpenV1 {
    owner: EligibleSourceBoundRawRootPackageV1,
    error: RawRootPhysicalOpenErrorV1,
}

impl RejectedRawRootPhysicalOpenV1 {
    pub(in crate::mir) const fn error(&self) -> &RawRootPhysicalOpenErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl EligibleSourceBoundRawRootPackageV1 {
    /// The only PHYSICAL0 terminal. The eligible owner is retained intact if
    /// the empty shell cannot be opened; package fields move only afterwards.
    pub(in crate::mir) fn open_physical(
        self,
        current: &MirBuilder,
    ) -> Result<RawRootInvocationV1, RejectedRawRootPhysicalOpenV1> {
        let physical = match RawRootPhysicalStateV1::open(
            self.token(),
            self.module_name().to_owned(),
            self.continuation().callable_main(),
        ) {
            Ok(physical) => physical,
            Err(error) => {
                return Err(RejectedRawRootPhysicalOpenV1 {
                    owner: self,
                    error: RawRootPhysicalOpenErrorV1::Shell(error),
                });
            }
        };
        let parts = self.into_physical_open_parts();
        let super::raw_root_manifest_package::ManifestBoundRawRootPartsV1 {
            token,
            source,
            continuation,
            module_name,
            plan,
            proof,
            manifest,
        } = parts;
        let (manifest, config) = manifest.into_physical_parts();
        let session = ModuleBuilderInvocationSessionV1::open_for_token(&token, current, config);
        let core = RawRootPhysicalCoreV1 {
            token,
            source,
            continuation,
            module_name,
            plan,
            proof,
            manifest,
            session,
            physical,
        };
        let is_script = matches!(
            core.plan.kind(),
            super::raw_root_plan0::RawRootKindV1::Script(_)
        );
        if is_script {
            Ok(RawRootInvocationV1::Script(RawScriptRootInvocationV1 {
                core: RawRootPhysicalCoreV1 {
                    physical: core.physical,
                    ..core
                },
            }))
        } else {
            Ok(RawRootInvocationV1::App(RawAppRootInvocationV1 { core }))
        }
    }
}

impl RejectedRawRootEligibilityV1 {
    pub(in crate::mir) const fn stage(&self) -> RawRootEligibilityStageV1 {
        self.stage
    }

    pub(in crate::mir) const fn error(&self) -> &RawRootEligibilityErrorV1 {
        &self.error
    }

    #[cfg(test)]
    pub(in crate::mir) const fn owner_brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        self.owner.brand()
    }

    #[cfg(test)]
    pub(in crate::mir) const fn owner_family(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationFamilyV1 {
        self.owner.family()
    }

    #[cfg(test)]
    pub(in crate::mir) fn owner_module_name(&self) -> &str {
        self.owner.module_name()
    }

    pub(in crate::mir) fn discard(self) {}
}

impl SourceBoundRawRootPackageV1 {
    pub(in crate::mir) fn prepare_eligibility(
        self,
    ) -> Result<EligibleSourceBoundRawRootPackageV1, RejectedRawRootEligibilityV1> {
        match RawRootEligibilityV1::verify(&self) {
            Ok(proof) => {
                let facts =
                    match RawRootEnvironmentManifestV1::source_facts(self.source(), self.plan()) {
                        Ok(facts) => facts,
                        Err(error) => {
                            return Err(RejectedRawRootEligibilityV1 {
                                owner: self,
                                stage: RawRootEligibilityStageV1::Manifest,
                                error: RawRootEligibilityErrorV1::Manifest(error),
                            });
                        }
                    };
                let (token, source, continuation, runtime_inputs, config, module_name, plan) =
                    self.into_manifest_parts();
                let manifest =
                    RawRootEnvironmentManifestV1::from_facts(facts, runtime_inputs, config);
                Ok(ManifestBoundRawRootPackageV1::new(
                    token,
                    source,
                    continuation,
                    module_name,
                    plan,
                    proof,
                    manifest,
                ))
            }
            Err((stage, error)) => Err(RejectedRawRootEligibilityV1 {
                owner: self,
                stage,
                error,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::builder::{MirBuilder, RawCallableMainCompatibilityDispositionV1};
    use crate::mir::compiler::lowering_input::LegacyModuleLoweringInputV1;
    use crate::mir::compiler::raw_source_binding::RawCallableMainSelectionV1;
    use crate::mir::MirCompiler;
    use std::collections::HashMap;

    fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body,
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn package(source: ASTNode) -> SourceBoundRawRootPackageV1 {
        package_with_selection(source, RawCallableMainSelectionV1::Omitted)
    }

    fn package_with_selection(
        source: ASTNode,
        selection: RawCallableMainSelectionV1,
    ) -> SourceBoundRawRootPackageV1 {
        let mut compiler = MirCompiler::new();
        compiler
            .bind_raw_source(
                LegacyModuleLoweringInputV1::bare_ast(source),
                None,
                "eligibility0",
                selection,
            )
            .unwrap()
            .into_root_package()
            .unwrap()
    }

    fn app(main_body: Vec<ASTNode>) -> ASTNode {
        let mut methods = HashMap::new();
        methods.insert("main".into(), function("main", main_body));
        ASTNode::Program {
            statements: vec![ASTNode::BoxDeclaration {
                name: "Main".into(),
                methods,
                is_static: true,
                fields: Vec::new(),
                field_decls: Vec::new(),
                public_fields: Vec::new(),
                private_fields: Vec::new(),
                constructors: HashMap::new(),
                init_fields: Vec::new(),
                weak_fields: Vec::new(),
                delegates: Vec::new(),
                invariants: Vec::new(),
                transitions: Vec::new(),
                is_interface: false,
                is_sync: false,
                is_record: false,
                type_parameters: Vec::new(),
                extends: Vec::new(),
                implements: Vec::new(),
                static_init: None,
                attrs: DeclarationAttrs::default(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    fn app_with_helper(main_body: Vec<ASTNode>) -> ASTNode {
        let mut source = app(main_body);
        if let ASTNode::Program { statements, .. } = &mut source {
            if let Some(ASTNode::BoxDeclaration { methods, .. }) = statements.first_mut() {
                methods.insert("helper".into(), function("helper", Vec::new()));
            }
        }
        source
    }

    #[test]
    fn empty_script_is_the_smallest_eligible_catalog() {
        let eligible = package(ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        })
        .prepare_eligibility()
        .unwrap();
        assert_eq!(
            eligible.proof().catalog(),
            RawEligibleCatalogV1::EmptyScript
        );
    }

    #[test]
    fn plain_static_main_keeps_scalar_body_source_only() {
        let eligible = package(app(vec![ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }]))
        .prepare_eligibility()
        .unwrap();
        assert_eq!(
            eligible.proof().catalog(),
            RawEligibleCatalogV1::PlainStaticMain { helper_count: 0 }
        );
    }

    #[test]
    fn plain_static_main_accepts_a_helper_catalog_row() {
        let eligible = package(app_with_helper(Vec::new()))
            .prepare_eligibility()
            .unwrap();
        assert_eq!(
            eligible.proof().catalog(),
            RawEligibleCatalogV1::PlainStaticMain { helper_count: 1 }
        );
    }

    #[test]
    fn lambda_and_static_data_reject_before_physical_open() {
        let lambda = package(app(vec![ASTNode::Print {
            expression: Box::new(ASTNode::Lambda {
                params: Vec::new(),
                body: Vec::new(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }]))
        .prepare_eligibility()
        .unwrap_err();
        assert_eq!(lambda.stage(), RawRootEligibilityStageV1::Access);
        assert!(matches!(
            lambda.error(),
            RawRootEligibilityErrorV1::UnsupportedClosureAccess { .. }
        ));

        let static_data = package(ASTNode::Program {
            statements: vec![ASTNode::StaticConstTable {
                name: "T".into(),
                element_type: "i64".into(),
                values: vec![1],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        })
        .prepare_eligibility()
        .unwrap_err();
        assert_eq!(static_data.stage(), RawRootEligibilityStageV1::Access);
        static_data.discard();
    }

    #[test]
    fn physical_open_keeps_script_route_empty_and_unselected() {
        let eligible = package(ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        })
        .prepare_eligibility()
        .unwrap();
        let invocation = eligible.open_physical(&MirBuilder::new()).unwrap();
        let RawRootInvocationV1::Script(invocation) = invocation else {
            panic!("empty script must open the Script physical route")
        };
        assert!(invocation.core.physical.shell_is_empty());
        assert_eq!(
            invocation.core.physical.callable_main(),
            RawCallableMainCompatibilityDispositionV1::NotSelected
        );
        assert_eq!(
            invocation.core.token.brand(),
            invocation.core.physical.brand()
        );
        assert_eq!(
            invocation.core.token.brand(),
            invocation.core.physical.ledger_brand()
        );
        assert_eq!(
            invocation.core.token.brand(),
            invocation.core.physical.tracker_brand()
        );
        assert_eq!(
            invocation.core.token.brand(),
            invocation.core.session.brand()
        );
    }

    #[test]
    fn physical_open_keeps_app_callable_main_disposition_without_descent() {
        let omitted = package(app(Vec::new()))
            .prepare_eligibility()
            .unwrap()
            .open_physical(&MirBuilder::new())
            .unwrap();
        let RawRootInvocationV1::App(omitted) = omitted else {
            panic!("static Main must open the App physical route")
        };
        assert_eq!(
            omitted.core.physical.callable_main(),
            RawCallableMainCompatibilityDispositionV1::NotSelected
        );
        assert!(omitted.core.physical.shell_is_empty());

        let selected =
            package_with_selection(app(Vec::new()), RawCallableMainSelectionV1::Required)
                .prepare_eligibility()
                .unwrap();
        let selected = selected.open_physical(&MirBuilder::new()).unwrap();
        let RawRootInvocationV1::App(selected) = selected else {
            panic!("static Main must open the App physical route")
        };
        assert_eq!(
            selected.core.physical.callable_main(),
            RawCallableMainCompatibilityDispositionV1::Selected
        );
        assert!(selected.core.physical.shell_is_empty());
    }
}
