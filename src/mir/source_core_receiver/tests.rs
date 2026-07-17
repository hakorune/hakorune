use crate::ast::ASTNode;
use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::parser::NyashParser;

use super::{
    SourceCoreReceiverFactV1, SourceCoreReceiverProofErrorV1, VerifiedSourceCoreReceiverV1,
};

fn parse(source: &str) -> ASTNode {
    NyashParser::parse_from_string(source).expect("source receiver fixture must parse")
}

fn return_expression(expression: &str) -> ASTNode {
    let root = parse(&format!(
        "static box FixtureV1 {{ run(x) {{ return {expression} }} }}"
    ));
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("fixture declaration catalog must seal");
    let declaration = catalog
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "FixtureV1",
            "run",
            1,
        )
        .expect("fixture method declaration");
    let [ASTNode::Return {
        value: Some(value), ..
    }] = declaration.body()
    else {
        panic!("fixture body must be one value return")
    };
    value.as_ref().clone()
}

fn fact(expression: &ASTNode) -> SourceCoreReceiverFactV1 {
    VerifiedSourceCoreReceiverV1::verify(expression)
        .expect("expression must prove a String receiver")
        .fact()
}

#[test]
fn string_literals_are_exact_on_success() {
    for source in ["\"\"", "\"text\""] {
        assert_eq!(
            fact(&return_expression(source)),
            SourceCoreReceiverFactV1::ExactStringOnSuccess
        );
    }
}

#[test]
fn string_left_add_ignores_the_rhs_representation() {
    for source in [r#""" + x"#, r#""" + 1"#, r#""" + void"#, r#""" + other()"#] {
        assert_eq!(
            fact(&return_expression(source)),
            SourceCoreReceiverFactV1::ExactStringOnSuccess,
            "source={source}"
        );
    }
}

#[test]
fn nested_string_left_add_follows_only_the_left_spine() {
    for source in [r#"("" + x) + 1"#, r#"(("" + x) + y) + other()"#] {
        assert_eq!(
            fact(&return_expression(source)),
            SourceCoreReceiverFactV1::ExactStringOnSuccess,
            "source={source}"
        );
    }
}

#[test]
fn unsupported_roots_and_string_right_add_reject() {
    for source in [
        "x",
        "1",
        "true",
        "null",
        "void",
        r#"x + "text""#,
        "1 + 2",
        "other()",
        "x.length()",
        r#""text" - 1"#,
    ] {
        assert_eq!(
            VerifiedSourceCoreReceiverV1::verify(&return_expression(source)).unwrap_err(),
            SourceCoreReceiverProofErrorV1::UnsupportedLeftSpineTerminal,
            "source={source}"
        );
    }
}

#[test]
fn actual_string_helpers_to_i64_initializer_is_exact_string_on_success() {
    let root = parse(include_str!(concat!(
        "../../../lang/src/shared/common/",
        "string_helpers.hako"
    )));
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("StringHelpers declaration catalog must seal");
    let declaration = catalog
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "StringHelpers",
            "to_i64",
            1,
        )
        .expect("StringHelpers.to_i64/1 declaration");
    let initializer = declaration
        .body()
        .iter()
        .find_map(|statement| match statement {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } if variables == &["s"] => initial_values.first()?.as_deref(),
            _ => None,
        })
        .expect("to_i64 must retain local s initializer");

    assert_eq!(
        fact(initializer),
        SourceCoreReceiverFactV1::ExactStringOnSuccess
    );
}
