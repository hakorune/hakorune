use crate::parser::{NyashParser, ParserBuildConfig};

use super::module_rows::{
    ParserNormalModuleSourceRowsDispositionV1, ParserNormalModuleSourceRowsIncompleteV1,
    ParserNormalModuleSourceRowsOutsideReasonV1,
};

#[test]
fn ordinary_box_and_direct_instance_method_form_one_source_row() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        "box Plain { run() { return 1 } }\n",
        ParserBuildConfig::default(),
    )
    .unwrap();

    let Some(disposition) = parsed.normal_module_source_rows() else {
        panic!("source-backed ordinary parser product must expose module rows")
    };
    let ParserNormalModuleSourceRowsDispositionV1::Ready(rows) = disposition else {
        panic!("ordinary one-box/one-method cohort must be Ready")
    };
    assert_eq!(rows.box_row().program_position(), 0);
    assert_eq!(rows.box_row().declaration_syntax().name(), "Plain");
    assert_eq!(rows.box_row().method().diagnostic_name(), "run");
    assert_eq!(rows.box_row().method().arity(), 0);
}

#[test]
fn multiple_ordinary_boxes_are_outside_the_bounded_row_cohort() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        "box First { run() {} }\nbox Second { run() {} }\n",
        ParserBuildConfig::default(),
    )
    .unwrap();

    let Some(disposition) = parsed.normal_module_source_rows() else {
        panic!("ordinary source authority must be present")
    };
    assert!(matches!(
        disposition,
        ParserNormalModuleSourceRowsDispositionV1::Outside(
            ParserNormalModuleSourceRowsOutsideReasonV1::UnsupportedProgramBody
        )
    ));
}

#[test]
fn static_box_entry_stops_on_missing_ordinary_source_seal() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        "static box Main { main() {} }\n",
        ParserBuildConfig::default(),
    )
    .unwrap();

    let Some(disposition) = parsed.normal_module_source_rows() else {
        panic!("source-backed program must retain the typed module-row disposition")
    };
    assert!(matches!(
        disposition,
        ParserNormalModuleSourceRowsDispositionV1::Incomplete(
            ParserNormalModuleSourceRowsIncompleteV1::BoxSourceSealMissing
        )
    ));
}
