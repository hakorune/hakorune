use crate::ast::ParamDecl;

use super::project_neutral_parameter_syntax_v1;

#[test]
fn preserves_typed_and_untyped_parameter_syntax_in_source_order() {
    let declarations = vec![
        ParamDecl {
            name: "source".to_owned(),
            declared_type_name: None,
        },
        ParamDecl {
            name: "count".to_owned(),
            declared_type_name: Some("i64".to_owned()),
        },
    ];

    let rows = project_neutral_parameter_syntax_v1(&declarations, &[]);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name(), "source");
    assert_eq!(rows[0].declared_type_name(), None);
    assert_eq!(rows[1].name(), "count");
    assert_eq!(rows[1].declared_type_name(), Some("i64"));
}

#[test]
fn preserves_legacy_name_fallback_without_inventing_type_syntax() {
    let params = vec!["left".to_owned(), "right".to_owned()];
    let rows = project_neutral_parameter_syntax_v1(&[], &params);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name(), "left");
    assert_eq!(rows[1].name(), "right");
    assert!(rows.iter().all(|row| row.declared_type_name().is_none()));
}
