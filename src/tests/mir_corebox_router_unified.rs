use crate::ast::ASTNode;
use crate::mir::{Callee, MirCompiler, MirInstruction, MirModule, MirType};
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

fn method_call_result_types(
    module: &MirModule,
    box_name: &str,
    method: &str,
) -> Vec<Option<MirType>> {
    let mut result_types = Vec::new();
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    dst,
                    callee:
                        Some(Callee::Method {
                            box_name: call_box,
                            method: call_method,
                            ..
                        }),
                    ..
                } = inst
                else {
                    continue;
                };
                if call_box == box_name && call_method == method {
                    result_types
                        .push(dst.and_then(|dst| function.metadata.value_types.get(&dst).cloned()));
                }
            }
        }
    }
    result_types
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
fn string_value_trim_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = " banana "
    local out = s.trim()
    return out
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "trim");

    assert_eq!(
        arg_lens,
        vec![1],
        "StringBox.trim should use the Unified method-call shape with receiver in args"
    );
}

#[test]
fn string_value_contains_uses_unified_receiver_arg_shape_and_bool_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local ok = s.contains("na")
    return ok
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "contains");
    let result_types = method_call_result_types(&module, "StringBox", "contains");

    assert_eq!(
        arg_lens,
        vec![2],
        "StringBox.contains should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        result_types,
        vec![Some(MirType::Bool)],
        "StringBox.contains should publish a Bool result type"
    );
}

#[test]
fn string_value_last_index_of_uses_unified_receiver_arg_shape_and_integer_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local idx = s.lastIndexOf("a")
    return idx
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "lastIndexOf");
    let result_types = method_call_result_types(&module, "StringBox", "lastIndexOf");

    assert_eq!(
        arg_lens,
        vec![2],
        "StringBox.lastIndexOf/1 should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        result_types,
        vec![Some(MirType::Integer)],
        "StringBox.lastIndexOf/1 should publish an Integer result type"
    );
}

#[test]
fn string_value_last_index_of_two_arg_stays_boxcall_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local idx = s.lastIndexOf("a", 4)
    return idx
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "lastIndexOf");

    assert_eq!(
        arg_lens,
        vec![2],
        "StringBox.lastIndexOf/2 is still deferred and should stay on the BoxCall fallback shape"
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
