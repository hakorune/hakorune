use crate::ast::ASTNode;
use crate::tests::helpers::parser::{
    find_box, parse_err_with_unified_members, parse_ok_with_unified_members,
};

#[test]
fn weak_field_is_stored_field_metadata() {
    let ast = parse_ok_with_unified_members(
        r#"
box Node {
  weak parent: Node
}
"#,
    );

    let ASTNode::BoxDeclaration {
        fields,
        field_decls,
        weak_fields,
        methods,
        ..
    } = find_box(&ast, "Node")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(fields.contains(&"parent".to_string()));
    assert!(weak_fields.contains(&"parent".to_string()));
    assert!(methods.is_empty());
    let parent_decl = field_decls
        .iter()
        .find(|decl| decl.name == "parent")
        .expect("parent field decl");
    assert_eq!(parent_decl.declared_type_name.as_deref(), Some("Node"));
    assert!(parent_decl.is_weak);
}

#[test]
fn public_weak_field_tracks_visibility_and_weak_metadata() {
    let ast = parse_ok_with_unified_members(
        r#"
box Node {
  public weak parent: Node
}
"#,
    );

    let ASTNode::BoxDeclaration {
        public_fields,
        fields,
        weak_fields,
        ..
    } = find_box(&ast, "Node")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(public_fields.contains(&"parent".to_string()));
    assert!(fields.contains(&"parent".to_string()));
    assert!(weak_fields.contains(&"parent".to_string()));
}

#[test]
fn weak_field_rejects_computed_arrow_body() {
    parse_err_with_unified_members(
        r#"
box Node {
  weak parent: Node => 1
}
"#,
    );
}

#[test]
fn weak_field_rejects_computed_block_body() {
    parse_err_with_unified_members(
        r#"
box Node {
  weak parent: Node {
    return 1
  }
}
"#,
    );
}
