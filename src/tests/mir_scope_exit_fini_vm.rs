#[cfg(test)]
mod tests {
    use crate::backend::VM;
    use crate::parser::NyashParser;

    fn enable_stage3() {
        std::env::set_var("NYASH_FEATURES", "stage3");
    }

    fn run(code: &str) -> String {
        enable_stage3();
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        let result = compiler.compile(ast).expect("compile");
        let mut vm = VM::new();
        let out = vm.execute_module(&result.module).expect("vm exec");
        out.to_string_box().value
    }

    fn compile_error(code: &str) -> String {
        enable_stage3();
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        compiler.compile(ast).expect_err("compile should fail")
    }

    #[test]
    fn dropscope_fini_lifo_runtime_order() {
        let code = r#"
        local x = 0
        {
          fini { x = x * 10 + 1 }
          fini { x = x * 10 + 2 }
          x = x * 10 + 3
        }
        return x
        "#;
        assert_eq!(run(code), "321");
    }

    #[test]
    fn local_fini_sugar_uses_updated_slot_value() {
        let code = r#"
        local x = 0
        {
          local y = 7 fini { x = x + y }
          y = 5
        }
        return x
        "#;
        assert_eq!(run(code), "5");
    }

    #[test]
    fn local_fini_keeps_outer_slot_when_inner_shadow_exists() {
        let code = r#"
        local x = 0
        {
          local y = 7 fini { x = x + y }
          {
            local y = 100
            x = x + y
          }
        }
        return x
        "#;
        assert_eq!(run(code), "107");
    }

    #[test]
    fn same_scope_local_redeclaration_is_fail_fast() {
        let code = r#"
        {
          local y = 1
          local y = 2
        }
        return 0
        "#;
        let err = compile_error(code);
        assert!(
            err.contains("[freeze:contract][local/redeclare_same_scope]"),
            "unexpected error: {}",
            err
        );
    }
}
