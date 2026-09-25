use super::*;

#[test]
fn arity_bearing_main_with_qualified_call_stays_off_canonical_route() {
    // MAIN-IMPORT-ARITY-SCOPE-S0: `main(args)` can never consume a row of
    // the arity-0-only qualified-method relation, so the relation is not
    // issued and the canonical diversion is not entered. The first owned
    // stop on the lifecycle route is the pre-existing wrapper boundary
    // `entry-shape-mismatch` (script `args` arrive as injected locals, not
    // formals); `main-import-view` must never appear for arity!=0 mains.
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = "static box Helpers { payload() { return \"hi\" } } static box Main { main(args) { local s = Helpers.payload() return 0 } }";
        let mut compiler = MirCompiler::with_options(false);
        let error = match compiler
            .compile_normal_with_published(published_request(source), |_view, _| Ok(()))
        {
            Err(error) => error,
            Ok(_) => panic!("arity-bearing app main declines the qualified route"),
        };
        assert!(
            !error.contains("main-import-view"),
            "arity!=0 main must not issue the arity-0 relation: {error}"
        );
        assert!(
            error.contains("entry-shape-mismatch"),
            "expected the wrapper entry-adoption boundary, got: {error}"
        );
    });
}
