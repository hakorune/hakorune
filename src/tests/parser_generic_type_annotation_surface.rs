use crate::ast::{ASTNode, ParamDecl};
use crate::tests::helpers::parser::program_statements;

#[test]
fn parser_generic_type_annotation_surface_parses_box_field_metadata() {
    let statements = program_statements(
        r#"
box Store {
    metas: PackedArray<Meta>
    weak span: Span<PageId>
}
"#,
    );
    let ASTNode::BoxDeclaration {
        field_decls,
        weak_fields,
        ..
    } = &statements[0]
    else {
        panic!("expected box declaration");
    };

    assert_eq!(
        field_decls[0].declared_type_name.as_deref(),
        Some("PackedArray<Meta>")
    );
    assert_eq!(
        field_decls[1].declared_type_name.as_deref(),
        Some("Span<PageId>")
    );
    assert_eq!(weak_fields, &vec!["span".to_string()]);
}

#[test]
fn parser_generic_type_annotation_surface_parses_signature_metadata() {
    let statements = program_statements(
        r#"
static box Main {
    main(items: Array<PageId>): Result<Page, Error> {
        return 0
    }
}
"#,
    );
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        panic!("expected box declaration");
    };
    let ASTNode::FunctionDeclaration {
        param_decls,
        return_type_name,
        ..
    } = &methods["main"]
    else {
        panic!("expected main method");
    };

    assert_eq!(
        param_decls,
        &vec![ParamDecl {
            name: "items".to_string(),
            declared_type_name: Some("Array<PageId>".to_string()),
        }]
    );
    assert_eq!(return_type_name.as_deref(), Some("Result<Page,Error>"));
}

#[test]
fn parser_generic_type_annotation_surface_parses_record_and_alias_metadata() {
    let statements = program_statements(
        r#"
type PageList = Array<PageId>
record MetaStore<T> {
    metas: PackedArray<T>
}
"#,
    );
    let ASTNode::TypeAliasDeclaration {
        target_type_name, ..
    } = &statements[0]
    else {
        panic!("expected type alias");
    };
    assert_eq!(target_type_name, "Array<PageId>");

    let ASTNode::BoxDeclaration {
        type_parameters,
        field_decls,
        is_record,
        ..
    } = &statements[1]
    else {
        panic!("expected record declaration");
    };
    assert!(*is_record);
    assert_eq!(type_parameters, &vec!["T".to_string()]);
    assert_eq!(
        field_decls[0].declared_type_name.as_deref(),
        Some("PackedArray<T>")
    );
}
