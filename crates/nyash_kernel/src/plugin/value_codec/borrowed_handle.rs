use super::string_classify::VerifiedTextSource;
use crate::observe;
use nyash_rust::{
    box_trait::{next_box_id, BoolBox, BoxBase, BoxCore, IntegerBox, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::{any::Any, cell::RefCell, sync::Arc};

const RETIRED_SOURCE_KEEP_BATCH_LEN: usize = 32;

thread_local! {
    // Keep the previous source boxes alive in a small cold sink so retarget no longer
    // pays every `Arc` retirement on the hottest alias-update edge.
    static RETIRED_SOURCE_KEEP_BATCH: RefCell<Vec<SourceLifetimeKeep>> = const { RefCell::new(Vec::new()) };
}

#[derive(Clone, Copy)]
pub(crate) enum BorrowedAliasEncodeCaller {
    Generic,
    ArrayGetIndexEncoded,
    MapRuntimeDataGetAnyKey,
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
        let replaced = std::mem::replace(&mut self.source_lifetime, keep);
        retire_replaced_source_keep(replaced);
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

    #[cold]
    #[inline(never)]
    fn cold_stable_object_ref(&self) -> &Arc<dyn NyashBox> {
        self.source_lifetime.backing().stable_box_ref()
    }

    #[inline(always)]
    fn ptr_eq_source_keep(&self, keep: &SourceLifetimeKeep) -> bool {
        self.source_lifetime
            .backing()
            .ptr_eq_backing(keep.backing())
    }

    #[cold]
    #[inline(never)]
    fn clone_stable_box_cold_fallback(&self) -> Arc<dyn NyashBox> {
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

    #[cold]
    #[inline(never)]
    fn cold_stable_object_ref(&self) -> &Arc<dyn NyashBox> {
        self.text_keep.cold_stable_object_ref()
    }

    #[inline(always)]
    fn copy_owned_text_cold(&self) -> String {
        self.text_keep.copy_owned_text_cold()
    }

    #[inline(always)]
    fn source_handle(&self) -> i64 {
        self.source_meta.source_handle()
    }

    #[inline(always)]
    fn source_drop_epoch(&self) -> u64 {
        self.source_meta.source_drop_epoch()
    }

    #[inline(always)]
    fn source_is_latest_fresh(&self) -> bool {
        self.source_handle() > 0
            && observe::len_route_matches_latest_fresh_handle(self.source_handle())
    }

    #[cold]
    #[inline(never)]
    fn equals_cold_promoted_object(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_alias) = other.as_any().downcast_ref::<BorrowedHandleBox>() {
            return self
                .text_keep
                .cold_stable_object_ref()
                .as_ref()
                .equals(other_alias.text_keep.cold_stable_object_ref().as_ref());
        }
        self.text_keep
            .cold_stable_object_ref()
            .as_ref()
            .equals(other)
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
        self.equals_cold_promoted_object(other)
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
        return promote_string_view_to_owned_box_cold(obj);
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
    promote_source_keep_to_owned_box_cold(keep)
}

#[inline(always)]
pub(crate) fn keep_borrowed_string_slot_source_keep(
    alias: &mut BorrowedHandleBox,
    source_keep: SourceLifetimeKeep,
) {
    observe::record_store_array_str_reason_retarget_keep_source_arc();
    if alias.text_keep.ptr_eq_source_keep(&source_keep) {
        if observe::enabled() {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
        }
        return;
    }
    if observe::enabled() {
        observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
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

#[inline(always)]
pub(crate) fn try_retarget_borrowed_string_slot_take_verified_text_source(
    slot: &mut Box<dyn NyashBox>,
    source_handle: i64,
    source_text: VerifiedTextSource,
    source_drop_epoch: u64,
) -> Result<(), VerifiedTextSource> {
    let proof = source_text.proof();
    match try_retarget_borrowed_string_slot_take_keep(
        slot,
        source_handle,
        source_text.into_keep(),
        source_drop_epoch,
    ) {
        Ok(()) => Ok(()),
        Err(source_keep) => Err(VerifiedTextSource::new(proof, source_keep)),
    }
}

#[inline(always)]
pub(crate) fn try_retarget_borrowed_string_slot_take_unpublished_keep(
    slot: &mut Box<dyn NyashBox>,
    source_keep: SourceLifetimeKeep,
    source_drop_epoch: u64,
) -> Result<(), SourceLifetimeKeep> {
    let Some(alias) = slot.as_any_mut().downcast_mut::<BorrowedHandleBox>() else {
        return Err(source_keep);
    };
    keep_borrowed_string_slot_source_keep(alias, source_keep);
    update_borrowed_string_slot_alias(alias, 0, source_drop_epoch);
    Ok(())
}

#[cold]
#[inline(never)]
fn promote_string_view_to_owned_box_cold(obj: Arc<dyn NyashBox>) -> Box<dyn NyashBox> {
    observe::record_birth_backend_publish_reason_need_stable_object();
    observe::record_birth_backend_carrier_kind_stable_box();
    Box::new(StringBox::new(
        SourceLifetimeKeep::string_view(obj).copy_owned_text_cold(),
    ))
}

#[cold]
#[inline(never)]
fn promote_source_keep_to_owned_box_cold(keep: SourceLifetimeKeep) -> Box<dyn NyashBox> {
    observe::record_birth_backend_publish_reason_need_stable_object();
    observe::record_birth_backend_carrier_kind_stable_box();
    Box::new(StringBox::new(keep.copy_owned_text_cold()))
}

enum BorrowedAliasEncodePlan {
    ReuseSourceHandle(i64),
    ReturnScalar(i64),
    EncodeFallback,
}

#[inline(always)]
pub(crate) fn runtime_i64_from_borrowed_alias(
    alias: &BorrowedHandleBox,
    caller: BorrowedAliasEncodeCaller,
) -> i64 {
    match plan_borrowed_alias_runtime_i64(alias) {
        BorrowedAliasEncodePlan::ReuseSourceHandle(handle) => handle,
        BorrowedAliasEncodePlan::ReturnScalar(value) => value,
        BorrowedAliasEncodePlan::EncodeFallback => {
            observe::record_borrowed_alias_encode_to_handle_arc();
            caller.record();
            handles::to_handle_arc(alias.text_keep.clone_stable_box_cold_fallback()) as i64
        }
    }
}

#[inline(always)]
fn plan_borrowed_alias_runtime_i64(alias: &BorrowedHandleBox) -> BorrowedAliasEncodePlan {
    let source_handle = alias.source_handle();
    if source_handle > 0 {
        let current_epoch = handles::drop_epoch();
        if alias.source_drop_epoch() == current_epoch {
            observe::record_borrowed_alias_encode_epoch_hit();
            return BorrowedAliasEncodePlan::ReuseSourceHandle(source_handle);
        }
    }
    let fallback = alias.cold_stable_object_ref().as_ref();
    if let Some(iv) = integer_box_to_i64(fallback) {
        return BorrowedAliasEncodePlan::ReturnScalar(iv);
    }
    if let Some(bv) = bool_box_to_i64(fallback) {
        return BorrowedAliasEncodePlan::ReturnScalar(bv);
    }
    if source_handle > 0 {
        if let Some(source_obj) = handles::get(source_handle as u64) {
            if Arc::ptr_eq(alias.cold_stable_object_ref(), &source_obj) {
                observe::record_borrowed_alias_encode_ptr_eq_hit();
                return BorrowedAliasEncodePlan::ReuseSourceHandle(source_handle);
            }
        }
    }
    BorrowedAliasEncodePlan::EncodeFallback
}

#[inline(always)]
fn integer_box_to_i64(value: &dyn NyashBox) -> Option<i64> {
    value
        .as_any()
        .downcast_ref::<IntegerBox>()
        .map(|ib| ib.value)
}

#[inline(always)]
fn bool_box_to_i64(value: &dyn NyashBox) -> Option<i64> {
    value
        .as_any()
        .downcast_ref::<BoolBox>()
        .map(|bb| if bb.value { 1 } else { 0 })
}

impl BorrowedAliasEncodeCaller {
    #[inline(always)]
    fn record(self) {
        match self {
            Self::Generic => {}
            Self::ArrayGetIndexEncoded => {
                observe::record_borrowed_alias_encode_to_handle_arc_array_get_index();
            }
            Self::MapRuntimeDataGetAnyKey => {
                observe::record_borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any();
            }
        }
    }
}
