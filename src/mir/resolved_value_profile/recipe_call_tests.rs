use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::compiler::VerifiedResolvedCallableProgramV1;
use crate::mir::if_recipe_contract::{
    IfOperationV1, IfRecipeArtifactV1, IfRecipeNormalizerV1, IfRecipeVerifierV1,
    IfSourceClaimRoleV1, IfSourcePathStepV1,
};
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_with_direct_call_v1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;

use super::{
    analyze_trivial_canonical_owner_with_finite_direct_calls_v1, map_trivial_if_recipe_v1,
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
    let analysis = analyze_trivial_canonical_owner_with_finite_direct_calls_v1(
        input,
        &completion,
        &if_control,
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
fn same_pass_facts_capture_one_direct_call_rhs() {
    let source = program(explicit_call_body(int(1), call()));
    let (product, input) = product(&source);
    let facts = product.recipe_facts().expect("Call RHS facts are admitted");
    let call_site = facts.direct_call_site().expect("direct call source site");
    assert_eq!(product.direct_calls().len(), 1);
    assert_eq!(product.direct_calls()[0].site(), call_site);
    assert!(matches!(
        call_site.node().segments().last(),
        Some(crate::mir::resolved_semantics::SourcePathSegmentV1::Value)
    ));
    assert_eq!(input.function().owner(), product.owner());
}

#[test]
fn mapper_emits_physical_id_free_direct_call_and_source_claim() {
    let source = program(explicit_call_body(int(1), call()));
    let (product, input) = product(&source);
    let artifact = map_trivial_if_recipe_v1(&product, input.function()).unwrap();
    let else_block = artifact.recipe().as_recipe().else_block.as_ref().unwrap();
    assert!(else_block
        .items
        .iter()
        .any(|item| matches!(item.operation, IfOperationV1::DirectStaticCall { .. })));
    assert_eq!(
        artifact.source_binding().as_source_binding().claims.len(),
        5
    );
    assert_eq!(
        artifact.source_binding().as_source_binding().claims[4].role,
        IfSourceClaimRoleV1::DirectStaticCall
    );
    let semantic = IfRecipeNormalizerV1::normalize_semantic(artifact.recipe()).unwrap();
    assert!(semantic.contains("direct_static_call"));
    assert!(!semantic.contains("ValueId"));
}

#[test]
fn direct_call_source_claim_mismatch_rejects_before_physicalization() {
    let source = program(explicit_call_body(int(1), call()));
    let (product, input) = product(&source);
    let artifact = map_trivial_if_recipe_v1(&product, input.function()).unwrap();
    let json = IfRecipeNormalizerV1::normalize_artifact(&artifact).unwrap();
    let mut artifact: IfRecipeArtifactV1 = serde_json::from_str(&json).unwrap();
    artifact.source_binding.claims[4].path.steps = vec![
        IfSourcePathStepV1::BodyItem { index: 1 },
        IfSourcePathStepV1::IfThenItem { index: 0 },
        IfSourcePathStepV1::AssignmentValue,
    ];
    let result = IfRecipeVerifierV1::verify_artifact(artifact);
    let is_branch_mismatch = matches!(
        &result,
        Err(crate::mir::if_recipe_contract::IfRecipeRejectReasonV1::DirectStaticCallBranchMismatch {
            ..
        })
    );
    assert!(
        is_branch_mismatch,
        "unexpected direct-call claim result: {result:?}"
    );
}

#[test]
fn call_outside_explicit_branch_rhs_is_not_recipe_facts() {
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
    assert!(analyzed.recipe_facts().is_none());
}

#[test]
fn explicit_two_call_arms_are_admitted_but_implicit_call_fallthrough_stays_admitted() {
    let source = program(explicit_call_body(call(), call()));
    let (explicit_product, input) = product(&source);
    let facts = explicit_product
        .recipe_facts()
        .expect("explicit two-call RHS facts are admitted");
    let [Some(then_site), Some(else_site)] = facts.direct_call_sites() else {
        panic!("expected one direct call site per explicit branch")
    };
    assert_ne!(then_site, else_site);
    assert_eq!(explicit_product.direct_calls().len(), 2);
    assert_eq!(explicit_product.direct_calls()[0].site(), then_site);
    assert_eq!(explicit_product.direct_calls()[1].site(), else_site);

    let artifact = map_trivial_if_recipe_v1(&explicit_product, input.function())
        .expect("explicit two-call RHS maps to a portable artifact");
    let recipe = artifact.recipe().as_recipe();
    assert_eq!(
        recipe
            .then_block
            .items
            .iter()
            .filter(|item| matches!(item.operation, IfOperationV1::DirectStaticCall { .. }))
            .count(),
        1
    );
    assert_eq!(
        recipe
            .else_block
            .as_ref()
            .unwrap()
            .items
            .iter()
            .filter(|item| matches!(item.operation, IfOperationV1::DirectStaticCall { .. }))
            .count(),
        1
    );
    let claims = &artifact.source_binding().as_source_binding().claims;
    assert_eq!(claims.len(), 6);
    assert_eq!(claims[4].role, IfSourceClaimRoleV1::DirectStaticCall);
    assert_eq!(claims[5].role, IfSourceClaimRoleV1::DirectStaticCall);
    assert!(matches!(
        claims[4].path.steps.as_slice(),
        [
            IfSourcePathStepV1::BodyItem { .. },
            IfSourcePathStepV1::IfThenItem { .. },
            IfSourcePathStepV1::AssignmentValue
        ]
    ));
    assert!(matches!(
        claims[5].path.steps.as_slice(),
        [
            IfSourcePathStepV1::BodyItem { .. },
            IfSourcePathStepV1::IfElseItem { .. },
            IfSourcePathStepV1::AssignmentValue
        ]
    ));

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
    let (product, input) = product(&source);
    let facts = product
        .recipe_facts()
        .expect("implicit direct-call RHS facts are admitted");
    assert!(facts.has_implicit_else());
    assert_eq!(facts.then_assignment_count(), 1);
    assert_eq!(facts.else_assignment_count(), 0);
    assert!(facts.direct_call_site().is_some());

    let artifact = map_trivial_if_recipe_v1(&product, input.function())
        .expect("implicit direct-call RHS maps to a portable artifact");
    assert!(artifact.recipe().as_recipe().else_block.is_none());
    assert_eq!(
        artifact.recipe().as_recipe().else_disposition,
        crate::mir::if_recipe_contract::IfElseDispositionV1::ImplicitFallthrough
    );
    assert_eq!(
        artifact.source_binding().as_source_binding().claims.len(),
        5
    );
    assert_eq!(
        artifact.source_binding().as_source_binding().claims[3].role,
        IfSourceClaimRoleV1::ImplicitBaseline
    );
    assert_eq!(
        artifact.source_binding().as_source_binding().claims[4].role,
        IfSourceClaimRoleV1::DirectStaticCall
    );
    let semantic = IfRecipeNormalizerV1::normalize_semantic(artifact.recipe()).unwrap();
    assert!(semantic.contains("implicit"));
    assert!(semantic.contains("direct_static_call"));
}

#[test]
fn explicit_two_call_claim_order_cannot_be_swapped() {
    let source = program(explicit_call_body(call(), call()));
    let (product, input) = product(&source);
    let artifact = map_trivial_if_recipe_v1(&product, input.function()).unwrap();
    let json = IfRecipeNormalizerV1::normalize_artifact(&artifact).unwrap();
    let mut artifact: IfRecipeArtifactV1 = serde_json::from_str(&json).unwrap();
    let then_path = artifact.source_binding.claims[4].path.clone();
    artifact.source_binding.claims[4].path = artifact.source_binding.claims[5].path.clone();
    artifact.source_binding.claims[5].path = then_path;
    assert!(matches!(
        IfRecipeVerifierV1::verify_artifact(artifact),
        Err(crate::mir::if_recipe_contract::IfRecipeRejectReasonV1::SourceClaimOrderMismatch)
    ));
}
