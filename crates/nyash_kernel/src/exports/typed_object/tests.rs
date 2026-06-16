use super::types::{
    STORAGE_HANDLE, STORAGE_I32, STORAGE_I64, STORAGE_U64, STORAGE_U8, STORAGE_USIZE,
};
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
