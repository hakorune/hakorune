use crate::exports::string_birth_placement::{
    concat_suffix_retention_class, insert_middle_retention_class, RetainedForm,
};
use crate::exports::string_literal_handle_from_text;
use crate::exports::string_plan::{
    concat_const_suffix_plan_from_handle, insert_const_mid_plan_from_handle,
};
use crate::observe;
use crate::plugin::{freeze_owned_string_into_slot, KernelTextSlot};
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
    concat_two_str, freeze_text_plan, freeze_text_plan_with_site,
    string_handle_from_owned_const_suffix, string_is_empty_from_handle, to_owned_string_handle_arg,
};
use crate::plugin::StringPublishSite;

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
        RetainedForm::KeepTransient | RetainedForm::MustFreeze(_) => freeze_text_plan_with_site(
            concat_const_suffix_plan_from_handle(a_h, suffix),
            StringPublishSite::ConstSuffix,
        ),
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
                    let handle = string_handle_from_owned_const_suffix(text);
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
fn materialize_concat_const_suffix_borrowed(lhs: &str, suffix: &str) -> String {
    if lhs.is_empty() {
        return suffix.to_owned();
    }
    if suffix.is_empty() {
        return lhs.to_owned();
    }
    concat_two_str(lhs, suffix)
}

#[inline(always)]
fn materialize_insert_const_mid_borrowed(source: &str, middle: &str, split: i64) -> String {
    if source.is_empty() {
        return middle.to_owned();
    }
    if middle.is_empty() {
        return source.to_owned();
    }
    let split = split.clamp(0, source.len() as i64) as usize;
    let prefix = source.get(0..split).unwrap_or("");
    let suffix = source.get(split..).unwrap_or("");
    let total = prefix.len() + middle.len() + suffix.len();
    let mut out = String::with_capacity(total);
    unsafe {
        let buf = out.as_mut_vec();
        buf.set_len(total);
        let mut cursor = 0usize;
        std::ptr::copy_nonoverlapping(prefix.as_ptr(), buf.as_mut_ptr().add(cursor), prefix.len());
        cursor += prefix.len();
        std::ptr::copy_nonoverlapping(middle.as_ptr(), buf.as_mut_ptr().add(cursor), middle.len());
        cursor += middle.len();
        std::ptr::copy_nonoverlapping(suffix.as_ptr(), buf.as_mut_ptr().add(cursor), suffix.len());
    }
    out
}

#[inline(always)]
pub(super) fn concat_const_suffix_into_slot(
    slot: &mut KernelTextSlot,
    a_h: i64,
    suffix_ptr: *const i8,
) -> bool {
    slot.clear();
    if suffix_ptr.is_null() {
        freeze_owned_string_into_slot(slot, to_owned_string_handle_arg(a_h));
        return true;
    }
    observe::record_const_suffix_enter();
    with_cached_const_suffix_text(suffix_ptr, |suffix| {
        if suffix.is_empty() {
            observe::record_const_suffix_empty_return();
            freeze_owned_string_into_slot(slot, to_owned_string_handle_arg(a_h));
            return true;
        }
        if a_h > 0 {
            slot.replace_deferred_const_suffix(a_h, suffix_ptr);
            return true;
        }
        let lhs = to_owned_string_handle_arg(a_h);
        freeze_owned_string_into_slot(
            slot,
            materialize_concat_const_suffix_borrowed(lhs.as_str(), suffix),
        );
        true
    })
}

#[inline(always)]
pub(super) fn insert_const_mid_into_slot(
    slot: &mut KernelTextSlot,
    source_h: i64,
    middle_ptr: *const i8,
    split: i64,
) -> bool {
    slot.clear();
    with_insert_middle_text(middle_ptr, |middle| {
        if let Some(hit) = handles::with_text_read_session_ready(|session| {
            session.str_handle(source_h as u64, |source| {
                freeze_owned_string_into_slot(
                    slot,
                    materialize_insert_const_mid_borrowed(source, middle, split),
                );
                true
            })
        })
        .flatten()
        {
            return hit;
        }
        let source = to_owned_string_handle_arg(source_h);
        freeze_owned_string_into_slot(
            slot,
            materialize_insert_const_mid_borrowed(source.as_str(), middle, split),
        );
        true
    })
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
pub(super) fn with_insert_middle_text<R>(middle_ptr: *const i8, f: impl FnOnce(&str) -> R) -> R {
    with_cached_const_text(&CONST_INSERT_TEXT_CACHE, middle_ptr, f)
}

#[inline(always)]
pub(super) fn insert_const_mid_fallback(source_h: i64, middle_ptr: *const i8, split: i64) -> i64 {
    with_insert_middle_text(middle_ptr, |middle| {
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
