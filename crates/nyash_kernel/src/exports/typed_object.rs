// Typed user object runtime helpers for EXE lowering.
//
// This is intentionally slot-based and opaque to the backend. MIR owns layout
// truth; the runtime owns allocation and field storage.

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use super::typed_object_pinned_arena::{read_direct_slot_compat_i64, write_direct_slot_compat_i64};
use super::typed_object_pinned_arena::{read_direct_slot_i64, write_direct_slot_i64};
use super::typed_object_store_backend::{
    exact_slot_record_alloc_success, exact_slot_record_release_success, exact_slot_rmw_add_u64,
    exact_slot_set4_i64, new_typed_object as backend_new_typed_object, with_field, with_field_mut,
};

const MAX_TYPED_OBJECT_FIELDS: i64 = 4096;

const STORAGE_I64: i64 = 1;
const STORAGE_HANDLE: i64 = 2;
const STORAGE_ISIZE: i64 = 3;
const STORAGE_USIZE: i64 = 4;
const STORAGE_I8: i64 = 5;
const STORAGE_I16: i64 = 6;
const STORAGE_I32: i64 = 7;
const STORAGE_U8: i64 = 8;
const STORAGE_U16: i64 = 9;
const STORAGE_U32: i64 = 10;
const STORAGE_U64: i64 = 11;

type FieldGetHiiImpl = fn(i64, i64) -> i64;
type FieldSetHiiImpl = fn(i64, i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TypedSlotStorage {
    I64,
    Handle,
    ISize,
    USize,
    I8,
    I16,
    I32,
    U8,
    U16,
    U32,
    U64,
}

impl TypedSlotStorage {
    fn from_tag(tag: i64) -> Option<Self> {
        match tag {
            STORAGE_I64 => Some(Self::I64),
            STORAGE_HANDLE => Some(Self::Handle),
            STORAGE_ISIZE => Some(Self::ISize),
            STORAGE_USIZE => Some(Self::USize),
            STORAGE_I8 => Some(Self::I8),
            STORAGE_I16 => Some(Self::I16),
            STORAGE_I32 => Some(Self::I32),
            STORAGE_U8 => Some(Self::U8),
            STORAGE_U16 => Some(Self::U16),
            STORAGE_U32 => Some(Self::U32),
            STORAGE_U64 => Some(Self::U64),
            _ => None,
        }
    }

    pub(crate) fn tag(self) -> i64 {
        match self {
            Self::I64 => STORAGE_I64,
            Self::Handle => STORAGE_HANDLE,
            Self::ISize => STORAGE_ISIZE,
            Self::USize => STORAGE_USIZE,
            Self::I8 => STORAGE_I8,
            Self::I16 => STORAGE_I16,
            Self::I32 => STORAGE_I32,
            Self::U8 => STORAGE_U8,
            Self::U16 => STORAGE_U16,
            Self::U32 => STORAGE_U32,
            Self::U64 => STORAGE_U64,
        }
    }

    fn supports_compat_i64(self) -> bool {
        matches!(self, Self::I64 | Self::Handle)
    }

    fn is_unsigned_exact(self) -> bool {
        matches!(
            self,
            Self::USize | Self::U8 | Self::U16 | Self::U32 | Self::U64
        )
    }

    fn unsigned_max(self) -> Option<u128> {
        match self {
            Self::USize => Some(usize::MAX as u128),
            Self::U8 => Some(u8::MAX as u128),
            Self::U16 => Some(u16::MAX as u128),
            Self::U32 => Some(u32::MAX as u128),
            Self::U64 => Some(u64::MAX as u128),
            _ => None,
        }
    }

    fn signed_range(self) -> Option<(i128, i128)> {
        match self {
            Self::I64 => Some((i64::MIN as i128, i64::MAX as i128)),
            Self::ISize => Some((isize::MIN as i128, isize::MAX as i128)),
            Self::I8 => Some((i8::MIN as i128, i8::MAX as i128)),
            Self::I16 => Some((i16::MIN as i128, i16::MAX as i128)),
            Self::I32 => Some((i32::MIN as i128, i32::MAX as i128)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypedSlotValue {
    I64(i64),
    Handle(i64),
    Signed(i128),
    Unsigned(u128),
}

impl TypedSlotValue {
    fn default_for(storage: TypedSlotStorage) -> Self {
        match storage {
            TypedSlotStorage::I64 => Self::I64(0),
            TypedSlotStorage::Handle => Self::Handle(0),
            storage if storage.is_unsigned_exact() => Self::Unsigned(0),
            _ => Self::Signed(0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypedSlot {
    pub(crate) storage: TypedSlotStorage,
    pub(crate) value: TypedSlotValue,
}

impl TypedSlot {
    pub(crate) fn new(storage: TypedSlotStorage) -> Self {
        Self {
            storage,
            value: TypedSlotValue::default_for(storage),
        }
    }

    pub(crate) fn set_i64_exact(&mut self, value: i64) -> bool {
        if self.storage != TypedSlotStorage::I64 {
            return false;
        }
        self.value = TypedSlotValue::I64(value);
        true
    }

    #[inline(always)]
    pub(crate) fn set_compat_i64(&mut self, value: i64) -> bool {
        if !self.storage.supports_compat_i64() {
            return false;
        }
        self.value = match self.storage {
            TypedSlotStorage::Handle => TypedSlotValue::Handle(value),
            _ => TypedSlotValue::I64(value),
        };
        true
    }

    #[inline(always)]
    pub(crate) fn as_compat_i64(&self) -> Option<i64> {
        if !self.storage.supports_compat_i64() {
            return None;
        }
        match self.value {
            TypedSlotValue::I64(value) | TypedSlotValue::Handle(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_exact_unsigned_u64(&self) -> Option<u64> {
        self.storage.unsigned_max()?;
        let TypedSlotValue::Unsigned(value) = self.value else {
            return None;
        };
        u64::try_from(value).ok()
    }

    pub(crate) fn set_exact_unsigned_u64(&mut self, value: u64) -> bool {
        let Some(max) = self.storage.unsigned_max() else {
            return false;
        };
        let value = value as u128;
        if value > max {
            return false;
        }
        self.value = TypedSlotValue::Unsigned(value);
        true
    }

    pub(crate) fn as_exact_signed_i64(&self) -> Option<i64> {
        self.storage.signed_range()?;
        match self.value {
            TypedSlotValue::I64(value) | TypedSlotValue::Handle(value) => Some(value),
            TypedSlotValue::Signed(value) => i64::try_from(value).ok(),
            TypedSlotValue::Unsigned(_) => None,
        }
    }

    pub(crate) fn set_exact_signed_i64(&mut self, value: i64) -> bool {
        let Some((min, max)) = self.storage.signed_range() else {
            return false;
        };
        let exact_value = value as i128;
        if exact_value < min || exact_value > max {
            return false;
        }
        self.value = match self.storage {
            TypedSlotStorage::I64 => TypedSlotValue::I64(value),
            _ => TypedSlotValue::Signed(exact_value),
        };
        true
    }

    pub(crate) fn rmw_add_exact_unsigned_u64(&mut self, delta: u128) -> Option<i64> {
        let value = self.as_exact_unsigned_u64()?;
        let next = u128::from(value).checked_add(delta)?;
        let next_i64 = i64::try_from(next).ok()?;
        let next_u64 = u64::try_from(next).ok()?;
        if !self.set_exact_unsigned_u64(next_u64) {
            return None;
        }
        Some(next_i64)
    }
}

#[derive(Debug, Clone)]
struct TypedSlotLayout {
    fields: Vec<TypedSlotStorage>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedSlotObject {
    pub(crate) type_id: i64,
    pub(crate) fields: Vec<TypedSlot>,
}

static TYPED_OBJECT_LAYOUTS: OnceLock<Mutex<BTreeMap<i64, TypedSlotLayout>>> = OnceLock::new();
static FIELD_GET_HII_DISPATCH: OnceLock<FieldGetHiiImpl> = OnceLock::new();
static FIELD_SET_HII_DISPATCH: OnceLock<FieldSetHiiImpl> = OnceLock::new();

fn typed_layouts() -> &'static Mutex<BTreeMap<i64, TypedSlotLayout>> {
    TYPED_OBJECT_LAYOUTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub(crate) fn handle_to_index(handle: i64) -> Option<usize> {
    if handle >= 0 {
        return None;
    }
    let idx = handle.checked_neg()?.checked_sub(1)?;
    usize::try_from(idx).ok()
}

fn normalize_field_count(field_count: i64) -> Option<usize> {
    if field_count < 0 || field_count > MAX_TYPED_OBJECT_FIELDS {
        return None;
    }
    usize::try_from(field_count).ok()
}

pub(crate) fn normalize_slot(slot: i64) -> Option<usize> {
    if slot < 0 || slot >= MAX_TYPED_OBJECT_FIELDS {
        return None;
    }
    usize::try_from(slot).ok()
}

fn default_layout_fields(type_id: i64, field_count: usize) -> Vec<TypedSlotStorage> {
    let mut fields = vec![TypedSlotStorage::I64; field_count];
    let Ok(layouts) = typed_layouts().lock() else {
        return fields;
    };
    let Some(layout) = layouts.get(&type_id) else {
        return fields;
    };
    for (slot, storage) in layout.fields.iter().copied().enumerate().take(field_count) {
        fields[slot] = storage;
    }
    fields
}

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
fn field_get_hii_dispatch() -> FieldGetHiiImpl {
    *FIELD_GET_HII_DISPATCH.get_or_init(|| {
        if matches!(
            super::typed_object_store_backend::selected_backend(),
            super::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
        ) {
            field_get_hii_direct
        } else {
            field_get_hii_generic
        }
    })
}

#[inline(always)]
fn field_set_hii_dispatch() -> FieldSetHiiImpl {
    *FIELD_SET_HII_DISPATCH.get_or_init(|| {
        if matches!(
            super::typed_object_store_backend::selected_backend(),
            super::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
        ) {
            field_set_hii_direct
        } else {
            field_set_hii_generic
        }
    })
}

#[inline(always)]
fn field_get_hii_generic(handle: i64, slot: i64) -> i64 {
    if slot < 0 {
        return 0;
    }
    let slot = slot as usize;
    get_compat_i64(handle, slot).unwrap_or(0)
}

#[inline(always)]
fn field_get_hii_direct(handle: i64, slot: i64) -> i64 {
    if slot < 0 || handle < 0 {
        return 0;
    }
    let slot = slot as usize;
    read_direct_slot_compat_i64(handle, slot).unwrap_or(0)
}

#[inline(always)]
fn field_set_hii_generic(handle: i64, slot: i64, value: i64) {
    if slot < 0 {
        return;
    }
    let slot = slot as usize;
    let _ = set_compat_i64(handle, slot, value);
}

#[inline(always)]
fn field_set_hii_direct(handle: i64, slot: i64, value: i64) {
    if slot < 0 || handle < 0 {
        return;
    }
    let slot = slot as usize;
    let _ = write_direct_slot_compat_i64(handle, slot, value);
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
mod tests {
    use super::*;

    #[test]
    fn typed_object_helpers_store_and_load_i64_slots() {
        let object = nyash_object_new_typed_hi(7, 2);
        assert!(object < 0);

        nyash_object_field_set_hii(object, 0, 10);
        nyash_object_field_set_hii(object, 1, 20);

        assert_eq!(nyash_object_field_get_hii(object, 0), 10);
        assert_eq!(nyash_object_field_get_hii(object, 1), 20);
        assert_eq!(nyash_object_field_get_hii(object, 2), 0);
    }

    #[test]
    fn typed_object_layout_registers_exact_usize_slot_kind() {
        let type_id = 294_019_001;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 2), 1);
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, STORAGE_USIZE),
            1
        );
        assert_eq!(
            nyash_object_layout_field_storage_ii(type_id, 0),
            STORAGE_USIZE
        );
        assert_eq!(
            nyash_object_layout_field_storage_ii(type_id, 1),
            STORAGE_I64
        );

        let object = nyash_object_new_typed_hi(type_id, 2);
        assert!(object < 0);
        assert_eq!(nyash_object_field_storage_hii(object, 0), STORAGE_USIZE);
        assert_eq!(nyash_object_field_storage_hii(object, 1), STORAGE_I64);
    }

    #[test]
    fn compat_i64_helpers_do_not_mutate_exact_numeric_slots() {
        let type_id = 294_019_002;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 2), 1);
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, STORAGE_USIZE),
            1
        );
        let object = nyash_object_new_typed_hi(type_id, 2);
        assert!(object < 0);

        nyash_object_field_set_hii(object, 0, 77);
        nyash_object_field_set_hii(object, 1, 88);

        assert_eq!(nyash_object_field_get_hii(object, 0), 0);
        assert_eq!(nyash_object_field_get_hii(object, 1), 88);
        assert_eq!(nyash_object_field_storage_hii(object, 0), STORAGE_USIZE);
    }

    #[test]
    fn typed_object_layout_rejects_unknown_storage_tags() {
        let type_id = 294_019_003;
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, 99_999),
            0
        );
        assert_eq!(nyash_object_layout_field_storage_ii(type_id, 0), 0);
    }

    #[test]
    fn exact_unsigned_abi_reads_and_writes_usize_slots() {
        let type_id = 294_019_101;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, STORAGE_USIZE),
            1
        );
        let object = nyash_object_new_typed_hi(type_id, 1);
        assert!(object < 0);

        assert_eq!(nyash_object_field_set_u64_hiu(object, 0, 123), 1);
        assert_eq!(nyash_object_field_get_u64_hii(object, 0), 123);
        assert_eq!(nyash_object_field_get_hii(object, 0), 0);
    }

    #[test]
    fn pinned_arena_exact_slot_helpers_roundtrip_when_selected() {
        if std::env::var("HAKO_TYPED_OBJECT_STORE").ok().as_deref() != Some("pinned_arena_exact") {
            return;
        }

        let type_id = 294_019_201;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 3), 1);
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, STORAGE_I64),
            1
        );
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 1, STORAGE_U64),
            1
        );
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 2, STORAGE_HANDLE),
            1
        );
        let object = nyash_object_new_typed_hi(type_id, 3);
        assert!(object < 0);

        assert_eq!(hako_object_exact_slot_set_i64_hii(object, 0, 11), 1);
        assert_eq!(hako_object_exact_slot_get_i64_hii(object, 0), 11);
        assert_eq!(hako_object_exact_slot_set_u64_hiu(object, 1, 20), 1);
        assert_eq!(hako_object_exact_slot_rmw_add_u64_hiii(object, 1, 3), 23);
        assert_eq!(hako_object_exact_slot_get_u64_hii(object, 1), 23);
        assert_eq!(hako_object_exact_slot_set_handle_hii(object, 2, -9), 1);
        assert_eq!(hako_object_exact_slot_get_handle_hii(object, 2), -9);
        assert_eq!(nyash_object_exact_slot_set_i64_hii(object, 0, 11), 1);
        assert_eq!(nyash_object_exact_slot_get_i64_hii(object, 0), 11);
        assert_eq!(nyash_object_exact_slot_set_u64_hiu(object, 1, 20), 1);
        assert_eq!(nyash_object_exact_slot_rmw_add_u64_hiii(object, 1, 3), 23);
        assert_eq!(nyash_object_exact_slot_get_u64_hii(object, 1), 23);
        assert_eq!(nyash_object_exact_slot_set_handle_hii(object, 2, -9), 1);
        assert_eq!(nyash_object_exact_slot_get_handle_hii(object, 2), -9);
    }

    #[test]
    fn exact_unsigned_abi_rejects_compat_i64_slots() {
        let object = nyash_object_new_typed_hi(294_019_102, 1);
        assert!(object < 0);

        assert_eq!(nyash_object_field_set_u64_hiu(object, 0, 44), 0);
        assert_eq!(nyash_object_field_get_u64_hii(object, 0), 0);

        nyash_object_field_set_hii(object, 0, 55);
        assert_eq!(nyash_object_field_get_hii(object, 0), 55);
    }

    #[test]
    fn exact_unsigned_abi_range_checks_narrow_slots() {
        let type_id = 294_019_103;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, STORAGE_U8),
            1
        );
        let object = nyash_object_new_typed_hi(type_id, 1);
        assert!(object < 0);

        assert_eq!(nyash_object_field_set_u64_hiu(object, 0, u8::MAX as u64), 1);
        assert_eq!(nyash_object_field_get_u64_hii(object, 0), u8::MAX as u64);
        assert_eq!(
            nyash_object_field_set_u64_hiu(object, 0, u8::MAX as u64 + 1),
            0
        );
        assert_eq!(nyash_object_field_get_u64_hii(object, 0), u8::MAX as u64);
    }

    #[test]
    fn exact_signed_abi_reads_and_writes_i32_slots() {
        let type_id = 294_019_104;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, STORAGE_I32),
            1
        );
        let object = nyash_object_new_typed_hi(type_id, 1);
        assert!(object < 0);

        assert_eq!(
            nyash_object_field_set_i64_hii(object, 0, i32::MIN as i64),
            1
        );
        assert_eq!(nyash_object_field_get_i64_hii(object, 0), i32::MIN as i64);
        assert_eq!(nyash_object_field_set_i64_hii(object, 0, i64::MAX), 0);
        assert_eq!(nyash_object_field_get_i64_hii(object, 0), i32::MIN as i64);
    }

    #[test]
    fn exact_signed_abi_rejects_unsigned_slots() {
        let type_id = 294_019_105;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
        assert_eq!(
            nyash_object_register_typed_layout_slot_iii(type_id, 0, STORAGE_U64),
            1
        );
        let object = nyash_object_new_typed_hi(type_id, 1);
        assert!(object < 0);

        assert_eq!(nyash_object_field_set_i64_hii(object, 0, 1), 0);
        assert_eq!(nyash_object_field_get_i64_hii(object, 0), 0);
        assert_eq!(nyash_object_field_set_u64_hiu(object, 0, u64::MAX), 1);
        assert_eq!(nyash_object_field_get_u64_hii(object, 0), u64::MAX);
    }

    #[test]
    fn exact_numeric_runtime_assert_helpers_accept_in_range_values() {
        assert_eq!(nyash_exact_numeric_assert_i64_min_ii(0, 0), 1);
        assert_eq!(nyash_exact_numeric_assert_i64_min_ii(42, 0), 1);
        assert_eq!(nyash_exact_numeric_assert_i64_range_iii(-5, -5, 5), 1);
        assert_eq!(nyash_exact_numeric_assert_i64_range_iii(5, -5, 5), 1);
    }
}
