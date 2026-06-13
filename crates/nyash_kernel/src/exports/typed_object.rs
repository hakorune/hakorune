// Typed user object runtime helpers for EXE lowering.
//
// This is intentionally slot-based and opaque to the backend. MIR owns layout
// truth; the runtime owns allocation and field storage.

mod types;
pub(crate) use types::{
    default_layout_fields, field_get_hii_dispatch, field_set_hii_dispatch, handle_to_index,
    normalize_field_count, normalize_slot, typed_layouts, TypedSlot, TypedSlotLayout,
    TypedSlotObject, TypedSlotStorage, TypedSlotValue,
};

use super::typed_object_pinned_arena::read_direct_slot_compat_i64;
use super::typed_object_pinned_arena::{read_direct_slot_i64, write_direct_slot_i64};
use super::typed_object_store_backend::{
    exact_slot_record_alloc_success, exact_slot_record_release_success, exact_slot_rmw_add_u64,
    exact_slot_set4_i64, new_typed_object as backend_new_typed_object, with_field, with_field_mut,
};

#[export_name = "nyash.object.register_typed_layout_hi"]
pub extern "C" fn nyash_object_register_typed_layout_hi(type_id: i64, field_count: i64) -> i64 {
    let Some(field_count) = normalize_field_count(field_count) else {
        return 0;
    };
    let mut layouts = match typed_layouts().lock() {
        Ok(layouts) => layouts,
        Err(_) => return 0,
    };
    let layout = layouts
        .entry(type_id)
        .or_insert_with(|| TypedSlotLayout { fields: Vec::new() });
    if layout.fields.len() < field_count {
        layout.fields.resize(field_count, TypedSlotStorage::I64);
    }
    1
}

#[export_name = "nyash.object.register_typed_layout_slot_iii"]
pub extern "C" fn nyash_object_register_typed_layout_slot_iii(
    type_id: i64,
    slot: i64,
    storage_tag: i64,
) -> i64 {
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    let Some(storage) = TypedSlotStorage::from_tag(storage_tag) else {
        return 0;
    };
    let mut layouts = match typed_layouts().lock() {
        Ok(layouts) => layouts,
        Err(_) => return 0,
    };
    let layout = layouts
        .entry(type_id)
        .or_insert_with(|| TypedSlotLayout { fields: Vec::new() });
    if layout.fields.len() <= slot {
        layout.fields.resize(slot + 1, TypedSlotStorage::I64);
    }
    layout.fields[slot] = storage;
    1
}

#[export_name = "nyash.object.layout_field_storage_ii"]
pub extern "C" fn nyash_object_layout_field_storage_ii(type_id: i64, slot: i64) -> i64 {
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    let layouts = match typed_layouts().lock() {
        Ok(layouts) => layouts,
        Err(_) => return 0,
    };
    layouts
        .get(&type_id)
        .and_then(|layout| layout.fields.get(slot))
        .copied()
        .map(TypedSlotStorage::tag)
        .unwrap_or(0)
}

#[export_name = "nyash.object.new_typed_hi"]
pub extern "C" fn nyash_object_new_typed_hi(type_id: i64, field_count: i64) -> i64 {
    let Some(field_count) = normalize_field_count(field_count) else {
        return 0;
    };
    let fields = default_layout_fields(type_id, field_count)
        .into_iter()
        .map(TypedSlot::new)
        .collect();
    backend_new_typed_object(TypedSlotObject { type_id, fields }).unwrap_or(0)
}

#[export_name = "nyash.object.new_typed_h"]
pub extern "C" fn nyash_object_new_typed_h(type_id: i64) -> i64 {
    nyash_object_new_typed_hi(type_id, 0)
}

#[export_name = "nyash.object.field_get_hii"]
pub extern "C" fn nyash_object_field_get_hii(handle: i64, slot: i64) -> i64 {
    field_get_hii_dispatch()(handle, slot)
}

#[export_name = "nyash.object.field_set_hii"]
pub extern "C" fn nyash_object_field_set_hii(handle: i64, slot: i64, value: i64) {
    field_set_hii_dispatch()(handle, slot, value)
}

#[export_name = "nyash.object.field_storage_hii"]
pub extern "C" fn nyash_object_field_storage_hii(handle: i64, slot: i64) -> i64 {
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    field_storage_tag(handle, slot).unwrap_or(0)
}

#[export_name = "nyash.object.field_get_u64_hii"]
pub extern "C" fn nyash_object_field_get_u64_hii(handle: i64, slot: i64) -> u64 {
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    get_exact_unsigned_u64(handle, slot).unwrap_or(0)
}

#[export_name = "nyash.object.field_set_u64_hiu"]
pub extern "C" fn nyash_object_field_set_u64_hiu(handle: i64, slot: i64, value: u64) -> i64 {
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    i64::from(set_exact_unsigned_u64(handle, slot, value))
}

#[export_name = "nyash.object.field_get_i64_hii"]
pub extern "C" fn nyash_object_field_get_i64_hii(handle: i64, slot: i64) -> i64 {
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    get_exact_signed_i64(handle, slot).unwrap_or(0)
}

#[export_name = "nyash.object.field_set_i64_hii"]
pub extern "C" fn nyash_object_field_set_i64_hii(handle: i64, slot: i64, value: i64) -> i64 {
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    i64::from(set_exact_signed_i64(handle, slot, value))
}

#[export_name = "nyash.exact_numeric.assert_i64_min_ii"]
pub extern "C" fn nyash_exact_numeric_assert_i64_min_ii(value: i64, min_value: i64) -> i64 {
    if value < min_value {
        std::process::abort();
    }
    1
}

#[export_name = "nyash.exact_numeric.assert_i64_range_iii"]
pub extern "C" fn nyash_exact_numeric_assert_i64_range_iii(
    value: i64,
    min_value: i64,
    max_value: i64,
) -> i64 {
    if value < min_value || value > max_value {
        std::process::abort();
    }
    1
}

fn exact_slot_index(slot: i64) -> Option<usize> {
    if slot < 0 {
        return None;
    }
    usize::try_from(slot).ok()
}

#[inline(always)]
pub(crate) fn get_compat_i64(handle: i64, slot: usize) -> Option<i64> {
    with_field(handle, slot, |field| {
        Some(field.as_compat_i64().unwrap_or(0))
    })?
}

#[inline(always)]
pub(crate) fn set_compat_i64(handle: i64, slot: usize, value: i64) -> bool {
    with_field_mut(handle, slot, |field| field.set_compat_i64(value)).unwrap_or(false)
}

#[inline(always)]
pub(crate) fn field_storage_tag(handle: i64, slot: usize) -> Option<i64> {
    with_field(handle, slot, |field| field.storage.tag())
}

#[inline(always)]
pub(crate) fn get_exact_unsigned_u64(handle: i64, slot: usize) -> Option<u64> {
    with_field(handle, slot, |field| field.as_exact_unsigned_u64())?
}

#[inline(always)]
pub(crate) fn set_exact_unsigned_u64(handle: i64, slot: usize, value: u64) -> bool {
    with_field_mut(handle, slot, |field| field.set_exact_unsigned_u64(value)).unwrap_or(false)
}

#[inline(always)]
pub(crate) fn get_exact_signed_i64(handle: i64, slot: usize) -> Option<i64> {
    if matches!(
        super::typed_object_store_backend::selected_backend(),
        super::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
    ) {
        if let Some(value) = read_direct_slot_i64(handle, slot) {
            return Some(value);
        }
    }
    with_field(handle, slot, |field| field.as_exact_signed_i64())?
}

#[inline(always)]
pub(crate) fn set_exact_signed_i64(handle: i64, slot: usize, value: i64) -> bool {
    if matches!(
        super::typed_object_store_backend::selected_backend(),
        super::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
    ) {
        if let Some(ok) = write_direct_slot_i64(handle, slot, value) {
            return ok;
        }
    }
    with_field_mut(handle, slot, |field| field.set_exact_signed_i64(value)).unwrap_or(false)
}

macro_rules! typed_object_exact_slot_export {
    (
        $fn_name:ident,
        $export_name:literal,
        $slot_name:ident : i64
        $(, $arg_name:ident : $arg_ty:ty )*
        ; -> $ret:ty;
        fallback = $fallback:expr;
        body = $body:expr
    ) => {
        #[export_name = $export_name]
        pub extern "C" fn $fn_name(
            handle: i64,
            $slot_name: i64,
            $( $arg_name : $arg_ty, )*
        ) -> $ret {
            let Some($slot_name) = exact_slot_index($slot_name) else {
                return $fallback;
            };
            $body(handle, $slot_name $(, $arg_name )*)
        }
    };
}

typed_object_exact_slot_export!(
    nyash_object_exact_slot_get_i64_hii,
    "nyash.object.exact_slot_get_i64_hii",
    slot: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot| get_exact_signed_i64(handle, slot).unwrap_or(0)
);
typed_object_exact_slot_export!(
    hako_object_exact_slot_get_i64_hii,
    "hako.object.exact_slot_get_i64_hii",
    slot: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot| get_exact_signed_i64(handle, slot).unwrap_or(0)
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_set_i64_hii,
    "nyash.object.exact_slot_set_i64_hii",
    slot: i64,
    value: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot, value| {
        i64::from(set_exact_signed_i64(handle, slot, value))
    }
);
typed_object_exact_slot_export!(
    hako_object_exact_slot_set_i64_hii,
    "hako.object.exact_slot_set_i64_hii",
    slot: i64,
    value: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot, value| {
        i64::from(set_exact_signed_i64(handle, slot, value))
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_set4_i64_hiiiii,
    "nyash.object.exact_slot_set4_i64_hiiiii",
    start_slot: i64,
    value0: i64,
    value1: i64,
    value2: i64,
    value3: i64;
    -> i64;
    fallback = 0;
    body = |handle, start_slot, value0, value1, value2, value3| {
        i64::from(exact_slot_set4_i64(
            handle, start_slot, value0, value1, value2, value3,
        ))
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_record_alloc_success_hii,
    "nyash.object.exact_slot_record_alloc_success_hii",
    slot: i64,
    selected_kind: i64;
    -> i64;
    fallback = 0;
    body = |handle, _slot, selected_kind| {
        i64::from(exact_slot_record_alloc_success(
            handle,
            selected_kind,
        ))
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_record_release_success_hiii,
    "nyash.object.exact_slot_record_release_success_hiii",
    slot: i64,
    page_id: i64,
    block_id: i64;
    -> i64;
    fallback = 0;
    body = |handle, _slot, page_id, block_id| {
        i64::from(exact_slot_record_release_success(
            handle, page_id, block_id,
        ))
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_get_u64_hii,
    "nyash.object.exact_slot_get_u64_hii",
    slot: i64;
    -> u64;
    fallback = 0;
    body = |handle, slot| {
        get_exact_unsigned_u64(handle, slot).unwrap_or(0)
    }
);
typed_object_exact_slot_export!(
    hako_object_exact_slot_get_u64_hii,
    "hako.object.exact_slot_get_u64_hii",
    slot: i64;
    -> u64;
    fallback = 0;
    body = |handle, slot| {
        get_exact_unsigned_u64(handle, slot).unwrap_or(0)
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_set_u64_hiu,
    "nyash.object.exact_slot_set_u64_hiu",
    slot: i64,
    value: u64;
    -> i64;
    fallback = 0;
    body = |handle, slot, value| {
        i64::from(set_exact_unsigned_u64(handle, slot, value))
    }
);
typed_object_exact_slot_export!(
    hako_object_exact_slot_set_u64_hiu,
    "hako.object.exact_slot_set_u64_hiu",
    slot: i64,
    value: u64;
    -> i64;
    fallback = 0;
    body = |handle, slot, value| {
        i64::from(set_exact_unsigned_u64(handle, slot, value))
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_rmw_add_u64_hiii,
    "nyash.object.exact_slot_rmw_add_u64_hiii",
    slot: i64,
    delta: i64;
    -> i64;
    fallback = -1;
    body = |handle, slot, delta| {
        exact_slot_rmw_add_u64(handle, slot, delta).unwrap_or(-1)
    }
);
typed_object_exact_slot_export!(
    hako_object_exact_slot_rmw_add_u64_hiii,
    "hako.object.exact_slot_rmw_add_u64_hiii",
    slot: i64,
    delta: i64;
    -> i64;
    fallback = -1;
    body = |handle, slot, delta| {
        exact_slot_rmw_add_u64(handle, slot, delta).unwrap_or(-1)
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_get_handle_hii,
    "nyash.object.exact_slot_get_handle_hii",
    slot: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot| {
        if matches!(
            super::typed_object_store_backend::selected_backend(),
            super::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
        ) && handle >= 0
        {
            read_direct_slot_compat_i64(handle, slot).unwrap_or(0)
        } else {
            get_compat_i64(handle, slot).unwrap_or(0)
        }
    }
);
typed_object_exact_slot_export!(
    hako_object_exact_slot_get_handle_hii,
    "hako.object.exact_slot_get_handle_hii",
    slot: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot| {
        if matches!(
            super::typed_object_store_backend::selected_backend(),
            super::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
        ) && handle >= 0
        {
            read_direct_slot_compat_i64(handle, slot).unwrap_or(0)
        } else {
            get_compat_i64(handle, slot).unwrap_or(0)
        }
    }
);

typed_object_exact_slot_export!(
    nyash_object_exact_slot_set_handle_hii,
    "nyash.object.exact_slot_set_handle_hii",
    slot: i64,
    value: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot, value| {
        i64::from(set_compat_i64(handle, slot, value))
    }
);
typed_object_exact_slot_export!(
    hako_object_exact_slot_set_handle_hii,
    "hako.object.exact_slot_set_handle_hii",
    slot: i64,
    value: i64;
    -> i64;
    fallback = 0;
    body = |handle, slot, value| {
        i64::from(set_compat_i64(handle, slot, value))
    }
);

#[cfg(test)]
mod tests;
