//! A declared `: Bool` result (R1, R2, R9): it takes the same route as the same function with no
//! declaration, and a declaration the body does not support is refused rather than lowered.
use super::tests::request;
use crate::mir::compiler::normal_default_pipeline::MirCompiler;

fn compile(annotation: &str, body: &str) -> Result<(), String> {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = format!(
        "static box P {{\n  t(s){annotation} {{ {body} }}\n}}\n\
         box R {{\n  n: i64\n  birth() {{ me.n = 0 }}\n  run(line) {{\n    local b = P.t(line)\n    \
         if b {{ me.n = me.n + 1 }} else {{ me.n = me.n + 10 }}\n    return true\n  }}\n}}\n\
         static box Main {{\n  main() {{\n    local r = new R()\n    r.run(\"a\")\n    print(r.n)\n    return 0\n  }}\n}}\n"
    );
    compile_source(&source)
}

fn compile_source(source: &str) -> Result<(), String> {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(source), |view, _| {
                view.issue_lifecycle_physical_abi_input().map(|_| ())
            })
            .map(|_| ())
    })
}

const LITERALS: &str = "if s == \"a\" { return true } return false";

/// R9: the declaration changes nothing about the route. Whatever the undeclared function does, the
/// declared one does, with the same named outcome.
#[test]
fn declared_bool_result_takes_the_route_of_the_undeclared_one() {
    for body in [
        LITERALS,
        "return s == \"a\"",
        "local c = s == \"a\"; return c",
        "if s == \"a\" { return true } else { return false }",
    ] {
        let undeclared = compile("", body);
        let declared = compile(": Bool", body);
        assert_eq!(undeclared.is_ok(), declared.is_ok(), "{body}: {undeclared:?} / {declared:?}");
        if let (Err(undeclared), Err(declared)) = (&undeclared, &declared) {
            let tag = |error: &str| error.split(']').take(2).collect::<String>();
            assert_eq!(tag(undeclared), tag(declared), "{body}");
        }
    }
    compile(": Bool", LITERALS).unwrap_or_else(|error| panic!("declared Bool literal returns: {error}"));
}

/// R2: a body whose returns are not Bool is not lowered under `: Bool`, and the other spellings of
/// the type stay refused by the header.
#[test]
fn declared_bool_result_refuses_returns_that_are_not_bool() {
    // An exit that proves the other scalar class is refused by name at the contract row.
    for body in ["if s == \"a\" { return 1 } return 2", "if s == \"a\" { return true } return 2"] {
        let error = compile(": Bool", body).unwrap_err();
        assert!(error.contains("DeclaredResultDisagrees"), "{body}: {error}");
    }
    // Any other body that does not prove Bool is not lowered either.
    for body in ["if s == \"a\" { return \"x\" } return \"y\"", "return 1"] {
        compile(": Bool", body).expect_err(body);
    }
    for annotation in [": bool", ": Boolean", ": BoolBox"] {
        let error = compile(annotation, LITERALS).unwrap_err();
        assert!(error.contains("UnsupportedResultAnnotation"), "{annotation}: {error}");
    }
}

/// R12/R14/R16: a static Bool callee over exact-i64 input takes the route the i64 callee takes,
/// declared or not, and refuses what the i64 callee refuses.
#[test]
fn exact_i64_bool_call_takes_the_i64_callee_route() {
    let program = |callee: &str, call: &str| format!(
        "static box Main {{\n  {callee}\n  main() {{\n    local b = {call}\n    \
         if b {{ print(1) }} else {{ print(0) }}\n    return 0\n  }}\n}}\n");
    for body in ["return x > 1", "local c = x > 1 return c", "if x > 1 { return true } return false"] {
        for annotation in [": Bool", ""] {
            compile_source(&program(&format!("t(x: i64){annotation} {{ {body} }}"), "Main.t(4)"))
                .unwrap_or_else(|error| panic!("{annotation} {body}: {error}"));
        }
    }
    // The Bool result never stands where an i64 does, and the required position stays exact-i64.
    let callee = "t(x: i64): Bool { return x > 1 }";
    compile_source(&program(callee, "Main.t(4) + 1")).expect_err("a Bool is not an i64 operand");
    compile_source(&program(callee, "Main.t(\"a\")")).expect_err("a Text actual at an i64 position");
    // R2: an exit that proves the other scalar kind is refused by name once the callee is lowered.
    let error = compile_source(&program("t(x: i64): Bool { if x > 1 { return true } return x + 1 }", "Main.t(4)")).unwrap_err();
    assert!(error.contains("DeclaredResultDisagrees"), "{error}");
}
