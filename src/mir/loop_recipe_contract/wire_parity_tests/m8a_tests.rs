use super::*;

// ---------------------------------------------------------------------------
// S7B1 — M8A recurrence wire-coverage cohort.
//
// The `.hako` entry `emit_m8a_recurrence_wire.hako` emits the canonical M8A
// artifact (provenance `variable_accum_recurrence_v1`) for the bounded source
// profile `main() { local i = 0; local acc = 0; loop(i < 4) { acc = acc + i;
// i = i + 1 }; print(acc); return 0 }`. The comparison target below is built
// through the same issuer calls the real producer makes — resolver facts ->
// `bind_resolved_loop_root_v1` + `recurrence_recipe(bound, delta)` +
// `VariableAccumRecurrenceV1` provenance — and the real producer product is
// also checked for semantic equality.
// ---------------------------------------------------------------------------

fn m8a_variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn m8a_integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn m8a_assignment(target: &str, left: &str, right: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(m8a_variable(target)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(m8a_variable(left)),
            right: Box::new(right),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn m8a_function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: Some("i64".into()),
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(m8a_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["acc".into()],
                initial_values: vec![Some(Box::new(m8a_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(m8a_variable("i")),
                    right: Box::new(m8a_integer(4)),
                    span: Span::unknown(),
                }),
                body: vec![
                    m8a_assignment("acc", "acc", m8a_variable("i")),
                    m8a_assignment("i", "i", m8a_integer(1)),
                ],
                span: Span::unknown(),
            },
            ASTNode::Print {
                expression: Box::new(m8a_variable("acc")),
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(m8a_integer(0))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn m8a_facts() -> VerifiedVariableAccumRecurrenceFactsV1 {
    let input: ResolvedFunctionLoweringInputV1<'static> = {
        let unit = Box::leak(Box::new(
            VerifiedResolvedSourceUnitV1::resolve_function(m8a_function())
                .expect("m8a fixture resolves"),
        ));
        unit.root_function_input().expect("root input")
    };
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    issue_variable_accum_recurrence_facts_from_membership_v1(input, &ledger, membership)
        .expect("candidate facts")
}

/// The canonical M8A artifact, assembled by the same issuer calls
/// `produce_variable_accum_recurrence_recipe_v1` makes: resolver-bound source
/// claim + `recurrence_recipe(bound, delta)` + `VariableAccumRecurrenceV1`
/// provenance.
fn m8a_rust_artifact() -> LoopRecipeArtifactV1 {
    let (source, _owner, _scope, _bindings, _inputs, condition, _update, step, _coverage) =
        m8a_facts().into_parts();
    let recipe = recurrence_recipe(condition.bound(), step.delta());
    let verified_recipe = LoopRecipeVerifierV1::verify(recipe.clone())
        .expect("m8a recipe verifies");
    let source_binding = bind_resolved_loop_root_v1(source)
        .expect("m8a root binding")
        .into_root_claim(&verified_recipe);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::VariableAccumRecurrenceV1),
        source_binding,
        recipe,
    )
}

#[test]
fn m8a_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8A_WIRE_EMISSION)
        .expect("m8a .hako emission decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8a_rust_artifact()).expect("m8a rust verifies");

    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&hako_verified)
            .expect("hako normalize_artifact"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified)
            .expect("rust normalize_artifact"),
    );
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(hako_verified.recipe())
            .expect("hako normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(rust_verified.recipe())
            .expect("rust normalize_semantic"),
    );
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_source_bound(&hako_verified)
            .expect("hako normalize_source_bound"),
        LoopRecipeNormalizerV1::normalize_source_bound(&rust_verified)
            .expect("rust normalize_source_bound"),
    );
}

#[test]
fn m8a_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(m8a_rust_artifact())
        .expect("m8a artifact verifies");
    let product = produce_variable_accum_recurrence_recipe_v1(m8a_facts())
        .expect("real producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.operations().core().recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn m8a_hako_emission_is_single_line_recurrence_provenance() {
    let trimmed = HAKO_M8A_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "m8a emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("m8a emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::VariableAccumRecurrenceV1
    );
}

#[test]
fn m8a_hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8A_WIRE_EMISSION)
        .expect("first m8a decode verifies");
    let second = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8A_WIRE_EMISSION)
        .expect("second m8a decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn m8a_wire_with_foreign_provenance_is_semantic_drift() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8A_WIRE_EMISSION).expect("m8a fixture is JSON");
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("direct_accum_v1");
    let json = serde_json::to_string(&value).expect("encodes");
    let drifted = LoopRecipeNormalizerV1::decode_and_verify(&json)
        .expect("drifted artifact still decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8a_rust_artifact()).expect("m8a rust verifies");
    assert_ne!(
        LoopRecipeNormalizerV1::normalize_artifact(&drifted).expect("drifted normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified).expect("rust normalize"),
    );
}
