use nyash_rust::{parser::NyashParser, ASTNode};

fn expanded_box_method_names(code: &str, box_name: &str, derive_all: Option<&str>) -> Vec<String> {
    let expanded = crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_ENABLE", Some("1")),
            ("NYASH_MACRO_TRACE", Some("0")),
            ("NYASH_MACRO_DERIVE", None),
            ("NYASH_MACRO_DERIVE_ALL", derive_all),
        ],
        || {
            let ast = NyashParser::parse_from_string(code).expect("parse ok");
            crate::r#macro::maybe_expand_and_dump(&ast, false)
        },
    );

    let ASTNode::Program { statements, .. } = expanded else {
        panic!("expected expanded program");
    };
    let Some(ASTNode::BoxDeclaration { methods, .. }) = statements.into_iter().find(
        |statement| matches!(statement, ASTNode::BoxDeclaration { name, .. } if name == box_name),
    ) else {
        panic!("{box_name} declaration not found after expansion");
    };
    methods
        .names_in_selected_order()
        .map(str::to_owned)
        .collect()
}

#[test]
fn macro_derive_injects_equals_and_tostring() {
    let methods = expanded_box_method_names(
        r#"
box UserBox {
  name: StringBox
  age: IntegerBox
}
"#,
        "UserBox",
        None,
    );
    assert!(methods.iter().any(|method| method == "equals"));
    assert!(methods.iter().any(|method| method == "toString"));
}

#[test]
fn macro_derive_skips_receiver_methods_for_static_box() {
    let methods = expanded_box_method_names(
        r#"
static box Utility {
  ping() { return 0 }
}
"#,
        "Utility",
        Some("1"),
    );
    assert!(methods.iter().any(|method| method == "ping"));
    assert!(!methods.iter().any(|method| method == "equals"));
    assert!(!methods.iter().any(|method| method == "toString"));
}

#[test]
fn macro_derive_preserves_static_main_without_receiver_methods() {
    let methods = expanded_box_method_names(
        r#"
static box Main {
  main() { return 0 }
}
"#,
        "Main",
        None,
    );
    assert!(methods.iter().any(|method| method == "main"));
    assert!(!methods.iter().any(|method| method == "equals"));
    assert!(!methods.iter().any(|method| method == "toString"));
}
