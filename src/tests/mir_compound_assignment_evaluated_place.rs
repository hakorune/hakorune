use crate::mir::{MirCompiler, MirModule};
use crate::parser::NyashParser;

fn compile_source(source: &str) -> MirModule {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(source).expect("parse compound assignment source");
    let mut compiler = MirCompiler::with_options(false);
    compiler
        .compile(ast)
        .expect("compile compound assignment source")
        .module
}

#[test]
fn evaluated_place_compound_assignment_lowers_local_field_and_index() {
    let module = compile_source(
        r#"
box Counter {
  value: i64
}

static box Main {
  main() {
    local x = 1
    x += 2

    local counter = new Counter { value: 3 }
    counter.value += x

    local values = [5]
    values[0] += counter.value
    return x
  }
}
"#,
    );

    let printed = crate::mir::MirPrinter::new().print_module(&module);
    let first_field_get = printed.find("field.get").expect("field read");
    let array_get = printed.find("ArrayBox.get").expect("index read");
    let rhs_field_get = printed[array_get + 1..]
        .find("field.get")
        .map(|offset| array_get + 1 + offset)
        .expect("index RHS field read");
    let array_set = printed
        .find("array.write #1 set")
        .expect("canonical index store");
    let compound_field_set = printed.rfind("field.set").expect("field store");

    assert!(
        printed.matches(" Add ").count() >= 3,
        "compound assignments apply"
    );
    assert!(
        first_field_get < compound_field_set,
        "field reads before its store"
    );
    assert!(
        array_get < rhs_field_get,
        "index old value reads before RHS"
    );
    assert!(
        rhs_field_get < array_set,
        "index store follows RHS evaluation"
    );
    assert_eq!(
        printed.matches("ArrayBox.get").count(),
        1,
        "index reads once"
    );
    assert_eq!(
        printed.matches("array.write #1 set").count(),
        1,
        "index stores once"
    );
}

#[test]
fn unsupported_compound_index_rejects_before_store_lowering() {
    crate::tests::helpers::env::with_env_var("NYASH_SYNTAX_SUGAR_LEVEL", "basic", || {
        let ast = NyashParser::parse_from_string(
            r#"
static box Main {
  main() {
    local scalar = 1
    scalar[index_side_effect()] += 2
  }
}
"#,
        )
        .expect("parse unsupported compound index");
        let mut compiler = MirCompiler::with_options(false);
        let error = compiler.compile(ast).expect_err("scalar index must reject");
        assert!(
            error.contains("index operator is only supported"),
            "{error}"
        );
    });
}

#[cfg(feature = "vm-reference")]
#[test]
fn evaluated_local_place_executes_read_modify_write_once() {
    let ast = NyashParser::parse_from_string(
        r#"
local x = 40
x += 2
return x
"#,
    )
    .expect("parse local compound assignment");
    let mut compiler = MirCompiler::with_options(false);
    let module = compiler
        .compile(ast)
        .expect("compile local compound assignment")
        .module;
    let mut vm = crate::backend::VM::new();
    let result = vm
        .execute_module(&module)
        .expect("execute local compound assignment");
    assert_eq!(result.to_string_box().value, "42");
}
