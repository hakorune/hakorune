//! HEADERPORT0 WIRING-I0-BORROW-P0-CANONICAL: candidate lifetime proof.
//!
//! The route-specific source-order census lives in the reusable guard. These
//! fixtures prove the shared physical owner: dropping a mutated candidate
//! leaves the live Builder unchanged, while the consuming success terminal
//! replaces it once. No production route is connected here.

use super::{module_session::CanonicalModuleLoweringSessionV1, MirCompiler};

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
