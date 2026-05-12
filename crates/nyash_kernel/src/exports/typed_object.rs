// Typed user object runtime helpers for EXE lowering.
//
// This is intentionally slot-based and opaque to the backend. MIR owns layout
// truth; the runtime owns allocation and field storage.

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypedSlotStorage {
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

    fn tag(self) -> i64 {
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

    fn is_legacy_i64_compatible(self) -> bool {
        matches!(self, Self::I64 | Self::Handle)
    }

    fn is_unsigned_exact(self) -> bool {
        matches!(
            self,
            Self::USize | Self::U8 | Self::U16 | Self::U32 | Self::U64
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypedSlotValue {
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
struct TypedSlot {
    storage: TypedSlotStorage,
    value: TypedSlotValue,
}

impl TypedSlot {
    fn new(storage: TypedSlotStorage) -> Self {
        Self {
            storage,
            value: TypedSlotValue::default_for(storage),
        }
    }

    fn as_legacy_i64(&self) -> Option<i64> {
        match self.value {
            TypedSlotValue::I64(value) => Some(value),
            TypedSlotValue::Handle(value) => Some(value),
            TypedSlotValue::Signed(_) | TypedSlotValue::Unsigned(_) => None,
        }
    }

    fn set_legacy_i64(&mut self, value: i64) -> bool {
        if !self.storage.is_legacy_i64_compatible() {
            return false;
        }
        self.value = match self.storage {
            TypedSlotStorage::Handle => TypedSlotValue::Handle(value),
            _ => TypedSlotValue::I64(value),
        };
        true
    }
}

#[derive(Debug, Clone)]
struct TypedSlotLayout {
    fields: Vec<TypedSlotStorage>,
}

#[derive(Debug, Clone)]
struct TypedSlotObject {
    #[allow(dead_code)]
    type_id: i64,
    fields: Vec<TypedSlot>,
}

static TYPED_OBJECT_LAYOUTS: OnceLock<Mutex<BTreeMap<i64, TypedSlotLayout>>> = OnceLock::new();
static TYPED_OBJECTS: OnceLock<Mutex<Vec<TypedSlotObject>>> = OnceLock::new();

fn typed_layouts() -> &'static Mutex<BTreeMap<i64, TypedSlotLayout>> {
    TYPED_OBJECT_LAYOUTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn typed_objects() -> &'static Mutex<Vec<TypedSlotObject>> {
    TYPED_OBJECTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn handle_to_index(handle: i64) -> Option<usize> {
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

fn normalize_slot(slot: i64) -> Option<usize> {
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
    let mut objects = match typed_objects().lock() {
        Ok(objects) => objects,
        Err(_) => return 0,
    };
    objects.push(TypedSlotObject { type_id, fields });
    -(objects.len() as i64)
}

#[export_name = "nyash.object.new_typed_h"]
pub extern "C" fn nyash_object_new_typed_h(type_id: i64) -> i64 {
    nyash_object_new_typed_hi(type_id, 0)
}

#[export_name = "nyash.object.field_get_hii"]
pub extern "C" fn nyash_object_field_get_hii(handle: i64, slot: i64) -> i64 {
    if slot < 0 {
        return 0;
    }
    let Some(idx) = handle_to_index(handle) else {
        return 0;
    };
    let slot = slot as usize;
    let objects = match typed_objects().lock() {
        Ok(objects) => objects,
        Err(_) => return 0,
    };
    objects
        .get(idx)
        .and_then(|object| object.fields.get(slot))
        .and_then(TypedSlot::as_legacy_i64)
        .unwrap_or(0)
}

#[export_name = "nyash.object.field_set_hii"]
pub extern "C" fn nyash_object_field_set_hii(handle: i64, slot: i64, value: i64) {
    if slot < 0 {
        return;
    }
    let Some(idx) = handle_to_index(handle) else {
        return;
    };
    let slot = slot as usize;
    let mut objects = match typed_objects().lock() {
        Ok(objects) => objects,
        Err(_) => return,
    };
    if let Some(field) = objects
        .get_mut(idx)
        .and_then(|object| object.fields.get_mut(slot))
    {
        let _ = field.set_legacy_i64(value);
    }
}

#[export_name = "nyash.object.field_storage_hii"]
pub extern "C" fn nyash_object_field_storage_hii(handle: i64, slot: i64) -> i64 {
    let Some(idx) = handle_to_index(handle) else {
        return 0;
    };
    let Some(slot) = normalize_slot(slot) else {
        return 0;
    };
    let objects = match typed_objects().lock() {
        Ok(objects) => objects,
        Err(_) => return 0,
    };
    objects
        .get(idx)
        .and_then(|object| object.fields.get(slot))
        .map(|field| field.storage.tag())
        .unwrap_or(0)
}

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
    fn legacy_i64_helpers_do_not_mutate_exact_numeric_slots() {
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
}
