use super::*;

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
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
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
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
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
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
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
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
    );
}

#[test]
fn map_value_get_existing_key_uses_unified_receiver_arg_shape_and_stored_value_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    m.set("k", 41)
    local g = m.get("k")
    return m.size()
  }
}
"#;

    let module = compile_src(src);
    let get_arg_lens = method_call_arg_lens(&module, "MapBox", "get");
    let get_result_types = method_call_result_types(&module, "MapBox", "get");

    assert_eq!(
        get_arg_lens,
        vec![2],
        "MapBox.get should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        get_result_types,
        vec![Some(MirType::Integer)],
        "MapBox.get should publish the stored value type for a known existing key"
    );
}

#[test]
fn map_value_get_missing_key_stays_unknown_after_typed_write() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    m.set("present", 41)
    local g = m.get("missing")
    return m.size()
  }
}
"#;

    let module = compile_src(src);
    let get_result_types = method_call_result_types(&module, "MapBox", "get");

    assert_eq!(
        get_result_types,
        vec![Some(MirType::Unknown)],
        "MapBox.get(missing-key) should preserve the landed Unknown contract"
    );
}

#[test]
fn map_value_get_mixed_value_results_stay_unknown() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    m.set("a", 41)
    m.set("b", "forty-two")
    local g = m.get("a")
    return m.size()
  }
}
"#;

    let module = compile_src(src);
    let get_result_types = method_call_result_types(&module, "MapBox", "get");

    assert_eq!(
        get_result_types,
        vec![Some(MirType::Unknown)],
        "mixed MapBox.get should preserve the Unknown contract"
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
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
    );
}

#[test]
fn map_value_clear_uses_unified_receiver_arg_shape_and_receipt_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local m = new MapBox()
    m.set("k", 41)
    local c = m.clear()
    return c
  }
}
"#;

    let module = compile_src(src);
    let clear_arg_lens = method_call_arg_lens(&module, "MapBox", "clear");
    let clear_result_types = method_call_result_types(&module, "MapBox", "clear");

    assert_eq!(
        clear_arg_lens,
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
    );
    assert_eq!(
        clear_result_types,
        vec![Some(MirType::String)],
        "MapBox.clear publishes the landed receipt-string write-return contract"
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
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
    );
}
