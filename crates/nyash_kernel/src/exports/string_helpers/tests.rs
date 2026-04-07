use super::cache::{
    concat_const_suffix_fast_cache_lookup, concat_const_suffix_fast_cache_store,
    concat_pair_fast_cache_lookup, concat_pair_fast_cache_store, string_len_fast_cache_lookup,
    string_len_fast_cache_store, substring_fast_cache_lookup, substring_fast_cache_store,
};
use super::materialize::string_handle_from_owned;
use super::{string_len_from_handle, string_substring_hii_export_impl};
use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::runtime::host_handles as handles;
use std::{ffi::CString, sync::Arc};

#[test]
fn concat_pair_fast_cache_invalidates_on_drop_epoch() {
    let lhs: Arc<dyn NyashBox> = Arc::new(StringBox::new("lhs-cache".to_string()));
    let rhs: Arc<dyn NyashBox> = Arc::new(StringBox::new("rhs-cache".to_string()));
    let result: Arc<dyn NyashBox> = Arc::new(StringBox::new("out-cache".to_string()));
    let lhs_h = handles::to_handle_arc(lhs) as i64;
    let rhs_h = handles::to_handle_arc(rhs) as i64;

    concat_pair_fast_cache_store(lhs_h, rhs_h, result.clone());
    assert!(concat_pair_fast_cache_lookup(lhs_h, rhs_h).is_some());

    handles::drop_handle(lhs_h as u64);
    assert!(concat_pair_fast_cache_lookup(lhs_h, rhs_h).is_none());
}

#[test]
fn const_suffix_fast_cache_invalidates_on_drop_epoch() {
    let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("source-cache".to_string()));
    let source_h = handles::to_handle_arc(source) as i64;
    let suffix = CString::new("xy").expect("CString");
    let suffix_ptr = suffix.as_ptr();

    concat_const_suffix_fast_cache_store(source_h, suffix_ptr, 77);
    assert_eq!(
        concat_const_suffix_fast_cache_lookup(source_h, suffix_ptr),
        Some(77)
    );

    handles::drop_handle(source_h as u64);
    assert_eq!(
        concat_const_suffix_fast_cache_lookup(source_h, suffix_ptr),
        None
    );
}

#[test]
fn substring_fast_cache_invalidates_on_drop_epoch() {
    let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("substring-cache".to_string()));
    let source_h = handles::to_handle_arc(source) as i64;

    substring_fast_cache_store(source_h, 2, 6, false, 88);
    assert_eq!(substring_fast_cache_lookup(source_h, 2, 6, false), Some(88));

    handles::drop_handle(source_h as u64);
    assert_eq!(substring_fast_cache_lookup(source_h, 2, 6, false), None);
}

#[test]
fn substring_fast_cache_keeps_two_recent_slices_hot() {
    let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("substring-cache".to_string()));
    let source_h = handles::to_handle_arc(source) as i64;

    substring_fast_cache_store(source_h, 0, 4, true, 101);
    substring_fast_cache_store(source_h, 4, 8, true, 202);

    assert_eq!(substring_fast_cache_lookup(source_h, 0, 4, true), Some(101));
    assert_eq!(substring_fast_cache_lookup(source_h, 4, 8, true), Some(202));
}

#[test]
fn string_len_fast_cache_keeps_two_recent_handles_hot() {
    let a: Arc<dyn NyashBox> = Arc::new(StringBox::new("abcd".to_string()));
    let b: Arc<dyn NyashBox> = Arc::new(StringBox::new("ef".to_string()));
    let a_h = handles::to_handle_arc(a) as i64;
    let b_h = handles::to_handle_arc(b) as i64;

    string_len_fast_cache_store(a_h, 4);
    string_len_fast_cache_store(b_h, 2);

    assert_eq!(string_len_fast_cache_lookup(a_h), Some(4));
    assert_eq!(string_len_fast_cache_lookup(b_h), Some(2));
}

#[test]
fn string_handle_from_owned_seeds_len_cache() {
    let handle = string_handle_from_owned("abcd".to_string());

    assert_eq!(string_len_fast_cache_lookup(handle), Some(4));
    assert_eq!(string_len_from_handle(handle), Some(4));
}

#[test]
fn substring_view_result_seeds_len_cache() {
    let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("substring-cache".to_string()));
    let source_h = handles::to_handle_arc(source) as i64;
    let result = string_substring_hii_export_impl(source_h, 0, 12);

    assert!(result > 0);
    assert_eq!(string_len_fast_cache_lookup(result), Some(12));
    assert_eq!(string_len_from_handle(result), Some(12));
}

#[test]
fn substring_view_arc_cache_reissues_same_view_object_after_drop_epoch() {
    let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("line-seed-abcdefxy".to_string()));
    let source_h = handles::to_handle_arc(source) as i64;

    let view_h1 = string_substring_hii_export_impl(source_h, 2, 18);
    let view_obj1 = handles::get(view_h1 as u64).expect("first substring view object");
    handles::drop_handle(view_h1 as u64);

    let view_h2 = string_substring_hii_export_impl(source_h, 2, 18);
    let view_obj2 = handles::get(view_h2 as u64).expect("reissued substring view object");

    assert!(
        Arc::ptr_eq(&view_obj1, &view_obj2),
        "cached view object should survive transient handle drops while the source stays live"
    );
    assert_eq!(string_len_fast_cache_lookup(view_h2), Some(16));
    assert_eq!(string_len_from_handle(view_h2), Some(16));
}
