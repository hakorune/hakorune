use crate::ast::{ASTNode, BoxMethodProvenanceV1};
use crate::parser::{NyashParser, ParseError};
use crate::tests::helpers::env::with_env_vars;
use crate::tests::helpers::parser::{find_box, parse_ok};

fn method_names(source: &str, box_name: &str) -> Vec<String> {
    let ast = parse_ok(source);
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, box_name) else {
        panic!("expected BoxDeclaration")
    };
    methods
        .names_in_selected_order()
        .map(str::to_owned)
        .collect()
}

#[test]
fn interface_methods_keep_lexical_order_and_explicit_sites() {
    let source = "interface box Api {\n  zeta()\n  alpha()\n  middle()\n}\n";
    let ast = parse_ok(source);
    let ASTNode::BoxDeclaration { methods, span, .. } = find_box(&ast, "Api") else {
        panic!("expected interface BoxDeclaration")
    };

    assert_eq!(
        methods.names_in_selected_order().collect::<Vec<_>>(),
        vec!["zeta", "alpha", "middle"]
    );
    assert_eq!((span.line, span.column), (1, 1));
    for (ordinal, entry) in methods.iter_selected_declaration_order().enumerate() {
        assert_eq!(entry.site().selected_method_ordinal(), ordinal as u32);
        assert!(matches!(
            entry.provenance(),
            BoxMethodProvenanceV1::ExplicitSource { .. }
        ));
        assert_eq!(entry.diagnostic_span().line, ordinal + 2);
    }
}

#[test]
fn static_methods_keep_lexical_order() {
    assert_eq!(
        method_names(
            "static box Ops {\n  zeta() { return 3 }\n  alpha() { return 1 }\n  middle() { return 2 }\n}\n",
            "Ops",
        ),
        vec!["zeta", "alpha", "middle"]
    );
}

#[test]
fn ordinary_box_source_authority_remains_closed_for_r3() {
    let ast = parse_ok("box Plain {\n  run() { return 1 }\n}\n");
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Plain") else {
        panic!("expected ordinary BoxDeclaration")
    };
    assert!(matches!(
        methods.get("run").unwrap().provenance(),
        BoxMethodProvenanceV1::CompatibilityOnly { .. }
    ));
}

#[test]
fn interface_duplicate_reports_first_and_duplicate_sites() {
    let error =
        NyashParser::parse_from_string("interface box Api {\n  run()\n  run()\n}\n").unwrap_err();
    assert!(matches!(
        error,
        ParseError::DuplicateBoxMethod {
            name,
            first_line: 2,
            first_column: 3,
            duplicate_line: 3,
            duplicate_column: 3,
        } if name == "run"
    ));
}

#[test]
fn static_duplicate_reports_first_and_duplicate_sites() {
    let error = NyashParser::parse_from_string(
        "static box Ops {\n  run() { return 1 }\n  run() { return 2 }\n}\n",
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ParseError::DuplicateBoxMethod {
            name,
            first_line: 2,
            first_column: 3,
            duplicate_line: 3,
            duplicate_column: 3,
        } if name == "run"
    ));
}

#[test]
fn build_cfg_prune_preserves_static_method_metadata() {
    let source = concat!(
        "static box Ops {\n",
        "  zeta() { return 3 }\n",
        "  alpha() {\n",
        "    gate Build.release { return 1 } else { return 2 }\n",
        "  }\n",
        "  middle() { return 2 }\n",
        "}\n",
    );
    let ast = parse_ok(source);
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Ops") else {
        panic!("expected static BoxDeclaration")
    };
    assert_eq!(
        methods.names_in_selected_order().collect::<Vec<_>>(),
        vec!["zeta", "alpha", "middle"]
    );
    let entry = methods.get("alpha").expect("alpha method");

    assert_eq!(entry.name(), "alpha");
    assert_eq!(entry.site().selected_method_ordinal(), 1);
    assert_eq!(
        (entry.diagnostic_span().line, entry.diagnostic_span().column),
        (3, 3)
    );
    assert!(matches!(
        entry.provenance(),
        BoxMethodProvenanceV1::ExplicitSource { .. }
    ));
    assert!(matches!(
        entry.declaration(),
        ASTNode::FunctionDeclaration { body, .. }
            if body.iter().all(|node| !matches!(node, ASTNode::BuildGate { .. }))
    ));
}

#[test]
fn newline_postfix_mutates_pending_static_method_before_single_commit() {
    let ast = with_env_vars(
        &[
            ("NYASH_FEATURES", Some("stage3")),
            ("NYASH_METHOD_CATCH", Some("1")),
        ],
        || {
            NyashParser::parse_from_string(
                "static box Ops {\n  run() { return 1 }\n\n  cleanup { print(\"done\") }\n}\n",
            )
            .expect("static postfix fixture parses")
        },
    );
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Ops") else {
        panic!("expected static BoxDeclaration")
    };
    assert_eq!(methods.len(), 1);
    let entry = methods.get("run").expect("run method");
    assert_eq!(entry.site().selected_method_ordinal(), 0);
    assert_eq!(
        (entry.diagnostic_span().line, entry.diagnostic_span().column),
        (2, 3)
    );
    assert!(matches!(
        entry.provenance(),
        BoxMethodProvenanceV1::ExplicitSource { .. }
    ));
    assert!(matches!(
        entry.declaration(),
        ASTNode::FunctionDeclaration { body, .. }
            if body.iter().any(|node| matches!(node, ASTNode::TryCatch { .. }))
    ));
}
