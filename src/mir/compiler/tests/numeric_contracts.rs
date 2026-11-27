//! Test-only exact numeric contract and signature facts.

use super::*;

fn assert_legacy_birth_global_boundary(source: &str, expectation: &str) {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(source).expect("parse");
    let mut compiler = MirCompiler::with_options(false);
    let error = { compiler.compile(ast) }.expect_err(expectation);
    assert!(
        error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{error}"
    );
}

#[test]
fn compile_attaches_dynamic_integer_range_contract_before_verify() {
    assert_legacy_birth_global_boundary(
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
        "new Page on this lane hits the retired birth-global boundary",
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
    assert_legacy_birth_global_boundary(
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
        "new Page on this lane hits the retired birth-global boundary",
    );
}

#[test]
fn compile_rejects_out_of_range_ordinary_literal_at_exact_numeric_box_field() {
    assert_legacy_birth_global_boundary(
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
        "new ByteCell on this lane hits the retired birth-global boundary",
    );
}

#[test]
fn compile_reads_a_typed_numeric_local() {
    verify_callable_source(
        "static box Main { main() { local x: i64 = 1 return x } }",
        "typed-local-read.hako",
    );
    verify_callable_source(
        "static box Main { main() { local x: i64 = 1 x = x + 1 return x } }",
        "typed-local-reassign.hako",
    );
}

#[test]
fn compile_reads_a_typed_numeric_loop_local_after_reassignment() {
    verify_callable_source(
        "static box Main { main() { local x: i64 = 1 \
         loop (x < 3) { x = x + 1 } return x } }",
        "typed-loop-reassign.hako",
    );
    verify_callable_source(
        "static box Main { main() { local x: i64 = 0 \
         if x == 0 { x = 1 } else { x = 2 } \
         loop (x < 3) { x = x + 1 } return x } }",
        "typed-branch-loop-reassign.hako",
    );
}

const DYNAMIC_FIELD_CASES: [(&str, &str, i32, &str); 3] = [
    (
        "counter.hako",
        r#"
box Counter {
  n
  birth(c) { me.n = c }
  read() { return me.n }
}
static box Main {
  main() {
    local c = new Counter(7)
    return c.read()
  }
}
"#,
        7,
        "",
    ),
    (
        "repeated.hako",
        r#"
box Counter {
  n
  birth(c) { me.n = c }
  bump(): i64 {
    print(me.n)
    me.n = 11
    print(me.n)
    me.n = 13
    print(me.n)
    return 13
  }
}
static box Main {
  main() {
    local c = new Counter(7)
    return c.bump()
  }
}
"#,
        13,
        "7\n11\n13\n",
    ),
    (
        "other-box.hako",
        r#"
box Counter {
  n
  birth(c) { me.n = c }
}
box Reader {
  read() {
    local c = new Counter(9)
    print(c.n)
    return 9
  }
}
static box Main {
  main() {
    local reader = new Reader()
    return reader.read()
  }
}
"#,
        9,
        "9\n",
    ),
];

#[test]
fn verify_callable_dynamic_field_reads_and_writes() {
    for (fixture, source, _, _) in DYNAMIC_FIELD_CASES {
        verify_callable_source(source, fixture);
    }
}

#[test]
#[ignore = "requires C FFI, LLVM and NYASH_EMIT_EXE_NYRT"]
fn execute_callable_dynamic_field_reads_and_writes() {
    let directory = tempfile::tempdir().expect("EXE test directory");
    for (fixture, source, exit, stdout) in DYNAMIC_FIELD_CASES {
        let path = directory.path().join(fixture).with_extension("exe");
        let output = run_callable_source_exe(source, fixture, &path);
        assert_eq!(output.status.code(), Some(exit), "{fixture}: {output:?}");
        assert_eq!(output.stdout, stdout.as_bytes(), "{fixture}");
    }
}
