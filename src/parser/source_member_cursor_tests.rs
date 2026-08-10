use super::*;
use crate::ast::ASTNode;
use crate::parser::NyashParser;

#[test]
fn cursor_keeps_box_identity_while_advancing_source_members() {
    let brand = ParserInvocationBrandV1::issue();
    let mut cursor = ParserBoxMemberSourceCursorV1::open(brand, 7);

    assert_eq!(cursor.box_site().statement_ordinal(), 7);
    assert_eq!(cursor.current_member_site().member_ordinal(), 0);
    cursor.finish_member().unwrap();
    assert_eq!(cursor.current_member_site().member_ordinal(), 1);
    cursor.finish_member().unwrap();
    assert_eq!(cursor.current_gate_site().box_member_ordinal(), 2);
}

#[test]
fn branch_reuses_box_identity_but_starts_a_fresh_member_sequence() {
    let brand = ParserInvocationBrandV1::issue();
    let mut parent = ParserBoxMemberSourceCursorV1::open(brand, 3);
    parent.finish_member().unwrap();

    let branch = parent.branch();
    assert_eq!(branch.box_site(), parent.box_site());
    assert_eq!(branch.current_member_ordinal(), 0);
    assert_eq!(parent.current_member_ordinal(), 1);
}

#[test]
fn static_box_mixed_members_keep_existing_ast_behavior_with_shared_cursor() {
    let ast = NyashParser::parse_from_string(
        r#"
static box CursorFixture {
    left
    first(value) { return value }
    right
    second(left, right) { return left }
}
"#,
    )
    .expect("static Box fields and methods must keep their existing parse behavior");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    let ASTNode::BoxDeclaration {
        fields,
        methods,
        is_static,
        ..
    } = &statements[0]
    else {
        panic!("expected static Box declaration");
    };
    assert!(*is_static);
    assert_eq!(fields, &["left", "right"]);
    assert_eq!(
        methods.names_in_selected_order().collect::<Vec<_>>(),
        ["first", "second"]
    );
}
