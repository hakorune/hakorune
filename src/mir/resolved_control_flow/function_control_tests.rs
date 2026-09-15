use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{
    RegionId, ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitRecordV1,
    SourcePathSegmentV1,
};
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

fn loop_with_control(control: ASTNode) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(literal(1)),
        body: vec![control],
        span: Span::unknown(),
    }
}

fn if_stmt(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn loop_stmt(condition: ASTNode, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(condition),
        body,
        span: Span::unknown(),
    }
}

fn bool_literal(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn break_stmt() -> ASTNode {
    ASTNode::Break {
        span: Span::unknown(),
    }
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
fn explicit_null_literal_is_a_value_return_not_unit() {
    let completion = verify(vec![return_stmt(Some(null_literal()))]).unwrap();
    assert!(completion.returns_value());
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitValue { .. }
    ));
}

#[test]
fn explicit_null_and_value_returns_share_one_value_classification() {
    let early = ASTNode::If {
        condition: Box::new(literal(1)),
        then_body: vec![return_stmt(Some(null_literal()))],
        else_body: None,
        span: Span::unknown(),
    };
    let completion = verify(vec![early, return_stmt(Some(literal(1)))]).unwrap();
    assert!(completion.returns_value());
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitValueSet { .. }
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
fn declared_void_normalizes_return_null_to_unit_with_null_provenance() {
    let completion =
        verify_with_return_type(vec![return_stmt(Some(null_literal()))], Some("void")).unwrap();
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
fn declared_void_mixed_null_and_void_returns_seal_one_unit_set() {
    let early = if_stmt(literal(1), vec![return_stmt(Some(null_literal()))], None);
    let completion =
        verify_with_return_type(vec![early, return_stmt(Some(void_literal()))], Some("void"))
            .unwrap();
    assert!(!completion.returns_value());
    assert_eq!(completion.explicit_sites().len(), 2);
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitUnitSet { .. }
    ));
}

#[test]
fn unannotated_mixed_null_and_void_returns_keep_invariant_rejection() {
    let early = if_stmt(literal(1), vec![return_stmt(Some(null_literal()))], None);
    assert!(matches!(
        verify(vec![early, return_stmt(Some(void_literal()))]).unwrap_err(),
        FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant
    ));
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

#[test]
fn loop_control_only_is_implicit_completion_not_a_return() {
    let completion = verify(vec![loop_with_control(ASTNode::Break {
        span: Span::unknown(),
    })])
    .unwrap();

    assert!(completion.is_implicit_void());
    assert!(completion.explicit_sites().is_empty());
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactZeroExitRootBody
    ));
}

#[test]
fn nested_loop_control_is_excluded_but_root_return_remains_candidate() {
    let completion = verify(vec![
        loop_with_control(ASTNode::Continue {
            span: Span::unknown(),
        }),
        return_stmt(Some(literal(7))),
    ])
    .unwrap();

    assert!(completion.returns_value());
    assert_eq!(completion.explicit_sites().len(), 1);
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactOneTerminalRootReturn
    ));
}

#[test]
fn if_terminal_with_both_branches_returning_seals_if_terminal_set() {
    let terminal = if_stmt(
        literal(1),
        vec![return_stmt(Some(literal(1)))],
        Some(vec![return_stmt(Some(literal(2)))]),
    );
    let completion = verify(vec![terminal]).unwrap();
    assert!(completion.returns_value());
    assert!(completion.explicit_site().is_none());
    assert_eq!(completion.explicit_sites().len(), 2);
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactIfTerminalReturnSet { count: 2 }
    ));
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitValueSet { .. }
    ));
}

#[test]
fn if_terminal_nested_if_chain_still_completes() {
    let inner = if_stmt(
        literal(2),
        vec![return_stmt(Some(literal(3)))],
        Some(vec![return_stmt(Some(literal(4)))]),
    );
    let terminal = if_stmt(
        literal(1),
        vec![return_stmt(Some(literal(1)))],
        Some(vec![inner]),
    );
    let completion = verify(vec![terminal]).unwrap();
    assert!(completion.returns_value());
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactIfTerminalReturnSet { count: 3 }
    ));
}

#[test]
fn if_terminal_without_else_or_fallthrough_branch_cannot_seal() {
    let no_else = if_stmt(literal(1), vec![return_stmt(Some(literal(1)))], None);
    assert!(matches!(
        verify(vec![no_else]).unwrap_err(),
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));

    let fallthrough_else = if_stmt(
        literal(1),
        vec![return_stmt(Some(literal(1)))],
        Some(vec![local("x", literal(2))]),
    );
    assert!(matches!(
        verify(vec![fallthrough_else]).unwrap_err(),
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}

#[test]
fn loop_true_terminal_return_seals_loop_terminal_set() {
    let terminal = loop_stmt(
        bool_literal(true),
        vec![
            local("next", literal(1)),
            if_stmt(
                literal(1),
                vec![
                    compound_assignment("next", literal(1)),
                    ASTNode::Continue {
                        span: Span::unknown(),
                    },
                ],
                None,
            ),
            return_stmt(Some(variable("next"))),
        ],
    );
    let completion = verify(vec![terminal]).unwrap();
    assert!(completion.returns_value());
    assert!(completion.explicit_site().is_none());
    assert_eq!(completion.explicit_sites().len(), 1);
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactLoopTerminalReturnSet { count: 1 }
    ));
}

#[test]
fn loop_terminal_with_targeted_break_cannot_seal_value_completion() {
    let terminal = loop_stmt(
        bool_literal(true),
        vec![
            if_stmt(literal(1), vec![break_stmt()], None),
            return_stmt(Some(literal(1))),
        ],
    );
    assert!(matches!(
        verify(vec![terminal]).unwrap_err(),
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}

#[test]
fn loop_terminal_with_nonconstant_condition_cannot_seal_value_completion() {
    let terminal = loop_stmt(literal(1), vec![return_stmt(Some(literal(1)))]);
    assert!(matches!(
        verify(vec![terminal]).unwrap_err(),
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}

#[test]
fn nested_loop_break_does_not_escape_the_outer_terminal() {
    let inner = loop_stmt(literal(2), vec![break_stmt()]);
    let terminal = loop_stmt(
        bool_literal(true),
        vec![inner, return_stmt(Some(literal(1)))],
    );
    let completion = verify(vec![terminal]).unwrap();
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactLoopTerminalReturnSet { count: 1 }
    ));
}

#[test]
fn void_early_exits_with_fallthrough_terminal_seal_unit_set_with_implicit_end() {
    let early = if_stmt(literal(1), vec![return_stmt(None)], None);
    let completion = verify(vec![early, local("x", literal(1))]).unwrap();
    assert!(!completion.returns_value());
    assert!(!completion.is_implicit_void());
    assert_eq!(completion.explicit_sites().len(), 1);
    let (body, end) = completion.implicit_body_end().unwrap();
    assert_eq!(body.owner(), completion.owner());
    assert_eq!(end, 2);
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactExplicitUnitSetWithImplicitEnd { count: 1 }
    ));
    assert!(matches!(
        completion.function_exit_contract().disposition(),
        SealedFunctionExitDispositionV1::ExplicitUnitSetWithImplicitEnd {
            origin: FunctionUnitOriginV1::ImplicitFallthrough,
            body_end: 2,
            ..
        }
    ));
}

#[test]
fn multiple_void_early_exits_share_one_implicit_end() {
    let early_a = if_stmt(literal(1), vec![return_stmt(None)], None);
    let early_b = if_stmt(literal(2), vec![return_stmt(None)], None);
    let completion = verify(vec![early_a, early_b, literal(3)]).unwrap();
    assert_eq!(completion.explicit_sites().len(), 2);
    assert!(matches!(
        completion.function_exit_contract().coverage(),
        FunctionExitCoverageV1::ExactExplicitUnitSetWithImplicitEnd { count: 2 }
    ));
}

#[test]
fn value_early_exit_with_fallthrough_terminal_cannot_seal() {
    let early = if_stmt(literal(1), vec![return_stmt(Some(literal(1)))], None);
    assert!(matches!(
        verify(vec![early, local("x", literal(2))]).unwrap_err(),
        FunctionCompletionVerificationErrorV1::NonTerminalReturn { .. }
    ));
}

#[test]
fn malformed_origin_transfer_is_not_projected_as_loop_control() {
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(function(vec![return_stmt(None)])).unwrap();
    let owner = unit.root_function_input().unwrap().owner();
    let region = RegionId::new(owner, 0);
    let malformed = ResolvedExitRecordV1::new(
        region,
        ResolvedExitOriginV1::ExplicitBreak,
        ResolvedControlTransferV1::Return {
            target_function: region,
        },
    );

    assert!(!super::function_control::is_loop_control_exit(&malformed));
}
