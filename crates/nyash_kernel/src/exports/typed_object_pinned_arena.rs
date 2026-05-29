//! Pinned typed-object arena substrate for future DirectSlotLease lowering.
//!
//! This module is intentionally storage-only. It does not export C ABI symbols,
//! does not change the default typed-object store, and does not emit direct
//! lowering. The purpose is to provide a stable object/slot substrate that can
//! later back lease-based NativeDirect plans.

use super::typed_object::{TypedSlot, TypedSlotObject};

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

    #[cfg(test)]
    fn field_address_token(&self, handle: i64, slot: usize) -> Option<usize> {
        self.get_field(handle, slot)
            .map(|field| field as *const TypedSlot as usize)
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
    use crate::exports::typed_object::{TypedSlotStorage, TypedSlotValue};

    fn object_with_i64_fields(field_count: usize) -> TypedSlotObject {
        TypedSlotObject {
            type_id: 7,
            fields: (0..field_count)
                .map(|_| TypedSlot::new(TypedSlotStorage::I64))
                .collect(),
        }
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
}
