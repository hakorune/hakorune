//! Focused source-authority tests for the bounded nested Map read chain.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::map_read_fact::{MapReadOperandV1, MapReadOperationV1, MapReadResultClassV1};

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
