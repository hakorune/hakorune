use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::compiler::VerifiedResolvedCallableProgramV1;
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_with_direct_call_v1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;

use super::{
    analyze_trivial_canonical_with_mode_v1, TrivialCanonicalAnalysisModeV1,
    TrivialCanonicalOwnerAnalysisV1,
};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
    literal(LiteralValue::Integer(value))
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
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

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn call() -> ASTNode {
    ASTNode::FunctionCall {
        name: "caller".into(),
        arguments: vec![variable("p0")],
        span: Span::unknown(),
    }
}

fn caller(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "caller".into(),
        params: vec!["p0".into()],
        param_decls: vec![ParamDecl {
            name: "p0".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(body: Vec<ASTNode>) -> VerifiedResolvedCallableProgramV1 {
    VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: vec![caller(body)],
        span: Span::unknown(),
    })
    .expect("call-valued If fixture resolves")
}

fn product<'a>(
    source: &'a VerifiedResolvedCallableProgramV1,
) -> (
    super::product::VerifiedTrivialCanonicalOwnerV1,
    crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
) {
    let key = source
        .module()
        .functions_by_key()
        .keys()
        .next()
        .expect("caller fixture has one function")
        .clone();
    let input = source.module().function_input(&key).unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let if_control =
        verify_resolved_function_if_control_with_direct_call_v1(input, &completion).unwrap();
    let analysis = analyze_trivial_canonical_with_mode_v1(
        input,
        &completion,
        &if_control,
        TrivialCanonicalAnalysisModeV1::OrdinaryFiniteDirectCalls,
    )
    .unwrap();
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analysis else {
        panic!("call-valued If fixture is admitted by the finite profile")
    };
    (product, input)
}

fn explicit_call_body(then_value: ASTNode, else_value: ASTNode) -> Vec<ASTNode> {
    vec![
        ASTNode::Local {
            variables: vec!["x".into()],
            initial_values: vec![Some(Box::new(int(0)))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(binary(BinaryOperator::Less, variable("x"), int(1))),
            then_body: vec![assignment("x", then_value)],
            else_body: Some(vec![assignment("x", else_value)]),
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        },
    ]
}

#[test]
fn same_pass_direct_call_rhs_retains_actual_source_site() {
    let source = program(explicit_call_body(int(1), call()));
    let (product, input) = product(&source);
    let [row] = product.direct_calls() else {
        panic!("one actual direct call RHS");
    };
    let site = row.site();
    assert!(matches!(
        site.node().segments().last(),
        Some(crate::mir::resolved_semantics::SourcePathSegmentV1::Value)
    ));
    assert!(site.node().segments().iter().any(|step| matches!(
        step,
        crate::mir::resolved_semantics::SourcePathSegmentV1::IfElse(0)
    )));
    assert_eq!(input.function().owner(), product.owner());
}

#[test]
fn condition_direct_call_retains_actual_source_site() {
    let condition_call = vec![
        ASTNode::Local {
            variables: vec!["x".into()],
            initial_values: vec![Some(Box::new(int(0)))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(binary(BinaryOperator::Less, call(), int(1))),
            then_body: vec![assignment("x", int(1))],
            else_body: Some(vec![assignment("x", int(2))]),
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        },
    ];
    let source = program(condition_call);
    let (analyzed, _) = product(&source);
    assert_eq!(analyzed.direct_calls().len(), 1);
    assert!(analyzed.direct_calls()[0]
        .site()
        .node()
        .segments()
        .iter()
        .any(|step| matches!(
            step,
            crate::mir::resolved_semantics::SourcePathSegmentV1::IfCondition
        )));
}

#[test]
fn explicit_two_call_arms_are_admitted_but_implicit_call_fallthrough_stays_admitted() {
    let source = program(explicit_call_body(call(), call()));
    let (explicit_product, _) = product(&source);
    let rows = explicit_product.direct_calls();
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].site(), rows[1].site());
    assert!(rows[0].site().node().segments().iter().any(|step| matches!(
        step,
        crate::mir::resolved_semantics::SourcePathSegmentV1::IfThen(0)
    )));
    assert!(rows[1].site().node().segments().iter().any(|step| matches!(
        step,
        crate::mir::resolved_semantics::SourcePathSegmentV1::IfElse(0)
    )));
    let source = program(vec![
        ASTNode::Local {
            variables: vec!["x".into()],
            initial_values: vec![Some(Box::new(int(0)))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(binary(BinaryOperator::Less, variable("x"), int(1))),
            then_body: vec![assignment("x", call())],
            else_body: None,
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        },
    ]);
    let (product, _) = product(&source);
    let [row] = product.direct_calls() else {
        panic!("one actual implicit-else call RHS");
    };
    assert!(row.site().node().segments().iter().any(|step| matches!(
        step,
        crate::mir::resolved_semantics::SourcePathSegmentV1::IfThen(0)
    )));
    assert_eq!(product.merge_profiles().len(), 1);
}
