use super::*;

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
fn array_value_slice_result_followup_uses_arraybox_receiver_path() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    a.push(8)
    local s = a.slice(0, 1)
    local n = s.length()
    return n
  }
}
"#;

    let module = compile_src(src);
    let slice_arg_lens = method_call_arg_lens(&module, "ArrayBox", "slice");
    let slice_result_types = method_call_result_types(&module, "ArrayBox", "slice");
    let array_length_arg_lens = method_call_arg_lens(&module, "ArrayBox", "length");
    let runtime_data_length_arg_lens = method_call_arg_lens(&module, "RuntimeDataBox", "length");

    assert_eq!(
        slice_arg_lens,
        vec![3],
        "ArrayBox.slice should keep the Unified receiver-plus-start-plus-end shape"
    );
    assert_eq!(
        slice_result_types,
        vec![Some(MirType::Box("ArrayBox".to_string()))],
        "ArrayBox.slice should publish an ArrayBox result type for follow-up calls"
    );
    assert_eq!(
        array_length_arg_lens,
        vec![1],
        "slice().length() should use the ArrayBox receiver path"
    );
    assert!(
        runtime_data_length_arg_lens.is_empty(),
        "slice().length() must not degrade to RuntimeDataBox.length"
    );
}

#[test]
fn array_value_get_uses_unified_receiver_arg_shape_and_element_return() {
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
        vec![Some(MirType::Integer)],
        "ArrayBox.get should publish the element type for a known Array<Integer>"
    );
}

#[test]
fn array_value_pop_uses_unified_receiver_arg_shape_and_element_return() {
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
        vec![Some(MirType::Integer)],
        "ArrayBox.pop should publish the element type for a known Array<Integer>"
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
fn array_value_clear_uses_unified_receiver_arg_shape_and_void_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local c = a.clear()
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let clear_arg_lens = method_call_arg_lens(&module, "ArrayBox", "clear");
    let clear_result_types = method_call_result_types(&module, "ArrayBox", "clear");

    assert_eq!(
        clear_arg_lens,
        vec![1],
        "ArrayBox.clear should use the Unified receiver-only shape"
    );
    assert_eq!(
        clear_result_types,
        vec![Some(MirType::Void)],
        "ArrayBox.clear should publish a Void result type"
    );
}

#[test]
fn array_value_contains_uses_unified_receiver_arg_shape_and_bool_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local ok = a.contains(7)
    return ok
  }
}
"#;

    let module = compile_src(src);
    let contains_arg_lens = method_call_arg_lens(&module, "ArrayBox", "contains");
    let contains_result_types = method_call_result_types(&module, "ArrayBox", "contains");

    assert_eq!(
        contains_arg_lens,
        vec![2],
        "ArrayBox.contains should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        contains_result_types,
        vec![Some(MirType::Bool)],
        "ArrayBox.contains should publish a Bool result type"
    );
}

#[test]
fn array_value_index_of_uses_unified_receiver_arg_shape_and_integer_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    local idx = a.indexOf(7)
    return idx
  }
}
"#;

    let module = compile_src(src);
    let index_arg_lens = method_call_arg_lens(&module, "ArrayBox", "indexOf");
    let index_result_types = method_call_result_types(&module, "ArrayBox", "indexOf");

    assert_eq!(
        index_arg_lens,
        vec![2],
        "ArrayBox.indexOf should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        index_result_types,
        vec![Some(MirType::Integer)],
        "ArrayBox.indexOf should publish an Integer result type"
    );
}

#[test]
fn array_value_join_uses_unified_receiver_arg_shape_and_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    a.push(8)
    local text = a.join("-")
    return text
  }
}
"#;

    let module = compile_src(src);
    let join_arg_lens = method_call_arg_lens(&module, "ArrayBox", "join");
    let join_result_types = method_call_result_types(&module, "ArrayBox", "join");

    assert_eq!(
        join_arg_lens,
        vec![2],
        "ArrayBox.join should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        join_result_types,
        vec![Some(MirType::String)],
        "ArrayBox.join should publish a String result type"
    );
}

#[test]
fn array_value_reverse_uses_unified_receiver_shape_and_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    a.push(8)
    local receipt = a.reverse()
    return receipt
  }
}
"#;

    let module = compile_src(src);
    let reverse_arg_lens = method_call_arg_lens(&module, "ArrayBox", "reverse");
    let reverse_result_types = method_call_result_types(&module, "ArrayBox", "reverse");

    assert_eq!(
        reverse_arg_lens,
        vec![1],
        "ArrayBox.reverse should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        reverse_result_types,
        vec![Some(MirType::String)],
        "ArrayBox.reverse should publish a String receipt result type"
    );
}

#[test]
fn array_value_sort_uses_unified_receiver_shape_and_string_return() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(8)
    a.push(7)
    local receipt = a.sort()
    return receipt
  }
}
"#;

    let module = compile_src(src);
    let sort_arg_lens = method_call_arg_lens(&module, "ArrayBox", "sort");
    let sort_result_types = method_call_result_types(&module, "ArrayBox", "sort");

    assert_eq!(
        sort_arg_lens,
        vec![1],
        "ArrayBox.sort should use the Unified method-call shape with receiver in args"
    );
    assert_eq!(
        sort_result_types,
        vec![Some(MirType::String)],
        "ArrayBox.sort should publish a String receipt result type"
    );
}

#[test]
fn array_value_remove_uses_unified_receiver_arg_shape_and_element_return() {
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
        vec![Some(MirType::Integer)],
        "ArrayBox.remove should publish the element type for a known Array<Integer>"
    );
}

#[test]
fn array_value_mixed_element_results_stay_unknown() {
    let _features = EnvGuard::set("NYASH_FEATURES", "stage3");
    let _unified = EnvGuard::set("NYASH_MIR_UNIFIED_CALL", "1");
    let src = r#"
static box Main {
  main() {
    local a = new ArrayBox()
    a.push(7)
    a.push("seven")
    local x = a.get(0)
    local p = a.pop()
    a.push(9)
    local r = a.remove(0)
    return a.length()
  }
}
"#;

    let module = compile_src(src);
    let get_result_types = method_call_result_types(&module, "ArrayBox", "get");
    let pop_result_types = method_call_result_types(&module, "ArrayBox", "pop");
    let remove_result_types = method_call_result_types(&module, "ArrayBox", "remove");

    assert_eq!(
        get_result_types,
        vec![Some(MirType::Unknown)],
        "mixed ArrayBox.get should preserve the previous Unknown contract"
    );
    assert_eq!(
        pop_result_types,
        vec![Some(MirType::Unknown)],
        "mixed ArrayBox.pop should preserve the previous Unknown contract"
    );
    assert_eq!(
        remove_result_types,
        vec![Some(MirType::Unknown)],
        "mixed ArrayBox.remove should preserve the previous Unknown contract"
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
        vec![1],
        "MapBox.clear should use the Unified receiver-only shape"
    );
}
