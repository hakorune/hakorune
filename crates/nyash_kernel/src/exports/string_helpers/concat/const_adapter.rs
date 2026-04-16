use crate::exports::string_birth_placement::{
    concat_suffix_retention_class, insert_middle_retention_class, RetainedForm,
};
use crate::exports::string_literal_handle_from_text;
use crate::exports::string_plan::{
    concat_const_suffix_plan_from_handle, insert_const_mid_plan_from_handle,
};
use crate::observe;
use nyash_rust::runtime::host_handles as handles;
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    thread::LocalKey,
};

use super::super::cache::{
    concat_const_suffix_fast_cache_lookup, concat_const_suffix_fast_cache_store,
    with_cached_const_suffix_text,
};
use super::super::materialize::{
    concat_two_str, freeze_text_plan, string_handle_from_owned, string_is_empty_from_handle,
};

enum ConstSuffixPath {
    ReuseHandle(i64),
    Owned(String),
}

struct ConstCStringCache {
    ptr: Cell<usize>,
    handle: Cell<i64>,
    text: RefCell<Option<String>>,
}

#[inline(always)]
fn execute_concat2_freeze_from_text(a_h: i64, suffix: &str, placement: RetainedForm) -> i64 {
    observe::record_const_suffix_freeze_fallback();
    match placement {
        RetainedForm::ReturnHandle => {
            observe::record_birth_placement_return_handle();
            a_h
        }
        RetainedForm::KeepTransient | RetainedForm::MustFreeze(_) => {
            freeze_text_plan(concat_const_suffix_plan_from_handle(a_h, suffix))
        }
        RetainedForm::RetainView => unreachable!("concat_hs cannot retain a view"),
    }
}

#[inline(always)]
fn execute_const_suffix_contract(a_h: i64, suffix_ptr: *const i8) -> i64 {
    if suffix_ptr.is_null() {
        return a_h;
    }
    observe::record_const_suffix_enter();
    with_cached_const_suffix_text(suffix_ptr, |suffix| {
        let suffix_is_empty = suffix.is_empty();
        let placement = concat_suffix_retention_class(suffix_is_empty);
        if matches!(placement, RetainedForm::ReturnHandle) {
            observe::record_const_suffix_empty_return();
            observe::record_birth_placement_return_handle();
            return a_h;
        }
        if let Some(hit) = concat_const_suffix_fast_cache_lookup(a_h, suffix_ptr) {
            observe::record_const_suffix_cached_fast_str_hit();
            observe::record_birth_placement_return_handle();
            return hit;
        }
        if let Some(plan) = handles::with_text_read_session(|session| {
            session.str_handle(a_h as u64, |lhs| {
                if let Some(hit) = concat_const_suffix_fast_cache_lookup(a_h, suffix_ptr) {
                    observe::record_const_suffix_cached_fast_str_hit();
                    observe::record_birth_placement_return_handle();
                    return ConstSuffixPath::ReuseHandle(hit);
                }
                observe::record_const_suffix_freeze_fallback();
                ConstSuffixPath::Owned(concat_two_str(lhs, suffix))
            })
        }) {
            return match plan {
                ConstSuffixPath::ReuseHandle(handle) => handle,
                ConstSuffixPath::Owned(text) => {
                    let handle = string_handle_from_owned(text);
                    if handle > 0 {
                        concat_const_suffix_fast_cache_store(a_h, suffix_ptr, handle);
                    }
                    handle
                }
            };
        }
        execute_concat2_freeze_from_text(a_h, suffix, placement)
    })
}

#[inline(always)]
pub(super) fn concat_const_suffix_fallback(a_h: i64, suffix_ptr: *const i8) -> i64 {
    // phase-149x: keep `concat_hs` as the current concrete executor path, but
    // read this route as `.hako const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`.
    execute_const_suffix_contract(a_h, suffix_ptr)
}

#[inline(always)]
fn with_cached_const_text<R>(
    cache: &'static LocalKey<ConstCStringCache>,
    ptr: *const i8,
    f: impl FnOnce(&str) -> R,
) -> R {
    if ptr.is_null() {
        return f("");
    }
    let addr = ptr as usize;
    cache.with(|cache| {
        if cache.ptr.get() != addr || cache.text.borrow().is_none() {
            let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
            let text = String::from_utf8_lossy(bytes).into_owned();
            cache.ptr.set(addr);
            *cache.text.borrow_mut() = Some(text);
        }
        let text_ref = cache.text.borrow();
        f(text_ref.as_deref().unwrap_or(""))
    })
}

thread_local! {
    static CONST_INSERT_TEXT_CACHE: ConstCStringCache = const { ConstCStringCache {
        ptr: Cell::new(0),
        handle: Cell::new(0),
        text: RefCell::new(None),
    } };
}

#[inline(always)]
pub(super) fn insert_const_mid_fallback(source_h: i64, middle_ptr: *const i8, split: i64) -> i64 {
    with_cached_const_text(&CONST_INSERT_TEXT_CACHE, middle_ptr, |middle| {
        let source_is_empty = string_is_empty_from_handle(source_h) == Some(true);
        match insert_middle_retention_class(source_is_empty, middle.is_empty()) {
            RetainedForm::ReturnHandle => source_h,
            RetainedForm::KeepTransient | RetainedForm::MustFreeze(_) => {
                if source_is_empty {
                    let addr = middle_ptr as usize;
                    CONST_INSERT_TEXT_CACHE.with(|cache| {
                        if cache.ptr.get() == addr {
                            let cached = cache.handle.get();
                            if cached > 0 {
                                return cached;
                            }
                        }
                        let handle = string_literal_handle_from_text(middle);
                        if handle > 0 {
                            cache.ptr.set(addr);
                            cache.handle.set(handle);
                        }
                        handle
                    })
                } else {
                    freeze_text_plan(insert_const_mid_plan_from_handle(source_h, middle, split))
                }
            }
            RetainedForm::RetainView => unreachable!("insert_hsi cannot retain a view"),
        }
    })
}
