use crate::observe;
use nyash_rust::{
    box_trait::{next_box_id, BoolBox, BoxBase, BoxCore, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::{any::Any, sync::Arc};

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
    fn stable_object_text_fast(&self) -> Option<&str> {
        self.stable_box.as_ref().as_str_fast()
    }

    #[inline(always)]
    fn with_text<R>(&self, f: impl FnOnce(&str) -> R) -> Option<R> {
        self.stable_object_text_fast().map(f)
    }

    #[inline(always)]
    fn copy_owned_text_cold(&self) -> String {
        self.stable_box.as_ref().to_string_box().value
    }

    #[inline(always)]
    fn clone_stable_box_cold_fallback(&self) -> Arc<dyn NyashBox> {
        self.stable_box.clone()
    }

    #[inline(always)]
    fn ptr_eq_backing(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.stable_box, &other.stable_box)
    }
}

#[derive(Debug, Clone)]
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
    pub(crate) fn with_text<R>(&self, f: impl FnOnce(&str) -> R) -> Option<R> {
        self.backing().with_text(f)
    }

    #[inline(always)]
    pub(crate) fn copy_owned_text_cold(&self) -> String {
        self.backing().copy_owned_text_cold()
    }

    #[inline(always)]
    fn supports_borrowed_alias(&self) -> bool {
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
struct TextKeep {
    source_lifetime: SourceLifetimeKeep,
}

impl TextKeep {
    #[inline(always)]
    fn replace_source_lifetime(&mut self, keep: SourceLifetimeKeep) {
        self.source_lifetime = keep;
    }

    #[inline(always)]
    fn with_text<R>(&self, f: impl FnOnce(&str) -> R) -> Option<R> {
        self.source_lifetime.with_text(f)
    }

    #[inline(always)]
    fn stable_object_text_fast(&self) -> Option<&str> {
        self.source_lifetime.backing().stable_object_text_fast()
    }

    #[inline(always)]
    fn copy_owned_text_cold(&self) -> String {
        self.source_lifetime.copy_owned_text_cold()
    }

    #[inline(always)]
    fn type_name(&self) -> &'static str {
        match self.source_lifetime.class() {
            TextKeepClass::StringBox => "StringBox",
            TextKeepClass::StringView => "StringViewBox",
        }
    }

    #[inline(always)]
    fn stable_object_ref(&self) -> &Arc<dyn NyashBox> {
        self.source_lifetime.backing().stable_box_ref()
    }

    #[inline(always)]
    fn ptr_eq_source_keep(&self, keep: &SourceLifetimeKeep) -> bool {
        self.source_lifetime
            .backing()
            .ptr_eq_backing(keep.backing())
    }

    #[inline(always)]
    fn clone_stable_box_cold_fallback(&self) -> Arc<dyn NyashBox> {
        self.source_lifetime.clone_stable_box_cold_fallback()
    }
}

#[derive(Debug, Clone, Copy)]
struct AliasSourceMeta {
    source_handle: i64,
    source_drop_epoch: u64,
}

impl AliasSourceMeta {
    #[inline(always)]
    fn new(source_handle: i64, source_drop_epoch: u64) -> Self {
        Self {
            source_handle,
            source_drop_epoch,
        }
    }

    #[inline(always)]
    fn source_handle(self) -> i64 {
        self.source_handle
    }

    #[inline(always)]
    fn source_drop_epoch(self) -> u64 {
        self.source_drop_epoch
    }

    #[inline(always)]
    fn replace(&mut self, source_handle: i64, source_drop_epoch: u64) {
        self.source_handle = source_handle;
        self.source_drop_epoch = source_drop_epoch;
    }

    #[inline(always)]
    fn borrowed_handle_source_fast(self) -> Option<(i64, u64)> {
        if self.source_handle > 0 {
            Some((self.source_handle, self.source_drop_epoch))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedHandleBox {
    text_keep: TextKeep,
    source_meta: AliasSourceMeta,
    base: BoxBase,
}

impl BorrowedHandleBox {
    pub(crate) fn new(
        keep: SourceLifetimeKeep,
        source_handle: i64,
        source_drop_epoch: u64,
    ) -> Self {
        let stable_id = if source_handle > 0 {
            source_handle as u64
        } else {
            next_box_id()
        };
        Self {
            text_keep: TextKeep {
                source_lifetime: keep,
            },
            source_meta: AliasSourceMeta::new(source_handle, source_drop_epoch),
            // Fast path: borrowed wrapper is an alias view for an existing handle.
            // Reuse source handle as stable id to avoid per-call id allocation churn.
            base: BoxBase {
                id: stable_id,
                parent_type_id: None,
            },
        }
    }

    #[inline(always)]
    fn stable_object_ref(&self) -> &Arc<dyn NyashBox> {
        self.text_keep.stable_object_ref()
    }

    #[inline(always)]
    pub(crate) fn encode_fallback_box_ref(&self) -> &dyn NyashBox {
        self.stable_object_ref().as_ref()
    }

    #[inline(always)]
    pub(crate) fn clone_stable_box_for_encode_fallback(&self) -> Arc<dyn NyashBox> {
        self.text_keep.clone_stable_box_cold_fallback()
    }

    #[inline(always)]
    pub(crate) fn copy_owned_text_cold(&self) -> String {
        self.text_keep.copy_owned_text_cold()
    }

    #[inline(always)]
    pub(crate) fn ptr_eq_source_object(&self, other: &Arc<dyn NyashBox>) -> bool {
        Arc::ptr_eq(self.stable_object_ref(), other)
    }

    #[inline(always)]
    pub(crate) fn source_handle(&self) -> i64 {
        self.source_meta.source_handle()
    }

    #[inline(always)]
    pub(crate) fn source_drop_epoch(&self) -> u64 {
        self.source_meta.source_drop_epoch()
    }

    #[inline(always)]
    fn source_is_latest_fresh(&self) -> bool {
        self.source_handle() > 0
            && observe::len_route_matches_latest_fresh_handle(self.source_handle())
    }
}

impl BoxCore for BorrowedHandleBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(result) = self.text_keep.with_text(|text| write!(f, "\"{}\"", text)) {
            return result;
        }
        write!(f, "\"{}\"", self.copy_owned_text_cold())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for BorrowedHandleBox {
    fn to_string_box(&self) -> StringBox {
        observe::record_borrowed_alias_to_string_box();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_to_string_box_latest_fresh();
        }
        StringBox::new(self.copy_owned_text_cold())
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        observe::record_borrowed_alias_equals();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_equals_latest_fresh();
        }
        if let Some(other_alias) = other.as_any().downcast_ref::<BorrowedHandleBox>() {
            return self
                .text_keep
                .stable_object_ref()
                .as_ref()
                .equals(other_alias.text_keep.stable_object_ref().as_ref());
        }
        self.text_keep.stable_object_ref().as_ref().equals(other)
    }

    fn type_name(&self) -> &'static str {
        self.text_keep.type_name()
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        observe::record_borrowed_alias_clone_box();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_clone_box_latest_fresh();
        }
        Box::new(Self::new(
            self.text_keep.source_lifetime.clone(),
            self.source_handle(),
            self.source_drop_epoch(),
        ))
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn is_identity(&self) -> bool {
        false
    }

    fn borrowed_handle_source_fast(&self) -> Option<(i64, u64)> {
        observe::record_borrowed_alias_borrowed_source_fast();
        self.source_meta.borrowed_handle_source_fast()
    }

    fn as_str_fast(&self) -> Option<&str> {
        observe::record_borrowed_alias_as_str_fast();
        if observe::enabled() {
            if self.source_handle() > 0 {
                if self.source_drop_epoch() == handles::drop_epoch() {
                    observe::record_borrowed_alias_as_str_fast_live_source();
                } else {
                    observe::record_borrowed_alias_as_str_fast_stale_source();
                }
            }
        }
        self.text_keep.stable_object_text_fast()
    }
}

#[inline(always)]
pub(crate) fn maybe_borrow_string_handle(
    obj: Arc<dyn NyashBox>,
    source_handle: i64,
) -> Box<dyn NyashBox> {
    maybe_borrow_string_handle_with_epoch(obj, source_handle, handles::drop_epoch())
}

#[inline(always)]
pub(crate) fn maybe_borrow_string_handle_with_epoch(
    obj: Arc<dyn NyashBox>,
    source_handle: i64,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if obj.as_any().downcast_ref::<StringBox>().is_some() {
        return Box::new(BorrowedHandleBox::new(
            SourceLifetimeKeep::string_box(obj),
            source_handle,
            source_drop_epoch,
        ));
    }
    if obj
        .as_any()
        .downcast_ref::<crate::exports::string_view::StringViewBox>()
        .is_some()
    {
        return Box::new(StringBox::new(
            SourceLifetimeKeep::string_view(obj).copy_owned_text_cold(),
        ));
    }
    obj.clone_box()
}

#[inline(always)]
pub(crate) fn maybe_borrow_string_keep_with_epoch(
    keep: SourceLifetimeKeep,
    source_handle: i64,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if keep.supports_borrowed_alias() {
        return Box::new(BorrowedHandleBox::new(
            keep,
            source_handle,
            source_drop_epoch,
        ));
    }
    Box::new(StringBox::new(keep.copy_owned_text_cold()))
}

#[inline(always)]
pub(crate) fn keep_borrowed_string_slot_source_keep(
    alias: &mut BorrowedHandleBox,
    source_keep: SourceLifetimeKeep,
) {
    observe::record_store_array_str_reason_retarget_keep_source_arc();
    if observe::enabled() {
        if alias.text_keep.ptr_eq_source_keep(&source_keep) {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
        } else {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
        }
    }
    alias.text_keep.replace_source_lifetime(source_keep);
}

#[inline(always)]
pub(crate) fn update_borrowed_string_slot_alias(
    alias: &mut BorrowedHandleBox,
    source_handle: i64,
    source_drop_epoch: u64,
) {
    observe::record_store_array_str_reason_retarget_alias_update();
    alias.source_meta.replace(source_handle, source_drop_epoch);
}

#[inline(always)]
pub(crate) fn try_retarget_borrowed_string_slot_take_keep(
    slot: &mut Box<dyn NyashBox>,
    source_handle: i64,
    source_keep: SourceLifetimeKeep,
    source_drop_epoch: u64,
) -> Result<(), SourceLifetimeKeep> {
    if source_handle <= 0 {
        return Err(source_keep);
    }
    let Some(alias) = slot.as_any_mut().downcast_mut::<BorrowedHandleBox>() else {
        return Err(source_keep);
    };
    keep_borrowed_string_slot_source_keep(alias, source_keep);
    update_borrowed_string_slot_alias(alias, source_handle, source_drop_epoch);
    Ok(())
}
