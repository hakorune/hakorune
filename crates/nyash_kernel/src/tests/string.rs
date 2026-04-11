use super::*;

#[test]
fn box_from_i8_string_const_reuses_handle() {
    let s = CString::new("phase21_5_fast").expect("CString");
    let h1 = nyash_box_from_i8_string_const(s.as_ptr());
    let h2 = nyash_box_from_i8_string_const(s.as_ptr());
    assert!(h1 > 0);
    assert_eq!(h1, h2, "const helper should intern and reuse handle");
    assert!(handles::get(h1 as u64).is_some());
}

#[test]
fn string_concat3_hhh_contract() {
    let a: Arc<dyn NyashBox> = Arc::new(StringBox::new("ha".to_string()));
    let b: Arc<dyn NyashBox> = Arc::new(StringBox::new("ko".to_string()));
    let c: Arc<dyn NyashBox> = Arc::new(StringBox::new("run".to_string()));
    let a_h = handles::to_handle_arc(a) as i64;
    let b_h = handles::to_handle_arc(b) as i64;
    let c_h = handles::to_handle_arc(c) as i64;

    let out_h = nyash_string_concat3_hhh_export(a_h, b_h, c_h);
    assert!(out_h > 0);
    let out_obj = handles::get(out_h as u64).expect("concat3 result handle");
    let out_str = out_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("concat3 result must be StringBox");
    assert_eq!(out_str.value, "hakorun");

    // Fallback contract: invalid handle is treated as empty string.
    let out_h2 = nyash_string_concat3_hhh_export(a_h, 0, c_h);
    assert!(out_h2 > 0);
    let out_obj2 = handles::get(out_h2 as u64).expect("concat3 fallback handle");
    let out_str2 = out_obj2
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("concat3 fallback must be StringBox");
    assert_eq!(out_str2.value, "harun");
}

#[test]
fn string_concat3_hhh_repeated_triplet_route_roundtrip() {
    let base_h = string_handle("prefix-middle-suffix");
    let left_h = nyash_string_substring_hii_export(base_h, 0, 6);
    let middle_h = string_handle("::");
    let right_h = nyash_string_substring_hii_export(base_h, 14, 20);

    let out_h1 = nyash_string_concat3_hhh_export(left_h, middle_h, right_h);
    let out_h2 = nyash_string_concat3_hhh_export(left_h, middle_h, right_h);

    assert!(out_h1 > 0);
    assert!(out_h2 > 0);
    assert_eq!(
        decode_string_like_handle(out_h1).as_deref(),
        Some("prefix::suffix")
    );
    assert_eq!(
        decode_string_like_handle(out_h2).as_deref(),
        Some("prefix::suffix")
    );
}

#[test]
fn string_concat_hs_contract() {
    let lhs_h = string_handle("line-seed");
    let suffix = CString::new("ln").expect("CString");
    let out_h = nyash_string_concat_hs_export(lhs_h, suffix.as_ptr());
    assert!(out_h > 0);
    let out = decode_string_like_handle(out_h).expect("concat_hs result");
    assert_eq!(out, "line-seedln");

    let empty = CString::new("").expect("CString");
    let same_h = nyash_string_concat_hs_export(lhs_h, empty.as_ptr());
    assert_eq!(same_h, lhs_h, "empty suffix should reuse lhs handle");
}

#[test]
fn string_concat_hs_repeated_suffix_reuses_handle_for_same_source_text() {
    let lhs_h = string_handle("xyxyxyxyxyxyxyxy");
    let suffix = CString::new("xy").expect("CString");

    let out_h1 = nyash_string_concat_hs_export(lhs_h, suffix.as_ptr());
    let out_h2 = nyash_string_concat_hs_export(lhs_h, suffix.as_ptr());

    assert!(out_h1 > 0);
    assert!(out_h2 > 0);
    assert_eq!(
        out_h1, out_h2,
        "repeat concat_hs should reuse the same handle for stable source text"
    );
    assert_eq!(
        decode_string_like_handle(out_h1).as_deref(),
        Some("xyxyxyxyxyxyxyxyxy")
    );
}

#[test]
fn string_concat_hs_different_sources_do_not_share_global_const_handle() {
    let lhs_h1 = string_handle("phase21_5_concat_hs_source");
    let lhs_h2 = string_handle("phase21_5_concat_hs_source");
    let suffix = CString::new("::tail").expect("CString");

    assert_ne!(lhs_h1, lhs_h2, "fixture needs distinct source handles");

    let out_h1 = nyash_string_concat_hs_export(lhs_h1, suffix.as_ptr());
    let out_h2 = nyash_string_concat_hs_export(lhs_h2, suffix.as_ptr());

    assert!(out_h1 > 0);
    assert!(out_h2 > 0);
    assert_ne!(
        out_h1, out_h2,
        "dynamic concat_hs results should not be interned through the global literal cache"
    );
    assert_eq!(
        decode_string_like_handle(out_h1).as_deref(),
        Some("phase21_5_concat_hs_source::tail")
    );
    assert_eq!(
        decode_string_like_handle(out_h2).as_deref(),
        Some("phase21_5_concat_hs_source::tail")
    );
}

#[test]
fn string_concat_hh_repeated_pair_keeps_fresh_handles_and_text() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let lhs_h = string_handle("line-seed-abcdef");
        let rhs_h = string_handle("xy");

        let out_h1 = nyash_string_concat_hh_export(lhs_h, rhs_h);
        let out_h2 = nyash_string_concat_hh_export(lhs_h, rhs_h);

        assert!(out_h1 > 0);
        assert!(out_h2 > 0);
        assert_ne!(out_h1, out_h2, "fresh concat handles should stay fresh");
        assert_eq!(
            decode_string_like_handle(out_h1).as_deref(),
            Some("line-seed-abcdefxy")
        );
        assert_eq!(
            decode_string_like_handle(out_h2).as_deref(),
            Some("line-seed-abcdefxy")
        );
    });
}

#[test]
fn string_insert_hsi_contract() {
    let source_h = string_handle("line-seed");
    let middle = CString::new("xx").expect("CString");
    let out_h = nyash_string_insert_hsi_export(source_h, middle.as_ptr(), 4);
    assert!(out_h > 0);
    let out = decode_string_like_handle(out_h).expect("insert_hsi result");
    assert_eq!(out, "linexx-seed");

    let utf8_source_h = string_handle("あい");
    let invalid_mid = nyash_string_insert_hsi_export(utf8_source_h, middle.as_ptr(), 1);
    let invalid_out = decode_string_like_handle(invalid_mid).expect("insert_hsi invalid boundary");
    assert_eq!(invalid_out, "xx");

    let empty = CString::new("").expect("CString");
    let same_h = nyash_string_insert_hsi_export(source_h, empty.as_ptr(), 4);
    assert_eq!(same_h, source_h, "empty middle should reuse source handle");
}

#[test]
fn string_substring_concat_hhii_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let lhs_h = string_handle("line-seed");
        let rhs_h = string_handle("-abcdef");
        let direct_h =
            nyash_string_substring_hii_export(nyash_string_concat_hh_export(lhs_h, rhs_h), 2, 12);
        let helper_h = nyash_string_substring_concat_hhii_export(lhs_h, rhs_h, 2, 12);

        assert!(helper_h > 0);
        assert_eq!(
            decode_string_like_handle(helper_h),
            decode_string_like_handle(direct_h)
        );
        assert_eq!(nyash_string_len_h(helper_h), nyash_string_len_h(direct_h));
    });
}

#[test]
fn string_substring_concat3_hhhii_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let a_h = string_handle("line");
        let b_h = string_handle("-xx-");
        let c_h = string_handle("seed");
        let direct_h =
            nyash_string_substring_hii_export(nyash_string_concat3_hhh_export(a_h, b_h, c_h), 1, 8);
        let helper_h = nyash_string_substring_concat3_hhhii_export(a_h, b_h, c_h, 1, 8);

        assert!(helper_h > 0);
        assert_eq!(
            decode_string_like_handle(helper_h),
            decode_string_like_handle(direct_h)
        );
        assert_eq!(nyash_string_len_h(helper_h), nyash_string_len_h(direct_h));
    });
}

#[test]
fn string_compare_hh_contract_roundtrip() {
    let a: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
    let b: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
    let c: Arc<dyn NyashBox> = Arc::new(StringBox::new("abd".to_string()));
    let a_h = handles::to_handle_arc(a) as i64;
    let b_h = handles::to_handle_arc(b) as i64;
    let c_h = handles::to_handle_arc(c) as i64;

    assert_eq!(nyash_string_eq_hh_export(a_h, b_h), 1);
    assert_eq!(nyash_string_eq_hh_export(a_h, c_h), 0);
    assert_eq!(nyash_string_lt_hh_export(a_h, c_h), 1);
    assert_eq!(nyash_string_lt_hh_export(c_h, a_h), 0);
}

#[test]
fn string_indexof_lastindexof_invalid_needle_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("abcabc".to_string()));
        let hay_h = handles::to_handle_arc(hay) as i64;

        // Invalid/zero handle is treated as empty needle by current contract.
        assert_eq!(nyash_string_indexof_hh_export(hay_h, 0), 0);
        assert_eq!(nyash_string_lastindexof_hh_export(hay_h, 0), 6);
    });
}

#[test]
fn string_indexof_hh_cached_pair_route_roundtrip() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
        let hay_h = handles::to_handle_arc(hay) as i64;
        let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("b".to_string()));
        let needle_h = handles::to_handle_arc(needle) as i64;

        // Repeated pair lookup must preserve semantics.
        assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 1);
        assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 1);
    });
}

#[test]
fn string_len_h_invalid_handle_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        assert_eq!(nyash_string_len_h(0), 0);
        assert_eq!(nyash_string_len_h(-1), 0);
    });
}

#[test]
fn string_substring_len_hii_matches_substring_handle_length() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let source_h = string_handle("prefix-middle-suffix");
        let sub_h = nyash_string_substring_hii_export(source_h, 7, 13);

        assert!(sub_h > 0);
        assert_eq!(nyash_string_len_h(sub_h), 6);
        assert_eq!(nyash_string_substring_len_hii_export(source_h, 7, 13), 6);
        assert_eq!(nyash_string_substring_len_hii_export(source_h, 0, 6), 6);
        assert_eq!(nyash_string_substring_len_hii_export(source_h, 99, 100), 0);
    });
}

#[test]
fn string_substring_len_hii_complementary_ranges_sum_to_source_length() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let source_h = string_handle("prefix-middle-suffix");
        let total = nyash_string_len_h(source_h);

        for split in [-5_i64, 0, 3, 7, 13, 99] {
            let left = nyash_string_substring_len_hii_export(source_h, 0, split);
            let right = nyash_string_substring_len_hii_export(source_h, split, total);
            assert_eq!(
                left + right,
                total,
                "split={} should partition the clamped source length",
                split
            );
        }
    });
}

#[test]
fn string_exports_prefer_hako_forward_hook_when_registered() {
    extern "C" fn string_hook(op: i64, a0: i64, a1: i64, a2: i64) -> i64 {
        op * 1000 + a0 + a1 + a2
    }

    crate::hako_forward_bridge::with_test_reset(|| {
        assert_eq!(
            crate::hako_forward_bridge::register_string_dispatch(Some(string_hook)),
            1
        );
        assert_eq!(nyash_string_len_h(7), 1007);
        assert_eq!(nyash_string_concat_hh_export(3, 4), 3007);
    });
}

#[test]
fn string_exports_disable_rust_fallback_when_policy_is_off() {
    with_env_var("NYASH_VM_USE_FALLBACK", "0", || {
        crate::hako_forward_bridge::with_test_reset(|| {
            let src: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
            let src_h = handles::to_handle_arc(src) as i64;
            assert_eq!(
                nyash_string_len_h(src_h),
                crate::hako_forward_bridge::NYRT_E_HOOK_MISS
            );
            let concat_h = nyash_string_concat_hh_export(src_h, src_h);
            assert!(concat_h > 0);
            let concat_text =
                decode_string_like_handle(concat_h).expect("concat freeze handle string");
            assert!(concat_text.contains("[freeze:contract][hako_forward/hook_miss]"));
            assert!(concat_text.contains("route=string.concat_hh"));
        });
    });
}

#[test]
fn string_to_i8p_h_fallback_contract() {
    use std::ffi::CStr;

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let c0 = nyash_string_to_i8p_h(0);
        assert!(!c0.is_null());
        let s0 = unsafe { CStr::from_ptr(c0) }.to_str().expect("utf8");
        assert_eq!(s0, "0");

        let missing = 9_876_543_210_i64;
        let c_missing = nyash_string_to_i8p_h(missing);
        assert!(!c_missing.is_null());
        let s_missing = unsafe { CStr::from_ptr(c_missing) }.to_str().expect("utf8");
        assert_eq!(s_missing, missing.to_string());
    });
}

#[test]
fn string_indexof_lastindexof_single_byte_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("abba-bba".to_string()));
        let hay_h = handles::to_handle_arc(hay) as i64;
        let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("b".to_string()));
        let needle_h = handles::to_handle_arc(needle) as i64;

        assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 1);
        assert_eq!(nyash_string_lastindexof_hh_export(hay_h, needle_h), 6);
    });
}

#[test]
fn string_indexof_lastindexof_multibyte_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("hako-hako".to_string()));
        let hay_h = handles::to_handle_arc(hay) as i64;
        let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("ko".to_string()));
        let needle_h = handles::to_handle_arc(needle) as i64;

        assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 2);
        assert_eq!(nyash_string_lastindexof_hh_export(hay_h, needle_h), 7);
    });
}

#[test]
fn substring_hii_repeated_same_input_reuses_handle_for_view_contract() {
    with_env_var("NYASH_LLVM_FAST", "1", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("line-seed-abcdefxy".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;

        let view_h1 = nyash_string_substring_hii_export(source_handle, 2, 18);
        let view_h2 = nyash_string_substring_hii_export(source_handle, 2, 18);

        assert!(view_h1 > 0);
        assert!(view_h2 > 0);
        assert_eq!(
            view_h1, view_h2,
            "repeat substring should reuse the same handle for a stable view source"
        );
        assert_eq!(nyash_string_len_h(view_h1), 16);
    });
}

#[test]
fn substring_hii_short_slice_materializes_under_fast_contract() {
    use nyash_rust::boxes::array::ArrayBox;
    use std::ffi::CStr;

    with_env_var("NYASH_LLVM_FAST", "1", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("hakorune".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;
        let sub_handle = nyash_string_substring_hii_export(source_handle, 1, 5);
        assert!(sub_handle > 0, "substring handle");

        let sub_obj = handles::get(sub_handle as u64).expect("substring object");
        let sub_sb = sub_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("short fast slice should materialize to StringBox");
        assert_eq!(sub_sb.value, "akor");
        assert_eq!(nyash_string_len_h(sub_handle), 4);

        let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("ko".to_string()));
        let needle_handle = handles::to_handle_arc(needle) as i64;
        assert_eq!(nyash_string_indexof_hh_export(sub_handle, needle_handle), 1);

        let c_ptr = nyash_string_to_i8p_h(sub_handle);
        assert!(!c_ptr.is_null());
        let c_view = unsafe { CStr::from_ptr(c_ptr) }
            .to_str()
            .expect("substring utf8");
        assert_eq!(c_view, "akor");

        // Persistent container boundary still stores owned StringBox.
        let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let array_handle = handles::to_handle_arc(array) as i64;
        assert_eq!(nyash_runtime_data_push_hh(array_handle, sub_handle), 1);
        let stored_handle = nyash_runtime_data_get_hh(array_handle, 0);
        assert!(stored_handle > 0);
        let stored_obj = handles::get(stored_handle as u64).expect("stored object");
        let stored_sb = stored_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("stored value should materialize to StringBox");
        assert_eq!(stored_sb.value, "akor");
    });
}

#[test]
fn substring_hii_short_nested_slice_materializes_under_fast_contract() {
    with_env_var("NYASH_LLVM_FAST", "1", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("hakorune".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;
        let view_handle = nyash_string_substring_hii_export(source_handle, 1, 5);
        assert!(view_handle > 0, "view handle");

        let nested_handle = nyash_string_substring_hii_export(view_handle, 0, 2);
        assert!(nested_handle > 0, "nested substring handle");

        let nested_obj = handles::get(nested_handle as u64).expect("nested substring object");
        let nested_sb = nested_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("short nested slice should materialize to StringBox");
        assert_eq!(nested_sb.value, "ak");
        assert_eq!(nyash_string_len_h(nested_handle), 2);
        let c_ptr = nyash_string_to_i8p_h(nested_handle);
        assert!(!c_ptr.is_null());
        let c_view = unsafe { std::ffi::CStr::from_ptr(c_ptr) }
            .to_str()
            .expect("nested substring utf8");
        assert_eq!(c_view, "ak");
    });
}

#[test]
fn substring_hii_mid_slice_keeps_stringview_contract() {
    with_env_var("NYASH_LLVM_FAST", "1", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("line-seed-abcdefxx".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;

        let view_handle = nyash_string_substring_hii_export(source_handle, 1, 17);
        assert!(view_handle > 0, "mid substring handle");

        let view_obj = handles::get(view_handle as u64).expect("mid substring object");
        assert_eq!(view_obj.type_name(), "StringViewBox");
        assert_eq!(nyash_string_len_h(view_handle), 16);
    });
}

#[test]
fn substring_hii_fast_off_keeps_stringbox_contract() {
    with_env_var("NYASH_LLVM_FAST", "0", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("hakorune".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;
        let sub_handle = nyash_string_substring_hii_export(source_handle, 1, 5);
        assert!(sub_handle > 0, "substring handle");
        let sub_obj = handles::get(sub_handle as u64).expect("substring object");
        let sub_sb = sub_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("fast off should keep StringBox result");
        assert_eq!(sub_sb.value, "akor");
    });
}

#[test]
fn invoke_by_name_accepts_stage1_using_resolver_module_receiver() {
    let result_handle = dispatch_stage1_module(
        "lang.compiler.entry.using_resolver_box",
        "resolve_for_source",
        "static box Main { main() { return 0 } }",
    );
    assert!(result_handle > 0, "expected StringBox handle");

    let result_object = handles::get(result_handle as u64).expect("result handle");
    let result_string = result_object
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("StringBox result");
    assert_eq!(result_string.value, "");
}

#[test]
fn invoke_by_name_accepts_stage1_mir_builder_source_route_for_stage1_cli_env() {
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!("../../../../lang/src/runner/stage1_cli_env.hako"),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {}",
        mir_json
    );
    assert!(mir_json.contains("\"functions\""));
    let mir_value: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let user_box_decls = mir_value["user_box_decls"]
        .as_array()
        .expect("user_box_decls array");
    let box_names = user_box_decls
        .iter()
        .filter_map(|decl| decl["name"].as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        box_names.contains("Stage1InputContractBox"),
        "source authority route should expose Stage1InputContractBox user_box_decl"
    );
    assert!(
        box_names.contains("Stage1SourceMirAuthorityBox"),
        "source authority route should expose Stage1SourceMirAuthorityBox user_box_decl"
    );
    assert!(
        box_names.contains("Stage1ProgramJsonCompatBox"),
        "source authority route should preserve explicit compat box decls for same-file closure"
    );
}

#[test]
fn invoke_by_name_export_accepts_stage1_mir_builder_source_route_for_stage1_cli_env() {
    with_env_vars(
        &[
            ("HAKO_MIR_BUILDER_INTERNAL", "1"),
            ("NYASH_VM_USE_FALLBACK", "1"),
        ],
        || {
            let recv_handle = handles::to_handle_arc(Arc::new(StringBox::new(
                "lang.mir.builder.MirBuilderBox".to_string(),
            ))) as i64;
            let method = CString::new("emit_from_source_v0").expect("CString");
            let source_handle = handles::to_handle_arc(Arc::new(StringBox::new(
                include_str!("../../../../lang/src/runner/stage1_cli_env.hako").to_string(),
            ))) as i64;

            let result_handle =
                nyash_plugin_invoke_by_name_i64(recv_handle, method.as_ptr(), 1, source_handle, 0);
            assert!(result_handle > 0, "expected MIR JSON StringBox handle");

            let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
            assert!(
                mir_json.starts_with('{'),
                "expected MIR JSON payload, got: {}",
                mir_json
            );
            assert!(mir_json.contains("\"functions\""));
            let mir_value: serde_json::Value =
                serde_json::from_str(&mir_json).expect("valid mir json");
            let user_box_decls = mir_value["user_box_decls"]
                .as_array()
                .expect("user_box_decls array");
            let box_names = user_box_decls
                .iter()
                .filter_map(|decl| decl["name"].as_str())
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                box_names.contains("Stage1InputContractBox"),
                "source authority route should expose Stage1InputContractBox user_box_decl"
            );
            assert!(
                box_names.contains("Stage1SourceMirAuthorityBox"),
                "source authority route should expose Stage1SourceMirAuthorityBox user_box_decl"
            );
            assert!(
                box_names.contains("Stage1ProgramJsonCompatBox"),
                "source authority route should preserve explicit compat box decls for same-file closure"
            );
        },
    );
}

#[test]
fn invoke_by_name_accepts_stage1_mir_builder_source_route_for_hello_simple_llvm() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!("../../../../apps/tests/hello_simple_llvm.hako"),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {}",
        mir_json
    );
    assert!(mir_json.contains("\"functions\""));
}

#[test]
fn invoke_by_name_stage1_mir_builder_source_route_accepts_decode_escapes_nested_loop_fixture() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!(
            "../../../../apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"
        ),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {mir_json}"
    );
    assert!(mir_json.contains("\"functions\""));
}

#[test]
fn invoke_by_name_stage1_using_resolver_route_is_stubbed_empty_in_kernel_dispatch() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.compiler.entry.using_resolver_box",
        "resolve_for_source",
        include_str!("../../../../lang/src/runner/stage1_cli_env.hako"),
    );
    assert!(result_handle > 0, "expected stub StringBox handle");

    let prefix = decode_string_like_handle(result_handle).expect("prefix text");
    assert_eq!(
        prefix, "",
        "kernel direct module dispatch intentionally stubs resolve_for_source"
    );
}
