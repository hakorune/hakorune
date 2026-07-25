//! PUBLIC-CUTOVER-COVERAGE0: exact empty StaticHelper0 source witness.
//!
//! This module is the only helper-grammar authority for the first public Raw
//! profile. It validates declarations before PHYSICAL0 and hands CHILDREN0
//! only an owned locator schedule; CHILDREN0 never re-reads helper AST bodies.

use crate::ast::ASTNode;
use crate::mir::builder::{OwnedRawSourceV1, RawSourceLocatorV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct RawPublicEligibilityProfileV1 {
    _seal: RawPublicEligibilityProfileSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawPublicEligibilityProfileSealV1;

impl RawPublicEligibilityProfileV1 {
    pub(in crate::mir) const fn narrow_v1() -> Self {
        Self {
            _seal: RawPublicEligibilityProfileSealV1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawStaticHelper0CoverageErrorV1 {
    RootNotProgram,
    LocatorOutOfRange,
    NotStaticBox,
    BoxNameMismatch,
    MainMethodNotHelper,
    MethodMissing,
    MethodNotFunction,
    MethodNameMismatch,
    MethodNotStatic,
    MethodOverride,
    ParametersPresent,
    ReturnTypePresent,
    UsesPresent,
    AttributesPresent,
    ContractsPresent,
    BodyNotEmpty,
    SymbolMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct RawStaticHelper0V1 {
    locator: RawSourceLocatorV1,
}

impl RawStaticHelper0V1 {
    pub(in crate::mir) fn locator(&self) -> &RawSourceLocatorV1 {
        &self.locator
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct RawStaticHelperCoverageV1 {
    rows: Box<[RawStaticHelper0V1]>,
}

impl RawStaticHelperCoverageV1 {
    pub(in crate::mir) fn empty() -> Self {
        Self { rows: Box::new([]) }
    }

    pub(in crate::mir) fn verify(
        source: &OwnedRawSourceV1,
        locators: &[RawSourceLocatorV1],
    ) -> Result<Self, RawStaticHelper0CoverageErrorV1> {
        let ASTNode::Program { statements, .. } = source.ast() else {
            return Err(RawStaticHelper0CoverageErrorV1::RootNotProgram);
        };
        Self::verify_statements(statements, locators)
    }

    fn verify_statements(
        statements: &[ASTNode],
        locators: &[RawSourceLocatorV1],
    ) -> Result<Self, RawStaticHelper0CoverageErrorV1> {
        let mut rows = Vec::with_capacity(locators.len());
        for locator in locators {
            if locator.method_name() == "main" {
                return Err(RawStaticHelper0CoverageErrorV1::MainMethodNotHelper);
            }
            let Some(ASTNode::BoxDeclaration {
                name,
                methods,
                is_static,
                ..
            }) = statements.get(locator.top_level_statement())
            else {
                return Err(RawStaticHelper0CoverageErrorV1::LocatorOutOfRange);
            };
            if !*is_static {
                return Err(RawStaticHelper0CoverageErrorV1::NotStaticBox);
            }
            if name != locator.box_name() {
                return Err(RawStaticHelper0CoverageErrorV1::BoxNameMismatch);
            }
            let Some(declaration) = methods.get(locator.method_name()) else {
                return Err(RawStaticHelper0CoverageErrorV1::MethodMissing);
            };
            let ASTNode::FunctionDeclaration {
                name: declared_name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
                is_static,
                is_override,
                contracts,
                ..
            } = declaration
            else {
                return Err(RawStaticHelper0CoverageErrorV1::MethodNotFunction);
            };
            if declared_name != locator.method_name() {
                return Err(RawStaticHelper0CoverageErrorV1::MethodNameMismatch);
            }
            if !*is_static {
                return Err(RawStaticHelper0CoverageErrorV1::MethodNotStatic);
            }
            if *is_override {
                return Err(RawStaticHelper0CoverageErrorV1::MethodOverride);
            }
            if !params.is_empty() || !param_decls.is_empty() {
                return Err(RawStaticHelper0CoverageErrorV1::ParametersPresent);
            }
            if return_type_name.is_some() {
                return Err(RawStaticHelper0CoverageErrorV1::ReturnTypePresent);
            }
            if !uses.is_empty() {
                return Err(RawStaticHelper0CoverageErrorV1::UsesPresent);
            }
            if !attrs.is_empty() {
                return Err(RawStaticHelper0CoverageErrorV1::AttributesPresent);
            }
            if !contracts.is_empty() {
                return Err(RawStaticHelper0CoverageErrorV1::ContractsPresent);
            }
            if !body.is_empty() {
                return Err(RawStaticHelper0CoverageErrorV1::BodyNotEmpty);
            }
            let symbol = crate::mir::naming::encode_static_method(name, declared_name, 0);
            if symbol != locator.symbol() || locator.arity() != 0 {
                return Err(RawStaticHelper0CoverageErrorV1::SymbolMismatch);
            }
            rows.push(RawStaticHelper0V1 {
                locator: locator.clone(),
            });
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }

    #[cfg(test)]
    pub(super) fn verify_program_for_test(
        program: &ASTNode,
        locators: &[RawSourceLocatorV1],
    ) -> Result<Self, RawStaticHelper0CoverageErrorV1> {
        let ASTNode::Program { statements, .. } = program else {
            return Err(RawStaticHelper0CoverageErrorV1::RootNotProgram);
        };
        Self::verify_statements(statements, locators)
    }

    pub(in crate::mir) fn len(&self) -> usize {
        self.rows.len()
    }

    pub(in crate::mir) fn matches_locators(&self, expected: &[RawSourceLocatorV1]) -> bool {
        self.rows.len() == expected.len()
            && self
                .rows
                .iter()
                .zip(expected)
                .all(|(row, locator)| row.locator == *locator)
    }

    pub(in crate::mir) fn into_locators(self) -> Box<[RawSourceLocatorV1]> {
        self.rows
            .into_vec()
            .into_iter()
            .map(|row| row.locator)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[cfg(test)]
    pub(super) fn for_test(locators: Box<[RawSourceLocatorV1]>) -> Self {
        Self {
            rows: locators
                .into_vec()
                .into_iter()
                .map(|locator| RawStaticHelper0V1 { locator })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        ContractClause, ContractKind, DeclarationAttrs, LiteralValue, RuneAttr, Span,
    };
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

    fn app_ast(helper_method: ASTNode) -> ASTNode {
        let mut methods = HashMap::new();
        methods.insert("main".into(), function("main", Vec::new()));
        methods.insert("helper".into(), helper_method);
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

    fn app_with(helper_method: ASTNode) -> OwnedRawSourceV1 {
        OwnedRawSourceV1::bind(
            app_ast(helper_method),
            crate::mir::builder::RawSourceOriginV1::BareAst,
        )
        .unwrap()
    }

    fn app(helper_body: Vec<ASTNode>) -> OwnedRawSourceV1 {
        app_with(function("helper", helper_body))
    }

    fn locator() -> RawSourceLocatorV1 {
        RawSourceLocatorV1::for_test(0, "Main", "helper", "Main.helper/0", 0)
    }

    #[test]
    fn exact_empty_static_helper_is_sealed_once() {
        let source = app(Vec::new());
        let coverage = RawStaticHelperCoverageV1::verify(&source, &[locator()]).unwrap();
        assert_eq!(coverage.len(), 1);
        assert_eq!(coverage.into_locators()[0].symbol(), "Main.helper/0");
    }

    #[test]
    fn locator_witness_parity_is_exact_and_ordered() {
        let source = app(Vec::new());
        let coverage = RawStaticHelperCoverageV1::verify(&source, &[locator()]).unwrap();
        assert!(coverage.matches_locators(&[locator()]));
        let forged = RawSourceLocatorV1::for_test(0, "Main", "other", "Main.other/0", 0);
        assert!(!coverage.matches_locators(&[forged]));
    }

    #[test]
    fn non_empty_helper_is_rejected_before_child_descent() {
        let source = app(vec![ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }]);
        let error = RawStaticHelperCoverageV1::verify(&source, &[locator()]).unwrap_err();
        assert_eq!(error, RawStaticHelper0CoverageErrorV1::BodyNotEmpty);
    }

    fn assert_error(method: ASTNode, expected: RawStaticHelper0CoverageErrorV1) {
        let error = RawStaticHelperCoverageV1::verify(&app_with(method), &[locator()]).unwrap_err();
        assert_eq!(error, expected);
    }

    #[test]
    fn helper_parameters_are_rejected() {
        let mut method = function("helper", Vec::new());
        if let ASTNode::FunctionDeclaration { params, .. } = &mut method {
            params.push("p".into());
        }
        assert_error(method, RawStaticHelper0CoverageErrorV1::ParametersPresent);
    }

    #[test]
    fn helper_metadata_is_rejected() {
        let mut method = function("helper", Vec::new());
        if let ASTNode::FunctionDeclaration {
            return_type_name,
            uses,
            attrs,
            contracts,
            ..
        } = &mut method
        {
            *return_type_name = Some("Integer".into());
            uses.push("FileBox".into());
            attrs.runes.push(RuneAttr {
                name: "Public".into(),
                args: Vec::new(),
            });
            contracts.push(ContractClause {
                kind: ContractKind::Requires,
                condition: ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                },
            });
        }
        assert_error(method, RawStaticHelper0CoverageErrorV1::ReturnTypePresent);
    }

    #[test]
    fn helper_uses_attrs_and_contracts_are_rejected() {
        let mut uses_method = function("helper", Vec::new());
        if let ASTNode::FunctionDeclaration { uses, .. } = &mut uses_method {
            uses.push("FileBox".into());
        }
        assert_error(uses_method, RawStaticHelper0CoverageErrorV1::UsesPresent);

        let mut attrs_method = function("helper", Vec::new());
        if let ASTNode::FunctionDeclaration { attrs, .. } = &mut attrs_method {
            attrs.runes.push(RuneAttr {
                name: "Public".into(),
                args: Vec::new(),
            });
        }
        assert_error(attrs_method, RawStaticHelper0CoverageErrorV1::AttributesPresent);

        let mut contracts_method = function("helper", Vec::new());
        if let ASTNode::FunctionDeclaration { contracts, .. } = &mut contracts_method {
            contracts.push(ContractClause {
                kind: ContractKind::Requires,
                condition: ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                },
            });
        }
        assert_error(
            contracts_method,
            RawStaticHelper0CoverageErrorV1::ContractsPresent,
        );
    }

    #[test]
    fn helper_instance_and_override_methods_are_rejected() {
        let mut instance = function("helper", Vec::new());
        if let ASTNode::FunctionDeclaration { is_static, .. } = &mut instance {
            *is_static = false;
        }
        assert_eq!(
            RawStaticHelperCoverageV1::verify_program_for_test(&app_ast(instance), &[locator()])
                .unwrap_err(),
            RawStaticHelper0CoverageErrorV1::MethodNotStatic
        );

        let mut override_method = function("helper", Vec::new());
        if let ASTNode::FunctionDeclaration { is_override, .. } = &mut override_method {
            *is_override = true;
        }
        assert_error(override_method, RawStaticHelper0CoverageErrorV1::MethodOverride);
    }
}
