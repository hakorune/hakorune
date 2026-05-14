use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn parse_program(source: &str) -> Vec<ASTNode> {
    let ast = NyashParser::parse_from_string(source).expect("parse transition surface");
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
}

#[test]
fn parser_transition_surface_parses_box_transition_metadata_only() {
    let statements = parse_program(
        r#"
box Page {
    transition PageState.Active -> PageState.Retired by retire
}
"#,
    );
    let ASTNode::BoxDeclaration {
        transitions,
        methods,
        ..
    } = &statements[0]
    else {
        panic!("expected box declaration");
    };

    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0].from_state, "PageState.Active");
    assert_eq!(transitions[0].to_state, "PageState.Retired");
    assert_eq!(transitions[0].method_name, "retire");
    assert!(methods.is_empty(), "Stage0 must not generate transition methods");
}

#[test]
fn parser_transition_surface_keeps_transition_and_by_contextual() {
    let statements = parse_program(
        r#"
static box Main {
    main() {
        local transition = 1
        local by = transition
        return by
    }
}
"#,
    );
    let ASTNode::BoxDeclaration { methods, transitions, .. } = &statements[0] else {
        panic!("expected box declaration");
    };
    let ASTNode::FunctionDeclaration { body, .. } = &methods["main"] else {
        panic!("expected main method");
    };

    assert!(transitions.is_empty());
    assert_eq!(body.len(), 3);
}

#[test]
fn parser_transition_surface_rejects_missing_by_clause() {
    NyashParser::parse_from_string(
        r#"
box Page {
    transition PageState.Active -> PageState.Retired retire
}
"#,
    )
    .expect_err("transition must require explicit by clause");
}
