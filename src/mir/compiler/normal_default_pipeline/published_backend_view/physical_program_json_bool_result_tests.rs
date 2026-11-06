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
         static box Main {{\n  main() {{ local r = new R()  r.run(\"a\")  print(r.n)  return 0 }}\n}}\n"
    );
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(&source), |view, _| {
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
        "local c = s == \"a\"  return c",
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
