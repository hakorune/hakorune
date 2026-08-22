//! Test-only finish schedule and candidate discard.

use super::*;

#[test]
fn trivial_binding_ssa_finish_schedule_skips_legacy_rc() {
    assert_eq!(
        MirFinishScheduleV1::Canonical(CanonicalFinishScheduleV1::TrivialBindingSsa)
            .legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Skip
    );
}

#[test]
fn current_canonical_and_legacy_finish_schedules_keep_legacy_rc() {
    assert_eq!(
        MirFinishScheduleV1::Canonical(CanonicalFinishScheduleV1::CurrentCanonicalAPlus)
            .legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Run
    );
    assert_eq!(
        MirFinishScheduleV1::Legacy.legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Run
    );
}

#[test]
fn selected_dynamic_finish_schedule_skips_legacy_postseal_mutators() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "ParserScanLoopBox.skip_while/4".to_owned(),
            params: vec![MirType::Unknown; 4],
            return_type: MirType::Integer,
            effects: EffectMask::READ,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    function
        .metadata
        .install_dynamic_v2_aot_metadata_for_test(
            crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(
            ),
        )
        .expect("AOT metadata install");

    let mut module = MirModule::new("selected".to_owned());
    module.add_function(function);

    let schedule = super::finish_schedule_for_normal_module(&module)
        .expect("selected pair should select the closed schedule");
    assert_eq!(schedule, MirFinishScheduleV1::SelectedDynamic);
    assert_eq!(
        schedule.legacy_rc_insertion(),
        LegacyRcInsertionScheduleV1::Skip
    );
}

#[test]
fn selected_dynamic_finish_schedule_rejects_scrubbed_or_partial_metadata() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "selected/0".to_owned(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    let mut partial = MirModule::new("partial".to_owned());
    partial.add_function(function);
    assert!(super::finish_schedule_for_normal_module(&partial)
        .unwrap_err()
        .contains("partial"));

    let mut function = MirFunction::new(
        FunctionSignature {
            name: "selected/0".to_owned(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .install_a_prime_i64_physical_receipt_for_test(crate::mir::test_support::a_prime_receipt())
        .expect("receipt install");
    function
        .metadata
        .install_dynamic_v2_aot_metadata_for_test(
            crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(
            ),
        )
        .expect("AOT metadata install");
    let mut scrubbed_function = function.clone();
    scrubbed_function.signature.name = "scrubbed/0".to_owned();
    let mut scrubbed = MirModule::new("scrubbed".to_owned());
    scrubbed.add_function(function);
    scrubbed.add_function(scrubbed_function);
    assert!(super::finish_schedule_for_normal_module(&scrubbed)
        .unwrap_err()
        .contains("scrubbed"));
}

#[test]
fn test_basic_mir_compilation() {
    let mut compiler = MirCompiler::new();

    // Create a simple literal AST node
    let ast = ASTNode::Literal {
        value: LiteralValue::Integer(42),
        span: crate::ast::Span::unknown(),
    };

    // Compile to MIR
    let result = compiler.compile(ast);
    assert!(result.is_ok(), "Basic MIR compilation should succeed");

    let compile_result = result.unwrap();
    assert!(
        !compile_result.module.functions.is_empty(),
        "Module should contain at least one function"
    );
}

#[test]
fn canonical_verification_failure_discards_candidate_before_commit() {
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.repl_mode = true;
    let mut session = CanonicalModuleLoweringSessionV1::open(&compiler.builder);
    session.builder_mut().repl_mode = false;

    let error = require_canonical_verification(Err(vec![
        crate::mir::VerificationError::UnreachableBlock {
            block: crate::mir::BasicBlockId::new(900),
        },
    ]))
    .unwrap_err();
    drop(session);

    assert!(matches!(
        error,
        CanonicalLoweringErrorV1::MirVerificationFailed { .. }
    ));
    assert!(compiler.builder.repl_mode);
}
