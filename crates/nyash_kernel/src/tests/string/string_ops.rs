use super::super::*;
use crate::c_string::cstring;
use crate::test_support::with_env_var;
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

#[test]
fn box_from_i8_string_const_reuses_handle() {
    let s = cstring("phase21_5_fast");
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
    let suffix = cstring("ln");
    let out_h = nyash_string_concat_hs_export(lhs_h, suffix.as_ptr());
    assert!(out_h > 0);
    let out = decode_string_like_handle(out_h).expect("concat_hs result");
    assert_eq!(out, "line-seedln");

    let empty = cstring("");
    let same_h = nyash_string_concat_hs_export(lhs_h, empty.as_ptr());
    assert_eq!(same_h, lhs_h, "empty suffix should reuse lhs handle");
}

#[test]
fn string_concat_hs_repeated_suffix_reuses_handle_for_same_source_text() {
    let lhs_h = string_handle("xyxyxyxyxyxyxyxy");
    let suffix = cstring("xy");

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
    let suffix = cstring("::tail");

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
    let middle = cstring("xx");
    let out_h = nyash_string_insert_hsi_export(source_h, middle.as_ptr(), 4);
    assert!(out_h > 0);
    let out = decode_string_like_handle(out_h).expect("insert_hsi result");
    assert_eq!(out, "linexx-seed");

    let utf8_source_h = string_handle("あい");
    let invalid_mid = nyash_string_insert_hsi_export(utf8_source_h, middle.as_ptr(), 1);
    let invalid_out = decode_string_like_handle(invalid_mid).expect("insert_hsi invalid boundary");
    assert_eq!(invalid_out, "xx");

    let empty = cstring("");
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
