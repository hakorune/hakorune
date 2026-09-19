//! Focused source-authority tests for the bounded nested Map read chain.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::map_read_fact::{
    MapReadFactIssueV1, MapReadOperandV1, MapReadOperationV1, MapReadResultClassV1,
};

const NESTED_MAP_READ_SOURCE: &str = r#"
static box Helpers {
    read_name(m: MapBox): i64 {
        local name = m.get("functions").get(0).get("name")
        return 30
    }
}
static box Main {
    main() { return read_name(%{"functions" => [%{"name" => "main"}]}) }
}
"#;

const ARRAY_LENGTH_READ_SOURCE: &str = r#"
static box Helpers {
    read_params(m: MapBox): i64 {
        local count = m.get("params").length()
        return count
    }
}
static box Main {
    main() { return read_params(%{"params" => []}) }
}
"#;

const BLOCKS_KIND_READ_SOURCE: &str = r#"
static box Helpers {
    read_kind(m: MapBox): i64 {
        local kind = m.get("blocks").get(0).get("kind")
        return 30
    }
}
static box Main {
    main() {
        local block = %{ "kind" => "entry" }
        return read_kind(%{ "blocks" => [block] })
    }
}
"#;

const BLOCKS_LENGTH_READ_SOURCE: &str = r#"
static box Helpers {
    read_length(m: MapBox): i64 {
        local count = m.get("blocks").length()
        return count
    }
}
static box Main {
    main() {
        local block = %{ "kind" => "entry" }
        return read_length(%{ "blocks" => [block] })
    }
}
"#;

const AGGREGATE_MAP_READ_SOURCE: &str = r#"
static box Helpers {
    read_all(m: MapBox): i64 {
        local name = m.get("functions").get(0).get("name")
        local params_count = m.get("params").length()
        local kind = m.get("blocks").get(0).get("kind")
        local blocks_count = m.get("blocks").length()
        return params_count
    }
}
static box Main {
    main() {
        local function_row = %{ "name" => "main" }
        local block = %{ "kind" => "entry" }
        return read_all(%{
            "functions" => [function_row],
            "params" => [],
            "blocks" => [block]
        })
    }
}
"#;

#[test]
fn source_issues_one_fact_for_each_bounded_nested_read() {
    let package = issue(NESTED_MAP_READ_SOURCE).expect("bounded source Map read package");
    let rows = package.map_read_facts().rows();
    assert_eq!(rows.len(), 3);
    let functions = rows
        .iter()
        .find(|row| row.operand() == &MapReadOperandV1::Key("functions".into()))
        .expect("functions read");
    assert_eq!(functions.operation(), MapReadOperationV1::MapLookup);
    assert_eq!(functions.result(), MapReadResultClassV1::ArrayView);
    let first = rows
        .iter()
        .find(|row| row.operand() == &MapReadOperandV1::Index(0))
        .expect("array index read");
    assert_eq!(first.operation(), MapReadOperationV1::ArrayIndex);
    assert_eq!(first.result(), MapReadResultClassV1::MapView);
    let name = rows
        .iter()
        .find(|row| row.operand() == &MapReadOperandV1::Key("name".into()))
        .expect("name read");
    assert_eq!(name.operation(), MapReadOperationV1::MapLookup);
    assert_eq!(name.result(), MapReadResultClassV1::TextView);
    assert!(rows.iter().all(|row| !row.containment().is_empty()));
}

#[test]
fn source_map_read_stops_before_physical_install() {
    let package = issue(NESTED_MAP_READ_SOURCE).expect("bounded source Map read package");
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(matches!(
        package.prepare_install(&mut context),
        Err((
            _,
            super::NormalCallableSemanticPackageInstallIssueV1::MapReadPhysicalConsumerMissing { .. }
        ))
    ));
    assert!(context.callable_declaration_catalog_vacant());
}

#[test]
fn unrelated_map_lookup_does_not_claim_the_bounded_chain() {
    let package = issue(
        "static box Helpers { read_k(m: MapBox): i64 { return m.get(\"k\") } } \
         static box Main { main() { return read_k(%{\"k\" => 7}) } }",
    )
    .expect("unrelated Map lookup remains admitted");
    assert!(package.map_read_facts().is_empty());
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn source_issues_array_length_for_exact_params_array() {
    let package = issue(ARRAY_LENGTH_READ_SOURCE).expect("bounded array length package");
    let rows = package.map_read_facts().rows();
    assert_eq!(rows.len(), 2);
    let params = rows
        .iter()
        .find(|row| {
            row.operation() == MapReadOperationV1::MapLookup
                && row.operand() == &MapReadOperandV1::Key("params".into())
        })
        .expect("params read");
    assert_eq!(params.operation(), MapReadOperationV1::MapLookup);
    assert_eq!(params.result(), MapReadResultClassV1::ArrayView);
    let length = rows
        .iter()
        .find(|row| row.operation() == MapReadOperationV1::ArrayLength)
        .expect("array length read");
    assert_eq!(length.operand(), &MapReadOperandV1::Key("params".into()));
    assert_eq!(length.result(), MapReadResultClassV1::I64);
    assert!(rows.iter().all(|row| !row.containment().is_empty()));
}

#[test]
fn source_issues_blocks_kind_for_non_empty_all_map_local_array() {
    let package = issue(BLOCKS_KIND_READ_SOURCE).expect("bounded blocks kind package");
    let rows = package.map_read_facts().rows();
    assert_eq!(rows.len(), 3);
    let blocks = rows
        .iter()
        .find(|row| row.operand() == &MapReadOperandV1::Key("blocks".into()))
        .expect("blocks read");
    assert_eq!(blocks.operation(), MapReadOperationV1::MapLookup);
    assert_eq!(blocks.result(), MapReadResultClassV1::ArrayView);
    let index = rows
        .iter()
        .find(|row| row.operation() == MapReadOperationV1::ArrayIndex)
        .expect("blocks index read");
    assert_eq!(index.operand(), &MapReadOperandV1::Index(0));
    assert_eq!(index.result(), MapReadResultClassV1::MapView);
    let kind = rows
        .iter()
        .find(|row| row.operand() == &MapReadOperandV1::Key("kind".into()))
        .expect("kind read");
    assert_eq!(kind.operation(), MapReadOperationV1::MapLookup);
    assert_eq!(kind.result(), MapReadResultClassV1::TextView);
    assert!(rows.iter().all(|row| !row.containment().is_empty()));
}

#[test]
fn source_issues_blocks_length_for_non_empty_all_map_local_array() {
    let package = issue(BLOCKS_LENGTH_READ_SOURCE).expect("bounded blocks length package");
    let rows = package.map_read_facts().rows();
    assert_eq!(rows.len(), 2);
    let blocks = rows
        .iter()
        .find(|row| {
            row.operation() == MapReadOperationV1::MapLookup
                && row.operand() == &MapReadOperandV1::Key("blocks".into())
        })
        .expect("blocks read");
    assert_eq!(blocks.operation(), MapReadOperationV1::MapLookup);
    assert_eq!(blocks.result(), MapReadResultClassV1::ArrayView);
    let length = rows
        .iter()
        .find(|row| row.operation() == MapReadOperationV1::ArrayLength)
        .expect("blocks length read");
    assert_eq!(length.operand(), &MapReadOperandV1::Key("blocks".into()));
    assert_eq!(length.result(), MapReadResultClassV1::I64);
    assert!(rows.iter().all(|row| !row.containment().is_empty()));
}

#[test]
fn source_tracks_distinct_blocks_lookups_when_kind_and_length_are_read() {
    let source = BLOCKS_KIND_READ_SOURCE.replace(
        "local kind = m.get(\"blocks\").get(0).get(\"kind\")",
        "local kind = m.get(\"blocks\").get(0).get(\"kind\")\n        local count = m.get(\"blocks\").length()",
    );
    let package = issue(&source).expect("combined bounded blocks package");
    let rows = package.map_read_facts().rows();
    assert_eq!(rows.len(), 5);
    assert_eq!(
        rows.iter()
            .filter(|row| {
                row.operation() == MapReadOperationV1::MapLookup
                    && row.operand() == &MapReadOperandV1::Key("blocks".into())
            })
            .count(),
        2
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.operation() == MapReadOperationV1::ArrayLength)
            .count(),
        1
    );
}

#[test]
fn source_issues_one_aggregate_row_set_for_functions_params_and_blocks() {
    let package = issue(AGGREGATE_MAP_READ_SOURCE).expect("aggregate bounded map package");
    let rows = package.map_read_facts().rows();
    assert_eq!(rows.len(), 10);
    assert_eq!(
        rows.iter()
            .filter(|row| row.operation() == MapReadOperationV1::MapLookup)
            .count(),
        6
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.operation() == MapReadOperationV1::ArrayIndex)
            .count(),
        2
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.operation() == MapReadOperationV1::ArrayLength)
            .count(),
        2
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.result() == MapReadResultClassV1::ArrayView)
            .count(),
        4
    );
    assert!(rows.iter().all(|row| !row.containment().is_empty()));
}

#[test]
fn aggregate_rejects_unsupported_storage_before_catalog_mutation() {
    let cases = [
        (
            "params scalar",
            AGGREGATE_MAP_READ_SOURCE.replace("\"params\" => []", "\"params\" => 7"),
        ),
        (
            "params opaque array",
            AGGREGATE_MAP_READ_SOURCE.replace("\"params\" => []", "\"params\" => [7]"),
        ),
        (
            "params nested array",
            AGGREGATE_MAP_READ_SOURCE
                .replace("\"params\" => []", "\"params\" => [[function_row]]"),
        ),
        (
            "blocks scalar",
            AGGREGATE_MAP_READ_SOURCE.replace("\"blocks\" => [block]", "\"blocks\" => 7"),
        ),
        (
            "blocks mixed array",
            AGGREGATE_MAP_READ_SOURCE
                .replace("\"blocks\" => [block]", "\"blocks\" => [block, 7]"),
        ),
        (
            "blocks nested array",
            AGGREGATE_MAP_READ_SOURCE
                .replace("\"blocks\" => [block]", "\"blocks\" => [[block]]"),
        ),
        (
            "foreign params entry",
            AGGREGATE_MAP_READ_SOURCE.replace("\"params\" => []", "\"other\" => []"),
        ),
        (
            "foreign blocks entry",
            AGGREGATE_MAP_READ_SOURCE.replace("\"blocks\" => [block]", "\"other\" => [block]"),
        ),
        (
            "duplicate blocks length site",
            AGGREGATE_MAP_READ_SOURCE.replace(
                "local blocks_count = m.get(\"blocks\").length()\n        return",
                "local blocks_count = m.get(\"blocks\").length()\n        local blocks_count_again = m.get(\"blocks\").length()\n        return",
            ),
        ),
    ];
    for (label, source) in cases {
        let error = issue(&source).expect_err(label);
        match label {
            "params scalar" => assert!(matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::MapReadFact {
                    _error: MapReadFactIssueV1::ParamsEntryNotArray { .. }
                }
            )),
            "params opaque array" | "params nested array" => assert!(matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::MapReadFact {
                    _error: MapReadFactIssueV1::ParamsEntryUnsupported { .. }
                }
            )),
            "blocks scalar" => assert!(matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::MapReadFact {
                    _error: MapReadFactIssueV1::BlocksEntryNotArray { .. }
                }
            )),
            "blocks mixed array" | "blocks nested array" => assert!(matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::MapReadFact {
                    _error: MapReadFactIssueV1::BlocksEntryUnsupported { .. }
                }
            )),
            "foreign params entry" => assert!(matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::MapReadFact {
                    _error: MapReadFactIssueV1::ParamsEntryMissing { .. }
                }
            )),
            "foreign blocks entry" => assert!(matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::MapReadFact {
                    _error: MapReadFactIssueV1::BlocksEntryMissing { .. }
                }
            )),
            "duplicate blocks length site" => assert!(matches!(
                error,
                super::NormalCallableSemanticPackageIssueV1::MapReadFact {
                    _error: MapReadFactIssueV1::FirstLookupDuplicate { .. }
                }
            )),
            _ => unreachable!("case is exhaustively matched above"),
        }
    }
}

#[test]
fn blocks_length_rejects_empty_array() {
    let source =
        BLOCKS_LENGTH_READ_SOURCE.replace("%{ \"blocks\" => [block] }", "%{ \"blocks\" => [] }");
    let error = issue(&source).expect_err("empty blocks array must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::BlocksElementMissing { .. }
        }
    ));
}

#[test]
fn blocks_length_rejects_scalar_entry() {
    let source =
        BLOCKS_LENGTH_READ_SOURCE.replace("%{ \"blocks\" => [block] }", "%{ \"blocks\" => 7 }");
    let error = issue(&source).expect_err("scalar blocks entry must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::BlocksEntryNotArray { .. }
        }
    ));
}

#[test]
fn blocks_length_rejects_mixed_array() {
    let source = BLOCKS_LENGTH_READ_SOURCE.replace(
        "%{ \"blocks\" => [block] }",
        "%{ \"blocks\" => [block, 7] }",
    );
    let error = issue(&source).expect_err("mixed blocks array must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::BlocksEntryUnsupported { .. }
        }
    ));
}

#[test]
fn blocks_length_rejects_nested_array() {
    let source = BLOCKS_LENGTH_READ_SOURCE
        .replace("%{ \"blocks\" => [block] }", "%{ \"blocks\" => [[block]] }");
    let error = issue(&source).expect_err("nested blocks array must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::BlocksEntryUnsupported { .. }
        }
    ));
}

#[test]
fn blocks_kind_rejects_scalar_entry() {
    let source =
        BLOCKS_KIND_READ_SOURCE.replace("%{ \"blocks\" => [block] }", "%{ \"blocks\" => 7 }");
    let error = issue(&source).expect_err("scalar blocks entry must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::BlocksEntryNotArray { .. }
        }
    ));
}

#[test]
fn blocks_kind_rejects_mixed_array() {
    let source = BLOCKS_KIND_READ_SOURCE.replace(
        "%{ \"blocks\" => [block] }",
        "%{ \"blocks\" => [block, 7] }",
    );
    let error = issue(&source).expect_err("mixed blocks array must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::BlocksEntryUnsupported { .. }
        }
    ));
}

#[test]
fn blocks_kind_rejects_nested_array() {
    let source = BLOCKS_KIND_READ_SOURCE
        .replace("%{ \"blocks\" => [block] }", "%{ \"blocks\" => [[block]] }");
    let error = issue(&source).expect_err("nested blocks array must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::BlocksEntryUnsupported { .. }
        }
    ));
}

#[test]
fn blocks_kind_rejects_nonzero_index() {
    let source = BLOCKS_KIND_READ_SOURCE.replace("get(0)", "get(1)");
    let error = issue(&source).expect_err("nonzero blocks index must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::ArrayIndexOperandMismatch { .. }
        }
    ));
}

#[test]
fn blocks_kind_rejects_nonliteral_kind_key() {
    let source = BLOCKS_KIND_READ_SOURCE.replace(
        "local kind = m.get(\"blocks\").get(0).get(\"kind\")",
        "local key = \"kind\" local kind = m.get(\"blocks\").get(0).get(key)",
    );
    let error = issue(&source).expect_err("nonliteral kind key must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::KindEntryOperandMismatch { .. }
        }
    ));
}

#[test]
fn blocks_kind_rejects_nontext_child_value() {
    let source = BLOCKS_KIND_READ_SOURCE.replace("\"entry\"", "7");
    let error = issue(&source).expect_err("nontext kind value must reject");
    assert!(matches!(
        error,
        super::NormalCallableSemanticPackageIssueV1::MapReadFact {
            _error: MapReadFactIssueV1::KindValueNotText { .. }
        }
    ));
}
