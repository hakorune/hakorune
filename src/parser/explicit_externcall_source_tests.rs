use super::NyashParser;
use crate::ast::ASTNode;

fn only_statement(source: &str) -> ASTNode {
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string(source).expect("source must parse")
    else {
        panic!("parser root must be Program");
    };
    assert_eq!(statements.len(), 1);
    statements.remove(0)
}

#[test]
fn canonical_explicit_externcall_keeps_target_out_of_arguments() {
    let ASTNode::ExplicitExternCall {
        target, arguments, ..
    } = only_statement("externcall \"env.get\"(1, 2)")
    else {
        panic!("canonical spelling must issue ExplicitExternCall");
    };
    assert_eq!(target, "env.get");
    assert_eq!(arguments.len(), 2);
}

#[test]
fn parenthesized_externcall_remains_an_ordinary_function_call() {
    let ASTNode::FunctionCall {
        name, arguments, ..
    } = only_statement("externcall(\"env.get\", 1)")
    else {
        panic!("generic parenthesized spelling must remain FunctionCall");
    };
    assert_eq!(name, "externcall");
    assert_eq!(arguments.len(), 2);
}

#[test]
fn explicit_externcall_string_without_argument_list_rejects() {
    assert!(NyashParser::parse_from_string("externcall \"env.get\"").is_err());
}
