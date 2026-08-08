use super::*;
use crate::parser::ParserBuildConfig;

#[test]
fn r6_s3b_d_i0_final_seal_retains_complete_delegate_relation_rows() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
box Target {
    run() { return 1 }
}
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("delegate source transport should not change parsing");

    let host_seal = &parsed.source_seals()[1];
    assert!(host_seal.prepared.delegate_source_declarations.is_empty());
    assert_eq!(host_seal.generated_delegate_source_relations().len(), 1);
    assert_eq!(
        host_seal.generated_delegate_source_relations()[0].exposed_method_name(),
        "runAlias"
    );
    assert!(host_seal.inventory().get("runAlias").is_none());
    let relations = parsed.generated_delegate_source_relations();
    assert_eq!(relations.len(), 1);
    assert_eq!(relations[0].exposed_method_name(), "runAlias");
    assert_eq!(relations[0].source_method_name(), "run");
    assert_eq!(relations[0].delegate_field_name(), "target");
    let generated = match parsed.ast() {
        ASTNode::Program { statements, .. } => statements
            .iter()
            .find_map(|statement| match statement {
                ASTNode::BoxDeclaration { name, methods, .. } if name == "Host" => {
                    methods.get("runAlias")
                }
                _ => None,
            })
            .expect("delegate generated method must remain in descriptive AST"),
        _ => panic!("source-sealed parse must return a Program AST"),
    };
    assert!(matches!(
        generated.provenance(),
        crate::ast::BoxMethodProvenanceV1::Generated(
            crate::ast::BoxMethodGeneratedProvenanceV1::Delegate { .. }
        )
    ));
}

fn open_postpass_product(source: &str) -> OpenParserPostpassProductV1 {
    let config = ParserBuildConfig::default();
    let pre = super::super::normalize_logical_ops(source);
    let mut tokenizer =
        crate::tokenizer::NyashTokenizer::with_grammar_profile(pre, config.grammar_profile);
    let tokens = tokenizer.tokenize().expect("test source must tokenize");
    let mut parser = NyashParser::new(tokens);
    parser.build_config = config;
    let ast = parser.parse_program().expect("test source must parse");
    parser
        .open_postpass_product(ast)
        .expect("test source must issue BuildGate decision set")
        .prune_build_gates(&parser)
        .expect("test source must produce an open postpass product")
        .lower_delegates()
        .expect("test source must lower delegates")
}

#[test]
fn r6_s3b_d_i0_rejects_duplicate_relation_key_before_final_seal() {
    let mut product = open_postpass_product(
        r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
    );
    let seal = product
        .source_session
        .prepared_source_seals
        .iter_mut()
        .find(|seal| !seal.generated_delegate_source_relations().is_empty())
        .expect("host must have generated relation rows");
    let row = seal.generated_delegate_source_relations()[0].clone();
    seal.generated_delegate_source_relations = vec![row.clone(), row].into_boxed_slice();

    let error = product
        .finalize()
        .expect_err("duplicate relation key must reject final seal");
    assert!(matches!(
        error,
        SourceSealFinalizationErrorV1::GeneratedDelegateCoverage(
            super::super::source_seal_finalizer::GeneratedDelegateCoverageErrorV1::DuplicateRelationKey
        )
    ));
}

#[test]
fn r6_s3b_d_i0_rejects_orphan_relation_placement_before_final_seal() {
    let mut product = open_postpass_product(
        r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
    );
    let host_index = match &product.ast {
        ASTNode::Program { statements, .. } => statements
            .iter()
            .position(|statement| {
                matches!(statement, ASTNode::BoxDeclaration { name, .. } if name == "Host")
            })
            .expect("host AST must exist"),
        _ => panic!("source product must contain a Program"),
    };
    let host_inventory = product.source_session.prepared_source_seals[host_index]
        .inventory()
        .clone();
    let ASTNode::Program { statements, .. } = &mut product.ast else {
        panic!("source product must contain a Program");
    };
    let ASTNode::BoxDeclaration { methods, .. } = &mut statements[host_index] else {
        panic!("host AST must be a Box");
    };
    *methods = host_inventory;

    let error = product
        .finalize()
        .expect_err("orphan relation placement must reject final seal");
    assert!(matches!(
        error,
        SourceSealFinalizationErrorV1::GeneratedDelegateCoverage(
            super::super::source_seal_finalizer::GeneratedDelegateCoverageErrorV1::RelationPlacementMissing
        )
    ));
}

#[test]
fn r6_s3b_c_i0_zero_delegate_program_is_an_exact_noop() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
box Plain {
    run() { return 1 }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("ordinary Box without delegates should remain a valid source product");

    assert!(parsed.generated_delegate_source_relations().is_empty());
    let ASTNode::Program { statements, .. } = parsed.ast() else {
        panic!("source-sealed parse must return a Program AST");
    };
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        panic!("fixture must contain one Box");
    };
    assert_eq!(methods.len(), 1);
    assert!(methods.get("runAlias").is_none());
}
