//! HEADERPORT0 WIRING-I0-BORROW-P0-CANONICAL: candidate lifetime proof.
//!
//! The route-specific source-order census lives in the reusable guard. These
//! fixtures prove the shared physical owner: dropping a mutated candidate
//! leaves the live Builder unchanged, while the consuming success terminal
//! replaces it once. No production route is connected here.

use super::{module_session::CanonicalModuleLoweringSessionV1, MirCompiler};

fn debug_policy_updates(enabled: bool) -> [(&'static str, Option<&'static str>); 11] {
    let value = if enabled { Some("1") } else { Some("0") };
    [
        ("HAKO_JOINIR_DEBUG", value),
        ("NYASH_JOINIR_DEBUG", value),
        ("HAKO_JOINIR_STRICT", value),
        ("NYASH_JOINIR_STRICT", value),
        ("HAKO_JOINIR_PLANNER_REQUIRED", value),
        ("NYASH_LOCAL_SSA_TRACE", value),
        ("NYASH_BUILDER_TRACE_RECV", value),
        (
            "NYASH_BUILDER_DEBUG",
            if enabled { Some("1") } else { None },
        ),
        ("NYASH_STATIC_CALL_TRACE", value),
        ("NYASH_STATIC_METHOD_TRACE", value),
        ("NYASH_CALL_RESOLVE_TRACE", value),
    ]
}

#[test]
fn canonical_module_session_drop_preserves_live_builder_after_candidate_mutation() {
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.repl_mode = true;
    compiler.builder.recursion_depth = 17;

    {
        let mut session = CanonicalModuleLoweringSessionV1::open(&compiler.builder);
        session.builder_mut().repl_mode = false;
        session.builder_mut().recursion_depth = 91;
    }

    assert!(compiler.builder.repl_mode);
    assert_eq!(compiler.builder.recursion_depth, 17);
}

#[test]
fn canonical_module_session_commit_replaces_live_builder_once() {
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.repl_mode = true;
    compiler.builder.recursion_depth = 17;
    let mut session = CanonicalModuleLoweringSessionV1::open(&compiler.builder);
    session.builder_mut().repl_mode = false;
    session.builder_mut().recursion_depth = 91;

    session.commit(&mut compiler.builder);

    assert!(!compiler.builder.repl_mode);
    assert_eq!(compiler.builder.recursion_depth, 91);
}

#[test]
fn canonical_module_session_snapshots_debug_policy_before_ambient_flip() {
    let policy_a = crate::test_support::with_env_vars(&debug_policy_updates(true), || {
        let compiler = MirCompiler::with_options(false);
        let mut session = CanonicalModuleLoweringSessionV1::open(&compiler.builder);
        session.builder_mut().emit_debug_policy_bits_for_test()
    });
    let policy_b = crate::test_support::with_env_vars(&debug_policy_updates(false), || {
        let compiler = MirCompiler::with_options(false);
        let mut session = CanonicalModuleLoweringSessionV1::open(&compiler.builder);
        session.builder_mut().emit_debug_policy_bits_for_test()
    });

    crate::test_support::with_env_vars(&debug_policy_updates(false), || {
        assert_eq!(policy_a, [true, true, true, true, true, true, true, true]);
        assert_eq!(
            policy_b,
            [false, false, false, false, false, false, false, false]
        );
    });
}
