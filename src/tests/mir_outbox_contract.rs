#[cfg(test)]
mod tests {
    use crate::parser::NyashParser;

    fn compile_error(code: &str) -> String {
        let ast = NyashParser::parse_from_string(code).expect("parse");
        let mut compiler = crate::mir::MirCompiler::new();
        compiler.compile(ast).expect_err("compile should fail")
    }

    #[test]
    fn outbox_lowers_as_explicit_contract_error_for_now() {
        let code = r#"
        outbox payload
        "#;
        let err = compile_error(code);
        assert!(
            err.contains("[freeze:contract][outbox/lowering_not_implemented]"),
            "unexpected error: {}",
            err
        );
    }
}
