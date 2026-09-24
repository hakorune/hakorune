use super::*;

// ---------------------------------------------------------------------------
// S7B2 — M8B exits/joins wire-coverage cohort.
//
// The `.hako` entry `emit_m8b_break_wire.hako` emits the canonical M8B
// artifact (provenance `variable_accum_break_v1`) for the bounded source
// profile `main() { local sum = 0; local i = 0; loop(i < 10) {
// if(i == 5) { sum = sum + 10; break }; sum = sum + 1; i = i + 1 };
// return sum }`. The comparison target below is built through the same
// issuer calls the real producer makes — attempt facts ->
// `bind_resolved_loop_root_v1` + `break_recipe(loop_bound, branch_bound)`
// + `VariableAccumBreakV1` provenance — and the real producer product is
// also checked for semantic equality.
// ---------------------------------------------------------------------------

fn m8b_variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn m8b_integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn m8b_binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn m8b_assignment(target: &str, left: &str, right: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(m8b_variable(target)),
        value: Box::new(m8b_binary(BinaryOperator::Add, m8b_variable(left), right)),
        span: Span::unknown(),
    }
}

fn m8b_function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: Some("i64".into()),
        body: vec![
            ASTNode::Local {
                variables: vec!["sum".into()],
                initial_values: vec![Some(Box::new(m8b_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(m8b_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(m8b_binary(
                    BinaryOperator::Less,
                    m8b_variable("i"),
                    m8b_integer(10),
                )),
                body: vec![
                    ASTNode::If {
                        condition: Box::new(m8b_binary(
                            BinaryOperator::Equal,
                            m8b_variable("i"),
                            m8b_integer(5),
                        )),
                        then_body: vec![
                            m8b_assignment("sum", "sum", m8b_integer(10)),
                            ASTNode::Break {
                                span: Span::unknown(),
                            },
                        ],
                        else_body: None,
                        span: Span::unknown(),
                    },
                    m8b_assignment("sum", "sum", m8b_integer(1)),
                    m8b_assignment("i", "i", m8b_integer(1)),
                ],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(m8b_variable("sum"))),
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

fn m8b_facts() -> VerifiedVariableAccumBreakFactsV1 {
    let input: ResolvedFunctionLoweringInputV1<'static> = {
        let unit = Box::leak(Box::new(
            VerifiedResolvedSourceUnitV1::resolve_function(m8b_function())
                .expect("m8b fixture resolves"),
        ));
        unit.root_function_input().expect("root input")
    };
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Complete,
    );
    let VariableAccumBreakSourceAttemptOutcomeV1::Candidate(facts) = attempt.into_parts().0
    else {
        panic!("expected m8b candidate facts");
    };
    facts
}

/// The canonical M8B artifact, assembled by the same issuer calls
/// `produce_variable_accum_break_recipe_v1` makes: resolver-bound source
/// claim + `break_recipe(loop_bound, branch_bound)` +
/// `VariableAccumBreakV1` provenance.
fn m8b_rust_artifact() -> LoopRecipeArtifactV1 {
    let (
        source,
        _owner,
        _scope,
        _bindings,
        _inputs,
        loop_condition,
        branch_condition,
        _terminal_update,
        _normal_update,
        _induction_step,
        _branch_site,
        _break_site,
        _coverage,
    ) = m8b_facts().into_parts();
    let recipe = break_recipe(loop_condition.bound(), branch_condition.bound());
    let verified_recipe =
        LoopRecipeVerifierV1::verify(recipe.clone()).expect("m8b recipe verifies");
    let source_binding = bind_resolved_loop_root_v1(source)
        .expect("m8b root binding")
        .into_root_claim(&verified_recipe);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::VariableAccumBreakV1),
        source_binding,
        recipe,
    )
}

#[test]
fn m8b_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8B_WIRE_EMISSION)
        .expect("m8b .hako emission decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8b_rust_artifact()).expect("m8b rust verifies");

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
fn m8b_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(m8b_rust_artifact())
        .expect("m8b artifact verifies");
    let product =
        produce_variable_accum_break_recipe_v1(m8b_facts()).expect("real producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn m8b_hako_emission_is_single_line_break_provenance_with_exit() {
    let trimmed = HAKO_M8B_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "m8b emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("m8b emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::VariableAccumBreakV1
    );
    assert_eq!(artifact.recipe.exits.len(), 1);
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, LoopRecipeItemV1::If { .. })));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, LoopRecipeItemV1::Exit { .. })));
}

#[test]
fn m8b_hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8B_WIRE_EMISSION)
        .expect("first m8b decode verifies");
    let second = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8B_WIRE_EMISSION)
        .expect("second m8b decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn m8b_wire_with_foreign_provenance_is_semantic_drift() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8B_WIRE_EMISSION).expect("m8b fixture is JSON");
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("variable_accum_recurrence_v1");
    let json = serde_json::to_string(&value).expect("encodes");
    let drifted = LoopRecipeNormalizerV1::decode_and_verify(&json)
        .expect("drifted artifact still decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8b_rust_artifact()).expect("m8b rust verifies");
    assert_ne!(
        LoopRecipeNormalizerV1::normalize_artifact(&drifted).expect("drifted normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified).expect("rust normalize"),
    );
}

