#[cfg(test)]
mod tests {
    use crate::mir::function::MirFunction;
    use crate::parser::NyashParser;

    fn compile_result(code: &str) -> crate::mir::MirCompileResult {
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::with_options(false);
        compiler.compile(ast).expect("compile should succeed")
    }

    fn first_outbox_function(module: &crate::mir::MirModule) -> &MirFunction {
        module
            .functions
            .values()
            .find(|function| !function.metadata.outbox_bindings.is_empty())
            .expect("expected a function with outbox bindings")
    }

    #[test]
    fn outbox_lowers_as_explicit_contract_binding() {
        let code = r#"
        outbox payload
        "#;
        let result = compile_result(code);
        assert!(
            result.verification_result.is_ok(),
            "unexpected verification failure: {:?}",
            result.verification_result
        );

        let function = first_outbox_function(&result.module);
        assert_eq!(
            function.metadata.outbox_bindings,
            vec!["payload".to_string()]
        );
        assert!(function
            .metadata
            .value_types
            .values()
            .any(|ty| matches!(ty, &crate::mir::MirType::Void)));
    }
}
