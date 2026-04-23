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
fn string_value_last_index_of_two_arg_uses_unified_receiver_arg_shape_and_integer_return() {
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
    let result_types = method_call_result_types(&module, "StringBox", "lastIndexOf");

    assert_eq!(
        arg_lens,
        vec![3],
        "StringBox.lastIndexOf/2 should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        result_types,
        vec![Some(MirType::Integer)],
        "StringBox.lastIndexOf/2 should publish an Integer result type"
    );
}

#[test]
fn string_value_replace_uses_unified_receiver_arg_shape_and_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local out = s.replace("a", "o")
    return out
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "replace");
    let result_types = method_call_result_types(&module, "StringBox", "replace");

    assert_eq!(
        arg_lens,
        vec![3],
        "StringBox.replace should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        result_types,
        vec![Some(MirType::String)],
        "StringBox.replace should publish a String result type"
    );
}

#[test]
fn string_value_index_of_and_find_use_unified_receiver_arg_shape_and_integer_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local idx1 = s.indexOf("a")
    local idx2 = s.indexOf("a", 2)
    local alias1 = s.find("a")
    local alias2 = s.find("a", 2)
    return alias2
  }
}
"#;

    let module = compile_src(src);
    let index_arg_lens = method_call_arg_lens(&module, "StringBox", "indexOf");
    let index_result_types = method_call_result_types(&module, "StringBox", "indexOf");
    let find_arg_lens = method_call_arg_lens(&module, "StringBox", "find");
    let find_result_types = method_call_result_types(&module, "StringBox", "find");

    assert_eq!(
        index_arg_lens,
        vec![2, 3],
        "StringBox.indexOf/1 and indexOf/2 should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        index_result_types,
        vec![Some(MirType::Integer), Some(MirType::Integer)],
        "StringBox.indexOf should publish Integer result types"
    );
    assert_eq!(
        find_arg_lens,
        vec![2, 3],
        "StringBox.find/1 and find/2 should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        find_result_types,
        vec![Some(MirType::Integer), Some(MirType::Integer)],
        "StringBox.find should publish Integer result types"
    );
}

#[test]
fn array_value_length_aliases_use_unified_receiver_arg_shape_and_integer_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local n1 = a.length()
    local n2 = a.size()
    local n3 = a.len()
    return n3
  }
}
"#;

    let module = compile_src(src);
    let length_arg_lens = method_call_arg_lens(&module, "ArrayBox", "length");
    let length_result_types = method_call_result_types(&module, "ArrayBox", "length");
    let size_arg_lens = method_call_arg_lens(&module, "ArrayBox", "size");
    let size_result_types = method_call_result_types(&module, "ArrayBox", "size");
    let len_arg_lens = method_call_arg_lens(&module, "ArrayBox", "len");
    let len_result_types = method_call_result_types(&module, "ArrayBox", "len");

    assert_eq!(
        length_arg_lens,
        vec![1],
        "ArrayBox.length should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        length_result_types,
        vec![Some(MirType::Integer)],
        "ArrayBox.length should publish an Integer result type"
    );
    assert_eq!(
        size_arg_lens,
        vec![1],
        "ArrayBox.size should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        size_result_types,
        vec![Some(MirType::Integer)],
        "ArrayBox.size should publish an Integer result type"
    );
    assert_eq!(
        len_arg_lens,
        vec![1],
        "ArrayBox.len should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        len_result_types,
        vec![Some(MirType::Integer)],
        "ArrayBox.len should publish an Integer result type"
    );
}

#[test]
fn array_value_push_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let push_arg_lens = method_call_arg_lens(&module, "ArrayBox", "push");

    assert_eq!(
        push_arg_lens,
        vec![2],
        "ArrayBox.push should use the Unified method-call shape with receiver in args"
    );
}

#[test]
fn array_value_slice_uses_unified_receiver_arg_shape_and_array_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local s = a.slice(0, 1)
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let slice_arg_lens = method_call_arg_lens(&module, "ArrayBox", "slice");
    let slice_result_types = method_call_result_types(&module, "ArrayBox", "slice");

    assert_eq!(
        slice_arg_lens,
        vec![3],
        "ArrayBox.slice should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        slice_result_types,
        vec![Some(MirType::Box("ArrayBox".to_string()))],
        "ArrayBox.slice should publish an ArrayBox result type"
    );
}

#[test]
fn array_value_get_uses_unified_receiver_arg_shape_and_generic_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local x = a.get(0)
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let get_arg_lens = method_call_arg_lens(&module, "ArrayBox", "get");
    let get_result_types = method_call_result_types(&module, "ArrayBox", "get");

    assert_eq!(
        get_arg_lens,
        vec![2],
        "ArrayBox.get should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        get_result_types,
        vec![Some(MirType::Unknown)],
        "ArrayBox.get returns a data-dependent element and should stay MIR-unknown"
    );
}

#[test]
fn array_value_pop_uses_unified_receiver_arg_shape_and_generic_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local p = a.pop()
    a.push(9)
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let pop_arg_lens = method_call_arg_lens(&module, "ArrayBox", "pop");
    let pop_result_types = method_call_result_types(&module, "ArrayBox", "pop");

    assert_eq!(
        pop_arg_lens,
        vec![1],
        "ArrayBox.pop should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        pop_result_types,
        vec![Some(MirType::Unknown)],
        "ArrayBox.pop returns a data-dependent element and should stay MIR-unknown"
    );
}

#[test]
fn array_value_set_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    a.set(0, 8)
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let set_arg_lens = method_call_arg_lens(&module, "ArrayBox", "set");

    assert_eq!(
        set_arg_lens,
        vec![3],
        "ArrayBox.set should use the Unified method-call shape with receiver in args"
    );
}

#[test]
fn array_value_remove_uses_unified_receiver_arg_shape_and_generic_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local r = a.remove(0)
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let remove_arg_lens = method_call_arg_lens(&module, "ArrayBox", "remove");
    let remove_result_types = method_call_result_types(&module, "ArrayBox", "remove");

    assert_eq!(
        remove_arg_lens,
        vec![2],
        "ArrayBox.remove should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        remove_result_types,
        vec![Some(MirType::Unknown)],
        "ArrayBox.remove returns a data-dependent element and should stay MIR-unknown"
    );
}

#[test]
fn array_value_insert_uses_unified_receiver_arg_shape() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local inserted = a.insert(0, 9)
    local m = new MapBox()
    m.clear()
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let insert_arg_lens = method_call_arg_lens(&module, "ArrayBox", "insert");
    let insert_result_types = method_call_result_types(&module, "ArrayBox", "insert");
    let map_clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        insert_arg_lens,
        vec![3],
        "ArrayBox.insert should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        insert_result_types,
        vec![Some(MirType::Void)],
        "ArrayBox.insert should publish a Void result type"
    );
    assert_eq!(
        map_clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}

#[test]
fn map_value_size_uses_unified_receiver_arg_shape_and_integer_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    local n = m.size()
    m.clear()
    return n
  }
}
"#;

    let module = compile_src(src);
    let size_arg_lens = method_call_arg_lens(&module, "MapBox", "size");
    let size_result_types = method_call_result_types(&module, "MapBox", "size");
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        size_arg_lens,
        vec![1],
        "MapBox.size should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        size_result_types,
        vec![Some(MirType::Integer)],
        "MapBox.size should publish an Integer result type"
    );
    assert_eq!(
        clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}

#[test]
fn map_value_len_uses_unified_receiver_arg_shape_and_integer_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    local n = m.len()
    m.clear()
    return n
  }
}
"#;

    let module = compile_src(src);
    let len_arg_lens = method_call_arg_lens(&module, "MapBox", "len");
    let len_result_types = method_call_result_types(&module, "MapBox", "len");
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        len_arg_lens,
        vec![1],
        "MapBox.len should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        len_result_types,
        vec![Some(MirType::Integer)],
        "MapBox.len should publish an Integer result type"
    );
    assert_eq!(
        clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}

#[test]
fn map_value_length_alias_uses_unified_receiver_arg_shape_and_integer_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    local n = m.length()
    m.clear()
    return n
  }
}
"#;

    let module = compile_src(src);
    let length_arg_lens = method_call_arg_lens(&module, "MapBox", "length");
    let length_result_types = method_call_result_types(&module, "MapBox", "length");
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        length_arg_lens,
        vec![1],
        "MapBox.length should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        length_result_types,
        vec![Some(MirType::Integer)],
        "MapBox.length should publish an Integer result type"
    );
    assert_eq!(
        clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}

#[test]
fn map_value_has_uses_unified_receiver_arg_shape_and_bool_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    local h = m.has("k")
    m.clear()
    return h
  }
}
"#;

    let module = compile_src(src);
    let has_arg_lens = method_call_arg_lens(&module, "MapBox", "has");
    let has_result_types = method_call_result_types(&module, "MapBox", "has");
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        has_arg_lens,
        vec![2],
        "MapBox.has should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        has_result_types,
        vec![Some(MirType::Bool)],
        "MapBox.has should publish a Bool result type"
    );
    assert_eq!(
        clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}

#[test]
fn map_value_get_uses_unified_receiver_arg_shape_and_generic_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    m.clear()
    local g = m.get("k")
    return m.size()
  }
}
"#;

    let module = compile_src(src);
    let get_arg_lens = method_call_arg_lens(&module, "MapBox", "get");
    let get_result_types = method_call_result_types(&module, "MapBox", "get");
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        get_arg_lens,
        vec![2],
        "MapBox.get should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        get_result_types,
        vec![Some(MirType::Unknown)],
        "MapBox.get returns a data-dependent stored value and should stay MIR-unknown"
    );
    assert_eq!(
        clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}

#[test]
fn map_value_set_uses_unified_receiver_arg_shape_and_receipt_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    local s = m.set("k", 41)
    m.clear()
    return m.size()
  }
}
"#;

    let module = compile_src(src);
    let set_arg_lens = method_call_arg_lens(&module, "MapBox", "set");
    let set_result_types = method_call_result_types(&module, "MapBox", "set");
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        set_arg_lens,
        vec![3],
        "MapBox.set should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        set_result_types,
        vec![Some(MirType::String)],
        "MapBox.set publishes the landed receipt-string write-return contract"
    );
    assert_eq!(
        clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}

#[test]
fn map_value_delete_remove_use_unified_receiver_arg_shape_and_receipt_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    m.set("a", 41)
    local d = m.delete("a")
    m.set("b", 42)
    local r = m.remove("b")
    m.clear()
    return r
  }
}
"#;

    let module = compile_src(src);
    let delete_arg_lens = method_call_arg_lens(&module, "MapBox", "delete");
    let delete_result_types = method_call_result_types(&module, "MapBox", "delete");
    let remove_arg_lens = method_call_arg_lens(&module, "MapBox", "remove");
    let remove_result_types = method_call_result_types(&module, "MapBox", "remove");
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");

    assert_eq!(
        delete_arg_lens,
        vec![2],
        "MapBox.delete should use the Unified receiver-plus-key shape"
    );
    assert_eq!(
        delete_result_types,
        vec![Some(MirType::String)],
        "MapBox.delete publishes the landed receipt-string write-return contract"
    );
    assert_eq!(
        remove_arg_lens,
        vec![2],
        "MapBox.remove should use the Unified receiver-plus-key shape"
    );
    assert_eq!(
        remove_result_types,
        vec![Some(MirType::String)],
        "MapBox.remove publishes the landed receipt-string write-return contract"
    );
    assert_eq!(
        clear_arg_lens,
        vec![0],
        "MapBox.clear is still deferred and should stay on the BoxCall fallback shape"
    );
}
