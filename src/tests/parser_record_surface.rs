use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn first_decl(source: &str) -> ASTNode {
    let ast = NyashParser::parse_from_string(source).expect("parse record source");
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements.into_iter().next().expect("first statement")
}

#[test]
fn c202_record_declaration_parses_typed_fields() {
    let decl = first_decl(
        r#"
record HakoAllocAlignedSmallMeta {
    ptr: i64
    alignment: i64
    requested_size: i64
    usable_size: i64
}
"#,
    );

    let ASTNode::BoxDeclaration {
        name,
        fields,
        field_decls,
        is_record,
        methods,
        constructors,
        weak_fields,
        ..
    } = decl
    else {
        panic!("record should use the declaration AST lane");
    };

    assert_eq!(name, "HakoAllocAlignedSmallMeta");
    assert!(is_record, "record declarations must be marked as record");
    assert_eq!(
        fields,
        vec!["ptr", "alignment", "requested_size", "usable_size"]
    );
    assert!(methods.is_empty(), "C202 record has no methods");
    assert!(constructors.is_empty(), "C202 record has no constructors");
    assert!(weak_fields.is_empty(), "C202 record has no weak fields");
    assert_eq!(field_decls.len(), 4);
    assert!(field_decls
        .iter()
        .all(|decl| decl.declared_type_name.is_some() && !decl.is_weak));
}

#[test]
fn c202_record_rejects_weak_untyped_and_method_bodies() {
    for source in [
        r#"
record WeakMeta {
    weak ptr: i64
}
"#,
        r#"
record UntypedMeta {
    ptr
}
"#,
        r#"
record MethodMeta {
    ptr: i64
    value() {
        return ptr
    }
}
"#,
    ] {
        NyashParser::parse_from_string(source).expect_err("invalid C202 record must reject");
    }
}
