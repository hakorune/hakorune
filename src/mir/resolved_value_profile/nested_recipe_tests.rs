use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

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

fn if_(condition: ASTNode, then_body: Vec<ASTNode>, else_body: Vec<ASTNode>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body: Some(else_body),
        span: Span::unknown(),
    }
}

fn return_(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "nested_recipe_fixture".into(),
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

fn nested_body() -> Vec<ASTNode> {
    vec![
        local("x", int(0)),
        if_(
            binary(BinaryOperator::Less, variable("x"), int(10)),
            vec![if_(
                binary(BinaryOperator::Less, variable("x"), int(5)),
                vec![assignment("x", int(1))],
                vec![assignment("x", int(2))],
            )],
            vec![assignment("x", int(3))],
        ),
        return_(variable("x")),
    ]
}

fn admitted<'a>(
    source: &'a VerifiedResolvedSourceUnitV1,
) -> (
    super::product::VerifiedTrivialCanonicalOwnerV1,
    crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
) {
    let input = source.root_function_input().expect("root input");
    let completion = crate::mir::resolved_control_flow::verify_function_completion_v1(input)
        .expect("completion");
    let if_control =
        crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_v1(
            input,
            &completion,
        )
        .expect("if control");
    let analysis = analyze_trivial_canonical_with_mode_v1(
        input,
        &completion,
        &if_control,
        TrivialCanonicalAnalysisModeV1::OrdinaryClosed,
    )
    .expect("analysis");
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analysis else {
        panic!("nested shape is expected to be admitted by the whole trivial owner")
    };
    (product, input)
}

// Match each merge to its original structural If and local declaration.
fn assert_merge_sources(
    product: &super::product::VerifiedTrivialCanonicalOwnerV1,
    expected: Vec<(
        Vec<crate::mir::resolved_semantics::SourcePathSegmentV1>,
        u32,
    )>,
) {
    use super::product::{TrivialBindingDefinitionOriginV1, TrivialRepresentationV1};
    use crate::mir::resolved_semantics::{SourceBindingSiteV1, SourcePathSegmentV1::Body};
    let mut expected = expected
        .into_iter()
        .map(|(path, local_index)| {
            let declarations = product
                .definitions()
                .iter()
                .filter(|row| {
                    matches!(
                        row.origin(), TrivialBindingDefinitionOriginV1::Declaration(
                            SourceBindingSiteV1::Local { statement, ordinal: 0 }
                        ) if statement.node().segments() == &[Body(local_index)]
                    )
                })
                .collect::<Vec<_>>();
            assert_eq!(declarations.len(), 1, "one original local declaration");
            (
                path,
                declarations[0].binding(),
                TrivialRepresentationV1::InlineI64,
            )
        })
        .collect::<Vec<_>>();
    let mut actual = product
        .merge_profiles()
        .iter()
        .map(|row| {
            (
                row.statement().node().segments().to_vec(),
                row.binding(),
                row.representation(),
            )
        })
        .collect::<Vec<_>>();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn nested_source_depth_one_retains_both_merge_profiles() {
    let source = VerifiedResolvedSourceUnitV1::resolve_function(function(nested_body()))
        .expect("resolve nested fixture");
    let (product, _) = admitted(&source);
    use crate::mir::resolved_semantics::SourcePathSegmentV1::{Body, IfThen};
    assert_merge_sources(
        &product,
        vec![(vec![Body(1)], 0), (vec![Body(1), IfThen(0)], 0)],
    );
}

#[test]
fn nested_source_depth_three_retains_all_merge_profiles() {
    let third = if_(
        binary(BinaryOperator::Less, variable("x"), int(2)),
        vec![assignment("x", int(1))],
        vec![assignment("x", int(2))],
    );
    let second = if_(
        binary(BinaryOperator::Less, variable("x"), int(5)),
        vec![third],
        vec![assignment("x", int(2))],
    );
    let source = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", int(0)),
        if_(
            binary(BinaryOperator::Less, variable("x"), int(10)),
            vec![second],
            vec![assignment("x", int(3))],
        ),
        return_(variable("x")),
    ]))
    .expect("resolve depth-three fixture");
    let (product, _) = admitted(&source);
    use crate::mir::resolved_semantics::SourcePathSegmentV1::{Body, IfThen};
    assert_merge_sources(
        &product,
        vec![
            (vec![Body(1)], 0),
            (vec![Body(1), IfThen(0)], 0),
            (vec![Body(1), IfThen(0), IfThen(0)], 0),
        ],
    );
}

#[test]
fn nested_source_implicit_child_else_and_multiple_bindings_remain_admitted() {
    let implicit_child = ASTNode::If {
        condition: Box::new(binary(BinaryOperator::Less, variable("x"), int(5))),
        then_body: vec![assignment("x", int(1))],
        else_body: None,
        span: Span::unknown(),
    };
    let implicit_source = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", int(0)),
        if_(
            binary(BinaryOperator::Less, variable("x"), int(10)),
            vec![implicit_child],
            vec![assignment("x", int(3))],
        ),
        return_(variable("x")),
    ]))
    .expect("resolve implicit child fixture");
    let (implicit_product, _) = admitted(&implicit_source);
    use crate::mir::resolved_semantics::SourcePathSegmentV1::{Body, IfThen};
    assert_merge_sources(
        &implicit_product,
        vec![(vec![Body(1)], 0), (vec![Body(1), IfThen(0)], 0)],
    );

    let multi_source = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", int(0)),
        local("y", int(0)),
        if_(
            binary(BinaryOperator::Less, variable("x"), int(10)),
            vec![if_(
                binary(BinaryOperator::Less, variable("x"), int(5)),
                vec![assignment("x", int(1))],
                vec![assignment("x", int(2))],
            )],
            vec![assignment("y", int(3))],
        ),
        return_(variable("x")),
    ]))
    .expect("resolve multiple-binding fixture");
    let (multi_product, _) = admitted(&multi_source);
    // Inner If updates x; the outer If reconciles both x and y.
    assert_merge_sources(
        &multi_product,
        vec![
            (vec![Body(2)], 0),
            (vec![Body(2)], 1),
            (vec![Body(2), IfThen(0)], 0),
        ],
    );
}
