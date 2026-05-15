//! Shared parser test helpers.

use crate::ast::ASTNode;
use crate::parser::NyashParser;

pub fn parse_ok(src: &str) -> ASTNode {
    NyashParser::parse_from_string(src).expect("parse ok")
}

pub fn program_statements(src: &str) -> Vec<ASTNode> {
    let ASTNode::Program { statements, .. } = parse_ok(src) else {
        panic!("expected Program");
    };
    statements
}

pub fn parse_ok_with_unified_members(src: &str) -> ASTNode {
    crate::tests::helpers::env::with_env_var("NYASH_ENABLE_UNIFIED_MEMBERS", "1", || parse_ok(src))
}

pub fn parse_err_with_unified_members(src: &str) {
    crate::tests::helpers::env::with_env_var("NYASH_ENABLE_UNIFIED_MEMBERS", "1", || {
        NyashParser::parse_from_string(src).expect_err("parse should fail");
    });
}

pub fn find_box<'a>(ast: &'a ASTNode, box_name: &str) -> &'a ASTNode {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
        .iter()
        .find(|stmt| matches!(stmt, ASTNode::BoxDeclaration { name, .. } if name == box_name))
        .unwrap_or_else(|| panic!("box declaration not found: {box_name}"))
}

pub fn find_method_decl<'a>(ast: &'a ASTNode, box_name: &str, method_name: &str) -> &'a ASTNode {
    let box_decl = find_box(ast, box_name);
    let ASTNode::BoxDeclaration { methods, .. } = box_decl else {
        panic!("expected BoxDeclaration");
    };
    methods
        .get(method_name)
        .unwrap_or_else(|| panic!("method not found: {box_name}.{method_name}"))
}

pub fn find_method_body<'a>(ast: &'a ASTNode, box_name: &str, method_name: &str) -> &'a [ASTNode] {
    let ASTNode::FunctionDeclaration { body, .. } = find_method_decl(ast, box_name, method_name)
    else {
        panic!("expected FunctionDeclaration");
    };
    body
}

pub fn find_method_params(ast: &ASTNode, box_name: &str, method_name: &str) -> Vec<String> {
    let ASTNode::FunctionDeclaration { params, .. } = find_method_decl(ast, box_name, method_name)
    else {
        panic!("expected FunctionDeclaration");
    };
    params.clone()
}

pub fn find_constructor_body<'a>(box_node: &'a ASTNode, key: &str) -> &'a [ASTNode] {
    let ASTNode::BoxDeclaration { constructors, .. } = box_node else {
        panic!("expected BoxDeclaration");
    };
    let ASTNode::FunctionDeclaration { body, .. } = constructors
        .get(key)
        .unwrap_or_else(|| panic!("constructor not found: {key}"))
    else {
        panic!("expected FunctionDeclaration");
    };
    body
}
