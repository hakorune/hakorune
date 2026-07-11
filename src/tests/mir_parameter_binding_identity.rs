use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

fn compile_and_verify(source: &str) {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(source).expect("parse parameter identity fixture");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler
        .compile(ast)
        .expect("compile parameter identity fixture");
    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&result.module) {
        panic!("parameter identity MIR verification failed: {errors:?}");
    }
}

#[test]
fn static_and_instance_parameter_reassignment_have_entry_identity() {
    compile_and_verify(
        r#"
box Counter {
  update(flag) {
    flag = false
    return flag
  }
}

static box Main {
  update(flag) {
    flag = false
    return flag
  }
}
"#,
    );
}

#[test]
fn local_shadow_restores_parameter_identity_after_scope_exit() {
    compile_and_verify(
        r#"
box Counter {
  update(flag) {
    if true {
      local flag = false
      flag = true
    }
    flag = false
    return flag
  }
}
"#,
    );
}
