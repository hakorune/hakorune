//! RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0.
//!
//! This is the last source-only gate before the future Raw physical owner.
//! It consumes no Builder state and does not re-scan source after success.

use super::raw_root_package::SourceBoundRawRootPackageV1;
use super::raw_root_plan0::RawStaticDataSourceRowV1;
use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEligibilityStageV1 {
    Work,
    Catalog,
    Access,
    Slots,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawEligibleCatalogV1 {
    EmptyScript,
    PlainStaticMain { helper_count: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEligibilityErrorV1 {
    UnsupportedWork { statement_index: usize },
    UnsupportedCatalog,
    MainMustBeArityZero,
    UnsupportedClosureAccess { statement_index: usize },
    UnsupportedStaticDataAuthority { statement_index: usize },
    UnsupportedProcessGlobalSlot { statement_index: usize },
    UnsupportedBodyGrammar { statement_index: usize },
    InvalidCallableRow { statement_index: usize },
}

impl std::fmt::Display for RawRootEligibilityErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][raw_root_eligibility] {self:?}")
    }
}

impl std::error::Error for RawRootEligibilityErrorV1 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct RawRootEligibilityV1 {
    catalog: RawEligibleCatalogV1,
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
                _ => None,
            };
            if let Some(failure) = failure {
                return Err(failure);
            }
        }

        let catalog = match package.plan().kind() {
            super::raw_root_plan0::RawRootKindV1::Script(_) => {
                RawEligibleCatalogV1::EmptyScript
            }
            super::raw_root_plan0::RawRootKindV1::App(_) => {
                verify_plain_static_main(statements)?
            }
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

        Ok(Self { catalog })
    }

    pub(in crate::mir) const fn catalog(&self) -> RawEligibleCatalogV1 {
        self.catalog
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
            if !std::ptr::eq(method, main) || !params.is_empty() {
                return Err((
                    RawRootEligibilityStageV1::Catalog,
                    if !params.is_empty() {
                        RawRootEligibilityErrorV1::MainMustBeArityZero
                    } else {
                        RawRootEligibilityErrorV1::InvalidCallableRow { statement_index: 0 }
                    },
                ));
            }
        } else {
            helper_count += 1;
        }
    }
    Ok(RawEligibleCatalogV1::PlainStaticMain { helper_count })
}

#[derive(Debug)]
pub(in crate::mir) struct EligibleSourceBoundRawRootPackageV1 {
    pub(in crate::mir) package: SourceBoundRawRootPackageV1,
    proof: RawRootEligibilityV1,
}

impl EligibleSourceBoundRawRootPackageV1 {
    pub(in crate::mir) const fn proof(&self) -> RawRootEligibilityV1 {
        self.proof
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootEligibilityV1 {
    owner: SourceBoundRawRootPackageV1,
    stage: RawRootEligibilityStageV1,
    error: RawRootEligibilityErrorV1,
}

impl RejectedRawRootEligibilityV1 {
    pub(in crate::mir) const fn stage(&self) -> RawRootEligibilityStageV1 {
        self.stage
    }

    pub(in crate::mir) const fn error(&self) -> &RawRootEligibilityErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl SourceBoundRawRootPackageV1 {
    pub(in crate::mir) fn prepare_eligibility(
        self,
    ) -> Result<EligibleSourceBoundRawRootPackageV1, RejectedRawRootEligibilityV1> {
        match RawRootEligibilityV1::verify(&self) {
            Ok(proof) => Ok(EligibleSourceBoundRawRootPackageV1 {
                package: self,
                proof,
            }),
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
        let mut compiler = MirCompiler::new();
        compiler
            .bind_raw_source(
                LegacyModuleLoweringInputV1::bare_ast(source),
                None,
                "eligibility0",
                RawCallableMainSelectionV1::Omitted,
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

    #[test]
    fn empty_script_is_the_smallest_eligible_catalog() {
        let eligible = package(ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        })
        .prepare_eligibility()
        .unwrap();
        assert_eq!(eligible.proof().catalog(), RawEligibleCatalogV1::EmptyScript);
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
}
