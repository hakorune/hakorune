use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn parse(src: &str) -> ASTNode {
    std::env::set_var("NYASH_ENABLE_UNIFIED_MEMBERS", "1");
    NyashParser::parse_from_string(src).expect("parse ok")
}

fn find_box<'a>(ast: &'a ASTNode, box_name: &str) -> &'a ASTNode {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
        .iter()
        .find(|stmt| matches!(stmt, ASTNode::BoxDeclaration { name, .. } if name == box_name))
        .expect("box should exist")
}

#[test]
fn get_computed_property_generates_standard_getter() {
    let ast = parse(
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
    let ast = parse(
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
    let ast = parse(
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
    let ast = parse(
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
    let ast = parse(
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
    let ast = parse(
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
    let ast = parse(
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
