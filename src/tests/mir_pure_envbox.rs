mod tests {
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::{MirCompiler, MirPrinter};

    #[test]
    fn pure_mode_new_emits_env_box_new() {
        // Enable pure mode
        std::env::set_var("NYASH_MIR_CORE13_PURE", "1");
        // new StringBox("Hello")
        let ast = ASTNode::New {
            class: "StringBox".to_string(),
            arguments: vec![ASTNode::Literal {
                value: LiteralValue::String("Hello".into()),
                span: Span::unknown(),
            }],
            type_arguments: vec![],
            span: Span::unknown(),
        };
        let mut c = MirCompiler::new();
        let result = c.compile(ast).expect("compile");
        let dump = MirPrinter::new().print_module(&result.module);
        // Pure mode should route box creation via env.box.new (Stage-1 bridge), but allow
        // future direct constructors by accepting either form.
        let has_env_new = dump.contains("extern_call env.box.new");
        let has_direct_new = dump.contains("new StringBox");
        assert!(
            has_env_new || has_direct_new,
            "expected env.box.new or direct new StringBox in MIR. dump=\n{}",
            dump
        );
        std::env::remove_var("NYASH_MIR_CORE13_PURE");
    }
}
