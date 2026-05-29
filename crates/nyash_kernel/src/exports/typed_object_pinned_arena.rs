//! Pinned typed-object arena substrate for future DirectSlotLease lowering.
//!
//! This module is intentionally storage-only. It does not export C ABI symbols,
//! does not change the default typed-object store, and does not emit direct
//! lowering. The purpose is to provide a stable object/slot substrate that can
//! later back lease-based NativeDirect plans.

use super::typed_object::{TypedSlot, TypedSlotObject, TypedSlotStorage, TypedSlotValue};

const INDEX_BITS: u32 = 31;
const INDEX_MASK: i64 = (1_i64 << INDEX_BITS) - 1;
const DIRECT_SLOT_TAG_I64: u32 = 1;
const DIRECT_SLOT_TAG_U64: u32 = 2;
const DIRECT_SLOT_TAG_HANDLE: u32 = 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirectSlotCellV0 {
    storage_tag: u32,
    flags: u32,
    payload: u64,
}

#[derive(Debug)]
struct PinnedTypedSlotObject {
    #[allow(dead_code)]
    type_id: i64,
    generation: u32,
    fields: Box<[TypedSlot]>,
    direct_cells: Box<[DirectSlotCellV0]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PinnedTypedObjectRef {
    index: usize,
    generation: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectSlotLeaseStorage {
    I64,
    U64,
    Handle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirectSlotLeaseToken {
    object_ref: PinnedTypedObjectRef,
    slot: usize,
    storage: DirectSlotLeaseStorage,
}

#[derive(Debug, Default)]
pub(crate) struct PinnedTypedObjectArena {
    objects: Vec<Option<Box<PinnedTypedSlotObject>>>,
}

impl PinnedTypedObjectArena {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, object: TypedSlotObject) -> Option<i64> {
        let generation = 1;
        let index = self.objects.len();
        let direct_cells = object
            .fields
            .iter()
            .map(DirectSlotCellV0::from_typed_slot)
            .collect();
        self.objects.push(Some(Box::new(PinnedTypedSlotObject {
            type_id: object.type_id,
            generation,
            fields: object.fields.into_boxed_slice(),
            direct_cells,
        })));
        encode_handle(PinnedTypedObjectRef { index, generation })
    }

    pub(crate) fn validate(&self, handle: i64) -> Option<PinnedTypedObjectRef> {
        let object_ref = decode_handle(handle)?;
        let object = self.objects.get(object_ref.index)?.as_ref()?;
        if object.generation != object_ref.generation {
            return None;
        }
        Some(object_ref)
    }

    pub(crate) fn get_field(&self, handle: i64, slot: usize) -> Option<&TypedSlot> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get(object_ref.index)?
            .as_ref()?
            .fields
            .get(slot)
    }

    pub(crate) fn get_field_mut(&mut self, handle: i64, slot: usize) -> Option<&mut TypedSlot> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get_mut(object_ref.index)?
            .as_mut()?
            .fields
            .get_mut(slot)
    }

    pub(crate) fn get_fields(&self, handle: i64) -> Option<&[TypedSlot]> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get(object_ref.index)?
            .as_ref()
            .map(|object| object.fields.as_ref())
    }

    pub(crate) fn get_fields_mut(&mut self, handle: i64) -> Option<&mut [TypedSlot]> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get_mut(object_ref.index)?
            .as_mut()
            .map(|object| object.fields.as_mut())
    }

    pub(crate) fn lease_slot(
        &self,
        handle: i64,
        slot: usize,
        storage: DirectSlotLeaseStorage,
    ) -> Option<DirectSlotLeaseToken> {
        let object_ref = self.validate(handle)?;
        let field = self
            .objects
            .get(object_ref.index)?
            .as_ref()?
            .fields
            .get(slot)?;
        if !lease_storage_matches(field.storage, storage) {
            return None;
        }
        Some(DirectSlotLeaseToken {
            object_ref,
            slot,
            storage,
        })
    }

    pub(crate) fn get_direct_cell(&self, handle: i64, slot: usize) -> Option<&DirectSlotCellV0> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get(object_ref.index)?
            .as_ref()?
            .direct_cells
            .get(slot)
    }

    pub(crate) fn get_direct_cell_mut(
        &mut self,
        handle: i64,
        slot: usize,
    ) -> Option<&mut DirectSlotCellV0> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get_mut(object_ref.index)?
            .as_mut()?
            .direct_cells
            .get_mut(slot)
    }

    pub(crate) fn read_i64(&self, token: DirectSlotLeaseToken) -> Option<i64> {
        if token.storage != DirectSlotLeaseStorage::I64 {
            return None;
        }
        self.cell_by_token(token)?.read_i64()
    }

    pub(crate) fn write_i64(&mut self, token: DirectSlotLeaseToken, value: i64) -> bool {
        if token.storage != DirectSlotLeaseStorage::I64 {
            return false;
        }
        let Some(cell) = self.cell_by_token_mut(token) else {
            return false;
        };
        if !cell.write_i64(value) {
            return false;
        }
        let Some(field) = self.field_by_token_mut(token) else {
            return false;
        };
        if field.storage != TypedSlotStorage::I64 {
            return false;
        }
        field.value = TypedSlotValue::I64(value);
        true
    }

    pub(crate) fn read_u64(&self, token: DirectSlotLeaseToken) -> Option<u64> {
        if token.storage != DirectSlotLeaseStorage::U64 {
            return None;
        }
        self.cell_by_token(token)?.read_u64()
    }

    pub(crate) fn write_u64(&mut self, token: DirectSlotLeaseToken, value: u64) -> bool {
        if token.storage != DirectSlotLeaseStorage::U64 {
            return false;
        }
        let Some(cell) = self.cell_by_token_mut(token) else {
            return false;
        };
        if !cell.write_u64(value) {
            return false;
        }
        let Some(field) = self.field_by_token_mut(token) else {
            return false;
        };
        if field.storage != TypedSlotStorage::U64 {
            return false;
        }
        field.value = TypedSlotValue::Unsigned(value as u128);
        true
    }

    pub(crate) fn read_handle(&self, token: DirectSlotLeaseToken) -> Option<i64> {
        if token.storage != DirectSlotLeaseStorage::Handle {
            return None;
        }
        self.cell_by_token(token)?.read_handle()
    }

    pub(crate) fn write_handle(&mut self, token: DirectSlotLeaseToken, value: i64) -> bool {
        if token.storage != DirectSlotLeaseStorage::Handle {
            return false;
        }
        let Some(cell) = self.cell_by_token_mut(token) else {
            return false;
        };
        if !cell.write_handle(value) {
            return false;
        }
        let Some(field) = self.field_by_token_mut(token) else {
            return false;
        };
        if field.storage != TypedSlotStorage::Handle {
            return false;
        }
        field.value = TypedSlotValue::Handle(value);
        true
    }

    fn cell_by_token(&self, token: DirectSlotLeaseToken) -> Option<&DirectSlotCellV0> {
        let object = self.objects.get(token.object_ref.index)?.as_ref()?;
        if object.generation != token.object_ref.generation {
            return None;
        }
        let cell = object.direct_cells.get(token.slot)?;
        if !cell.storage_matches(token.storage) {
            return None;
        }
        Some(cell)
    }

    fn cell_by_token_mut(&mut self, token: DirectSlotLeaseToken) -> Option<&mut DirectSlotCellV0> {
        let object = self.objects.get_mut(token.object_ref.index)?.as_mut()?;
        if object.generation != token.object_ref.generation {
            return None;
        }
        let cell = object.direct_cells.get_mut(token.slot)?;
        if !cell.storage_matches(token.storage) {
            return None;
        }
        Some(cell)
    }

    fn field_by_token_mut(&mut self, token: DirectSlotLeaseToken) -> Option<&mut TypedSlot> {
        let object = self.objects.get_mut(token.object_ref.index)?.as_mut()?;
        if object.generation != token.object_ref.generation {
            return None;
        }
        let field = object.fields.get_mut(token.slot)?;
        if !lease_storage_matches(field.storage, token.storage) {
            return None;
        }
        Some(field)
    }

    #[cfg(test)]
    fn field_address_token(&self, handle: i64, slot: usize) -> Option<usize> {
        self.get_field(handle, slot)
            .map(|field| field as *const TypedSlot as usize)
    }
}

fn lease_storage_matches(storage: TypedSlotStorage, lease_storage: DirectSlotLeaseStorage) -> bool {
    matches!(
        (storage, lease_storage),
        (TypedSlotStorage::I64, DirectSlotLeaseStorage::I64)
            | (TypedSlotStorage::U64, DirectSlotLeaseStorage::U64)
            | (TypedSlotStorage::Handle, DirectSlotLeaseStorage::Handle)
    )
}

impl DirectSlotCellV0 {
    fn from_typed_slot(slot: &TypedSlot) -> Self {
        match (slot.storage, &slot.value) {
            (TypedSlotStorage::I64, TypedSlotValue::I64(value)) => Self::from_i64(*value),
            (TypedSlotStorage::U64, TypedSlotValue::Unsigned(value)) => {
                Self::from_u64(u64::try_from(*value).unwrap_or(0))
            }
            (TypedSlotStorage::Handle, TypedSlotValue::Handle(value)) => Self::from_handle(*value),
            _ => Self {
                storage_tag: 0,
                flags: 0,
                payload: 0,
            },
        }
    }

    fn from_i64(value: i64) -> Self {
        Self {
            storage_tag: DIRECT_SLOT_TAG_I64,
            flags: 0,
            payload: value as u64,
        }
    }

    fn from_u64(value: u64) -> Self {
        Self {
            storage_tag: DIRECT_SLOT_TAG_U64,
            flags: 0,
            payload: value,
        }
    }

    fn from_handle(value: i64) -> Self {
        Self {
            storage_tag: DIRECT_SLOT_TAG_HANDLE,
            flags: 0,
            payload: value as u64,
        }
    }

    fn storage_matches(&self, lease_storage: DirectSlotLeaseStorage) -> bool {
        matches!(
            (self.storage_tag, lease_storage),
            (DIRECT_SLOT_TAG_I64, DirectSlotLeaseStorage::I64)
                | (DIRECT_SLOT_TAG_U64, DirectSlotLeaseStorage::U64)
                | (DIRECT_SLOT_TAG_HANDLE, DirectSlotLeaseStorage::Handle)
        )
    }

    fn read_i64(&self) -> Option<i64> {
        (self.storage_tag == DIRECT_SLOT_TAG_I64).then_some(self.payload as i64)
    }

    fn write_i64(&mut self, value: i64) -> bool {
        if self.storage_tag != DIRECT_SLOT_TAG_I64 {
            return false;
        }
        self.payload = value as u64;
        true
    }

    fn read_u64(&self) -> Option<u64> {
        (self.storage_tag == DIRECT_SLOT_TAG_U64).then_some(self.payload)
    }

    fn write_u64(&mut self, value: u64) -> bool {
        if self.storage_tag != DIRECT_SLOT_TAG_U64 {
            return false;
        }
        self.payload = value;
        true
    }

    fn read_handle(&self) -> Option<i64> {
        (self.storage_tag == DIRECT_SLOT_TAG_HANDLE).then_some(self.payload as i64)
    }

    fn write_handle(&mut self, value: i64) -> bool {
        if self.storage_tag != DIRECT_SLOT_TAG_HANDLE {
            return false;
        }
        self.payload = value as u64;
        true
    }
}

fn encode_handle(object_ref: PinnedTypedObjectRef) -> Option<i64> {
    if object_ref.generation == 0 {
        return None;
    }
    let index_payload = i64::try_from(object_ref.index).ok()?.checked_add(1)?;
    if index_payload <= 0 || index_payload > INDEX_MASK {
        return None;
    }
    let generation_payload = i64::from(object_ref.generation).checked_shl(INDEX_BITS)?;
    let payload = generation_payload | index_payload;
    payload.checked_neg()
}

fn decode_handle(handle: i64) -> Option<PinnedTypedObjectRef> {
    if handle >= 0 {
        return None;
    }
    let payload = handle.checked_neg()?;
    let index_payload = payload & INDEX_MASK;
    if index_payload == 0 {
        return None;
    }
    let generation_payload = payload >> INDEX_BITS;
    let generation = u32::try_from(generation_payload).ok()?;
    if generation == 0 {
        return None;
    }
    let index = usize::try_from(index_payload.checked_sub(1)?).ok()?;
    Some(PinnedTypedObjectRef { index, generation })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object_with_fields(fields: &[TypedSlotStorage]) -> TypedSlotObject {
        TypedSlotObject {
            type_id: 7,
            fields: fields.iter().copied().map(TypedSlot::new).collect(),
        }
    }

    fn object_with_i64_fields(field_count: usize) -> TypedSlotObject {
        object_with_fields(&vec![TypedSlotStorage::I64; field_count])
    }

    #[test]
    fn pinned_arena_allocates_generation_checked_negative_handles() {
        let mut arena = PinnedTypedObjectArena::new();
        let handle = arena.insert(object_with_i64_fields(2)).unwrap();

        assert!(handle < 0);
        let object_ref = arena.validate(handle).unwrap();
        assert_eq!(object_ref.index, 0);
        assert_eq!(object_ref.generation, 1);

        let stale = encode_handle(PinnedTypedObjectRef {
            index: 0,
            generation: 2,
        })
        .unwrap();
        assert!(arena.validate(stale).is_none());
        assert!(arena.validate(-1).is_none());
    }

    #[test]
    fn pinned_arena_keeps_slot_address_stable_across_mutation() {
        let mut arena = PinnedTypedObjectArena::new();
        let handle = arena.insert(object_with_i64_fields(1)).unwrap();
        let before = arena.field_address_token(handle, 0).unwrap();

        let field = arena.get_field_mut(handle, 0).unwrap();
        field.value = TypedSlotValue::I64(42);

        let after = arena.field_address_token(handle, 0).unwrap();
        assert_eq!(before, after);
        assert_eq!(
            arena.get_field(handle, 0).unwrap().value,
            TypedSlotValue::I64(42)
        );
    }

    #[test]
    fn direct_slot_lease_token_reads_and_writes_supported_storage() {
        let mut arena = PinnedTypedObjectArena::new();
        let handle = arena
            .insert(object_with_fields(&[
                TypedSlotStorage::I64,
                TypedSlotStorage::U64,
                TypedSlotStorage::Handle,
            ]))
            .unwrap();

        let i64_token = arena
            .lease_slot(handle, 0, DirectSlotLeaseStorage::I64)
            .unwrap();
        let u64_token = arena
            .lease_slot(handle, 1, DirectSlotLeaseStorage::U64)
            .unwrap();
        let handle_token = arena
            .lease_slot(handle, 2, DirectSlotLeaseStorage::Handle)
            .unwrap();

        assert!(arena.write_i64(i64_token, 11));
        assert_eq!(arena.read_i64(i64_token), Some(11));
        assert!(arena.write_u64(u64_token, 22));
        assert_eq!(arena.read_u64(u64_token), Some(22));
        assert!(arena.write_handle(handle_token, -9));
        assert_eq!(arena.read_handle(handle_token), Some(-9));
    }

    #[test]
    fn direct_slot_lease_rejects_wrong_storage_class() {
        let mut arena = PinnedTypedObjectArena::new();
        let handle = arena.insert(object_with_i64_fields(1)).unwrap();

        assert!(arena
            .lease_slot(handle, 0, DirectSlotLeaseStorage::U64)
            .is_none());
        let token = arena
            .lease_slot(handle, 0, DirectSlotLeaseStorage::I64)
            .unwrap();
        assert_eq!(arena.read_u64(token), None);
        assert!(!arena.write_u64(token, 7));
    }

    #[test]
    fn direct_slot_cell_v0_layout_is_stable() {
        assert_eq!(std::mem::size_of::<DirectSlotCellV0>(), 16);
        assert_eq!(std::mem::align_of::<DirectSlotCellV0>(), 8);
    }

    #[test]
    fn direct_slot_cells_preserve_tagged_payloads() {
        let mut arena = PinnedTypedObjectArena::new();
        let handle = arena
            .insert(object_with_fields(&[
                TypedSlotStorage::I64,
                TypedSlotStorage::U64,
                TypedSlotStorage::Handle,
            ]))
            .unwrap();

        let i64_cell = arena.get_direct_cell(handle, 0).unwrap();
        assert_eq!(i64_cell.storage_tag, DIRECT_SLOT_TAG_I64);
        assert_eq!(i64_cell.read_i64(), Some(0));

        let u64_cell = arena.get_direct_cell(handle, 1).unwrap();
        assert_eq!(u64_cell.storage_tag, DIRECT_SLOT_TAG_U64);
        assert_eq!(u64_cell.read_u64(), Some(0));

        let handle_cell = arena.get_direct_cell(handle, 2).unwrap();
        assert_eq!(handle_cell.storage_tag, DIRECT_SLOT_TAG_HANDLE);
        assert_eq!(handle_cell.read_handle(), Some(0));

        assert!(arena.get_direct_cell_mut(handle, 0).unwrap().write_i64(-13));
        assert_eq!(
            arena.get_direct_cell(handle, 0).unwrap().read_i64(),
            Some(-13)
        );
    }
}
