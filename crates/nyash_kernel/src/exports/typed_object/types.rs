use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

pub(crate) const MAX_TYPED_OBJECT_FIELDS: i64 = 4096;

pub(crate) const STORAGE_I64: i64 = 1;
pub(crate) const STORAGE_HANDLE: i64 = 2;
pub(crate) const STORAGE_ISIZE: i64 = 3;
pub(crate) const STORAGE_USIZE: i64 = 4;
pub(crate) const STORAGE_I8: i64 = 5;
pub(crate) const STORAGE_I16: i64 = 6;
pub(crate) const STORAGE_I32: i64 = 7;
pub(crate) const STORAGE_U8: i64 = 8;
pub(crate) const STORAGE_U16: i64 = 9;
pub(crate) const STORAGE_U32: i64 = 10;
pub(crate) const STORAGE_U64: i64 = 11;

pub(crate) type FieldGetHiiImpl = fn(i64, i64) -> i64;
pub(crate) type FieldSetHiiImpl = fn(i64, i64, i64);

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
pub(crate) struct TypedSlotLayout {
    pub(crate) fields: Vec<TypedSlotStorage>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypedSlotObject {
    pub(crate) type_id: i64,
    pub(crate) fields: Vec<TypedSlot>,
}

static TYPED_OBJECT_LAYOUTS: OnceLock<Mutex<BTreeMap<i64, TypedSlotLayout>>> = OnceLock::new();
static FIELD_GET_HII_DISPATCH: OnceLock<FieldGetHiiImpl> = OnceLock::new();
static FIELD_SET_HII_DISPATCH: OnceLock<FieldSetHiiImpl> = OnceLock::new();

pub(crate) fn typed_layouts() -> &'static Mutex<BTreeMap<i64, TypedSlotLayout>> {
    TYPED_OBJECT_LAYOUTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub(crate) fn handle_to_index(handle: i64) -> Option<usize> {
    if handle >= 0 {
        return None;
    }
    let idx = handle.checked_neg()?.checked_sub(1)?;
    usize::try_from(idx).ok()
}

pub(crate) fn normalize_field_count(field_count: i64) -> Option<usize> {
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

pub(crate) fn default_layout_fields(type_id: i64, field_count: usize) -> Vec<TypedSlotStorage> {
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

#[inline(always)]
pub(crate) fn field_get_hii_dispatch() -> FieldGetHiiImpl {
    *FIELD_GET_HII_DISPATCH.get_or_init(|| {
        if matches!(
            crate::exports::typed_object_store_backend::selected_backend(),
            crate::exports::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
        ) {
            field_get_hii_direct
        } else {
            field_get_hii_generic
        }
    })
}

#[inline(always)]
pub(crate) fn field_set_hii_dispatch() -> FieldSetHiiImpl {
    *FIELD_SET_HII_DISPATCH.get_or_init(|| {
        if matches!(
            crate::exports::typed_object_store_backend::selected_backend(),
            crate::exports::typed_object_store_backend::TypedObjectStoreBackend::DirectSlotExact
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
    super::get_compat_i64(handle, slot).unwrap_or(0)
}

#[inline(always)]
fn field_get_hii_direct(handle: i64, slot: i64) -> i64 {
    if slot < 0 || handle < 0 {
        return 0;
    }
    let slot = slot as usize;
    crate::exports::typed_object_pinned_arena::read_direct_slot_compat_i64(handle, slot)
        .unwrap_or(0)
}

#[inline(always)]
fn field_set_hii_generic(handle: i64, slot: i64, value: i64) {
    if slot < 0 {
        return;
    }
    let slot = slot as usize;
    let _ = super::set_compat_i64(handle, slot, value);
}

#[inline(always)]
fn field_set_hii_direct(handle: i64, slot: i64, value: i64) {
    if slot < 0 || handle < 0 {
        return;
    }
    let slot = slot as usize;
    let _ = crate::exports::typed_object_pinned_arena::write_direct_slot_compat_i64(
        handle, slot, value,
    );
}
