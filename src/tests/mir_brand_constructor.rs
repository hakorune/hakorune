#[cfg(test)]
mod tests {
    use crate::mir::MirBuilder;
    use crate::parser::NyashParser;

    fn ensure_ring0_initialized() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    }

    fn build(source: &str) -> Result<(), String> {
        ensure_ring0_initialized();
        let ast = NyashParser::parse_from_string(source).expect("parse");
        let mut builder = MirBuilder::new();
        builder.build_module(ast).map(|_| ())
    }

    #[test]
    fn mir_builder_accepts_declared_brand_constructor_as_transparent_value() {
        let source = r#"
brand BlockId: i64

static box Main {
  main() {
    local block = BlockId(7)
    return block
  }
}
"#;

        build(source).expect("declared brand constructor should lower");
    }

    #[test]
    fn mir_builder_rejects_brand_constructor_wrong_arity() {
        let source = r#"
brand BlockId: i64

static box Main {
  main() {
    return BlockId()
  }
}
"#;

        let err = build(source).expect_err("invalid brand constructor arity");
        assert!(err.contains("[brand/constructor-arity]"), "{err}");
    }

    #[test]
    fn mir_builder_keeps_unresolved_function_failure_for_non_brand_calls() {
        let source = r#"
static box Main {
  main() {
    return MissingBrand(7)
  }
}
"#;

        let err = build(source).expect_err("non-brand function should remain unresolved");
        assert!(err.contains("Unresolved function: 'MissingBrand'"), "{err}");
    }
}
