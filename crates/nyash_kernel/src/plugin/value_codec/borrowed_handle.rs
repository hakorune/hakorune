use nyash_rust::{
    box_trait::{next_box_id, BoolBox, BoxBase, BoxCore, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use crate::observe;
use std::{any::Any, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) enum SourceLifetimeKeep {
    StableBox(Arc<dyn NyashBox>),
}

impl SourceLifetimeKeep {
    #[inline(always)]
    pub(crate) fn stable_box(obj: Arc<dyn NyashBox>) -> Self {
        Self::StableBox(obj)
    }

    #[inline(always)]
    pub(crate) fn stable_box_ref(&self) -> &Arc<dyn NyashBox> {
        match self {
            Self::StableBox(obj) => obj,
        }
    }

    #[inline(always)]
    pub(crate) fn replace_stable_box(&mut self, obj: Arc<dyn NyashBox>) {
        *self = Self::StableBox(obj);
    }

    #[inline(always)]
    pub(crate) fn into_stable_box(self) -> Arc<dyn NyashBox> {
        match self {
            Self::StableBox(obj) => obj,
        }
    }

    #[inline(always)]
    fn as_str_fast(&self) -> Option<&str> {
        self.stable_box_ref().as_ref().as_str_fast()
    }

    #[inline(always)]
    fn to_string_box(&self) -> StringBox {
        self.stable_box_ref().as_ref().to_string_box()
    }

    #[inline(always)]
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        self.stable_box_ref().as_ref().equals(other)
    }

    #[inline(always)]
    fn type_name(&self) -> &'static str {
        self.stable_box_ref().as_ref().type_name()
    }

    #[inline(always)]
    fn clone_box(&self) -> Box<dyn NyashBox> {
        self.stable_box_ref().clone().clone_box()
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.stable_box_ref().as_ref().is_identity()
    }
}

#[derive(Debug, Clone)]
enum BorrowedStringKeep {
    SourceLifetime(SourceLifetimeKeep),
}

impl BorrowedStringKeep {
    #[inline(always)]
    fn source_lifetime_ref(&self) -> &SourceLifetimeKeep {
        match self {
            Self::SourceLifetime(keep) => keep,
        }
    }

    #[inline(always)]
    fn replace_source_lifetime(&mut self, keep: SourceLifetimeKeep) {
        *self = Self::SourceLifetime(keep);
    }

    #[inline(always)]
    fn stable_box_ref(&self) -> &Arc<dyn NyashBox> {
        self.source_lifetime_ref().stable_box_ref()
    }

    #[inline(always)]
    fn as_str_fast(&self) -> Option<&str> {
        self.source_lifetime_ref().as_str_fast()
    }

    #[inline(always)]
    fn to_string_box(&self) -> StringBox {
        self.source_lifetime_ref().to_string_box()
    }

    #[inline(always)]
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        self.source_lifetime_ref().equals(other)
    }

    #[inline(always)]
    fn type_name(&self) -> &'static str {
        self.source_lifetime_ref().type_name()
    }

    #[inline(always)]
    fn clone_box(&self) -> Box<dyn NyashBox> {
        self.source_lifetime_ref().clone_box()
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.source_lifetime_ref().is_identity()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedHandleBox {
    keep: BorrowedStringKeep,
    pub(crate) source_handle: i64,
    pub(crate) source_drop_epoch: u64,
    base: BoxBase,
}

impl BorrowedHandleBox {
    pub(crate) fn new(
        inner: Arc<dyn NyashBox>,
        source_handle: i64,
        source_drop_epoch: u64,
    ) -> Self {
        let stable_id = if source_handle > 0 {
            source_handle as u64
        } else {
            next_box_id()
        };
        Self {
            keep: BorrowedStringKeep::SourceLifetime(SourceLifetimeKeep::stable_box(inner)),
            source_handle,
            source_drop_epoch,
            // Fast path: borrowed wrapper is an alias view for an existing handle.
            // Reuse source handle as stable id to avoid per-call id allocation churn.
            base: BoxBase {
                id: stable_id,
                parent_type_id: None,
            },
        }
    }

    #[inline(always)]
    pub(crate) fn stable_box_ref(&self) -> &Arc<dyn NyashBox> {
        self.keep.stable_box_ref()
    }

    #[inline(always)]
    fn source_is_latest_fresh(&self) -> bool {
        self.source_handle > 0 && observe::len_route_matches_latest_fresh_handle(self.source_handle)
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
        self.keep.stable_box_ref().fmt_box(f)
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
        self.keep.to_string_box()
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        observe::record_borrowed_alias_equals();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_equals_latest_fresh();
        }
        if let Some(other_alias) = other.as_any().downcast_ref::<BorrowedHandleBox>() {
            return self
                .keep
                .stable_box_ref()
                .as_ref()
                .equals(other_alias.keep.stable_box_ref().as_ref());
        }
        self.keep.equals(other)
    }

    fn type_name(&self) -> &'static str {
        self.keep.type_name()
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        observe::record_borrowed_alias_clone_box();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_clone_box_latest_fresh();
        }
        Box::new(Self::new(
            self.keep.stable_box_ref().clone(),
            self.source_handle,
            self.source_drop_epoch,
        ))
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn is_identity(&self) -> bool {
        self.keep.is_identity()
    }

    fn borrowed_handle_source_fast(&self) -> Option<(i64, u64)> {
        observe::record_borrowed_alias_borrowed_source_fast();
        if self.source_handle > 0 {
            Some((self.source_handle, self.source_drop_epoch))
        } else {
            None
        }
    }

    fn as_str_fast(&self) -> Option<&str> {
        observe::record_borrowed_alias_as_str_fast();
        if observe::enabled() {
            if self.source_handle > 0 {
                if self.source_drop_epoch == handles::drop_epoch() {
                    observe::record_borrowed_alias_as_str_fast_live_source();
                } else {
                    observe::record_borrowed_alias_as_str_fast_stale_source();
                }
            }
        }
        self.keep.as_str_fast()
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
    // Only string-like sources may produce a borrowed string alias.
    if obj.as_any().downcast_ref::<StringBox>().is_some() {
        return Box::new(BorrowedHandleBox::new(
            obj,
            source_handle,
            source_drop_epoch,
        ));
    }
    obj.clone_box()
}

#[inline(always)]
pub(crate) fn try_retarget_borrowed_string_slot_with_source(
    slot: &mut Box<dyn NyashBox>,
    source_handle: i64,
    source_obj: &Arc<dyn NyashBox>,
    source_drop_epoch: u64,
) -> bool {
    // Retarget only existing borrowed-string aliases.
    // Non-borrowed slots and non-string sources must fail closed here.
    if source_handle <= 0 {
        return false;
    }
    let Some(alias) = slot.as_any_mut().downcast_mut::<BorrowedHandleBox>() else {
        return false;
    };
    if source_obj.as_any().downcast_ref::<StringBox>().is_none() {
        return false;
    }
    keep_borrowed_string_slot_source_arc(alias, source_obj);
    update_borrowed_string_slot_alias(alias, source_handle, source_drop_epoch);
    true
}

#[inline(always)]
pub(crate) fn try_retarget_borrowed_string_slot_verified(
    slot: &mut Box<dyn NyashBox>,
    source_handle: i64,
    source_obj: &Arc<dyn NyashBox>,
    source_drop_epoch: u64,
) -> bool {
    if source_handle <= 0 {
        return false;
    }
    let Some(alias) = slot.as_any_mut().downcast_mut::<BorrowedHandleBox>() else {
        return false;
    };
    keep_borrowed_string_slot_source_arc(alias, source_obj);
    update_borrowed_string_slot_alias(alias, source_handle, source_drop_epoch);
    true
}

#[inline(always)]
pub(crate) fn keep_borrowed_string_slot_source_arc(
    alias: &mut BorrowedHandleBox,
    source_obj: &Arc<dyn NyashBox>,
) {
    observe::record_store_array_str_reason_retarget_keep_source_arc();
    if observe::enabled() {
        if Arc::ptr_eq(alias.keep.stable_box_ref(), source_obj) {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
        } else {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
        }
    }
    alias
        .keep
        .replace_source_lifetime(SourceLifetimeKeep::stable_box(source_obj.clone()));
}

#[inline(always)]
pub(crate) fn keep_borrowed_string_slot_source_keep(
    alias: &mut BorrowedHandleBox,
    source_keep: SourceLifetimeKeep,
) {
    observe::record_store_array_str_reason_retarget_keep_source_arc();
    if observe::enabled() {
        if Arc::ptr_eq(alias.keep.stable_box_ref(), source_keep.stable_box_ref()) {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
        } else {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
        }
    }
    alias.keep.replace_source_lifetime(source_keep);
}

#[inline(always)]
pub(crate) fn update_borrowed_string_slot_alias(
    alias: &mut BorrowedHandleBox,
    source_handle: i64,
    source_drop_epoch: u64,
) {
    observe::record_store_array_str_reason_retarget_alias_update();
    alias.source_handle = source_handle;
    alias.source_drop_epoch = source_drop_epoch;
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
