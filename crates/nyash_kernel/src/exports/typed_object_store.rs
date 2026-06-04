//! Storage backends for typed user object exported helpers.
//!
//! `typed_object.rs` owns the C ABI. This module keeps the exported API thin
//! and delegates backend routing and exact-field handling to focused helpers.

#[cfg(test)]
use super::typed_object::TypedSlotObject;
#[cfg(test)]
use super::typed_object::TypedSlotStorage;
#[cfg(test)]
use super::typed_object::{TypedSlot, TypedSlotValue};
#[cfg(test)]
use super::typed_object_direct_slot_backend::{
    materialize_direct_slot_snapshot as bridge_materialize_direct_slot_snapshot,
    materialize_direct_slot_view_handle as bridge_materialize_direct_slot_view_handle,
};
#[cfg(test)]
use super::typed_object_pinned_arena::DirectSlotObjectV0Box;
#[cfg(test)]
use super::typed_object_store_backend::new_typed_object as backend_new_typed_object;
#[cfg(test)]
use super::typed_object_store_backend::TYPED_OBJECT_STORE_ENV;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exports::typed_object::{
        get_compat_i64, get_exact_signed_i64, get_exact_unsigned_u64, set_compat_i64,
        set_exact_unsigned_u64,
    };
    use crate::exports::typed_object_store_backend::{
        exact_slot_record_alloc_success, exact_slot_record_release_success, exact_slot_set4_i64,
    };

    fn new_typed_object(object: TypedSlotObject) -> Option<i64> {
        backend_new_typed_object(object)
    }

    fn require_direct_slot_exact() -> bool {
        if std::env::var(TYPED_OBJECT_STORE_ENV).ok().as_deref() != Some("direct_slot_exact") {
            eprintln!("skip: set HAKO_TYPED_OBJECT_STORE=direct_slot_exact");
            return false;
        }
        true
    }

    fn direct_slot_object(
        type_id: i64,
        fields: Vec<(TypedSlotStorage, TypedSlotValue)>,
    ) -> TypedSlotObject {
        TypedSlotObject {
            type_id,
            fields: fields
                .into_iter()
                .map(|(storage, value)| TypedSlot { storage, value })
                .collect(),
        }
    }

    #[test]
    fn direct_slot_exact_new_object_returns_tagged_pointer_handle() {
        if !require_direct_slot_exact() {
            return;
        }
        let object = direct_slot_object(
            17,
            vec![
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::U64, TypedSlotValue::Unsigned(0)),
                (TypedSlotStorage::Handle, TypedSlotValue::Handle(0)),
            ],
        );
        let handle = new_typed_object(object).unwrap();

        assert_eq!(handle & 1, 1);
        assert!(DirectSlotObjectV0Box::from_handle(handle).is_some());
        assert_eq!(get_compat_i64(handle, 0), Some(0));
        assert!(set_compat_i64(handle, 0, 1));
        assert_eq!(get_exact_signed_i64(handle, 0), Some(1));
        assert!(set_exact_unsigned_u64(handle, 1, 2));
        assert_eq!(get_exact_unsigned_u64(handle, 1), Some(2));
        assert!(set_compat_i64(handle, 2, -5));
        assert_eq!(get_compat_i64(handle, 2), Some(-5));
    }

    #[test]
    fn direct_slot_exact_materializes_typed_slot_snapshot_explicitly() {
        if !require_direct_slot_exact() {
            return;
        }
        let object = direct_slot_object(
            23,
            vec![
                (TypedSlotStorage::I64, TypedSlotValue::I64(-7)),
                (TypedSlotStorage::U64, TypedSlotValue::Unsigned(11)),
                (TypedSlotStorage::Handle, TypedSlotValue::Handle(-3)),
            ],
        );
        let handle = new_typed_object(object).unwrap();

        let snapshot = bridge_materialize_direct_slot_snapshot(handle).unwrap();
        assert_eq!(snapshot.type_id, 23);
        assert_eq!(snapshot.fields.len(), 3);
        assert_eq!(snapshot.fields[0].value, TypedSlotValue::I64(-7));
        assert_eq!(snapshot.fields[1].value, TypedSlotValue::Unsigned(11));
        assert_eq!(snapshot.fields[2].value, TypedSlotValue::Handle(-3));
        assert!(bridge_materialize_direct_slot_snapshot(-1).is_none());
        assert_eq!(get_compat_i64(handle, 0), Some(-7));
    }

    #[test]
    fn direct_slot_exact_materialized_view_handle_routes_existing_helpers() {
        if !require_direct_slot_exact() {
            return;
        }
        let object = direct_slot_object(
            31,
            vec![
                (TypedSlotStorage::I64, TypedSlotValue::I64(-9)),
                (TypedSlotStorage::U64, TypedSlotValue::Unsigned(99)),
                (TypedSlotStorage::Handle, TypedSlotValue::Handle(-5)),
            ],
        );
        let direct_handle = new_typed_object(object).unwrap();
        assert!(direct_handle > 0);
        assert_eq!(get_compat_i64(direct_handle, 0), Some(-9));

        let view_handle = bridge_materialize_direct_slot_view_handle(direct_handle).unwrap();
        assert!(view_handle < 0);
        assert_eq!(get_compat_i64(view_handle, 0), Some(-9));
        assert_eq!(get_exact_unsigned_u64(view_handle, 1), Some(99));
        assert_eq!(get_compat_i64(view_handle, 2), Some(-5));
        assert!(set_compat_i64(view_handle, 0, 13));
        assert_eq!(get_compat_i64(view_handle, 0), Some(13));

        let direct_snapshot = bridge_materialize_direct_slot_snapshot(direct_handle).unwrap();
        assert_eq!(direct_snapshot.fields[0].value, TypedSlotValue::I64(-9));
    }

    #[test]
    fn direct_slot_exact_positive_handle_helpers_update_primary_cells() {
        if !require_direct_slot_exact() {
            return;
        }
        let object = direct_slot_object(
            41,
            vec![
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::USize, TypedSlotValue::Unsigned(0)),
                (TypedSlotStorage::Handle, TypedSlotValue::Handle(0)),
            ],
        );
        let handle = new_typed_object(object).unwrap();

        assert!(set_compat_i64(handle, 0, -11));
        assert!(set_exact_unsigned_u64(handle, 1, 29));
        assert!(set_compat_i64(handle, 2, -17));
        assert_eq!(get_compat_i64(handle, 0), Some(-11));
        assert_eq!(get_exact_unsigned_u64(handle, 1), Some(29));
        assert_eq!(get_compat_i64(handle, 2), Some(-17));

        let snapshot = bridge_materialize_direct_slot_snapshot(handle).unwrap();
        assert_eq!(snapshot.fields[0].value, TypedSlotValue::I64(-11));
        assert_eq!(snapshot.fields[1].value, TypedSlotValue::Unsigned(29));
        assert_eq!(snapshot.fields[2].value, TypedSlotValue::Handle(-17));
    }

    #[test]
    fn direct_slot_exact_set4_i64_accepts_exact_length_objects() {
        if !require_direct_slot_exact() {
            return;
        }
        let object = direct_slot_object(
            51,
            vec![
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
            ],
        );
        let handle = new_typed_object(object).unwrap();

        assert!(exact_slot_set4_i64(handle, 0, 1, 2, 3, 4));
        assert_eq!(get_exact_signed_i64(handle, 0), Some(1));
        assert_eq!(get_exact_signed_i64(handle, 1), Some(2));
        assert_eq!(get_exact_signed_i64(handle, 2), Some(3));
        assert_eq!(get_exact_signed_i64(handle, 3), Some(4));
    }

    #[test]
    fn direct_slot_exact_success_records_update_expected_counters() {
        if !require_direct_slot_exact() {
            return;
        }

        let alloc_object = direct_slot_object(
            61,
            vec![
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::U64, TypedSlotValue::Unsigned(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::U64, TypedSlotValue::Unsigned(0)),
                (TypedSlotStorage::U64, TypedSlotValue::Unsigned(0)),
            ],
        );
        let alloc_handle = new_typed_object(alloc_object).unwrap();

        assert!(exact_slot_record_alloc_success(alloc_handle, 1));
        assert_eq!(get_exact_signed_i64(alloc_handle, 2), Some(0));
        assert_eq!(get_exact_signed_i64(alloc_handle, 3), Some(1));
        assert_eq!(get_exact_unsigned_u64(alloc_handle, 5), Some(1));
        assert_eq!(get_exact_unsigned_u64(alloc_handle, 7), Some(1));
        assert_eq!(get_exact_unsigned_u64(alloc_handle, 8), Some(0));

        let release_object = direct_slot_object(
            62,
            vec![
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::I64, TypedSlotValue::I64(0)),
                (TypedSlotStorage::U64, TypedSlotValue::Unsigned(0)),
            ],
        );
        let release_handle = new_typed_object(release_object).unwrap();

        assert!(exact_slot_record_release_success(release_handle, 21, 22));
        assert_eq!(get_exact_signed_i64(release_handle, 0), Some(21));
        assert_eq!(get_exact_signed_i64(release_handle, 1), Some(22));
        assert_eq!(get_exact_signed_i64(release_handle, 2), Some(0));
        assert_eq!(get_exact_signed_i64(release_handle, 3), Some(1));
        assert_eq!(get_exact_unsigned_u64(release_handle, 4), Some(1));
    }
}
