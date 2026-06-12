use super::*;
use crate::c_string::cstring;
#[path = "string/common.rs"]
mod common;
#[path = "string/kernel_slot.rs"]
mod kernel_slot;
#[path = "string/runtime_data.rs"]
mod runtime_data;
#[path = "string/stage1.rs"]
mod stage1;
#[path = "string/string_ops.rs"]
mod string_ops;
use common::*;

#[test]
fn string_substring_concat3_publish_explicit_api_owned_materializes_string_box() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let a_h = string_handle("aa");
        let b_h = string_handle("CENTER");
        let c_h = string_handle("zz");

        let helper_h = nyash_string_substring_concat3_publish_explicit_api_owned_hhhii_export(
            a_h, b_h, c_h, 2, 8,
        );

        assert!(helper_h > 0);
        let object = handles::get(helper_h as u64).expect("explicit api publish handle");
        let string_box = object
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("stable-owned publish must materialize StringBox");
        assert_eq!(string_box.value, "CENTER");
        assert!(
            object
                .as_any()
                .downcast_ref::<crate::exports::string_view::StringViewBox>()
                .is_none(),
            "stable-owned publish must not leave a StringViewBox carrier"
        );
    });
}

#[test]
fn string_substring_concat3_publish_need_stable_owned_materializes_string_box() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let a_h = string_handle("aa");
        let b_h = string_handle("CENTER");
        let c_h = string_handle("zz");

        let helper_h = nyash_string_substring_concat3_publish_need_stable_owned_hhhii_export(
            a_h, b_h, c_h, 2, 8,
        );

        assert!(helper_h > 0);
        let object = handles::get(helper_h as u64).expect("need-stable publish handle");
        let string_box = object
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("stable-owned publish must materialize StringBox");
        assert_eq!(string_box.value, "CENTER");
        assert!(
            object
                .as_any()
                .downcast_ref::<crate::exports::string_view::StringViewBox>()
                .is_none(),
            "stable-owned publish must not leave a StringViewBox carrier"
        );
    });
}

#[test]
fn string_piecewise_subrange_hsiii_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let source_h = string_handle("prefix-suffix");
        let middle = cstring("::mid::");
        let inserted_h = nyash_string_insert_hsi_export(source_h, middle.as_ptr(), 6);
        let direct_h = nyash_string_substring_hii_export(inserted_h, 3, 16);
        let helper_h =
            nyash_string_piecewise_subrange_hsiii_export(source_h, middle.as_ptr(), 6, 3, 16);

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
    use crate::c_string::c_string_text;

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let c0 = nyash_string_to_i8p_h(0);
        assert!(!c0.is_null());
        let s0 = c_string_text(c0).expect("utf8");
        assert_eq!(s0, "0");

        let missing = 9_876_543_210_i64;
        let c_missing = nyash_string_to_i8p_h(missing);
        assert!(!c_missing.is_null());
        let s_missing = c_string_text(c_missing).expect("utf8");
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
    use crate::c_string::c_string_text;
    use nyash_rust::boxes::array::ArrayBox;

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
        let c_view = c_string_text(c_ptr).expect("substring utf8");
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
        let c_view = crate::c_string::c_string_text(c_ptr).expect("nested substring utf8");
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
fn substring_publish_explicit_api_view_hii_forces_stringview_replay_even_when_fast_off() {
    with_env_var("NYASH_LLVM_FAST", "0", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("line-seed-abcdefxx".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;

        let public_handle = nyash_string_substring_hii_export(source_handle, 1, 17);
        let public_obj = handles::get(public_handle as u64).expect("public substring object");
        assert_eq!(public_obj.type_name(), "StringBox");

        let view_handle =
            nyash_string_substring_publish_explicit_api_view_hii_export(source_handle, 1, 17);
        assert!(view_handle > 0, "explicit stable-view handle");

        let view_obj = handles::get(view_handle as u64).expect("explicit stable-view object");
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
