use super::*;

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
fn string_value_case_conversion_uses_unified_receiver_arg_shape_and_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local u = s.toUpper()
    local l = "BANANA".toLower()
    return l
  }
}
"#;

    let module = compile_src(src);
    let upper_arg_lens = method_call_arg_lens(&module, "StringBox", "toUpper");
    let upper_result_types = method_call_result_types(&module, "StringBox", "toUpper");
    let lower_arg_lens = method_call_arg_lens(&module, "StringBox", "toLower");
    let lower_result_types = method_call_result_types(&module, "StringBox", "toLower");

    assert_eq!(
        upper_arg_lens,
        vec![1],
        "StringBox.toUpper should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        upper_result_types,
        vec![Some(MirType::String)],
        "StringBox.toUpper should publish a String result type"
    );
    assert_eq!(
        lower_arg_lens,
        vec![1],
        "StringBox.toLower should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        lower_result_types,
        vec![Some(MirType::String)],
        "StringBox.toLower should publish a String result type"
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
fn string_value_starts_with_uses_unified_receiver_arg_shape_and_bool_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local s = "banana"
    local ok = s.startsWith("ban")
    return ok
  }
}
"#;

    let module = compile_src(src);
    let arg_lens = method_call_arg_lens(&module, "StringBox", "startsWith");
    let result_types = method_call_result_types(&module, "StringBox", "startsWith");

    assert_eq!(
        arg_lens,
        vec![2],
        "StringBox.startsWith should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        result_types,
        vec![Some(MirType::Bool)],
        "StringBox.startsWith should publish a Bool result type"
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
