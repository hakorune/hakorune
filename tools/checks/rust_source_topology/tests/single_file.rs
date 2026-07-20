use rust_source_topology_check::{
    extract_single_file_source, DirectCallExpressionKindV1, DirectCallResolutionV1,
    DirectCallUnresolvedReasonV1, ExtractErrorV1, LexicalContextKindV1, OpaqueSyntaxKindV1,
};

const FIXTURE_PATH: &str = "tests/fixtures/single_file.rs";
const MODULE_PATH: &str = "fixture::single_file";

#[test]
fn fixture_has_deterministic_neutral_shape() {
    let source = include_str!("fixtures/single_file.rs");
    let first = extract_single_file_source(FIXTURE_PATH, MODULE_PATH, source).unwrap();
    let second = extract_single_file_source(FIXTURE_PATH, MODULE_PATH, source).unwrap();

    assert_eq!(first, second);
    assert_eq!(first.schema, "rust-source-topology-v1");
    assert_eq!(first.schema_version, 1);
    assert_eq!(first.source_file.items.len(), 13);
    assert_eq!(first.source_file.direct_call_sites.len(), 13);
    assert_eq!(first.source_file.unresolved_call_sites.len(), 13);
    assert_eq!(first.source_file.opaque_syntax_sites.len(), 4);
    assert!(first
        .source_file
        .direct_call_sites
        .iter()
        .all(|call| matches!(call.resolution, DirectCallResolutionV1::Unresolved { .. })));
}

#[test]
fn ranges_roundtrip_and_call_kinds_remain_distinct() {
    let source = include_str!("fixtures/single_file.rs");
    let topology = extract_single_file_source(FIXTURE_PATH, MODULE_PATH, source).unwrap();
    let calls = &topology.source_file.direct_call_sites;

    for call in calls {
        assert_eq!(
            source
                .get(call.source_range.byte_start..call.source_range.byte_end)
                .unwrap(),
            call.source_text
        );
        assert!(!call.source_text.is_empty());
    }
    assert_eq!(
        calls
            .iter()
            .filter(|call| call.expression_kind == DirectCallExpressionKindV1::ExprMethodCall)
            .count(),
        1
    );
    assert_eq!(
        calls
            .iter()
            .filter(|call| call.expression_kind == DirectCallExpressionKindV1::ExprCall)
            .count(),
        12
    );
}

#[test]
fn cfg_lexical_and_opaque_evidence_are_not_silently_dropped() {
    let source = include_str!("fixtures/single_file.rs");
    let topology = extract_single_file_source(FIXTURE_PATH, MODULE_PATH, source).unwrap();
    let calls = &topology.source_file.direct_call_sites;

    let nested = calls
        .iter()
        .find(|call| call.normalized_callee_syntax == "crate::nested_call")
        .unwrap();
    assert_eq!(nested.inherited_cfg_syntax.len(), 2);
    let closure_call = calls
        .iter()
        .find(|call| call.normalized_callee_syntax == "crate::inside")
        .unwrap();
    assert_eq!(closure_call.lexical_context.len(), 1);
    assert_eq!(
        closure_call.lexical_context[0].kind,
        LexicalContextKindV1::Closure
    );
    let async_call = calls
        .iter()
        .find(|call| call.normalized_callee_syntax == "crate::later")
        .unwrap();
    assert_eq!(
        async_call.lexical_context[0].kind,
        LexicalContextKindV1::AsyncBlock
    );

    let opaque = &topology.source_file.opaque_syntax_sites;
    assert!(opaque
        .iter()
        .any(|site| site.kind == OpaqueSyntaxKindV1::IncludeMacro));
    assert!(opaque
        .iter()
        .any(|site| site.kind == OpaqueSyntaxKindV1::MacroInvocation));
    assert!(opaque
        .iter()
        .any(|site| site.kind == OpaqueSyntaxKindV1::ExternalModule));
    assert!(opaque
        .iter()
        .any(|site| site.kind == OpaqueSyntaxKindV1::PathAttributedExternalModule));
}

#[test]
fn unresolved_reasons_are_typed_and_parse_failure_is_not_empty_success() {
    let source = include_str!("fixtures/single_file.rs");
    let topology = extract_single_file_source(FIXTURE_PATH, MODULE_PATH, source).unwrap();
    assert!(topology
        .source_file
        .unresolved_call_sites
        .iter()
        .any(|site| { site.reason == DirectCallUnresolvedReasonV1::GeneralReceiverInference }));
    assert!(topology
        .source_file
        .unresolved_call_sites
        .iter()
        .any(|site| { site.reason == DirectCallUnresolvedReasonV1::UnsupportedCalleeExpression }));
    assert!(matches!(
        extract_single_file_source("broken.rs", "fixture::broken", "fn {").unwrap_err(),
        ExtractErrorV1::Parse { .. }
    ));
}

#[test]
fn varied_call_syntax_remains_observed_without_resolution_guessing() {
    let source = r#"
fn calls<T: Trait>(fp: fn(), object: Holder) {
    crate::m::f();
    Type::new();
    local::<T>();
    <T as Trait>::run();
    (fp)();
    (|| crate::inside())();
    object.method::<T>();
    make().method();
}
"#;
    let topology = extract_single_file_source("calls.rs", "fixture::calls", source).unwrap();
    assert_eq!(topology.source_file.direct_call_sites.len(), 10);
    assert!(topology
        .source_file
        .unresolved_call_sites
        .iter()
        .any(|site| { site.reason == DirectCallUnresolvedReasonV1::ResolutionDeferredToS0c }));
    assert!(topology
        .source_file
        .unresolved_call_sites
        .iter()
        .any(|site| { site.reason == DirectCallUnresolvedReasonV1::IndirectFunctionValue }));
    assert!(topology
        .source_file
        .unresolved_call_sites
        .iter()
        .any(|site| { site.reason == DirectCallUnresolvedReasonV1::ClosureInvocation }));
    assert_eq!(
        topology
            .source_file
            .direct_call_sites
            .iter()
            .filter(|call| call.expression_kind == DirectCallExpressionKindV1::ExprMethodCall)
            .count(),
        2
    );
}

#[test]
fn unicode_crlf_and_duplicate_callees_keep_exact_distinct_ranges() {
    let source = "fn run() {\r\n    let café = 1; crate::same(café); crate::same(café);\r\n}\r\n";
    let topology = extract_single_file_source("ranges.rs", "fixture::ranges", source).unwrap();
    let calls = &topology.source_file.direct_call_sites;
    assert_eq!(calls.len(), 2);
    assert_ne!(calls[0].call_site_id, calls[1].call_site_id);
    assert_ne!(calls[0].source_range, calls[1].source_range);
    for call in calls {
        assert_eq!(
            &source[call.source_range.byte_start..call.source_range.byte_end],
            call.source_text
        );
    }
}

#[test]
fn missing_identity_inputs_fail_with_stable_variants() {
    assert_eq!(
        extract_single_file_source("", MODULE_PATH, "fn run() {}").unwrap_err(),
        ExtractErrorV1::EmptyPath
    );
    assert_eq!(
        extract_single_file_source("sample.rs", "", "fn run() {}").unwrap_err(),
        ExtractErrorV1::EmptyModuleSyntaxPath
    );
}
