use super::*;
use crate::mir::normal_source_plan::{
    NormalSourcePlanClassifierV1, NormalSourcePlanErrorV1, NormalSourcePlanStageV1,
    PreparedNormalSourcePlanInputV1, SealedNormalScalarRootV1, SealedNormalSourcePlanV1,
};
use crate::runner::reference::normal_file_vm_frontdoor::NormalFileSourceReceiptSealV1;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn request(path: PathBuf) -> super::super::NormalFileRequestV1 {
    super::super::NormalFileVmFrontDoorV1::file_no_import_request(path)
}

fn canonical_core_request(path: PathBuf) -> super::super::NormalFileRequestV1 {
    super::super::NormalFileVmFrontDoorV1::file_canonical_core_request(path)
}

fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, source).expect("write source-plan fixture");
    path
}

#[test]
fn ast_only_source_plan_fixture_is_not_parser_backed() {
    let ast = crate::parser::NyashParser::parse_from_string_with_build_config(
        "42",
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("AST-only fixture parse");
    let input = PreparedNormalSourcePlanInputV1::new(ast, "ast-only-fixture");
    assert!(!input.has_parser_postpass());
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("AST-only fixture plan");
    assert!(matches!(
        plan,
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
    ));
}

fn classify(
    dir: &Path,
    name: &str,
    source: &str,
) -> Result<ClassifiedNormalFileSourcePlanV1, RejectedNormalFileSourcePlanningV1> {
    request(write_source(dir, name, source))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request()
        .classify()
}

fn classify_canonical_core(
    dir: &Path,
    name: &str,
    source: &str,
) -> ClassifiedNormalFileSourcePlanV1 {
    canonical_core_request(write_source(dir, name, source))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request()
        .classify()
        .expect("canonical-core source plan")
}

#[test]
fn parsed_empty_and_scalar_sources_become_script_plans_once() {
    let dir = tempdir().expect("tempdir");
    for (name, source) in [("empty.hako", ""), ("scalar.hako", "42")] {
        let classified = classify(dir.path(), name, source).expect("Script source plan");
        assert!(matches!(
            classified.plan(),
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
        ));
        assert!(classified.retains_parser_postpass());
        let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(script)) =
            classified.plan()
        else {
            panic!("expected Script source plan");
        };
        assert!(script.parser_postpass().is_some());
        assert_eq!(classified.receipt_counts(), (1, 1));
        assert!(classified.retained_source_identity().ends_with(name));
    }
}

#[test]
fn parser_lineage_is_borrowed_from_the_sealed_script_plan() {
    let dir = tempdir().expect("tempdir");
    let classified = classify(dir.path(), "lineage.hako", "42").expect("Script source plan");
    let lineage = classified
        .plan()
        .parser_lineage()
        .expect("parser-backed plan retains lineage");
    assert!(lineage.source_identity().ends_with("lineage.hako"));
    assert_eq!(lineage.utf8_len(), 2);
    assert_eq!(lineage.receipt_counts(), (1, 1));
}

#[test]
fn parser_lineage_digest_drift_rejects_before_source_classifier() {
    let dir = tempdir().expect("tempdir");
    let loaded = request(write_source(dir.path(), "drift.hako", "42"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse");
    let mut plan_request = loaded.prepare_source_plan_request();
    plan_request.receipt.source_digest =
        crate::mir::CanonicalSourceBytesDigestV1::from_utf8_bytes(b"foreign-source");
    let rejected = plan_request
        .classify()
        .expect_err("lineage/receipt digest drift must reject");
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::RootSurface);
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::SourceIdentityMismatch {
            field: crate::mir::normal_source_plan::NormalSourcePlanIdentityFieldV1::Digest,
        }
    );
    rejected.discard();
}

#[test]
fn ast_only_source_plan_request_has_no_canonical_lineage_authority() {
    let ast = crate::parser::NyashParser::parse_from_string_with_build_config(
        "42",
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("AST-only fixture parse");
    let request = PreparedNormalFileSourcePlanRequestV1 {
        input: PreparedNormalSourcePlanInputV1::new(ast, "ast-only-fixture"),
        script_input: CanonicalScriptSourceInputDispositionV1::SourceAuthorityUnavailable,
        profile: SealedNormalEntryProfileV1::file_no_import_vm_reference(),
        receipt: NormalFileSourceReceiptV1 {
            source_identity: "ast-only-fixture".into(),
            source_digest: crate::mir::CanonicalSourceBytesDigestV1::from_utf8_bytes(b"42"),
            utf8_len: 2,
            read_count: 1,
            parse_count: 1,
            _seal: NormalFileSourceReceiptSealV1,
        },
        _seal: PreparedNormalFileSourcePlanRequestSealV1,
    };
    let rejected = request
        .classify()
        .expect_err("AST-only input cannot enter canonical file planning");
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::RootSurface);
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::SourceAuthorityUnavailable
    );
    rejected.discard();
}

#[test]
fn canonical_core_profile_reaches_the_same_one_read_one_parse_source_plan_boundary() {
    let dir = tempdir().expect("tempdir");
    let classified = canonical_core_request(write_source(dir.path(), "core-script.hako", "42"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request()
        .classify()
        .expect("Script source plan");
    assert!(matches!(
        classified.plan(),
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(_))
    ));
    assert!(classified.retains_parser_postpass());
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(script)) =
        classified.plan()
    else {
        panic!("expected Script source plan");
    };
    assert!(script.parser_postpass().is_some());
    assert_eq!(classified.receipt_counts(), (1, 1));
    assert!(classified.is_canonical_core_profile_for_test());
}

#[test]
fn source_digest_is_issued_once_and_moves_into_canonical_request() {
    let dir = tempdir().expect("tempdir");
    let path = write_source(dir.path(), "digest.hako", "42");
    let loaded = canonical_core_request(path.clone())
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read");
    std::fs::write(&path, "43").expect("rewrite after read");
    let classified = loaded
        .parse_once()
        .expect("parse retained bytes")
        .prepare_source_plan_request()
        .classify()
        .expect("source plan");
    let expected = crate::mir::CanonicalSourceBytesDigestV1::from_utf8_bytes(b"42");
    assert_eq!(classified.source_digest(), expected);
    let request = classified
        .into_canonical_core_compile_request()
        .expect("canonical handoff");
    assert_eq!(request.source_digest(), expected);
}

#[test]
fn canonical_core_dispatch_script_handoff_moves_only_the_sealed_plan_and_receipt() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(dir.path(), "handoff.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical-core handoff");
    assert_eq!(request.script_input_state(), "HandoffReady");
    let mut compiler = crate::mir::MirCompiler::new();
    let candidate = compiler
        .compile_canonical_core_source_plan(request)
        .expect("Script candidate through the canonical physical entry");
    assert!(candidate.is_script());
    assert_eq!(candidate.receipt_counts(), (1, 1));
}

#[test]
fn canonical_core_script_candidate_retains_sealed_publication_evidence() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(dir.path(), "evidence.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical-core Script handoff");
    let mut compiler = crate::mir::MirCompiler::new();
    let candidate = compiler
        .compile_canonical_core_source_plan(request)
        .expect("unpublished Script candidate");
    let evidence = candidate
        .script_candidate_evidence_for_test()
        .expect("Script evidence");

    assert!(evidence.target_is_main);
    assert_eq!(evidence.target_symbol, "main");
    assert_eq!(evidence.target_arity, 0);
    assert!(evidence.source_identity.ends_with("evidence.hako"));
    assert_eq!(evidence.schema_row_count, 1);
    assert_eq!(evidence.result_kind, "integer");
    assert_eq!(evidence.verification_function_count, 1);
    assert_eq!(evidence.module_function_count, 1);
}

#[test]
fn canonical_core_publication_projects_main_and_script_without_reobservation() {
    let dir = tempdir().expect("tempdir");
    let script = classify_canonical_core(dir.path(), "published-script.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical Script handoff");
    let main = classify_canonical_core(
        dir.path(),
        "published-main.hako",
        "static box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical Main handoff");
    let mut compiler = crate::mir::MirCompiler::new();

    let script = compiler
        .compile_canonical_core_source_plan_publication_summary_for_test(script)
        .expect("Script publication");
    assert_eq!(script.target_symbol, "main");
    assert_eq!(script.target_arity, 0);
    assert_eq!(script.result_kind, "integer");
    assert_eq!(script.family, "script");

    let main = compiler
        .compile_canonical_core_source_plan_publication_summary_for_test(main)
        .expect("Main publication");
    assert_eq!(main.target_symbol, "main");
    assert_eq!(main.target_arity, 0);
    assert_eq!(main.result_kind, "unit");
    assert_eq!(main.family, "main");
}

#[cfg(feature = "vm-reference")]
#[test]
fn canonical_core_vm_reference_executes_script_and_main_through_one_published_owner() {
    let dir = tempdir().expect("tempdir");
    let script = classify_canonical_core(dir.path(), "vm-script.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical Script handoff");
    let main = classify_canonical_core(dir.path(), "vm-main.hako", "static box Main { main() {} }")
        .into_canonical_core_compile_request()
        .expect("canonical Main handoff");
    let mut compiler = crate::mir::MirCompiler::new();

    let script = compiler
        .run_canonical_core_source_plan_vm_reference_summary_for_test(script)
        .expect("Script VM execution");
    assert_eq!(script.status, 42);
    assert_eq!(script.fault_tag, None);
    assert_eq!(script.route, "script");

    let main = compiler
        .run_canonical_core_source_plan_vm_reference_summary_for_test(main)
        .expect("Main VM execution");
    assert_eq!(main.status, 0);
    assert_eq!(main.fault_tag, None);
    assert_eq!(main.route, "main");
}

#[cfg(feature = "vm-reference")]
#[test]
fn canonical_core_script_vm_reference_preserves_unit_and_fault_projection() {
    let dir = tempdir().expect("tempdir");
    let cases = [
        ("empty.hako", "", 0, None),
        ("void.hako", "void", 0, None),
        ("null.hako", "null", 0, None),
        ("print.hako", "print(1)", 0, None),
        ("local.hako", "local x = 3", 0, None),
        ("assignment.hako", "local x = 1\nx = 3", 0, None),
        ("compound.hako", "local x = 1\nx += 2", 0, None),
        ("range.hako", "256", 70, Some("exit-code-out-of-range")),
        ("bool.hako", "true", 70, Some("unsupported-result")),
        ("float.hako", "1.5", 70, Some("unsupported-result")),
        ("string.hako", "\"nyan\"", 70, Some("unsupported-result")),
        ("division.hako", "1 / 0", 70, Some("vm-division-by-zero")),
    ];
    let mut compiler = crate::mir::MirCompiler::new();

    for (name, source, expected_status, expected_fault) in cases {
        let request = classify_canonical_core(dir.path(), name, source)
            .into_canonical_core_compile_request()
            .expect("canonical Script handoff");
        let outcome = compiler
            .run_canonical_core_source_plan_vm_reference_summary_for_test(request)
            .expect("Script VM execution");
        assert_eq!(outcome.status, expected_status, "{name}");
        assert_eq!(outcome.fault_tag, expected_fault, "{name}");
        assert_eq!(outcome.route, "script", "{name}");
    }
}

#[test]
fn canonical_core_dispatch_script_candidate_preserves_compiler_reuse_for_main() {
    let dir = tempdir().expect("tempdir");
    let script = classify_canonical_core(dir.path(), "script-reuse.hako", "42")
        .into_canonical_core_compile_request()
        .expect("canonical Script handoff");
    let main = classify_canonical_core(
        dir.path(),
        "main-reuse.hako",
        "static box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical Main handoff");

    let mut compiler = crate::mir::MirCompiler::new();
    let script_candidate = compiler
        .compile_canonical_core_source_plan(script)
        .expect("unpublished Script candidate");
    assert!(script_candidate.is_script());
    let main_candidate = compiler
        .compile_canonical_core_source_plan(main)
        .expect("unpublished Main candidate after Script");
    assert!(main_candidate.is_main());
}

#[test]
fn narrow_profile_cannot_enter_the_canonical_core_dispatch_handoff() {
    let dir = tempdir().expect("tempdir");
    let rejected = classify(dir.path(), "narrow-handoff.hako", "42")
        .expect("narrow source plan")
        .into_canonical_core_compile_request()
        .expect_err("narrow profile is not canonical-core");
    assert_eq!(
        rejected.error(),
        CanonicalCoreSourcePlanHandoffErrorV1::ProfileExcludesCanonicalCore
    );
    rejected.discard();
}

#[test]
fn canonical_core_dispatch_builds_only_main0_candidate_in_s0() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(
        dir.path(),
        "main-dispatch.hako",
        "static box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core handoff");
    assert_eq!(request.script_input_state(), "CompatibilitySource");
    let mut compiler = crate::mir::MirCompiler::new();
    let candidate = compiler
        .compile_canonical_core_source_plan(request)
        .expect("unpublished Main candidate");
    assert!(candidate.is_main());
    assert_eq!(candidate.receipt_counts(), (1, 1));
}

#[test]
fn canonical_core_dispatch_connects_callable_to_the_shared_publication_path() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(
        dir.path(),
        "callable-dispatch.hako",
        "static function helper(x: i64): i64 { return x }\nstatic box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core handoff");
    let mut compiler = crate::mir::MirCompiler::new();
    let candidate = compiler
        .compile_canonical_core_source_plan(request)
        .expect("Callable candidate through its sealed transaction");
    assert!(candidate.is_callable());
    assert_eq!(candidate.receipt_counts(), (1, 1));

    let published = classify_canonical_core(
        dir.path(),
        "callable-publication.hako",
        "static function helper(x: i64): i64 { return x }\nstatic box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core callable handoff");
    let published = compiler
        .compile_canonical_core_source_plan_publication_summary_for_test(published)
        .expect("Callable publication through the shared core");
    assert_eq!(published.family, "callable");
    assert_eq!(published.target_symbol, "main");
    assert_eq!(published.target_arity, 0);
    assert_eq!(published.result_kind, "unit");
}

#[test]
fn canonical_core_callable_direct_call_rejects_at_its_existing_preflight_and_reuses_compiler() {
    let dir = tempdir().expect("tempdir");
    let direct_call = classify_canonical_core(
        dir.path(),
        "callable-direct-call.hako",
        "static function helper(x: i64): i64 { return x }\nstatic box Main { main() { helper(42) } }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core callable handoff");
    let later = classify_canonical_core(
        dir.path(),
        "callable-later.hako",
        "static function helper(x: i64): i64 { return x }\nstatic box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core callable handoff");
    let mut compiler = crate::mir::MirCompiler::new();

    let rejected = compiler
        .compile_canonical_core_source_plan(direct_call)
        .expect_err("Main direct calls remain outside the first callable slice");
    assert_eq!(
        rejected.stage(),
        crate::mir::CanonicalCoreDispatchStageV1::Callable
    );
    assert!(matches!(
        rejected.cause(),
        crate::mir::CanonicalCoreDispatchErrorV1::Callable(
            crate::mir::CanonicalCallableDispatchStageV1::MainPlan
        )
    ));
    assert_eq!(rejected.receipt_counts(), (1, 1));
    rejected.discard();

    let later = compiler
        .compile_canonical_core_source_plan(later)
        .expect("typed direct-call rejection leaves compiler reusable");
    assert!(later.is_callable());
}

#[cfg(feature = "vm-reference")]
#[test]
fn canonical_core_vm_reference_executes_admitted_callable_module() {
    let dir = tempdir().expect("tempdir");
    let request = classify_canonical_core(
        dir.path(),
        "callable-vm.hako",
        "static function helper(x: i64): i64 { return x }\nstatic box Main { main() {} }",
    )
    .into_canonical_core_compile_request()
    .expect("canonical-core callable handoff");
    let outcome = crate::mir::MirCompiler::new()
        .run_canonical_core_source_plan_vm_reference_summary_for_test(request)
        .expect("Callable VM-reference execution");
    assert_eq!(outcome.status, 0);
    assert_eq!(outcome.fault_tag, None);
    assert_eq!(outcome.route, "main");
}

#[cfg(feature = "vm-reference")]
#[test]
fn canonical_core_reuses_one_compiler_after_callable_rejection_and_program_fault() {
    let dir = tempdir().expect("tempdir");
    let request = |name, source| {
        classify_canonical_core(dir.path(), name, source)
            .into_canonical_core_compile_request()
            .expect("canonical-core handoff")
    };
    let callable =
        "static function helper(x: i64): i64 { return x }\nstatic box Main { main() {} }";
    let direct_call = "static function helper(x: i64): i64 { return x }\nstatic box Main { main() { helper(42) } }";
    let mut compiler = crate::mir::MirCompiler::new();

    let script = compiler
        .run_canonical_core_source_plan_vm_reference_summary_for_test(request(
            "reuse-script.hako",
            "42",
        ))
        .expect("Script success");
    assert_eq!(script.status, 42);

    let main = compiler
        .run_canonical_core_source_plan_vm_reference_summary_for_test(request(
            "reuse-main.hako",
            "static box Main { main() {} }",
        ))
        .expect("Main success");
    assert_eq!(main.status, 0);

    let first_callable = compiler
        .run_canonical_core_source_plan_vm_reference_summary_for_test(request(
            "reuse-callable-first.hako",
            callable,
        ))
        .expect("Callable success");
    assert_eq!(first_callable.status, 0);

    let rejected = compiler
        .compile_canonical_core_source_plan(request("reuse-direct-call.hako", direct_call))
        .expect_err("direct call remains a typed capability rejection");
    assert!(matches!(
        rejected.cause(),
        crate::mir::CanonicalCoreDispatchErrorV1::Callable(
            crate::mir::CanonicalCallableDispatchStageV1::MainPlan
        )
    ));
    rejected.discard();

    let fault = compiler
        .run_canonical_core_source_plan_vm_reference_summary_for_test(request(
            "reuse-fault.hako",
            "1 / 0",
        ))
        .expect("program Fault remains a terminal result, not a compile rejection");
    assert_eq!(fault.status, 70);
    assert_eq!(fault.fault_tag, Some("vm-division-by-zero"));

    let later_callable = compiler
        .run_canonical_core_source_plan_vm_reference_summary_for_test(request(
            "reuse-callable-later.hako",
            callable,
        ))
        .expect("later Callable success");
    assert_eq!(later_callable.status, 0);
    assert_eq!(later_callable.route, "main");
}

#[test]
fn parsed_main_zero_becomes_scalar_main_plan_once() {
    let dir = tempdir().expect("tempdir");
    let classified = classify(dir.path(), "main.hako", "static box Main { main() {} }")
        .expect("Main0 source plan");
    assert!(matches!(
        classified.plan(),
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(_))
    ));
    assert_eq!(classified.receipt_counts(), (1, 1));
}

#[test]
fn parsed_main_with_top_level_or_box_helper_becomes_callable_module() {
    let dir = tempdir().expect("tempdir");
    let cases = [
        (
            "top-level-helper.hako",
            "function helper() {}\nstatic box Main { main() {} }",
        ),
        (
            "main-box-helper.hako",
            "static box Main { helper() {} main() {} }",
        ),
    ];
    for (name, source) in cases {
        let classified = classify(dir.path(), name, source).expect("callable source plan");
        assert!(matches!(
            classified.plan(),
            SealedNormalSourcePlanV1::CallableModule(_)
        ));
        assert_eq!(classified.receipt_counts(), (1, 1));
    }
}

#[test]
fn parsed_function_only_retains_missing_entry_rejection() {
    let dir = tempdir().expect("tempdir");
    let rejected = classify(dir.path(), "function-only.hako", "function helper() {}")
        .expect_err("function-only source has no entry");
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::SourceEntry);
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MissingSourceEntry
    );
    assert_eq!(rejected.receipt_counts(), (1, 1));
    rejected.discard();
}

#[test]
fn parsed_script_plus_main_retains_mixed_family_rejection() {
    let dir = tempdir().expect("tempdir");
    let rejected = classify(
        dir.path(),
        "mixed.hako",
        "42\nstatic box Main { main() {} }",
    )
    .expect_err("mixed source families reject");
    assert_eq!(rejected.stage(), &NormalSourcePlanStageV1::FamilyClosure);
    assert_eq!(
        rejected.error(),
        &NormalSourcePlanErrorV1::MixedSourceFamilies
    );
    assert_eq!(rejected.receipt_counts(), (1, 1));
    rejected.discard();
}

#[test]
fn parse_and_using_rejections_never_issue_source_plan_requests() {
    let dir = tempdir().expect("tempdir");

    let parse_rejected = request(write_source(dir.path(), "invalid.hako", "@"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect_err("parse rejects before source-plan request");
    assert_eq!(
        parse_rejected.stage(),
        super::super::NormalFileSourceStageV1::Parse
    );

    let using_rejected = request(write_source(dir.path(), "using.hako", "using foo"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect_err("using rejects before source-plan request");
    assert_eq!(
        using_rejected.stage(),
        super::super::NormalFileSourceStageV1::SourceProfile
    );
}

#[test]
fn canonical_pure_script_retains_one_ast_free_source_input_handoff() {
    let dir = tempdir().expect("tempdir");
    let classified = canonical_core_request(write_source(dir.path(), "pure.hako", "42\n"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request()
        .classify()
        .expect("pure Script source plan");
    let super::CanonicalScriptSourceInputDispositionV1::HandoffReady(handoff) =
        classified.script_input()
    else {
        panic!("canonical pure Script must retain a ready input handoff");
    };
    assert_eq!(handoff.rows().statement_count(), 1);
    assert_eq!(handoff.rows().body_rows().len(), 1);
    assert_eq!(handoff.receipt_counts(), (1, 1));
    assert_eq!(handoff.utf8_len(), 3);
}

#[test]
fn compatibility_script_input_is_typed_and_never_empty_ready() {
    let dir = tempdir().expect("tempdir");
    let request = canonical_core_request(write_source(dir.path(), "boxed.hako", "box Plain {}\n"))
        .prepare()
        .expect("profile")
        .read_once()
        .expect("one read")
        .parse_once()
        .expect("one canonical parse")
        .prepare_source_plan_request();
    assert!(matches!(
        request.script_input,
        super::CanonicalScriptSourceInputDispositionV1::CompatibilitySource
    ));
}
