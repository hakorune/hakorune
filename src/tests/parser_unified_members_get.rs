use crate::ast::ASTNode;
use crate::tests::helpers::parser::{find_box, parse_ok_with_unified_members};

#[test]
fn get_computed_property_generates_standard_getter() {
    let ast = parse_ok_with_unified_members(
        r#"
box Greeter {
  name: StringBox = "Nya"
  get greeting: StringBox => "Hello " + me.name
}
"#,
    );

    let ASTNode::BoxDeclaration {
        fields, methods, ..
    } = find_box(&ast, "Greeter")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(fields.contains(&"name".to_string()));
    assert!(!fields.contains(&"greeting".to_string()));
    assert!(methods.contains_key("__get_greeting"));
}

#[test]
fn get_identifier_can_remain_stored_field_name() {
    let ast = parse_ok_with_unified_members(
        r#"
box HasGetField {
  get: StringBox
  name: StringBox
}
"#,
    );

    let ASTNode::BoxDeclaration {
        fields, methods, ..
    } = find_box(&ast, "HasGetField")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(fields.contains(&"get".to_string()));
    assert!(fields.contains(&"name".to_string()));
    assert!(!methods.contains_key("__get_get"));
}

#[test]
fn get_identifier_can_remain_method_name() {
    let ast = parse_ok_with_unified_members(
        r#"
box HasGetMethod {
  get() {
    return 1
  }
}
"#,
    );

    let ASTNode::BoxDeclaration {
        fields, methods, ..
    } = find_box(&ast, "HasGetMethod")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(!fields.contains(&"get".to_string()));
    assert!(methods.contains_key("get"));
    assert!(!methods.contains_key("__get_get"));
}

#[test]
fn get_identifier_on_previous_line_stays_stored_field() {
    let ast = parse_ok_with_unified_members(
        r#"
box HasBareGetField {
  get
  name: StringBox
}
"#,
    );

    let ASTNode::BoxDeclaration {
        fields, methods, ..
    } = find_box(&ast, "HasBareGetField")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(fields.contains(&"get".to_string()));
    assert!(fields.contains(&"name".to_string()));
    assert!(!methods.contains_key("__get_name"));
}

#[test]
fn public_get_computed_property_tracks_visibility_name() {
    let ast = parse_ok_with_unified_members(
        r#"
box VisibleComputed {
  public get size: IntegerBox => 1
}
"#,
    );

    let ASTNode::BoxDeclaration {
        public_fields,
        fields,
        methods,
        ..
    } = find_box(&ast, "VisibleComputed")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(public_fields.contains(&"size".to_string()));
    assert!(!fields.contains(&"size".to_string()));
    assert!(methods.contains_key("__get_size"));
}

#[test]
fn private_get_computed_property_tracks_visibility_name() {
    let ast = parse_ok_with_unified_members(
        r#"
box PrivateComputed {
  private get hidden: IntegerBox => 1
}
"#,
    );

    let ASTNode::BoxDeclaration {
        private_fields,
        public_fields,
        fields,
        methods,
        ..
    } = find_box(&ast, "PrivateComputed")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(private_fields.contains(&"hidden".to_string()));
    assert!(!public_fields.contains(&"hidden".to_string()));
    assert!(!fields.contains(&"hidden".to_string()));
    assert!(methods.contains_key("__get_hidden"));
}

#[test]
fn visible_get_identifier_can_remain_stored_field_name() {
    let ast = parse_ok_with_unified_members(
        r#"
box VisibleGetField {
  public get: StringBox
}
"#,
    );

    let ASTNode::BoxDeclaration {
        public_fields,
        fields,
        methods,
        ..
    } = find_box(&ast, "VisibleGetField")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(public_fields.contains(&"get".to_string()));
    assert!(fields.contains(&"get".to_string()));
    assert!(!methods.contains_key("__get_get"));
}

#[test]
fn stored_field_initializers_generate_birth_prologue() {
    let ast = parse_ok_with_unified_members(
        r#"
box FieldDefaults {
  count = 41
  name: StringBox = "Nya"
}
"#,
    );

    let ASTNode::BoxDeclaration {
        fields,
        field_decls,
        constructors,
        ..
    } = find_box(&ast, "FieldDefaults")
    else {
        panic!("expected BoxDeclaration");
    };

    assert!(fields.contains(&"count".to_string()));
    assert!(fields.contains(&"name".to_string()));
    assert!(field_decls
        .iter()
        .any(|decl| decl.name == "count" && decl.declared_type_name.is_none()));
    assert!(
        field_decls
            .iter()
            .any(|decl| decl.name == "name"
                && decl.declared_type_name.as_deref() == Some("StringBox"))
    );

    let Some(ASTNode::FunctionDeclaration { body, .. }) = constructors.get("birth/0") else {
        panic!("expected synthetic birth/0 constructor");
    };
    assert_eq!(body.len(), 2);
    assert!(matches!(
        &body[0],
        ASTNode::Assignment { target, .. }
            if matches!(
                target.as_ref(),
                ASTNode::FieldAccess { object, field, .. }
                    if field == "count" && matches!(object.as_ref(), ASTNode::Me { .. })
            )
    ));
    assert!(matches!(
        &body[1],
        ASTNode::Assignment { target, .. }
            if matches!(
                target.as_ref(),
                ASTNode::FieldAccess { object, field, .. }
                    if field == "name" && matches!(object.as_ref(), ASTNode::Me { .. })
            )
    ));
}
