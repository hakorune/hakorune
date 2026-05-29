//! Pinned typed-object arena substrate for future DirectSlotLease lowering.
//!
//! This module is intentionally storage-only. It does not export C ABI symbols,
//! does not change the default typed-object store, and does not emit direct
//! lowering. The purpose is to provide a stable object/slot substrate that can
//! later back lease-based NativeDirect plans.

use super::typed_object::{TypedSlot, TypedSlotObject, TypedSlotStorage, TypedSlotValue};

const INDEX_BITS: u32 = 31;
const INDEX_MASK: i64 = (1_i64 << INDEX_BITS) - 1;

#[derive(Debug)]
struct PinnedTypedSlotObject {
    #[allow(dead_code)]
    type_id: i64,
    generation: u32,
    fields: Box<[TypedSlot]>,
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
        self.objects.push(Some(Box::new(PinnedTypedSlotObject {
            type_id: object.type_id,
            generation,
            fields: object.fields.into_boxed_slice(),
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

    pub(crate) fn read_i64(&self, token: DirectSlotLeaseToken) -> Option<i64> {
        if token.storage != DirectSlotLeaseStorage::I64 {
            return None;
        }
        let field = self.field_by_token(token)?;
        match field.value {
            TypedSlotValue::I64(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn write_i64(&mut self, token: DirectSlotLeaseToken, value: i64) -> bool {
        if token.storage != DirectSlotLeaseStorage::I64 {
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
        let field = self.field_by_token(token)?;
        let TypedSlotValue::Unsigned(value) = field.value else {
            return None;
        };
        u64::try_from(value).ok()
    }

    pub(crate) fn write_u64(&mut self, token: DirectSlotLeaseToken, value: u64) -> bool {
        if token.storage != DirectSlotLeaseStorage::U64 {
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
        let field = self.field_by_token(token)?;
        match field.value {
            TypedSlotValue::Handle(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn write_handle(&mut self, token: DirectSlotLeaseToken, value: i64) -> bool {
        if token.storage != DirectSlotLeaseStorage::Handle {
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

    fn field_by_token(&self, token: DirectSlotLeaseToken) -> Option<&TypedSlot> {
        let object = self.objects.get(token.object_ref.index)?.as_ref()?;
        if object.generation != token.object_ref.generation {
            return None;
        }
        let field = object.fields.get(token.slot)?;
        if !lease_storage_matches(field.storage, token.storage) {
            return None;
        }
        Some(field)
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
}
