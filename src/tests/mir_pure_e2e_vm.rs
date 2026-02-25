#[cfg(test)]
mod tests {
    use crate::backend::VM;
    use crate::mir::{MirCompiler, MirPrinter, MirVerifier};
    use crate::parser::NyashParser;

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        let _ = std::panic::catch_unwind(|| {
            init_global_ring0(default_ring0());
        });
    }

    #[test]
    #[ignore = "env.box externcall unsupported in current pure VM path; kept as historical smoke"]
    fn vm_exec_new_string_length_under_pure_mode() {
        // Enable Core-13 pure mode
        std::env::set_var("NYASH_MIR_CORE13_PURE", "1");

        // Nyash code: return (new StringBox("Hello")).length()
        let code = r#"
return (new StringBox("Hello")).length()
"#;

        // Parse -> MIR -> VM execute
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        let result = compiler.compile(ast).expect("compile");

        let mut vm = VM::new();
        let out = vm.execute_module(&result.module).expect("vm exec");
        // Expect 5 as string (to_string_box) for convenience
        assert_eq!(out.to_string_box().value, "5");

        // Cleanup
        std::env::remove_var("NYASH_MIR_CORE13_PURE");
    }

    /// Minimal smoke for MirInstruction::Debug (post-RDN-0 DebugLog retire)
    #[test]
    fn mir_debug_minimal_printer_and_verifier() {
        ensure_ring0_initialized();
        std::env::set_var("NYASH_FEATURES", "stage3");
        std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
        std::env::set_var("NYASH_MIR_DEBUG_LOG", "1");

        let src = r#"
static box Main {
  method main(args) {
    local x = 1
    local y = 2
    // Debug 命令の存在が Verifier/Printer で問題にならないことの確認用。
    return x + y
  }
}
"#;

        let ast = NyashParser::parse_from_string(src).expect("parse ok");
        let mut mc = MirCompiler::with_options(false);
        let cr = mc.compile(ast).expect("compile ok");

        let mut verifier = MirVerifier::new();
        if let Err(errors) = verifier.verify_module(&cr.module) {
            for e in &errors {
                eprintln!("[mir-debuglog] {}", e);
            }
            panic!("MIR verification failed for Debug minimal case");
        }

        let dump = MirPrinter::new().print_module(&cr.module);
        eprintln!("----- MIR DUMP (Debug.min) -----\n{}", dump);
    }
}
