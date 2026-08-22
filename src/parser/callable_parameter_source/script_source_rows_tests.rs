use super::*;
use crate::parser::{NyashParser, ParserBuildConfig};

fn parse(source: &str) -> super::ParsedProgramWithCallableParameterSourceV1 {
    NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser product")
}

#[test]
fn admitted_empty_and_executable_scripts_have_complete_rows() {
    for source in ["", "print(1)\n"] {
        let product = parse(source);
        let (_disposition, rows) = product.into_source_disposition_with_script_rows();
        let CanonicalScriptSourceRowsDispositionV1::HandoffReady(rows) = rows else {
            panic!("pure Script rows should be ready")
        };
        assert_eq!(rows.statement_count() as usize, rows.body_rows().len());
        assert!(rows.import_config().is_explicit());
        assert!(rows.import_config().is_complete());
    }
}

#[test]
fn declarations_and_brand_syntax_are_snapshotted_without_semantics() {
    let product = parse("function helper(value: i64) { return value }\nbrand PageId: i64\n");
    let (_disposition, rows) = product.into_source_disposition_with_script_rows();
    let CanonicalScriptSourceRowsDispositionV1::HandoffReady(rows) = rows else {
        panic!("pure Script rows should be ready")
    };
    assert_eq!(rows.declarations().len(), 1);
    assert_eq!(rows.declarations()[0].name(), "helper");
    assert_eq!(rows.declarations()[0].parameters()[0].name(), "value");
    assert_eq!(rows.brands()[0].name(), "PageId");
    assert_eq!(rows.brands()[0].underlying_type_name(), "i64");
}

#[test]
fn compatibility_and_imports_never_become_empty_ready_rows() {
    let boxed = parse("box Plain {}\n");
    let (_disposition, boxed_rows) = boxed.into_source_disposition_with_script_rows();
    assert!(matches!(
        boxed_rows,
        CanonicalScriptSourceRowsDispositionV1::CompatibilitySource
    ));
    let imported = parse("using plain\nprint(1)\n");
    let (_disposition, imported_rows) = imported.into_source_disposition_with_script_rows();
    assert!(matches!(
        imported_rows,
        CanonicalScriptSourceRowsDispositionV1::CohortUnresolved
            | CanonicalScriptSourceRowsDispositionV1::Deferred
            | CanonicalScriptSourceRowsDispositionV1::SourceAuthorityUnavailable
    ));
}
