use crate::observe;
use nyash_rust::box_trait::NyashBox;
use nyash_rust::runtime::host_handles as handles;
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    sync::Arc,
};

pub(super) enum SubstringViewCacheHit {
    Handle(i64),
    Reissue {
        result_obj: Arc<dyn NyashBox>,
        len: i64,
    },
}

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

#[derive(Default)]
struct StringLenFastCache {
    drop_epoch: Cell<u64>,
    handle: Cell<i64>,
    len: Cell<i64>,
    handle2: Cell<i64>,
    len2: Cell<i64>,
}

#[derive(Default)]
struct SubstringViewArcCache {
    source_handle: Cell<i64>,
    source_box_id: Cell<u64>,
    start: Cell<i64>,
    end: Cell<i64>,
    result_drop_epoch: Cell<u64>,
    result_handle: Cell<i64>,
    len: Cell<i64>,
    result_obj: RefCell<Option<Arc<dyn NyashBox>>>,
    source_handle2: Cell<i64>,
    source_box_id2: Cell<u64>,
    start2: Cell<i64>,
    end2: Cell<i64>,
    result_drop_epoch2: Cell<u64>,
    result_handle2: Cell<i64>,
    len2: Cell<i64>,
    result_obj2: RefCell<Option<Arc<dyn NyashBox>>>,
}

impl SubstringViewArcCache {
    #[inline(always)]
    fn clear_entry(&self, secondary: bool) {
        if secondary {
            self.source_handle2.set(0);
            self.source_box_id2.set(0);
            self.start2.set(0);
            self.end2.set(0);
            self.result_drop_epoch2.set(0);
            self.result_handle2.set(0);
            self.len2.set(0);
            *self.result_obj2.borrow_mut() = None;
        } else {
            self.source_handle.set(0);
            self.source_box_id.set(0);
            self.start.set(0);
            self.end.set(0);
            self.result_drop_epoch.set(0);
            self.result_handle.set(0);
            self.len.set(0);
            *self.result_obj.borrow_mut() = None;
        }
    }

    #[inline(always)]
    fn matches_primary(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
    ) -> bool {
        self.source_handle.get() == source_handle
            && self.start.get() == start
            && self.end.get() == end
    }

    #[inline(always)]
    fn matches_secondary(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
    ) -> bool {
        self.source_handle2.get() == source_handle
            && self.start2.get() == start
            && self.end2.get() == end
    }

    #[inline(always)]
    fn entry_hit(
        &self,
        source_handle: i64,
        current_drop_epoch: u64,
        secondary: bool,
    ) -> Option<SubstringViewCacheHit> {
        let (result_handle, result_drop_epoch, len) = if secondary {
            (
                self.result_handle2.get(),
                self.result_drop_epoch2.get(),
                self.len2.get(),
            )
        } else {
            (
                self.result_handle.get(),
                self.result_drop_epoch.get(),
                self.len.get(),
            )
        };
        if result_handle > 0 && result_drop_epoch == current_drop_epoch {
            return Some(SubstringViewCacheHit::Handle(result_handle));
        }

        enum EntryAction {
            Clear,
            Hit(SubstringViewCacheHit),
        }

        let action = if secondary {
            let result_obj = self.result_obj2.borrow();
            match result_obj.as_ref() {
                Some(result_obj) => {
                    let source_box_id = self.source_box_id2.get();
                    let source_still_live = handles::with_handle(source_handle as u64, |obj| {
                        obj.is_some_and(|current| current.box_id() == source_box_id)
                    });
                    if source_still_live {
                        EntryAction::Hit(SubstringViewCacheHit::Reissue {
                            result_obj: result_obj.clone(),
                            len,
                        })
                    } else {
                        EntryAction::Clear
                    }
                }
                None => EntryAction::Clear,
            }
        } else {
            let result_obj = self.result_obj.borrow();
            match result_obj.as_ref() {
                Some(result_obj) => {
                    let source_box_id = self.source_box_id.get();
                    let source_still_live = handles::with_handle(source_handle as u64, |obj| {
                        obj.is_some_and(|current| current.box_id() == source_box_id)
                    });
                    if source_still_live {
                        EntryAction::Hit(SubstringViewCacheHit::Reissue {
                            result_obj: result_obj.clone(),
                            len,
                        })
                    } else {
                        EntryAction::Clear
                    }
                }
                None => EntryAction::Clear,
            }
        };

        match action {
            EntryAction::Clear => {
                self.clear_entry(secondary);
                None
            }
            EntryAction::Hit(hit) => Some(hit),
        }
    }

    #[inline(always)]
    fn lookup(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
    ) -> Option<SubstringViewCacheHit> {
        let current_drop_epoch = handles::drop_epoch();
        if self.matches_primary(source_handle, start, end) {
            if let Some(hit) = self.entry_hit(source_handle, current_drop_epoch, false) {
                return Some(hit);
            }
        }
        if self.matches_secondary(source_handle, start, end) {
            return self.entry_hit(source_handle, current_drop_epoch, true);
        }
        None
    }

    #[inline(always)]
    fn store(
        &self,
        source_handle: i64,
        source_box_id: u64,
        start: i64,
        end: i64,
        len: i64,
        result_obj: Arc<dyn NyashBox>,
        result_handle: i64,
    ) {
        let prev_result = self.result_obj.borrow().as_ref().cloned();
        self.source_handle2.set(self.source_handle.get());
        self.source_box_id2.set(self.source_box_id.get());
        self.start2.set(self.start.get());
        self.end2.set(self.end.get());
        self.result_drop_epoch2.set(self.result_drop_epoch.get());
        self.result_handle2.set(self.result_handle.get());
        self.len2.set(self.len.get());
        *self.result_obj2.borrow_mut() = prev_result;

        self.source_handle.set(source_handle);
        self.source_box_id.set(source_box_id);
        self.start.set(start);
        self.end.set(end);
        self.result_drop_epoch.set(handles::drop_epoch());
        self.result_handle.set(result_handle);
        self.len.set(len);
        *self.result_obj.borrow_mut() = Some(result_obj);
    }

    #[inline(always)]
    fn refresh_handle(
        &self,
        source_handle: i64,
        start: i64,
        end: i64,
        result_handle: i64,
    ) {
        let drop_epoch = handles::drop_epoch();
        if self.matches_primary(source_handle, start, end) {
            self.result_drop_epoch.set(drop_epoch);
            self.result_handle.set(result_handle);
        } else if self.matches_secondary(source_handle, start, end) {
            self.result_drop_epoch2.set(drop_epoch);
            self.result_handle2.set(result_handle);
        }
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
            source_box_id: Cell::new(0),
            start: Cell::new(0),
            end: Cell::new(0),
            result_drop_epoch: Cell::new(0),
            result_handle: Cell::new(0),
            len: Cell::new(0),
            result_obj: RefCell::new(None),
            source_handle2: Cell::new(0),
            source_box_id2: Cell::new(0),
            start2: Cell::new(0),
            end2: Cell::new(0),
            result_drop_epoch2: Cell::new(0),
            result_handle2: Cell::new(0),
            len2: Cell::new(0),
            result_obj2: RefCell::new(None),
        } };
    static STRING_LEN_FAST_CACHE: StringLenFastCache =
        const { StringLenFastCache {
            drop_epoch: Cell::new(0),
            handle: Cell::new(0),
            len: Cell::new(0),
            handle2: Cell::new(0),
            len2: Cell::new(0),
        } };
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
) -> Option<SubstringViewCacheHit> {
    SUBSTRING_VIEW_ARC_CACHE.with(|cache| cache.lookup(source_handle, start, end))
}

#[inline(always)]
pub(super) fn substring_view_arc_cache_store(
    source_handle: i64,
    source_box_id: u64,
    start: i64,
    end: i64,
    len: i64,
    result_obj: Arc<dyn NyashBox>,
    result_handle: i64,
) {
    SUBSTRING_VIEW_ARC_CACHE.with(|cache| {
        cache.store(
            source_handle,
            source_box_id,
            start,
            end,
            len,
            result_obj,
            result_handle,
        );
    });
}

#[inline(always)]
pub(super) fn substring_view_arc_cache_refresh_handle(
    source_handle: i64,
    start: i64,
    end: i64,
    result_handle: i64,
) {
    SUBSTRING_VIEW_ARC_CACHE.with(|cache| {
        cache.refresh_handle(source_handle, start, end, result_handle);
    });
}

#[inline(always)]
pub(super) fn string_len_fast_cache_lookup(handle: i64) -> Option<i64> {
    STRING_LEN_FAST_CACHE.with(|cache| {
        let current_drop_epoch = handles::drop_epoch();
        if cache.handle.get() == handle {
            if cache.drop_epoch.get() == current_drop_epoch {
                return Some(cache.len.get());
            }
            return None;
        }
        if cache.handle2.get() == handle && cache.drop_epoch.get() == current_drop_epoch {
            return Some(cache.len2.get());
        }
        None
    })
}

#[inline(always)]
pub(super) fn string_len_fast_cache_store(handle: i64, len: i64) {
    let drop_epoch = handles::drop_epoch();
    STRING_LEN_FAST_CACHE.with(|cache| {
        let prev_handle = cache.handle.replace(handle);
        let prev_len = cache.len.replace(len);
        cache.handle2.set(prev_handle);
        cache.len2.set(prev_len);
        cache.drop_epoch.set(drop_epoch);
    });
}
