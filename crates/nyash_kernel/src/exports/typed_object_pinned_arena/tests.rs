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
    let handle = arena
        .insert(object_with_i64_fields(2))
        .expect("pinned handle");
    assert!(handle < 0);
    assert!(arena.validate(handle).is_some());
    assert!(arena.get_field(handle, 0).is_some());
    assert!(arena.get_fields_mut(handle).is_some());
}

#[test]
fn pinned_arena_keeps_slot_address_stable_across_mutation() {
    let mut arena = PinnedTypedObjectArena::new();
    let handle = arena
        .insert(object_with_fields(&[TypedSlotStorage::I64]))
        .expect("pinned handle");
    let before = arena
        .get_field(handle, 0)
        .map(|field| field as *const TypedSlot)
        .expect("field");
    let field = arena.get_field_mut(handle, 0).expect("field mut");
    field.set_compat_i64(7);
    let after = arena
        .get_field(handle, 0)
        .map(|field| field as *const TypedSlot)
        .expect("field");
    assert_eq!(before, after);
}

#[test]
fn direct_slot_cell_v0_layout_is_stable() {
    assert!(std::mem::size_of::<DirectSlotCellV0>() >= 16);
    assert!(std::mem::align_of::<DirectSlotCellV0>() >= std::mem::align_of::<u64>());
}

#[test]
fn direct_slot_cell_v0_preserves_tagged_payloads() {
    let slot = TypedSlot {
        storage: TypedSlotStorage::I64,
        value: TypedSlotValue::I64(42),
    };
    let cell = DirectSlotCellV0::from_typed_slot(&slot);
    assert_eq!(cell.read_compat_i64(), Some(42));
}

#[test]
fn direct_slot_object_snapshot_preserves_usize_storage() {
    let object = DirectSlotObjectV0Box::new(
        7,
        1,
        &[
            DirectSlotCellV0::from_i64(1),
            DirectSlotCellV0::from_u64(2),
            DirectSlotCellV0::from_usize(3),
        ],
    )
    .expect("direct slot object");
    let snapshot = object
        .materialize_typed_object_snapshot()
        .expect("snapshot");
    assert_eq!(snapshot.fields[0].storage, TypedSlotStorage::I64);
    assert_eq!(snapshot.fields[1].storage, TypedSlotStorage::U64);
    assert_eq!(snapshot.fields[2].storage, TypedSlotStorage::USize);
}

#[test]
fn direct_slot_object_v0_header_and_field_offsets_are_stable() {
    let object = DirectSlotObjectV0Box::new(7, 1, &[DirectSlotCellV0::from_i64(1)]).expect("box");
    assert!(object.as_ptr() as usize > 0);
    assert!(object.handle().expect("handle") < 0);
}

#[test]
fn direct_slot_object_handle_roundtrips_stable_pointer() {
    let object = DirectSlotObjectV0Box::new(7, 1, &[DirectSlotCellV0::from_i64(1)]).expect("box");
    let handle = object.handle().expect("handle");
    assert!(object.matches_handle(handle));
    assert!(DirectSlotObjectV0Box::from_handle(handle).is_some());
}
