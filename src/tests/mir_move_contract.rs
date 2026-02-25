#[cfg(test)]
mod tests {
    use crate::parser::NyashParser;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        let _ = std::panic::catch_unwind(|| {
            init_global_ring0(default_ring0());
        });
    }

    fn with_strict_planner_required_env<F: FnOnce()>(enabled: bool, f: F) {
        let _lock = env_guard().lock().unwrap_or_else(|e| e.into_inner());
        ensure_ring0_initialized();
        let prev_strict = std::env::var("HAKO_JOINIR_STRICT").ok();
        let prev_required = std::env::var("HAKO_JOINIR_PLANNER_REQUIRED").ok();

        if enabled {
            std::env::set_var("HAKO_JOINIR_STRICT", "1");
            std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");
        } else {
            std::env::remove_var("HAKO_JOINIR_STRICT");
            std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED");
        }

        f();

        match prev_strict {
            Some(v) => std::env::set_var("HAKO_JOINIR_STRICT", v),
            None => std::env::remove_var("HAKO_JOINIR_STRICT"),
        }
        match prev_required {
            Some(v) => std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", v),
            None => std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED"),
        }
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
