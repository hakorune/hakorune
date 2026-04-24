#![allow(dead_code)] // Phase 291x-127: source-keep replacement cache is staged with borrowed-alias retargeting.

use super::super::TextRef;
use nyash_rust::box_trait::NyashBox;
use std::{cell::RefCell, sync::Arc};

const RETIRED_SOURCE_KEEP_BATCH_LEN: usize = 32;

thread_local! {
    // Keep the previous source boxes alive in a small cold sink so retarget no longer
    // pays every `Arc` retirement on the hottest alias-update edge.
    static RETIRED_SOURCE_KEEP_BATCH: RefCell<Vec<SourceLifetimeKeep>> = const { RefCell::new(Vec::new()) };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextKeepClass {
    StringBox,
    StringView,
}

#[derive(Debug, Clone)]
struct TextKeepBacking {
    stable_box: Arc<dyn NyashBox>,
}

impl TextKeepBacking {
    #[inline(always)]
    fn new(stable_box: Arc<dyn NyashBox>) -> Self {
        Self { stable_box }
    }

    #[inline(always)]
    fn stable_box_ref(&self) -> &Arc<dyn NyashBox> {
        &self.stable_box
    }

    #[inline(always)]
    fn stable_object_text_fast(&self) -> Option<TextRef<'_>> {
        self.stable_box.as_ref().as_str_fast().map(TextRef::new)
    }

    #[inline(always)]
    fn with_text<R>(&self, f: impl FnOnce(TextRef<'_>) -> R) -> Option<R> {
        self.stable_object_text_fast().map(f)
    }

    #[inline(always)]
    fn copy_owned_text_cold(&self) -> String {
        self.stable_box.as_ref().to_string_box().value
    }

    #[cold]
    #[inline(never)]
    fn clone_stable_box_cold_fallback(&self) -> Arc<dyn NyashBox> {
        self.stable_box.clone()
    }

    #[inline(always)]
    fn ptr_eq_backing(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.stable_box, &other.stable_box)
    }
}

#[derive(Debug, Clone)]
/// Stable source proof + cached object reference for the current semantic text read lane.
/// This is runtime-private proof/caching state, not the semantic `TextRef` itself.
pub(crate) struct SourceLifetimeKeep {
    class: TextKeepClass,
    backing: TextKeepBacking,
}

impl SourceLifetimeKeep {
    #[inline(always)]
    pub(crate) fn string_box(obj: Arc<dyn NyashBox>) -> Self {
        Self {
            class: TextKeepClass::StringBox,
            backing: TextKeepBacking::new(obj),
        }
    }

    #[inline(always)]
    pub(crate) fn string_view(obj: Arc<dyn NyashBox>) -> Self {
        Self {
            class: TextKeepClass::StringView,
            backing: TextKeepBacking::new(obj),
        }
    }

    #[inline(always)]
    pub(crate) fn class(&self) -> TextKeepClass {
        self.class
    }

    #[inline(always)]
    pub(crate) fn with_text<R>(&self, f: impl FnOnce(TextRef<'_>) -> R) -> Option<R> {
        self.backing().with_text(f)
    }

    #[inline(always)]
    pub(crate) fn copy_owned_text_cold(&self) -> String {
        self.backing().copy_owned_text_cold()
    }

    #[inline(always)]
    pub(crate) fn supports_borrowed_alias(&self) -> bool {
        matches!(self.class(), TextKeepClass::StringBox)
    }

    #[inline(always)]
    fn backing(&self) -> &TextKeepBacking {
        &self.backing
    }

    #[inline(always)]
    fn clone_stable_box_cold_fallback(&self) -> Arc<dyn NyashBox> {
        self.backing().clone_stable_box_cold_fallback()
    }
}

#[derive(Debug, Clone)]
/// Internal lifetime helper that keeps semantic text reads anchored to a validated
/// source object. This is support infrastructure for `TextRef`, not a public carrier.
pub(crate) struct TextKeep {
    source_lifetime: SourceLifetimeKeep,
}

impl TextKeep {
    #[inline(always)]
    pub(crate) fn new(source_lifetime: SourceLifetimeKeep) -> Self {
        Self { source_lifetime }
    }

    #[inline(always)]
    pub(crate) fn replace_source_lifetime(&mut self, keep: SourceLifetimeKeep) {
        let replaced = std::mem::replace(&mut self.source_lifetime, keep);
        retire_replaced_source_keep(replaced);
    }

    #[inline(always)]
    pub(crate) fn with_text<R>(&self, f: impl FnOnce(TextRef<'_>) -> R) -> Option<R> {
        self.source_lifetime.with_text(f)
    }

    #[inline(always)]
    pub(crate) fn stable_object_text_fast(&self) -> Option<TextRef<'_>> {
        self.source_lifetime.backing().stable_object_text_fast()
    }

    #[inline(always)]
    pub(crate) fn copy_owned_text_cold(&self) -> String {
        self.source_lifetime.copy_owned_text_cold()
    }

    #[inline(always)]
    pub(crate) fn type_name(&self) -> &'static str {
        match self.source_lifetime.class() {
            TextKeepClass::StringBox => "StringBox",
            TextKeepClass::StringView => "StringViewBox",
        }
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn cold_stable_object_ref(&self) -> &Arc<dyn NyashBox> {
        self.source_lifetime.backing().stable_box_ref()
    }

    #[inline(always)]
    pub(crate) fn ptr_eq_source_keep(&self, keep: &SourceLifetimeKeep) -> bool {
        self.source_lifetime
            .backing()
            .ptr_eq_backing(keep.backing())
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn clone_stable_box_cold_fallback(&self) -> Arc<dyn NyashBox> {
        self.source_lifetime.clone_stable_box_cold_fallback()
    }
}

#[inline(always)]
fn retire_replaced_source_keep(keep: SourceLifetimeKeep) {
    let retired = RETIRED_SOURCE_KEEP_BATCH.with(|slot| {
        let mut batch = slot.borrow_mut();
        batch.push(keep);
        if batch.len() < RETIRED_SOURCE_KEEP_BATCH_LEN {
            return None;
        }
        Some(std::mem::take(&mut *batch))
    });
    if let Some(retired) = retired {
        drop_retired_source_keep_batch_cold(retired);
    }
}

#[cold]
#[inline(never)]
fn drop_retired_source_keep_batch_cold(mut retired: Vec<SourceLifetimeKeep>) {
    retired.clear();
}
