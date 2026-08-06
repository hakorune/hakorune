use super::loop_true_break_continue_projection::{
    issue_loop_true_break_continue_source_projection_v1, LoopTrueBreakContinueProjectionRejectV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

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

fn branch_condition(operator: BinaryOperator) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(variable("flag")),
        right: Box::new(integer(1)),
        span: Span::unknown(),
    }
}

fn loop_true_function(
    loop_condition: ASTNode,
    if_condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "loop_true_projection".into(),
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
                body: vec![ASTNode::If {
                    condition: Box::new(if_condition),
                    then_body,
                    else_body,
                    span: Span::unknown(),
                }],
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

pub(crate) fn positive_function() -> ASTNode {
    loop_true_function(
        boolean(true),
        branch_condition(BinaryOperator::Equal),
        vec![ASTNode::Break {
            span: Span::unknown(),
        }],
        Some(vec![ASTNode::Continue {
            span: Span::unknown(),
        }]),
    )
}

fn two_loop_function() -> ASTNode {
    let mut function = positive_function();
    let ASTNode::FunctionDeclaration { body, .. } = &mut function else {
        unreachable!("positive fixture is a function");
    };
    body.push(ASTNode::Loop {
        condition: Box::new(boolean(true)),
        body: vec![ASTNode::If {
            condition: Box::new(branch_condition(BinaryOperator::Equal)),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Continue {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    });
    function
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
fn projection_seals_exact_loop_true_break_continue_shape() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    let expected_frame = source.frame_key();
    let projection = issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source)
        .expect("bounded source projection");
    let shape = projection.shape();
    assert_eq!(shape.loop_site.node().segments().len(), 1);
    assert_eq!(shape.branch_condition_bound, 1);
    assert_ne!(shape.then_exit_site, shape.else_exit_site);
    assert!(projection.root_frame_key().matches(&expected_frame));
}

#[test]
fn projection_rejects_implicit_else() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(loop_true_function(
        boolean(true),
        branch_condition(BinaryOperator::Equal),
        vec![ASTNode::Break {
            span: Span::unknown(),
        }],
        None,
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source),
        Err(LoopTrueBreakContinueProjectionRejectV1::ExplicitElseRequired)
    );
}

#[test]
fn projection_rejects_branch_write_and_fallthrough_shape() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(loop_true_function(
        boolean(true),
        branch_condition(BinaryOperator::Equal),
        vec![
            ASTNode::Assignment {
                target: Box::new(variable("flag")),
                value: Box::new(integer(2)),
                span: Span::unknown(),
            },
            ASTNode::Break {
                span: Span::unknown(),
            },
        ],
        Some(vec![ASTNode::Continue {
            span: Span::unknown(),
        }]),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source),
        Err(LoopTrueBreakContinueProjectionRejectV1::BranchBodyArity)
    );
}

#[test]
fn projection_rejects_return_arm() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(loop_true_function(
        boolean(true),
        branch_condition(BinaryOperator::Equal),
        vec![ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }],
        Some(vec![ASTNode::Continue {
            span: Span::unknown(),
        }]),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source),
        Err(LoopTrueBreakContinueProjectionRejectV1::BranchShape)
    );
}

#[test]
fn projection_rejects_non_true_root_condition() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(loop_true_function(
        boolean(false),
        branch_condition(BinaryOperator::Equal),
        vec![ASTNode::Break {
            span: Span::unknown(),
        }],
        Some(vec![ASTNode::Continue {
            span: Span::unknown(),
        }]),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source),
        Err(LoopTrueBreakContinueProjectionRejectV1::LoopConditionShape)
    );
}

#[test]
fn projection_rejects_non_comparison_branch_condition() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(loop_true_function(
        boolean(true),
        branch_condition(BinaryOperator::Add),
        vec![ASTNode::Break {
            span: Span::unknown(),
        }],
        Some(vec![ASTNode::Continue {
            span: Span::unknown(),
        }]),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source),
        Err(LoopTrueBreakContinueProjectionRejectV1::BranchConditionShape)
    );
}

#[test]
fn projection_rejects_foreign_located_loop() {
    let first = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let second = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let (input, _) = root_input_and_loop(&first);
    let (foreign_input, foreign_loop) = root_input_and_loop(&second);
    let foreign_source = foreign_input
        .function()
        .resolved_loop_source(foreign_loop.site())
        .unwrap();
    assert_eq!(
        issue_loop_true_break_continue_source_projection_v1(input, &foreign_loop, foreign_source),
        Err(LoopTrueBreakContinueProjectionRejectV1::ForeignOwner)
    );
}

#[test]
fn projection_rejects_source_token_for_a_different_loop_site() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(two_loop_function()).unwrap();
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("function body");
    let first_loop = input.source().body_stmt(&body, 1).expect("first Loop");
    let second_loop = input.source().body_stmt(&body, 2).expect("second Loop");
    let second_source = input
        .function()
        .resolved_loop_source(second_loop.site())
        .unwrap();
    assert_eq!(
        issue_loop_true_break_continue_source_projection_v1(input, &first_loop, second_source),
        Err(LoopTrueBreakContinueProjectionRejectV1::SourceLookup)
    );
}
