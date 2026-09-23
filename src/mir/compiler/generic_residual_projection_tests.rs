use super::generic_residual_projection::{
    issue_generic_residual_source_projection_v1, GenericResidualProjectionRejectV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::GenericResidualBodyStatementKindV1;

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

fn local_integer(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(integer(value)))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}

fn step(name: &str, operator: BinaryOperator, delta: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(binary(operator, variable(name), integer(delta))),
        span: Span::unknown(),
    }
}

fn generic_residual_function(
    params: Vec<String>,
    condition: ASTNode,
    body: Vec<ASTNode>,
    tail: Vec<ASTNode>,
) -> ASTNode {
    let mut root = vec![ASTNode::Loop {
        condition: Box::new(condition),
        body,
        span: Span::unknown(),
    }];
    root.extend(tail);
    ASTNode::FunctionDeclaration {
        name: "generic_residual_projection".into(),
        params,
        param_decls: Vec::new(),
        return_type_name: None,
        body: root,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

pub(crate) fn positive_function() -> ASTNode {
    generic_residual_function(
        vec!["i".into(), "limit".into()],
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        vec![local_integer("tmp", 0), step("i", BinaryOperator::Add, 1)],
        Vec::new(),
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
    let loop_stmt = input.source().body_stmt(&body, 0).expect("Loop statement");
    (input, loop_stmt)
}

#[test]
fn projection_seals_bounded_single_induction_shape() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    let expected_frame = source.frame_key();
    let projection =
        issue_generic_residual_source_projection_v1(input, &loop_stmt, source)
            .expect("bounded source projection");
    let shape = projection.shape();
    assert_eq!(shape.body_statements.len(), 2);
    assert_eq!(
        shape.body_statements[0].kind,
        GenericResidualBodyStatementKindV1::LocalDeclaration
    );
    assert_eq!(
        shape.body_statements[1].kind,
        GenericResidualBodyStatementKindV1::Rebind
    );
    assert!(projection.root_frame_key().matches(&expected_frame));
}

#[test]
fn projection_rejects_loop_true_condition() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(generic_residual_function(
        vec!["i".into(), "limit".into()],
        boolean(true),
        vec![local_integer("tmp", 0), step("i", BinaryOperator::Add, 1)],
        Vec::new(),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_generic_residual_source_projection_v1(input, &loop_stmt, source),
        Err(GenericResidualProjectionRejectV1::LoopTrueCondition)
    );
}

#[test]
fn projection_rejects_non_rebind_terminal_statement() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(generic_residual_function(
        vec!["i".into(), "limit".into()],
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        vec![step("i", BinaryOperator::Add, 1), local_integer("tmp", 0)],
        Vec::new(),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_generic_residual_source_projection_v1(input, &loop_stmt, source),
        Err(GenericResidualProjectionRejectV1::TerminalStepMissing)
    );
}

#[test]
fn projection_rejects_non_decl_non_rebind_body_statement() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(generic_residual_function(
        vec!["i".into(), "limit".into()],
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        vec![
            ASTNode::Break {
                span: Span::unknown(),
            },
            step("i", BinaryOperator::Add, 1),
        ],
        Vec::new(),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_generic_residual_source_projection_v1(input, &loop_stmt, source),
        Err(GenericResidualProjectionRejectV1::BodyStatementShape)
    );
}

#[test]
fn projection_rejects_empty_body() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(generic_residual_function(
        vec!["i".into(), "limit".into()],
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        Vec::new(),
        Vec::new(),
    ))
    .unwrap();
    let (input, loop_stmt) = root_input_and_loop(&unit);
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    assert_eq!(
        issue_generic_residual_source_projection_v1(input, &loop_stmt, source),
        Err(GenericResidualProjectionRejectV1::BodyEmpty)
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
        issue_generic_residual_source_projection_v1(input, &foreign_loop, foreign_source),
        Err(GenericResidualProjectionRejectV1::ForeignOwner)
    );
}
