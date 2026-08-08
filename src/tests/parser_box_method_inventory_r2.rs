use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodProvenanceV1, BoxMethodSourceSelectionV1,
};
use crate::parser::{BuildMode, NyashParser, ParseError, ParserBuildConfig};
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
fn ordinary_box_methods_keep_lexical_order_and_explicit_sites_after_r3() {
    let ast = parse_ok(
        "box Plain {\n  zeta() { return 3 }\n  alpha() { return 1 }\n  middle() { return 2 }\n}\n",
    );
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Plain") else {
        panic!("expected ordinary BoxDeclaration")
    };
    assert_eq!(
        methods.names_in_selected_order().collect::<Vec<_>>(),
        vec!["zeta", "alpha", "middle"]
    );
    for (ordinal, entry) in methods.iter_selected_declaration_order().enumerate() {
        assert_eq!(entry.site().selected_method_ordinal(), ordinal as u32);
        assert_eq!(entry.diagnostic_span().line, ordinal + 2);
        assert!(matches!(
            entry.provenance(),
            BoxMethodProvenanceV1::ExplicitSource { .. }
        ));
    }
}

#[test]
fn ordinary_duplicate_reports_first_and_duplicate_sites() {
    let error = NyashParser::parse_from_string(
        "box Plain {\n  run() { return 1 }\n  run() { return 2 }\n}\n",
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

#[test]
fn newline_postfix_mutates_pending_ordinary_method_before_single_commit() {
    let ast = with_env_vars(
        &[
            ("NYASH_FEATURES", Some("stage3")),
            ("NYASH_METHOD_CATCH", Some("1")),
        ],
        || {
            NyashParser::parse_from_string(
                "box Plain {\n  run() { return 1 }\n\n  cleanup { print(\"done\") }\n}\n",
            )
            .expect("ordinary postfix fixture parses")
        },
    );
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Plain") else {
        panic!("expected ordinary BoxDeclaration")
    };
    assert_eq!(methods.len(), 1);
    let entry = methods.get("run").expect("run method");
    assert_eq!(entry.site().selected_method_ordinal(), 0);
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

#[test]
fn ordinary_selected_gate_uses_exact_syntactic_member_ordinals() {
    let ast = NyashParser::parse_from_string_with_build_config(
        r#"
box Choice {
  direct() { return 0 }
  outer_field: i64
  gate Build.test {
    branch_field: i64
    selected() { return 1 }
  } else {
    branch_field: i64
    selected() { return 2 }
  }
  after() { return 3 }
}
"#,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("selected ordinary Box fixture parses");
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Choice") else {
        panic!("expected ordinary BoxDeclaration")
    };
    assert_eq!(
        methods.names_in_selected_order().collect::<Vec<_>>(),
        vec!["direct", "selected", "after"]
    );
    let entry = methods.get("selected").unwrap();
    assert_eq!(entry.site().selected_method_ordinal(), 1);
    let BoxMethodProvenanceV1::ExplicitSource {
        selection: BoxMethodSourceSelectionV1::SelectedBuildGate { path },
    } = entry.provenance()
    else {
        panic!("selected method must retain source selection")
    };
    assert_eq!(path.len(), 1);
    assert_eq!(path[0].gate_site().box_member_ordinal(), 2);
    assert_eq!(path[0].branch_member_ordinal(), 1);
}

#[test]
fn ordinary_nested_selected_else_keeps_outer_to_inner_source_path() {
    let ast = NyashParser::parse_from_string_with_build_config(
        r#"
box NestedChoice {
  direct() { return 0 }
  outer_field: i64
  gate Build.test {
    outer_branch_field: i64
    gate Build.release {
      ignored() { return 1 }
    } else {
      inner_field_a: i64
      inner_field_b: i64
      selected_else() { return 2 }
    }
  } else {
    ignored_outer() { return 3 }
  }
  after() { return 4 }
}
"#,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("nested selected else fixture parses");
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "NestedChoice") else {
        panic!("expected ordinary BoxDeclaration")
    };
    assert_eq!(
        methods.names_in_selected_order().collect::<Vec<_>>(),
        vec!["direct", "selected_else", "after"]
    );
    let entry = methods.get("selected_else").unwrap();
    assert_eq!(entry.site().selected_method_ordinal(), 1);
    let BoxMethodProvenanceV1::ExplicitSource {
        selection: BoxMethodSourceSelectionV1::SelectedBuildGate { path },
    } = entry.provenance()
    else {
        panic!("nested selected method must retain source selection")
    };
    assert_eq!(path.len(), 2);
    assert_eq!(path[0].gate_site().box_member_ordinal(), 2);
    assert_eq!(path[0].branch_member_ordinal(), 1);
    assert_eq!(path[1].gate_site().box_member_ordinal(), 1);
    assert_eq!(path[1].branch_member_ordinal(), 2);
}

#[test]
fn selected_property_helpers_share_the_property_source_member_ordinal() {
    let ast = with_env_vars(&[("NYASH_ENABLE_UNIFIED_MEMBERS", Some("1"))], || {
        NyashParser::parse_from_string_with_build_config(
            r#"
box Choice {
  gate Build.test {
    pad: i64
    once value: i64 => 1
  } else {
    pad: i64
    once value: i64 => 2
  }
}
"#,
            ParserBuildConfig {
                mode: BuildMode::Test,
                ..ParserBuildConfig::default()
            },
        )
        .expect("selected property fixture parses")
    });
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Choice") else {
        panic!("expected ordinary BoxDeclaration")
    };
    assert_eq!(
        methods.names_in_selected_order().collect::<Vec<_>>(),
        vec!["__compute_once_value", "__get_once_value"]
    );
    for entry in methods.iter_selected_declaration_order() {
        let BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Property {
            selection: BoxMethodSourceSelectionV1::SelectedBuildGate { path },
            ..
        }) = entry.provenance()
        else {
            panic!("selected property must retain generated provenance")
        };
        assert_eq!(path.len(), 1);
        assert_eq!(path[0].gate_site().box_member_ordinal(), 0);
        assert_eq!(path[0].branch_member_ordinal(), 1);
    }
}
