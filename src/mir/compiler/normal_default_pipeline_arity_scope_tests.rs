use super::*;

#[test]
fn arity_bearing_main_with_qualified_call_stays_off_canonical_route() {
    // MAIN-IMPORT-ARITY-SCOPE-S0 + MAIN-WRAPPER-PARAM-ENTRY-S0:
    // `main(args)` can never consume a row of the arity-0-only
    // qualified-method relation, so the relation is not issued; entry
    // adoption binds the declared `args` parameter to the injector's
    // published local (StaticInjectedLocals), and the diversion gate
    // checks the DECLARED arity so the canonical qualified route is not
    // entered.  The body lowers through `inner.lower_body` and the
    // String-returning qualified callee reaches the publication lane;
    // the first owned stop is the published backend view's integer-only
    // static-callee contract (`StaticMethodRequiresIntegerReturn`), which
    // is a separate downstream family.
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
            !error.contains("entry-shape-mismatch"),
            "declared params must adopt the injector's locals: {error}"
        );
        assert!(
            !error.contains("qualified-preflight"),
            "declared arity must keep main(args) off the canonical route: {error}"
        );
        assert!(
            error.contains("StaticMethodRequiresIntegerReturn"),
            "expected the published-view integer-return boundary, got: {error}"
        );
    });
}
