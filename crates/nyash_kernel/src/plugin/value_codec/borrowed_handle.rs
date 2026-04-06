use nyash_rust::{
    box_trait::{next_box_id, BoolBox, BoxBase, BoxCore, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use crate::observe;
use std::{any::Any, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct BorrowedHandleBox {
    pub(crate) inner: Arc<dyn NyashBox>,
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
            inner,
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
}

impl BoxCore for BorrowedHandleBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt_box(f)
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
        self.inner.to_string_box()
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        observe::record_borrowed_alias_equals();
        if let Some(other_alias) = other.as_any().downcast_ref::<BorrowedHandleBox>() {
            return self.inner.equals(other_alias.inner.as_ref());
        }
        self.inner.equals(other)
    }

    fn type_name(&self) -> &'static str {
        self.inner.type_name()
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        observe::record_borrowed_alias_clone_box();
        Box::new(Self::new(
            self.inner.clone(),
            self.source_handle,
            self.source_drop_epoch,
        ))
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn is_identity(&self) -> bool {
        self.inner.is_identity()
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
        if self.source_handle > 0 {
            if self.source_drop_epoch == handles::drop_epoch() {
                observe::record_borrowed_alias_as_str_fast_live_source();
            } else {
                observe::record_borrowed_alias_as_str_fast_stale_source();
            }
        }
        self.inner.as_str_fast()
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
        if Arc::ptr_eq(&alias.inner, source_obj) {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
        } else {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
        }
    }
    alias.inner = source_obj.clone();
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
