use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;
use crate::parser::NyashParser;

use super::function_control::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1,
    FunctionCompletionVerificationErrorV1, FunctionExitCoverageV1, FunctionUnitOriginV1,
    ReturnExitRelationV1, SealedFunctionExitDispositionV1, VerifiedFunctionCompletionV1,
};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn return_stmt(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

fn void_literal() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Void,
        span: Span::unknown(),
    }
}

fn null_literal() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Null,
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn compound_assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::CompoundAssignment {
        target: Box::new(variable(name)),
        operator: BinaryOperator::Add,
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn function_with_return_type(body: Vec<ASTNode>, return_type_name: Option<&str>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "completion_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: return_type_name.map(str::to_string),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    function_with_return_type(body, None)
}

fn verify(
    body: Vec<ASTNode>,
) -> Result<VerifiedFunctionCompletionV1, FunctionCompletionVerificationErrorV1> {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(body)).unwrap();
    verify_function_completion_v1(unit.root_function_input().unwrap())
}

fn verify_with_return_type(
    body: Vec<ASTNode>,
    return_type_name: Option<&str>,
) -> Result<VerifiedFunctionCompletionV1, FunctionCompletionVerificationErrorV1> {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function_with_return_type(
        body,
        return_type_name,
    ))
    .unwrap();
    verify_function_completion_v1(unit.root_function_input().unwrap())
}

#[test]
fn source_backed_loop_accepts_exact_early_and_terminal_return_set() {
    let program = NyashParser::parse_from_string(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .unwrap();
    let ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    let function = statements
        .into_iter()
        .find_map(|statement| match statement {
            ASTNode::BoxDeclaration { name, methods, .. } if name == "ParserScanLoopBox" => {
                methods.get_declaration("skip_while").cloned()
            }
            _ => None,
        })
        .unwrap();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap();
    let completion = verify_function_completion_v1(unit.root_function_input().unwrap()).unwrap();

    assert!(completion.returns_value());
    assert_eq!(completion.explicit_sites().len(), 2);
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactExplicitReturnSet { count: 2 }
    ));
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitValueSet { sites } if sites.len() == 2
    ));
}

#[test]
fn explicit_value_return_seals_exact_site_target_and_empty_cleanup() {
    let completion = verify(vec![return_stmt(Some(literal(7)))]).unwrap();
    assert!(completion.returns_value());
    assert!(!completion.is_implicit_void());
    assert_eq!(completion.unreachable_suffix_count(), 0);
    assert!(completion.cleanup().crossed_scopes().is_empty());
    assert_eq!(
        completion.explicit_site().unwrap().node().segments(),
        &[SourcePathSegmentV1::Body(0)]
    );
    assert_eq!(completion.target_function().owner(), completion.owner());
    let contract = completion.function_exit_contract();
    assert_eq!(contract.owner(), completion.owner());
    assert_eq!(
        contract.declared_result(),
        &DeclaredFunctionResultContractV1::Unannotated
    );
    assert_eq!(
        contract.coverage(),
        FunctionExitCoverageV1::ExactOneTerminalRootReturn
    );
    assert_eq!(
        contract.return_contract_relation(),
        ReturnExitRelationV1::NotRequired
    );
    assert!(matches!(
        contract.disposition(),
        SealedFunctionExitDispositionV1::ExplicitValue { .. }
    ));
}

#[test]
fn bare_return_is_not_implicit_fallthrough() {
    let completion = verify(vec![return_stmt(None)]).unwrap();
    assert!(!completion.returns_value());
    assert!(!completion.is_implicit_void());
    assert!(completion.explicit_site().is_some());
    assert!(completion.cleanup().crossed_scopes().is_empty());
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitUnit {
            origin: FunctionUnitOriginV1::BareReturn,
            ..
        }
    ));
}

#[test]
fn explicit_void_literal_has_explicit_void_provenance() {
    let completion = verify(vec![return_stmt(Some(void_literal()))]).unwrap();
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitUnit {
            origin: FunctionUnitOriginV1::ExplicitVoid,
            ..
        }
    ));
}

#[test]
fn explicit_null_literal_has_unit_provenance_without_void_reclassification() {
    let completion = verify(vec![return_stmt(Some(null_literal()))]).unwrap();
    assert!(!completion.returns_value());
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitUnit {
            origin: FunctionUnitOriginV1::ExplicitNull,
            ..
        }
    ));
}

#[test]
fn implicit_void_is_a_separate_exact_completion_form() {
    let completion = verify(vec![literal(1)]).unwrap();
    assert!(!completion.returns_value());
    assert!(completion.is_implicit_void());
    assert!(completion.explicit_site().is_none());
    assert!(completion.cleanup().crossed_scopes().is_empty());
    assert_eq!(completion.target_function().owner(), completion.owner());
    let (body, end) = completion.implicit_body_end().unwrap();
    assert_eq!(body.owner(), completion.owner());
    assert_eq!(end, 1);
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ImplicitUnit {
            origin: FunctionUnitOriginV1::ImplicitFallthrough,
            body_end: 1,
            ..
        }
    ));
}

#[test]
fn empty_body_seals_unit_with_empty_body_provenance() {
    let completion = verify(Vec::new()).unwrap();
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ImplicitUnit {
            origin: FunctionUnitOriginV1::EmptyBody,
            body_end: 0,
            ..
        }
    ));
    assert_eq!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactZeroExitRootBody
    );
}

#[test]
fn statement_tails_remain_unit_completion() {
    let cases = [
        vec![literal(1)],
        vec![ASTNode::Print {
            expression: Box::new(literal(1)),
            span: Span::unknown(),
        }],
        vec![local("x", literal(1))],
        vec![local("x", literal(1)), assignment("x", literal(2))],
        vec![local("x", literal(1)), compound_assignment("x", literal(2))],
    ];
    for body in cases {
        let completion = verify(body).unwrap();
        assert!(completion.is_implicit_void());
        assert!(completion.explicit_site().is_none());
    }
}

#[test]
fn declared_void_accepts_unit_completion() {
    let empty = verify_with_return_type(Vec::new(), Some("void")).unwrap();
    assert!(matches!(
        empty.function_exit_contract().declared_result(),
        DeclaredFunctionResultContractV1::Void
    ));
    let explicit =
        verify_with_return_type(vec![return_stmt(Some(void_literal()))], Some("void")).unwrap();
    assert!(matches!(
        explicit.function_exit_contract().declared_result(),
        DeclaredFunctionResultContractV1::Void
    ));
    assert!(verify_with_return_type(vec![return_stmt(None)], Some("void")).is_ok());
}

#[test]
fn declared_void_defers_nonliteral_return_relation() {
    let completion = verify_with_return_type(
        vec![local("x", literal(1)), return_stmt(Some(variable("x")))],
        Some("void"),
    )
    .unwrap();
    assert!(completion.returns_value());
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitValue { .. }
    ));
}

#[test]
fn declared_nonvoid_unit_completion_is_rejected() {
    let empty = verify_with_return_type(Vec::new(), Some("i64")).unwrap_err();
    assert!(matches!(
        empty,
        FunctionCompletionVerificationErrorV1::MissingReturnValueOnPath { .. }
    ));
    let explicit_void = verify_with_return_type(vec![return_stmt(None)], Some("i64")).unwrap_err();
    assert!(matches!(
        explicit_void,
        FunctionCompletionVerificationErrorV1::MissingReturnValueOnPath { .. }
    ));
}

#[test]
fn declared_void_rejects_value_return_and_exact_numeric_relation_is_deferred() {
    let void_value =
        verify_with_return_type(vec![return_stmt(Some(literal(1)))], Some("void")).unwrap_err();
    assert!(matches!(
        void_value,
        FunctionCompletionVerificationErrorV1::ReturnContractMismatch { .. }
    ));

    let exact = verify_with_return_type(vec![return_stmt(Some(literal(1)))], Some("i64")).unwrap();
    assert_eq!(
        exact.function_exit_contract().return_contract_relation(),
        ReturnExitRelationV1::ExistingExactNumericDeferred
    );
}

#[test]
fn nonterminal_root_return_cannot_seal() {
    let error = verify(vec![return_stmt(Some(literal(1))), literal(2)]).unwrap_err();
    assert!(matches!(
        error,
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}

#[test]
fn nested_return_cannot_impersonate_the_root_terminal_site() {
    let nested = ASTNode::If {
        condition: Box::new(literal(1)),
        then_body: vec![return_stmt(Some(literal(1)))],
        else_body: None,
        span: Span::unknown(),
    };
    let error = verify(vec![nested]).unwrap_err();
    assert!(matches!(
        error,
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}
