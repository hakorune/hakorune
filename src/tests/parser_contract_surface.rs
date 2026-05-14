use crate::ast::{ASTNode, ContractKind};
use crate::parser::NyashParser;

fn parse_program(source: &str) -> Vec<ASTNode> {
    let ast = NyashParser::parse_from_string(source).expect("parse contract surface");
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
}

#[test]
fn parser_contract_surface_parses_method_requires_ensures_metadata_only() {
    let statements = parse_program(
        r#"
static box Main {
    main(x: i64): i64
        requires x >= 0
        ensures x >= 0
    {
        return x
    }
}
"#,
    );
    let main_box = statements
        .iter()
        .find(|statement| matches!(statement, ASTNode::BoxDeclaration { name, .. } if name == "Main"))
        .expect("Main box");
    let ASTNode::BoxDeclaration { methods, .. } = main_box else {
        panic!("expected box declaration");
    };
    let ASTNode::FunctionDeclaration { contracts, body, .. } = &methods["main"] else {
        panic!("expected main method");
    };

    assert_eq!(contracts.len(), 2);
    assert!(matches!(contracts[0].kind, ContractKind::Requires));
    assert!(matches!(contracts[1].kind, ContractKind::Ensures));
    assert_eq!(body.len(), 1, "Stage0 must not inject runtime contract checks");
    assert!(matches!(body[0], ASTNode::Return { .. }));
}

#[test]
fn parser_contract_surface_parses_free_function_contract_metadata() {
    let statements = parse_program(
        r#"
function validate(x: i64): i64
    requires x >= 0
{
    return x
}
"#,
    );
    let ASTNode::FunctionDeclaration { contracts, body, .. } = &statements[0] else {
        panic!("expected function declaration");
    };

    assert_eq!(contracts.len(), 1);
    assert!(matches!(contracts[0].kind, ContractKind::Requires));
    assert_eq!(body.len(), 1, "contracts are metadata only in Stage0");
}

#[test]
fn parser_contract_surface_parses_box_invariant_member_metadata() {
    let statements = parse_program(
        r#"
box Page {
    used: i64
    invariant used >= 0

    getUsed() {
        return me.used
    }
}
"#,
    );
    let ASTNode::BoxDeclaration {
        invariants,
        methods,
        ..
    } = &statements[0]
    else {
        panic!("expected box declaration");
    };

    assert_eq!(invariants.len(), 1);
    assert!(methods.contains_key("getUsed"));
}

#[test]
fn parser_contract_surface_parses_record_invariant_member_metadata() {
    let statements = parse_program(
        r#"
record Meta {
    ptr: i64
    invariant ptr >= 0
}
"#,
    );
    let ASTNode::BoxDeclaration {
        is_record,
        invariants,
        methods,
        ..
    } = &statements[0]
    else {
        panic!("expected record declaration");
    };

    assert!(*is_record);
    assert_eq!(invariants.len(), 1);
    assert!(methods.is_empty(), "record invariant metadata must not create methods");
}

#[test]
fn parser_contract_surface_keeps_contract_words_contextual() {
    let statements = parse_program(
        r#"
static box Main {
    main() {
        local requires = 1
        local ensures = requires
        return ensures
    }
}
"#,
    );
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        panic!("expected box declaration");
    };
    let ASTNode::FunctionDeclaration { contracts, body, .. } = &methods["main"] else {
        panic!("expected main method");
    };

    assert!(contracts.is_empty());
    assert_eq!(body.len(), 3);
}
