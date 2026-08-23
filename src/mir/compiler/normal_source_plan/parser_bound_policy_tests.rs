use super::rejection::NormalUnsupportedTopLevelKindV1;
use super::{
    NormalSourcePlanClassifierV1, NormalSourcePlanErrorV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1,
};
use crate::mir::CanonicalSourceBytesDigestV1;
use crate::parser::{
    NormalParserSourceLineageV1, NyashParser, ParserBuildConfig,
    ParserNormalRootSourcePlanConsumerV1,
};

fn classify(source: &str) -> Result<SealedNormalSourcePlanV1, super::RejectedNormalSourcePlanV1> {
    let product = NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("parser product");
    let lineage = NormalParserSourceLineageV1::issue(
        "parser-bound-policy-test",
        CanonicalSourceBytesDigestV1::from_utf8_bytes(source.as_bytes()),
        hakorune_frontend_parser::parser::GrammarProfile::Canonical,
        source.len(),
        1,
        1,
    )
    .expect("lineage");
    let bound = ParserNormalRootSourcePlanConsumerV1::consume_for_test(product, lineage)
        .expect("ready parser-bound source");
    NormalSourcePlanClassifierV1::seal_parser_bound(bound)
}

#[test]
fn parser_bound_empty_and_executable_sources_are_script() {
    for source in ["", "print(1)\n"] {
        let plan = classify(source).expect("Script plan");
        assert!(matches!(
            &plan,
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
        ));
        plan.discard_before_dispatch();
    }
}

#[test]
fn parser_bound_static_main_zero_is_main0() {
    let plan = classify("static box Main { main() { return 1 } }\n").expect("Main0 plan");
    assert!(matches!(
        &plan,
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(_))
    ));
    plan.discard_before_dispatch();
}

#[test]
fn parser_bound_main_helpers_and_top_level_callable_form_one_module() {
    let plan = classify(
        "static box Main { main() { return 1 } helper() { return 2 } }\nfunction free() {}\n",
    )
    .expect("callable module");
    assert!(matches!(&plan, SealedNormalSourcePlanV1::CallableModule(_)));
    plan.discard_before_dispatch();
}

#[test]
fn parser_bound_non_static_main_is_policy_rejected() {
    let rejected = classify("box Main { main() { return 1 } }\n")
        .expect_err("non-static Main must not become Script");
    assert_eq!(rejected.error(), &NormalSourcePlanErrorV1::MainMustBeStatic);
    rejected.discard();
}

#[test]
fn parser_bound_mixed_app_and_executable_source_is_rejected() {
    let rejected = classify("static box Main { main() { return 1 } }\nprint(1)\n")
        .expect_err("mixed source families");
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MixedSourceFamilies
    );
    rejected.discard();
}

#[test]
fn parser_bound_non_main_box_is_unsupported_not_script() {
    let rejected = classify("box Plain { run() { return 1 } }\n")
        .expect_err("ordinary Box is outside the source-plan cohort");
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::UnsupportedTopLevelSurface {
            statement_index: 0,
            kind: NormalUnsupportedTopLevelKindV1::Box,
        }
    );
    rejected.discard();
}

#[test]
fn parser_bound_main_arity_is_checked_by_policy_once() {
    let rejected =
        classify("static box Main { main(x) { return x } }\n").expect_err("main/N is not Main0");
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MainArityMismatch { actual: 1 }
    );
    rejected.discard();
}

#[test]
fn parser_bound_main_member_coverage_is_checked_by_policy_once() {
    let rejected = classify("static box Main { field main() { return 1 } }\n")
        .expect_err("field plus main method must retain complete member coverage");
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MainMemberCoverageMismatch {
            observed: 2,
            callable: 1,
        }
    );
    rejected.discard();
}

#[test]
fn parser_bound_top_level_callable_without_main_is_missing_entry() {
    let rejected = classify("function free() {}\n").expect_err("missing Main entry");
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MissingSourceEntry
    );
    rejected.discard();
}
