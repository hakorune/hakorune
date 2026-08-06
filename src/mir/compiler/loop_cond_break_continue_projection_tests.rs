use super::loop_cond_break_continue_projection::{
    issue_loop_cond_break_continue_source_projection_v1, LoopCondBreakContinueProjectionRejectV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{ResolvedControlTransferV1, ResolvedExitOriginV1};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn loop_cond_function(loop_condition: ASTNode, branch: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "loop_cond_projection".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["flag".into()],
                initial_values: vec![Some(Box::new(integer(1)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(loop_condition),
                body: vec![branch],
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn branch() -> ASTNode {
    ASTNode::If {
        condition: Box::new(binary(BinaryOperator::Equal, variable("flag"), integer(1))),
        then_body: vec![ASTNode::Break {
            span: Span::unknown(),
        }],
        else_body: Some(vec![ASTNode::Continue {
            span: Span::unknown(),
        }]),
        span: Span::unknown(),
    }
}

pub(crate) fn positive_function() -> ASTNode {
    loop_cond_function(
        binary(BinaryOperator::Less, variable("flag"), integer(2)),
        branch(),
    )
}

fn root_input_and_loop(
    unit: &VerifiedResolvedSourceUnitV1,
) -> (
    crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    crate::mir::compiler::located::LocatedStmtV1<'_>,
) {
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("function body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("Loop statement");
    (input, loop_stmt)
}

#[test]
fn projection_seals_exact_non_true_loop_cond_shape() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    let expected_frame = source.frame_key();
    let projection = issue_loop_cond_break_continue_source_projection_v1(input, &loop_stmt, source)
        .expect("bounded source projection");
    let shape = projection.shape();
    assert_eq!(shape.loop_site.node().segments().len(), 1);
    assert_eq!(shape.then_exit_origin, ResolvedExitOriginV1::ExplicitBreak);
    assert_eq!(
        shape.else_exit_origin,
        ResolvedExitOriginV1::ExplicitContinue
    );
    assert!(matches!(
        shape.then_exit_transfer,
        ResolvedControlTransferV1::Break { .. }
    ));
    assert!(matches!(
        shape.else_exit_transfer,
        ResolvedControlTransferV1::Continue { .. }
    ));
    assert!(projection.root_frame_key().matches(&expected_frame));
}

#[test]
fn projection_rejects_root_true_as_loop_true_overlap() {
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(loop_cond_function(boolean(true), branch()))
            .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_loop_cond_break_continue_source_projection_v1(input, &loop_stmt, source),
        Err(LoopCondBreakContinueProjectionRejectV1::LoopTrueCondition)
    );
}

#[test]
fn projection_accepts_non_true_literal_without_type_policy_claim() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(loop_cond_function(
        boolean(false),
        branch(),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert!(issue_loop_cond_break_continue_source_projection_v1(input, &loop_stmt, source).is_ok());
}

#[test]
fn projection_rejects_implicit_else_and_body_arity() {
    let no_else = ASTNode::If {
        condition: Box::new(binary(BinaryOperator::Equal, variable("flag"), integer(1))),
        then_body: vec![ASTNode::Break {
            span: Span::unknown(),
        }],
        else_body: None,
        span: Span::unknown(),
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(loop_cond_function(
        binary(BinaryOperator::Less, variable("flag"), integer(2)),
        no_else,
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_loop_cond_break_continue_source_projection_v1(input, &loop_stmt, source),
        Err(LoopCondBreakContinueProjectionRejectV1::ExplicitElseRequired)
    );
}

#[test]
fn projection_rejects_foreign_loop_owner() {
    let first = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let second = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let (input, _) = root_input_and_loop(&first);
    let (foreign_input, foreign_loop) = root_input_and_loop(&second);
    let foreign_source = foreign_input
        .function()
        .resolved_loop_source(foreign_loop.site())
        .unwrap();
    assert_eq!(
        issue_loop_cond_break_continue_source_projection_v1(input, &foreign_loop, foreign_source),
        Err(LoopCondBreakContinueProjectionRejectV1::ForeignOwner)
    );
}
