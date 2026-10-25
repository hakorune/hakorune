//! Core point-premise boundaries in the existing source-domain test owner.
use super::*;

#[test]
fn static_source_domain_core_point_premises_keep_original_reaching_definitions() {
    // Same original Core requests with point-sensitive bounds. These are
    // source loans, not physical Loop/Home/executable argument permissions.
    for (setup, actual, closed) in [
        ("local start = 0 local end = start end = end + 1", "q.substring(start,end)", true),
        ("local needle = \"\" + q + \"x\" local pos = q.indexOf(needle) local start = pos + needle.length() local end = start", "q.substring(start,end)", true),
        ("local start = 0 local end = start local limit = q.length() loop(end < limit) { local ch = q.substring(end,end + 1) if ch == \"x\" { break } end = end + 1 }", "q.substring(start,end)", true),
        ("local start = 0 local end = start loop(end < 0) { end = end + 1 }", "q.substring(start,end)", true),
        ("local start = 0 local end = start loop(end < 3) { if end == 1 { continue } end = end + 1 }", "q.substring(start,end)", true),
        ("local start = 0 local end = start loop(end < 3) { loop(end < 2) { end = end + 1 break } end = end + 1 }", "q.substring(start,end)", true),
        ("local start = 0 local end = 0 if q.length() == 0 { end = 1 } else { end = 2 }", "q.substring(start,end)", true),
        ("local start = 0 local end = start loop(end < 3) { end = \"bad\" break }", "q.substring(start,end)", false),
        ("local start = 0 local end = start loop(end < 3) { end = null break }", "q.substring(start,end)", false),
        ("local start = 0 local end = start loop(end < 3) { end = true break }", "q.substring(start,end)", false),
        ("local start = 0 local end if q.length() == 0 { end = 1 }", "q.substring(start,end)", false),
        ("local start = 0 local end = 0 loop(end < 3) { end = end + 1 } end = \"bad\"", "q.substring(start,end)", false),
        ("local start = 0 local end = 1 local saved = q.substring(start,end) end = \"bad\"", "saved", true),
        ("local start = 0 local end = 1 local capture = fn() { return end }", "q.substring(start,end)", false),
        ("local start = 0 local end = \"1\" + 1", "q.substring(start,end)", false),
    ] {
        let text = format!("static box Values {{ sink(p) {{ return 0 }} chain(q) {{ {setup} local result = Values.sink({actual}) return 0 }} unused() {{ local a = Values.chain(\"text\") return 0 }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let source = ingress(&package);
        if setup.contains("fn()") {
            let call = source.source_incoming.source_rows_v1().find(|row| row.source.target().name() == "chain")
                .expect("original selected source reservation");
            assert!(package.ordinary_new_claim_ledger.selected_static_local_source_v1(&call.call).is_err(),
                "passive arguments must not grant a captured callee an executable packet");
        }
        let mut checked = 0;
        for declaration in package.batch().declarations() {
            package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                for (site, method) in input.function().method_calls().filter(|(_, method)| method.selector() == "substring") {
                    // This cache follows original actual/New demands, not
                    // every Core call. The scan's local ch feeds only control;
                    // its execution remains owned by Loop Facts/Recipe.
                    if site.node().segments().iter().any(|role| matches!(role,
                        crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_))) { continue; }
                    let receipt = source.closed_core_source_v1(&OwnedExprSiteV1::new(input.owner(), site.clone()));
                    assert_eq!(receipt.is_some(), closed, "{setup}; {actual}; {site:?}");
                    if let Some(receipt) = receipt {
                        assert!(std::sync::Arc::ptr_eq(receipt.method(), input.function().method_call_source_v1(site).unwrap()));
                        assert_eq!(receipt.manifest().arity(), method.arity());
                    }
                    checked += 1;
                }
            }).unwrap();
        }
        assert_eq!(checked, 1);
        assert!(!source.checked_static_input(formal(&package, "Values", "sink")));
    }
}
