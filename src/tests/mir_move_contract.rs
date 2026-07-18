#[cfg(test)]
mod tests {
    use crate::parser::NyashParser;

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        let _ = std::panic::catch_unwind(|| {
            init_global_ring0(default_ring0());
        });
    }

    fn with_strict_planner_required_env<F: FnOnce()>(enabled: bool, f: F) {
        ensure_ring0_initialized();
        let value = enabled.then_some("1");
        crate::test_support::with_env_vars(
            &[
                ("HAKO_JOINIR_STRICT", value),
                ("HAKO_JOINIR_PLANNER_REQUIRED", value),
            ],
            f,
        );
    }

    fn compile_ok(code: &str) {
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        compiler.compile(ast).expect("compile should succeed");
    }

    fn compile_error(code: &str) -> String {
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        compiler.compile(ast).expect_err("compile should fail")
    }

    #[test]
    fn moved_same_call_args_is_fail_fast_in_strict_planner_required() {
        with_strict_planner_required_env(true, || {
            let code = r#"
            local x = 1
            unknown_call(x, x)
            return 0
            "#;
            let err = compile_error(code);
            assert!(
                err.contains("[freeze:contract][moved/use_after_move_same_call]"),
                "unexpected error: {}",
                err
            );
        });
    }

    #[test]
    fn moved_same_method_call_args_is_fail_fast_in_strict_planner_required() {
        with_strict_planner_required_env(true, || {
            let code = r#"
            local s = "abc"
            local x = 1
            s.substring(x, x)
            return 0
            "#;
            let err = compile_error(code);
            assert!(
                err.contains("[freeze:contract][moved/use_after_move_same_call]"),
                "unexpected error: {}",
                err
            );
        });
    }

    #[test]
    fn moved_same_call_args_keeps_release_mode_behavior() {
        with_strict_planner_required_env(false, || {
            let code = r#"
            local s = "abc"
            local x = 1
            s.substring(x, x)
            return 0
            "#;
            compile_ok(code);
        });
    }
}
