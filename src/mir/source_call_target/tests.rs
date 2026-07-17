use crate::mir::builder::SameModuleCallableNamespaceV1;

use super::test_support::*;
use super::*;

const QUALIFIED_SOURCE: &str = r#"
static box Helpers {
  run(x) { return x }
  zero() { return 0 }
}
static box Other { run(x) { return x } }
static box Caller {
  direct(x) { return Helpers.run(x) }
  imported(Alias, x) { return Alias.run(x) }
  wrong(x) { return Helpers.run(x, x) }
  missing(x) { return Helpers.absent(x) }
}
"#;

fn static_key(
    declarations: &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &str,
    method: &str,
    arity: usize,
) -> crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
    key(
        declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        owner,
        method,
        arity,
    )
}

#[test]
fn seals_direct_canonical_qualified_target_from_exact_route_facts() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);
    let caller = static_key(&declarations, "Caller", "direct", 1);
    let call_site = return_site();
    let targets = seal_one_qualified(&declarations, &imports, &caller, call_site.clone());

    let row = qualified(&targets, &caller, &call_site);
    assert_eq!(row.target().owner(), "Helpers");
    assert_eq!(row.target().name(), "run");
    assert_eq!(row.target().arity(), 1);
    assert_eq!(
        row.receiver(),
        &QualifiedStaticReceiverV1::UnshadowedCanonicalOwner {
            canonical_owner: "Helpers".into(),
        }
    );
}

#[test]
fn imported_alias_precedes_same_spelled_lexical_binding() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("Alias".to_string(), "Helpers".to_string())],
    )
    .unwrap();
    let caller = static_key(&declarations, "Caller", "imported", 2);
    let call_site = return_site();
    let targets = seal_one_qualified(&declarations, &imports, &caller, call_site.clone());

    assert_eq!(imports.len(), 1);
    assert_eq!(
        qualified(&targets, &caller, &call_site).receiver(),
        &QualifiedStaticReceiverV1::ImportedAlias {
            source_alias: "Alias".into(),
            canonical_owner: "Helpers".into(),
        }
    );
}

#[test]
fn exact_wrong_arity_and_missing_target_reject_at_target_lookup() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);

    let wrong_caller = static_key(&declarations, "Caller", "wrong", 1);
    let wrong_call = exact_call(&declarations, &wrong_caller, return_site());
    let wrong_lexical =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&wrong_call]).unwrap();
    let wrong_facts =
        VerifiedQualifiedCallRouteFactsV1::verify(&wrong_call, &wrong_lexical, &imports).unwrap();
    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, [wrong_facts])
            .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::TargetOutsideCatalog {
            receiver: "Helpers".into(),
            canonical_owner: "Helpers".into(),
            method: "run".into(),
            arity: 2,
        }
    );

    let missing_caller = static_key(&declarations, "Caller", "missing", 1);
    let missing_call = exact_call(&declarations, &missing_caller, return_site());
    let missing_lexical =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&missing_call]).unwrap();
    let missing_facts =
        VerifiedQualifiedCallRouteFactsV1::verify(&missing_call, &missing_lexical, &imports)
            .unwrap();
    assert!(matches!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, [missing_facts]),
        Err(QualifiedStaticCallTargetErrorV1::TargetOutsideCatalog { method, .. })
            if &*method == "absent"
    ));
}

#[test]
fn duplicate_exact_route_fact_site_rejects() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);
    let caller = static_key(&declarations, "Caller", "direct", 1);
    let call = exact_call(&declarations, &caller, return_site());
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    let first = VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &imports).unwrap();
    let second = VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &imports).unwrap();

    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, [first, second])
            .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::DuplicateCallSite {
            caller,
            site: return_site(),
        }
    );
}

#[test]
fn exact_import_view_instance_is_part_of_qualified_seal() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let first_imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("Alias".to_string(), "Helpers".to_string())],
    )
    .unwrap();
    let second_imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("Alias".to_string(), "Other".to_string())],
    )
    .unwrap();
    let caller = static_key(&declarations, "Caller", "imported", 2);
    let call = exact_call(&declarations, &caller, return_site());
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    let facts = VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &first_imports).unwrap();

    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&second_imports, [facts])
            .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::RouteFactImportViewMismatch {
            caller,
            site: return_site(),
        }
    );
}

#[test]
fn route_facts_from_equal_foreign_catalog_reject_by_identity() {
    let left = catalog(QUALIFIED_SOURCE);
    let right = catalog(QUALIFIED_SOURCE);
    let left_imports = empty_imports(&left);
    let right_imports = empty_imports(&right);
    let caller = static_key(&left, "Caller", "direct", 1);
    let call = exact_call(&left, &caller, return_site());
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    let facts = VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &left_imports).unwrap();

    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&right_imports, [facts])
            .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::RouteFactCatalogMismatch {
            caller,
            site: return_site(),
        }
    );
}

#[test]
fn import_view_rejects_duplicate_and_foreign_owners() {
    let declarations = catalog(QUALIFIED_SOURCE);
    assert!(matches!(
        VerifiedStaticImportAliasViewV1::seal(
            &declarations,
            [
                ("Alias".to_string(), "Helpers".to_string()),
                ("Alias".to_string(), "Helpers".to_string()),
            ]
        ),
        Err(StaticImportAliasViewErrorV1::DuplicateAlias { .. })
    ));
    assert!(matches!(
        VerifiedStaticImportAliasViewV1::seal(
            &declarations,
            [("Alias".to_string(), "Missing".to_string())]
        ),
        Err(StaticImportAliasViewErrorV1::TargetOwnerOutsideCatalog { .. })
    ));
}

#[test]
fn actual_parser_wrapper_projects_import_alias_to_string_helpers() {
    let source = format!(
        "{}\n{}",
        include_str!("../../../lang/src/shared/common/string_helpers.hako"),
        include_str!("../../../lang/src/compiler/parser/scan/parser_string_utils_box.hako"),
    );
    let declarations = catalog(&source);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("StringHelpers".to_string(), "StringHelpers".to_string())],
    )
    .unwrap();
    let caller = static_key(&declarations, "ParserStringUtilsBox", "skip_ws", 2);
    let call_site = return_site();
    let targets = seal_one_qualified(&declarations, &imports, &caller, call_site.clone());

    let row = qualified(&targets, &caller, &call_site);
    assert_eq!(row.target().owner(), "StringHelpers");
    assert_eq!(row.target().name(), "skip_ws");
    assert_eq!(row.target().arity(), 2);
}

#[test]
fn declaration_reorder_preserves_normalized_target_rows() {
    let reordered = QUALIFIED_SOURCE.replacen(
        "static box Helpers {\n  run(x) { return x }\n  zero() { return 0 }\n}\n",
        "",
        1,
    ) + "\nstatic box Helpers { run(x) { return x } zero() { return 0 } }\n";

    fn normalized(source: &str) -> Vec<(String, String, u32)> {
        let declarations = catalog(source);
        let imports = empty_imports(&declarations);
        let caller = static_key(&declarations, "Caller", "direct", 1);
        seal_one_qualified(&declarations, &imports, &caller, return_site())
            .rows()
            .map(|(_, row)| match row {
                VerifiedSourceStaticCallTargetV1::QualifiedStatic(row) => (
                    row.target().owner().to_string(),
                    row.target().name().to_string(),
                    row.target().arity(),
                ),
                VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(_) => unreachable!(),
            })
            .collect()
    }

    assert_eq!(normalized(QUALIFIED_SOURCE), normalized(&reordered));
}
