use crate::parser::NyashParser;

use super::*;

const FIXTURE_ROOT: &str = "../../../../apps/bare-static-recovery-proof";

fn parse_catalog(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = NyashParser::parse_from_string(source).expect("bare-static fixture must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("bare-static fixture catalog must seal")
}

fn decide(
    source: &str,
    name: &str,
    arity: usize,
) -> Result<BareStaticRecoveryDecisionV1, BareStaticRecoveryDecisionErrorV1> {
    let catalog = parse_catalog(source);
    BareStaticRecoveryDecisionV1::decide(&catalog, name, arity)
}

fn unique_key(source: &str, name: &str, arity: usize) -> CanonicalSameModuleCallableKeyV1 {
    match decide(source, name, arity).unwrap() {
        BareStaticRecoveryDecisionV1::Unique(key) => key,
        other => panic!("expected Unique, got {other:?}"),
    }
}

fn fixture(name: &str) -> &'static str {
    match name {
        "provider_first_script" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "provider_first_script.hako"
        )),
        "caller_first_script" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "caller_first_script.hako"
        )),
        "provider_first_app" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "provider_first_app.hako"
        )),
        "caller_first_app" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "caller_first_app.hako"
        )),
        "cross_provider_first" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "cross_provider_first.hako"
        )),
        "cross_caller_first" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "cross_caller_first.hako"
        )),
        "ambiguous" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "ambiguous.hako"
        )),
        "instance_control" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "instance_control.hako"
        )),
        "arity_overload" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "arity_overload.hako"
        )),
        "zero_arg" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "zero_arg.hako"
        )),
        "wrong_arity" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "wrong_arity.hako"
        )),
        "no_candidate" => include_str!(concat!(
            "../../../../apps/bare-static-recovery-proof/",
            "no_candidate.hako"
        )),
        _ => panic!("unknown fixture {name}"),
    }
}

#[test]
fn same_box_provider_caller_order_and_root_mode_select_same_key() {
    let keys = [
        "provider_first_script",
        "caller_first_script",
        "provider_first_app",
        "caller_first_app",
    ]
    .map(|name| {
        let source = fixture(name);
        assert!(source.contains("return m_seed(x)"));
        unique_key(source, "m_seed", 1)
    });

    assert!(keys.iter().all(|key| key == &keys[0]));
    assert_eq!(keys[0].owner(), "Helpers");
    assert_eq!(keys[0].name(), "m_seed");
    assert_eq!(keys[0].arity(), 1);
}

#[test]
fn cross_box_declaration_order_selects_same_key() {
    let provider_first = unique_key(fixture("cross_provider_first"), "m_seed", 1);
    let caller_first = unique_key(fixture("cross_caller_first"), "m_seed", 1);
    assert_eq!(provider_first, caller_first);
    assert_eq!(provider_first.owner(), "Provider");
}

#[test]
fn zero_and_ambiguous_candidates_do_not_recover() {
    assert_eq!(
        decide(fixture("no_candidate"), "missing_seed", 1).unwrap(),
        BareStaticRecoveryDecisionV1::NoRecovery(BareStaticRecoveryNoRecoveryReasonV1::NoCandidate)
    );
    assert_eq!(
        decide(fixture("wrong_arity"), "m_seed", 1).unwrap(),
        BareStaticRecoveryDecisionV1::NoRecovery(BareStaticRecoveryNoRecoveryReasonV1::NoCandidate)
    );
    assert_eq!(
        decide(fixture("ambiguous"), "m_seed", 1).unwrap(),
        BareStaticRecoveryDecisionV1::NoRecovery(BareStaticRecoveryNoRecoveryReasonV1::Ambiguous {
            candidate_count: 2
        })
    );
}

#[test]
fn exact_arity_and_static_namespace_are_the_only_candidate_inputs() {
    let unary = unique_key(fixture("arity_overload"), "m_seed", 1);
    let binary = unique_key(fixture("arity_overload"), "m_seed", 2);
    assert_eq!(unary.owner(), "UnaryProvider");
    assert_eq!(binary.owner(), "BinaryProvider");

    let instance_control = unique_key(fixture("instance_control"), "m_seed", 1);
    assert_eq!(instance_control.owner(), "StaticProvider");
    assert_eq!(
        instance_control.namespace(),
        SameModuleCallableNamespaceV1::StaticBoxMethod
    );

    let zero = unique_key(fixture("zero_arg"), "m_seed", 0);
    assert_eq!(zero.owner(), "Helpers");
}

#[test]
fn checked_arity_overflow_rejects_before_candidate_lookup() {
    let Some(overflow) = usize::try_from(u64::from(u32::MAX) + 1).ok() else {
        return;
    };
    assert_eq!(
        decide(fixture("zero_arg"), "m_seed", overflow).unwrap_err(),
        BareStaticRecoveryDecisionErrorV1::ArityOverflow { arity: overflow }
    );
}

#[test]
fn text_merged_source_normalizes_to_the_same_catalog_decision() {
    let helper = include_str!(concat!(
        "../../../../apps/bare-static-recovery-proof/",
        "text_merged_helper.hako"
    ));
    let main = include_str!(concat!(
        "../../../../apps/bare-static-recovery-proof/",
        "text_merged_main.hako"
    ));
    assert!(main
        .lines()
        .next()
        .unwrap_or_default()
        .starts_with("using "));
    let stripped_main = main
        .lines()
        .filter(|line| !line.trim_start().starts_with("using "))
        .collect::<Vec<_>>()
        .join("\n");
    let merged = format!("{helper}\n{stripped_main}");
    let key = unique_key(&merged, "m_seed", 1);
    assert_eq!(key.owner(), "TextMergedHelpers");
}

#[test]
fn fixture_root_pointer_remains_stable() {
    assert_eq!(FIXTURE_ROOT, "../../../../apps/bare-static-recovery-proof");
}
