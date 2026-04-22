use crate::ast::ASTNode;
use crate::mir::{Callee, MirCompiler, MirInstruction, MirModule};
use crate::parser::NyashParser;

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

fn compile_src(src: &str) -> MirModule {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut compiler = MirCompiler::with_options(false);
    compiler.compile(ast).expect("compile ok").module
}

fn method_call_arg_lens(module: &MirModule, box_name: &str, method: &str) -> Vec<usize> {
    let mut arg_lens = Vec::new();
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    callee:
                        Some(Callee::Method {
                            box_name: call_box,
                            method: call_method,
                            ..
                        }),
                    args,
                    ..
                } = inst
                else {
                    continue;
                };
                if call_box == box_name && call_method == method {
                    arg_lens.push(args.len());
                }
            }
        }
    }
    arg_lens
}

#[test]
fn string_value_length_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local n = s.length()
    return n
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "length");

    assert_eq!(
        arg_lens,
        vec![1],
        "StringBox.length should use the Unified method-call shape with receiver in args"
    );
}

#[test]
fn string_value_substring_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local part = s.substring(1, 4)
    return part
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "substring");

    assert_eq!(
        arg_lens,
        vec![3],
        "StringBox.substring should use the Unified method-call shape with receiver in args"
    );
}

#[test]
fn string_value_substr_alias_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local part = s.substr(1, 4)
    return part
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "substr");

    assert_eq!(
        arg_lens,
        vec![3],
        "StringBox.substr should use the Unified method-call shape with receiver in args"
    );
}

#[test]
fn string_value_concat_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local out = s.concat("!")
    return out
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "concat");

    assert_eq!(
        arg_lens,
        vec![2],
        "StringBox.concat should use the Unified method-call shape with receiver in args"
    );
}

#[test]
fn string_value_index_of_stays_boxcall_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local idx = s.indexOf("a")
    return idx
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "indexOf");

    assert_eq!(
        arg_lens,
        vec![1],
        "StringBox.indexOf is not allowlisted yet and should stay on the BoxCall fallback shape"
    );
}
