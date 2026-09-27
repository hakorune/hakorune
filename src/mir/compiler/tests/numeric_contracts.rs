//! Test-only exact numeric contract and signature facts.

use super::*;

#[test]
fn compile_attaches_dynamic_integer_range_contract_before_verify() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box Page {
  capacity: usize = 0
}

static box Main {
  main(x) {
    local p = new Page()
    p.capacity = x
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let error = crate::test_support::with_env_vars(&[("NYASH_MIR_CORE13_PURE", None)], || {
        compiler.compile(ast)
    })
    .expect_err("new Page on this lane hits the retired birth-global boundary");
    assert!(
        error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{error}"
    );
}

#[test]
fn compile_preserves_exact_numeric_signature_facts() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
static box Main {
  id(x: usize): u64 {
    return x
  }

  main() {
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler.compile(ast).expect("compile");
    let function = result.module.get_function("Main.id/1").expect("Main.id/1");
    let param = function.params[0];

    assert_eq!(
        function
            .metadata
            .declared_param_decls
            .iter()
            .map(|decl| (
                decl.name.as_str(),
                decl.declared_type_name.as_deref().unwrap_or("<none>")
            ))
            .collect::<Vec<_>>(),
        vec![("x", "usize")]
    );
    assert_eq!(
        function.metadata.declared_return_type_name.as_deref(),
        Some("u64")
    );
    let fact = function
        .metadata
        .exact_numeric_value_facts
        .get(&param)
        .expect("param exact numeric fact");
    assert_eq!(fact.declared_type_name, "usize");
    assert_eq!(
        fact.source,
        ExactNumericValueFactSource::Param {
            index: 0,
            name: "x".to_string(),
        }
    );
    assert_eq!(
        function.metadata.exact_numeric_return_fact,
        Some(ExactNumericReturnFact {
            declared_type_name: "u64".to_string(),
        })
    );
}

#[test]
fn compile_publishes_declared_method_param_types_to_signature() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box Scanner {
  text: StringBox

  birth(input_text: StringBox) {
    me.text = input_text
  }
}

static box Main {
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler.compile(ast).expect("compile");
    let function = result
        .module
        .get_function("Scanner.birth/1")
        .expect("Scanner.birth/1");

    assert_eq!(
        function.signature.params.get(1),
        Some(&MirType::String),
        "declared method parameter type should be callable signature truth"
    );
    assert_eq!(
        function.metadata.value_types.get(&function.params[1]),
        Some(&MirType::String),
        "method parameter value type should be seeded from signature"
    );
}

#[test]
fn compile_publishes_exact_numeric_box_field_proof_from_ordinary_literal() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box Page {
  capacity: usize = 0
}

static box Main {
  main() {
    local page = new Page()
    page.capacity = 7
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let error = crate::test_support::with_env_vars(&[("NYASH_MIR_CORE13_PURE", None)], || {
        compiler.compile(ast)
    })
    .expect_err("new Page on this lane hits the retired birth-global boundary");
    assert!(
        error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{error}"
    );
}

#[test]
fn compile_rejects_out_of_range_ordinary_literal_at_exact_numeric_box_field() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(
        r#"
box ByteCell {
  value: u8 = 0
}

static box Main {
  main() {
    local cell = new ByteCell()
    cell.value = 256
    return 0
  }
}
"#,
    )
    .expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let error = crate::test_support::with_env_vars(&[("NYASH_MIR_CORE13_PURE", None)], || {
        compiler.compile(ast)
    })
    .expect_err("new ByteCell on this lane hits the retired birth-global boundary");
    assert!(
        error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{error}"
    );
}
