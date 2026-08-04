use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::if_recipe_contract::{
    NestedIfJoinCompositionRoleV1, NestedIfJoinSigComposerV1, NestedIfSourceClaimRoleV1,
};

use super::{
    analyze_trivial_canonical_owner_v1, map_nested_trivial_if_recipe_v1,
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
    let analysis =
        analyze_trivial_canonical_owner_v1(input, &completion, &if_control).expect("analysis");
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analysis else {
        panic!("nested shape is expected to be admitted by the whole trivial owner")
    };
    (product, input)
}

#[test]
fn nested_depth_one_emits_separate_facts_artifact_and_composed_join_sig() {
    let source = VerifiedResolvedSourceUnitV1::resolve_function(function(nested_body()))
        .expect("resolve nested fixture");
    let (product, input) = admitted(&source);
    assert!(
        product.recipe_facts().is_none(),
        "fixed one-If V1 stays immutable"
    );
    let facts = product
        .nested_recipe_facts()
        .expect("depth-one nested facts");
    assert_eq!(facts.outer().statement().node().segments().len(), 1);
    assert_eq!(facts.inner().statement().node().segments().len(), 2);
    assert_eq!(facts.inner().then_assignments().len(), 1);
    assert_eq!(facts.inner().else_assignments().len(), 1);

    let artifact = map_nested_trivial_if_recipe_v1(&product, input.function())
        .expect("nested portable artifact");
    let roles: Vec<_> = artifact
        .artifact()
        .source_binding
        .claims
        .iter()
        .map(|claim| claim.role)
        .collect();
    assert_eq!(
        roles,
        vec![
            NestedIfSourceClaimRoleV1::OuterIfNode,
            NestedIfSourceClaimRoleV1::OuterCondition,
            NestedIfSourceClaimRoleV1::InnerIfNode,
            NestedIfSourceClaimRoleV1::InnerCondition,
            NestedIfSourceClaimRoleV1::InnerThenAssignment,
            NestedIfSourceClaimRoleV1::InnerElseAssignment,
            NestedIfSourceClaimRoleV1::OuterElseAssignment,
            NestedIfSourceClaimRoleV1::ContinuationRead,
        ]
    );
    let join = NestedIfJoinSigComposerV1::compose(&artifact).expect("nested JoinSig");
    assert_eq!(
        join.as_sig().composition.role,
        NestedIfJoinCompositionRoleV1::InnerMergeToOuterThen
    );
    assert_eq!(join.as_sig().inner.node.raw(), 1);
    assert_eq!(join.as_sig().outer.node.raw(), 0);
}

#[test]
fn nested_profile_rejects_depth_greater_than_one_before_mapping() {
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
    let (product, input) = admitted(&source);
    assert!(product.nested_recipe_facts().is_none());
    assert!(map_nested_trivial_if_recipe_v1(&product, input.function()).is_err());
}

#[test]
fn nested_profile_rejects_implicit_child_else_and_multiple_bindings() {
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
    assert!(implicit_product.nested_recipe_facts().is_none());

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
    assert!(multi_product.nested_recipe_facts().is_none());
}
