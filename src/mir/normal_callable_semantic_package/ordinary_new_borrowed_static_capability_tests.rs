//! Capability exclusion keeps every original row and declines the whole target.
use super::*;
fn package(
    text: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        text,
    )
    .unwrap()
}

#[test]
fn static_capability_unsupported_spelling_declines_whole_callee_and_keeps_sibling() {
    for actual in ["new Token()", "7 + 1"] {
        let text = format!("box Token {{}} static box Layout {{ pick(p) {{ return 0 }} ok(q) {{ return 0 }} }} static box Main {{ main() {{ local a = Layout.pick(7) local b = Layout.pick({actual}) local c = Layout.ok(9) return 0 }} }}");
        let package = package(&text);
        let source = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let rows: Vec<_> = source
            .source_incoming
            .exact_rows()
            .filter(|row| row.source.target().name() == "pick")
            .collect();
        assert_eq!(rows.len(), 2, "{actual}");
        let owner = rows[0].callee;
        assert!(
            source
                .source_incoming
                .has_unsupported_static_spelling(owner),
            "{actual}"
        );
        assert!(!source.definitions.contains_key(&owner));
        assert!(source.incoming.iter().all(|row| row.callee != owner));
        assert_eq!(source.source_incoming.static_observations().len(), 3);
        let good = source
            .source_incoming
            .exact_rows()
            .find(|row| row.source.target().name() == "ok")
            .unwrap();
        assert!(source.definitions.contains_key(&good.callee));
        assert_eq!(source.incoming.len(), 1);
    }
}

#[test]
fn static_capability_nonlocal_context_declines_whole_callee_and_keeps_sibling() {
    for suffix in ["Layout.pick(8) return 0", "return Layout.pick(8)"] {
        let text = format!("static box Layout {{ pick(p) {{ return 0 }} ok(q) {{ return 0 }} }} static box Main {{ main() {{ local a = Layout.pick(7) local c = Layout.ok(9) {suffix} }} }}");
        let package = package(&text);
        let source = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let rows: Vec<_> = source
            .source_incoming
            .exact_rows()
            .filter(|row| row.source.target().name() == "pick")
            .collect();
        assert_eq!(rows.len(), 2, "{suffix}");
        let owner = rows[0].callee;
        assert!(source.source_incoming.has_unsupported_static_context(owner));
        assert!(!source
            .source_incoming
            .has_unsupported_static_spelling(owner));
        assert!(!source.definitions.contains_key(&owner));
        assert!(source.incoming.iter().all(|row| row.callee != owner));
        assert_eq!(source.source_incoming.static_observations().len(), 3);
        let good = source
            .source_incoming
            .exact_rows()
            .find(|row| row.source.target().name() == "ok")
            .unwrap();
        assert!(source.definitions.contains_key(&good.callee));
        assert_eq!(source.incoming.len(), 1);
    }
}
