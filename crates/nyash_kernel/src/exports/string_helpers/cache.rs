use crate::observe;
use nyash_rust::box_trait::NyashBox;
use nyash_rust::runtime::host_handles as handles;
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    sync::Arc,
};

#[derive(Default)]
struct ConstSuffixTextCache {
    ptr: Cell<usize>,
    text: RefCell<Option<String>>,
}

#[derive(Default)]
struct ConcatPairFastCache {
    drop_epoch: Cell<u64>,
    lhs_handle: Cell<i64>,
    rhs_handle: Cell<i64>,
    result: RefCell<Option<Arc<dyn NyashBox>>>,
}

#[derive(Default)]
struct Concat3FastCache {
    drop_epoch: Cell<u64>,
    a_handle: Cell<i64>,
    b_handle: Cell<i64>,
    c_handle: Cell<i64>,
    result_handle: Cell<i64>,
}

#[derive(Default)]
struct ConcatConstSuffixFastCache {
    drop_epoch: Cell<u64>,
    source_handle: Cell<i64>,
    suffix_ptr: Cell<usize>,
    result_handle: Cell<i64>,
}

#[derive(Clone, Copy, Default)]
struct SubstringFastCacheState {
    drop_epoch: u64,
    source_handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    result_handle: i64,
    source_handle2: i64,
    start2: i64,
    end2: i64,
    view_enabled2: bool,
    result_handle2: i64,
}

#[derive(Clone, Copy, Default)]
struct StringLenFastCacheState {
    drop_epoch: u64,
    handle: i64,
    len: i64,
    handle2: i64,
    len2: i64,
}

#[derive(Default)]
struct SubstringViewArcCache {
    source_handle: Cell<i64>,
    start: Cell<i64>,
    end: Cell<i64>,
    view_enabled: Cell<bool>,
    len: Cell<i64>,
    source_obj: RefCell<Option<Arc<dyn NyashBox>>>,
    result_obj: RefCell<Option<Arc<dyn NyashBox>>>,
    source_handle2: Cell<i64>,
    start2: Cell<i64>,
    end2: Cell<i64>,
    view_enabled2: Cell<bool>,
    len2: Cell<i64>,
    source_obj2: RefCell<Option<Arc<dyn NyashBox>>>,
    result_obj2: RefCell<Option<Arc<dyn NyashBox>>>,
}

impl SubstringViewArcCache {
    #[inline(always)]
    fn clear_entry(&self, secondary: bool) {
        if secondary {
            self.source_handle2.set(0);
            self.start2.set(0);
            self.end2.set(0);
            self.view_enabled2.set(false);
            self.len2.set(0);
            *self.source_obj2.borrow_mut() = None;
            *self.result_obj2.borrow_mut() = None;
        } else {
            self.source_handle.set(0);
            self.start.set(0);
            self.end.set(0);
            self.view_enabled.set(false);
            self.len.set(0);
            *self.source_obj.borrow_mut() = None;
            *self.result_obj.borrow_mut() = None;
        }
    }

    #[inline(always)]
    fn matches_primary(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
        view_enabled: bool,
    ) -> bool {
        self.source_handle.get() == source_handle
            && self.start.get() == start
            && self.end.get() == end
            && self.view_enabled.get() == view_enabled
    }

    #[inline(always)]
    fn matches_secondary(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
        view_enabled: bool,
    ) -> bool {
        self.source_handle2.get() == source_handle
            && self.start2.get() == start
            && self.end2.get() == end
            && self.view_enabled2.get() == view_enabled
    }

    #[inline(always)]
    fn entry_hit(&self, source_handle: i64, secondary: bool) -> Option<(Arc<dyn NyashBox>, i64)> {
        let (source_obj, result_obj, len) = if secondary {
            (
                self.source_obj2.borrow().as_ref().cloned(),
                self.result_obj2.borrow().as_ref().cloned(),
                self.len2.get(),
            )
        } else {
            (
                self.source_obj.borrow().as_ref().cloned(),
                self.result_obj.borrow().as_ref().cloned(),
                self.len.get(),
            )
        };
        let (Some(source_obj), Some(result_obj)) = (source_obj, result_obj) else {
            self.clear_entry(secondary);
            return None;
        };
        let source_still_live = handles::with_handle(source_handle as u64, |obj| {
            obj.is_some_and(|current| Arc::ptr_eq(current, &source_obj))
        });
        if !source_still_live {
            self.clear_entry(secondary);
            return None;
        }
        Some((result_obj, len))
    }

    #[inline(always)]
    fn lookup(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
        view_enabled: bool,
    ) -> Option<(Arc<dyn NyashBox>, i64)> {
        if self.matches_primary(source_handle, start, end, view_enabled) {
            if let Some(hit) = self.entry_hit(source_handle, false) {
                return Some(hit);
            }
        }
        if self.matches_secondary(source_handle, start, end, view_enabled) {
            return self.entry_hit(source_handle, true);
        }
        None
    }

    #[inline(always)]
    fn store(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
        view_enabled: bool,
        len: i64,
        source_obj: Arc<dyn NyashBox>,
        result_obj: Arc<dyn NyashBox>,
    ) {
        let prev_source = self.source_obj.borrow().as_ref().cloned();
        let prev_result = self.result_obj.borrow().as_ref().cloned();
        self.source_handle2.set(self.source_handle.get());
        self.start2.set(self.start.get());
        self.end2.set(self.end.get());
        self.view_enabled2.set(self.view_enabled.get());
        self.len2.set(self.len.get());
        *self.source_obj2.borrow_mut() = prev_source;
        *self.result_obj2.borrow_mut() = prev_result;

        self.source_handle.set(source_handle);
        self.start.set(start);
        self.end.set(end);
        self.view_enabled.set(view_enabled);
        self.len.set(len);
        *self.source_obj.borrow_mut() = Some(source_obj);
        *self.result_obj.borrow_mut() = Some(result_obj);
    }
}

thread_local! {
    static CONST_SUFFIX_TEXT_CACHE: ConstSuffixTextCache = const { ConstSuffixTextCache {
        ptr: Cell::new(0),
        text: RefCell::new(None),
    } };
    static CONCAT_PAIR_FAST_CACHE: ConcatPairFastCache = const { ConcatPairFastCache {
        drop_epoch: Cell::new(0),
        lhs_handle: Cell::new(0),
        rhs_handle: Cell::new(0),
        result: RefCell::new(None),
    } };
    static CONCAT3_FAST_CACHE: Concat3FastCache = const { Concat3FastCache {
        drop_epoch: Cell::new(0),
        a_handle: Cell::new(0),
        b_handle: Cell::new(0),
        c_handle: Cell::new(0),
        result_handle: Cell::new(0),
    } };
    static CONCAT_CONST_SUFFIX_FAST_CACHE: ConcatConstSuffixFastCache =
        const { ConcatConstSuffixFastCache {
            drop_epoch: Cell::new(0),
            source_handle: Cell::new(0),
            suffix_ptr: Cell::new(0),
            result_handle: Cell::new(0),
        } };
    static SUBSTRING_FAST_CACHE: Cell<SubstringFastCacheState> =
        const { Cell::new(SubstringFastCacheState {
            drop_epoch: 0,
            source_handle: 0,
            start: 0,
            end: 0,
            view_enabled: false,
            result_handle: 0,
            source_handle2: 0,
            start2: 0,
            end2: 0,
            view_enabled2: false,
            result_handle2: 0,
        }) };
    static SUBSTRING_VIEW_ARC_CACHE: SubstringViewArcCache =
        const { SubstringViewArcCache {
            source_handle: Cell::new(0),
            start: Cell::new(0),
            end: Cell::new(0),
            view_enabled: Cell::new(false),
            len: Cell::new(0),
            source_obj: RefCell::new(None),
            result_obj: RefCell::new(None),
            source_handle2: Cell::new(0),
            start2: Cell::new(0),
            end2: Cell::new(0),
            view_enabled2: Cell::new(false),
            len2: Cell::new(0),
            source_obj2: RefCell::new(None),
            result_obj2: RefCell::new(None),
        } };
    static STRING_LEN_FAST_CACHE: Cell<StringLenFastCacheState> =
        const { Cell::new(StringLenFastCacheState {
            drop_epoch: 0,
            handle: 0,
            len: 0,
            handle2: 0,
            len2: 0,
        }) };
}

#[inline(always)]
pub(super) fn with_cached_const_suffix_text<R>(ptr: *const i8, f: impl FnOnce(&str) -> R) -> R {
    if ptr.is_null() {
        return f("");
    }
    let addr = ptr as usize;
    CONST_SUFFIX_TEXT_CACHE.with(|cache| {
        if cache.ptr.get() != addr || cache.text.borrow().is_none() {
            let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
            let text = String::from_utf8_lossy(bytes).into_owned();
            observe::record_const_suffix_text_cache_reload();
            cache.ptr.set(addr);
            *cache.text.borrow_mut() = Some(text);
        }
        let text_ref = cache.text.borrow();
        f(text_ref.as_deref().unwrap_or(""))
    })
}

#[inline(always)]
pub(super) fn concat_pair_fast_cache_lookup(a_h: i64, b_h: i64) -> Option<Arc<dyn NyashBox>> {
    let drop_epoch = handles::drop_epoch();
    CONCAT_PAIR_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() != drop_epoch
            || cache.lhs_handle.get() != a_h
            || cache.rhs_handle.get() != b_h
        {
            return None;
        }
        let result = cache.result.borrow();
        result.as_ref().cloned()
    })
}

#[inline(always)]
pub(super) fn concat_pair_fast_cache_store(a_h: i64, b_h: i64, result: Arc<dyn NyashBox>) {
    let drop_epoch = handles::drop_epoch();
    CONCAT_PAIR_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.lhs_handle.set(a_h);
        cache.rhs_handle.set(b_h);
        *cache.result.borrow_mut() = Some(result);
    });
}

#[inline(always)]
pub(super) fn concat3_fast_cache_lookup(a_h: i64, b_h: i64, c_h: i64) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    CONCAT3_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() == drop_epoch
            && cache.a_handle.get() == a_h
            && cache.b_handle.get() == b_h
            && cache.c_handle.get() == c_h
            && cache.result_handle.get() > 0
        {
            Some(cache.result_handle.get())
        } else {
            None
        }
    })
}

#[inline(always)]
pub(super) fn concat3_fast_cache_store(a_h: i64, b_h: i64, c_h: i64, result_handle: i64) {
    let drop_epoch = handles::drop_epoch();
    CONCAT3_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.a_handle.set(a_h);
        cache.b_handle.set(b_h);
        cache.c_handle.set(c_h);
        cache.result_handle.set(result_handle);
    });
}

#[inline(always)]
pub(super) fn concat_const_suffix_fast_cache_lookup(
    source_handle: i64,
    suffix_ptr: *const i8,
) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    CONCAT_CONST_SUFFIX_FAST_CACHE.with(|cache| {
        if cache.drop_epoch.get() == drop_epoch
            && cache.source_handle.get() == source_handle
            && cache.suffix_ptr.get() == suffix_ptr as usize
            && cache.result_handle.get() > 0
        {
            Some(cache.result_handle.get())
        } else {
            None
        }
    })
}

#[inline(always)]
pub(super) fn concat_const_suffix_fast_cache_store(
    source_handle: i64,
    suffix_ptr: *const i8,
    result_handle: i64,
) {
    let drop_epoch = handles::drop_epoch();
    CONCAT_CONST_SUFFIX_FAST_CACHE.with(|cache| {
        cache.drop_epoch.set(drop_epoch);
        cache.source_handle.set(source_handle);
        cache.suffix_ptr.set(suffix_ptr as usize);
        cache.result_handle.set(result_handle);
    });
}

#[inline(always)]
pub(super) fn substring_fast_cache_lookup(
    source_handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    SUBSTRING_FAST_CACHE.with(|cache| {
        let state = cache.get();
        if state.drop_epoch == drop_epoch {
            if state.source_handle == source_handle
                && state.start == start
                && state.end == end
                && state.view_enabled == view_enabled
                && state.result_handle > 0
            {
                return Some(state.result_handle);
            }
            if state.source_handle2 == source_handle
                && state.start2 == start
                && state.end2 == end
                && state.view_enabled2 == view_enabled
                && state.result_handle2 > 0
            {
                return Some(state.result_handle2);
            }
        }
        None
    })
}

#[inline(always)]
pub(super) fn substring_fast_cache_store(
    source_handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    result_handle: i64,
) {
    let drop_epoch = handles::drop_epoch();
    SUBSTRING_FAST_CACHE.with(|cache| {
        let state = cache.get();
        cache.set(SubstringFastCacheState {
            drop_epoch,
            source_handle,
            start,
            end,
            view_enabled,
            result_handle,
            source_handle2: state.source_handle,
            start2: state.start,
            end2: state.end,
            view_enabled2: state.view_enabled,
            result_handle2: state.result_handle,
        });
    });
}

#[inline(always)]
pub(super) fn substring_view_arc_cache_lookup(
    source_handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
) -> Option<(Arc<dyn NyashBox>, i64)> {
    SUBSTRING_VIEW_ARC_CACHE.with(|cache| cache.lookup(source_handle, start, end, view_enabled))
}

#[inline(always)]
pub(super) fn substring_view_arc_cache_store(
    source_handle: i64,
    start: i64,
    end: i64,
    view_enabled: bool,
    len: i64,
    source_obj: Arc<dyn NyashBox>,
    result_obj: Arc<dyn NyashBox>,
) {
    SUBSTRING_VIEW_ARC_CACHE.with(|cache| {
        cache.store(
            source_handle,
            start,
            end,
            view_enabled,
            len,
            source_obj,
            result_obj,
        );
    });
}

#[inline(always)]
pub(super) fn string_len_fast_cache_lookup(handle: i64) -> Option<i64> {
    let drop_epoch = handles::drop_epoch();
    STRING_LEN_FAST_CACHE.with(|cache| {
        let state = cache.get();
        if state.drop_epoch == drop_epoch {
            if state.handle == handle {
                Some(state.len)
            } else if state.handle2 == handle {
                Some(state.len2)
            } else {
                None
            }
        } else {
            None
        }
    })
}

#[inline(always)]
pub(super) fn string_len_fast_cache_store(handle: i64, len: i64) {
    let drop_epoch = handles::drop_epoch();
    STRING_LEN_FAST_CACHE.with(|cache| {
        let state = cache.get();
        cache.set(StringLenFastCacheState {
            drop_epoch,
            handle,
            len,
            handle2: state.handle,
            len2: state.len,
        });
    });
}
