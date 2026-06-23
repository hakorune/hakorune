pub use super::map_aliases::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exports::typed_object::{
        nyash_object_new_typed_hi, nyash_object_register_typed_layout_hi, nyash_object_type_id_h,
    };
    use crate::nyash_runtime_data_has_hh;
    use nyash_rust::box_trait::{IntegerBox, NyashBox, StringBox};
    use nyash_rust::boxes::map_box::MapBox;
    use nyash_rust::runtime::host_handles as handles;
    use std::sync::Arc;

    fn new_map_handle() -> i64 {
        let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
        handles::to_handle_arc(map) as i64
    }

    fn string_handle(value: &str) -> i64 {
        let value: Arc<dyn NyashBox> = Arc::new(StringBox::new(value.to_string()));
        handles::to_handle_arc(value) as i64
    }

    fn decode_string_from_handle(handle: i64) -> String {
        let object = handles::get(handle as u64).expect("map raw load handle");
        let string_box = object
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("map raw load must resolve StringBox");
        string_box.value.clone()
    }

    #[test]
    fn slot_probe_raw_aliases_keep_hh_contract() {
        let handle = new_map_handle();
        let key_handle = string_handle("slot-key");
        let value_handle = string_handle("slot-value");

        assert_eq!(
            nyash_map_slot_store_hhh_alias(handle, key_handle, value_handle),
            1
        );
        assert_eq!(nyash_map_probe_hh_alias(handle, key_handle), 1);
        let got_handle = nyash_map_slot_load_hh_alias(handle, key_handle);
        assert!(got_handle > 0);
        assert_eq!(decode_string_from_handle(got_handle), "slot-value");

        assert_eq!(
            nyash_map_probe_hh_alias(handle, string_handle("missing")),
            0
        );
        assert_eq!(
            nyash_map_slot_load_hh_alias(handle, string_handle("missing")),
            0
        );
    }

    #[test]
    fn slot_probe_raw_aliases_keep_hi_contract() {
        let handle = new_map_handle();
        let value_handle = string_handle("value-hi");

        assert_eq!(
            nyash_map_slot_store_hih_alias(handle, -70001, value_handle),
            1
        );
        assert_eq!(nyash_map_probe_hi_alias(handle, -70001), 1);
        let got_handle = nyash_map_slot_load_hi_alias(handle, -70001);
        assert!(got_handle > 0);
        assert_eq!(decode_string_from_handle(got_handle), "value-hi");

        assert_eq!(nyash_map_probe_hi_alias(handle, -70002), 0);
        assert_eq!(nyash_map_slot_load_hi_alias(handle, -70002), 0);
    }

    #[test]
    fn scalar_load_hi_keeps_no_publication_scalar_contract() {
        let handle = new_map_handle();
        let string_value = string_handle("scalar-load-non-scalar");

        assert_eq!(nyash_map_slot_store_hih_alias(handle, -71001, 42), 1);
        assert_eq!(nyash_map_scalar_load_hi_alias(handle, -71001), 42);

        assert_eq!(
            nyash_map_slot_store_hih_alias(handle, -71002, string_value),
            1
        );
        assert_eq!(nyash_map_slot_load_hi_alias(handle, -71002), string_value);
        assert_eq!(nyash_map_scalar_load_hi_alias(handle, -71002), 0);

        assert_eq!(nyash_map_scalar_load_hi_alias(handle, -71003), 0);
        assert_eq!(nyash_map_scalar_load_hi_alias(0, -71001), 0);
    }

    #[test]
    fn slot_load_hi_preserves_typed_object_handle_values() {
        let handle = new_map_handle();
        let type_id = 710_240_001;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
        let object = nyash_object_new_typed_hi(type_id, 1);
        assert_ne!(object, 0);
        assert_eq!(nyash_object_type_id_h(object), type_id);

        assert_eq!(nyash_map_slot_store_hih_alias(handle, -72001, object), 1);
        let loaded = nyash_map_slot_load_hi_alias(handle, -72001);
        assert_eq!(loaded, object);
        assert_eq!(nyash_object_type_id_h(loaded), type_id);
    }

    #[test]
    fn slot_load_hi_keeps_negative_carrier_bits_without_sign_inference() {
        let handle = new_map_handle();
        let type_id = 710_240_002;
        assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
        let object = nyash_object_new_typed_hi(type_id, 1);
        assert!(object < 0);

        assert_eq!(nyash_map_slot_store_hih_alias(handle, -72011, object), 1);
        handles::with_handle(handle as u64, |map| {
            let map = map
                .expect("map handle")
                .as_any()
                .downcast_ref::<MapBox>()
                .expect("MapBox");
            map.insert_key_str("-72012".to_string(), Box::new(IntegerBox::new(object)));
        });

        let object_carrier = nyash_map_slot_load_hi_alias(handle, -72011);
        let scalar_carrier = nyash_map_slot_load_hi_alias(handle, -72012);
        assert_eq!(object_carrier, scalar_carrier);
        assert_eq!(nyash_object_type_id_h(object_carrier), type_id);
    }

    #[test]
    fn slot_load_hi_materializes_borrowed_string_after_source_drop() {
        let _guard = crate::test_support::handle_registry_test_lock();
        let handle = new_map_handle();
        let value_handle = string_handle("borrowed-map-slot");

        assert_eq!(
            nyash_map_slot_store_hih_alias(handle, -72021, value_handle),
            1
        );
        handles::drop_handle(value_handle as u64);

        let loaded = nyash_map_slot_load_hi_alias(handle, -72021);
        assert!(loaded > 0);
        assert_eq!(decode_string_from_handle(loaded), "borrowed-map-slot");
    }

    #[test]
    fn raw_aliases_keep_fail_safe_contract() {
        assert_eq!(nyash_map_slot_load_hi_alias(0, 1), 0);
        assert_eq!(nyash_map_slot_load_hh_alias(0, 1), 0);
        assert_eq!(nyash_map_slot_store_hih_alias(0, 1, 2), 0);
        assert_eq!(nyash_map_slot_store_hhh_alias(0, 1, 2), 0);
        assert_eq!(nyash_map_probe_hi_alias(0, 1), 0);
        assert_eq!(nyash_map_probe_hh_alias(0, 1), 0);
    }

    #[test]
    fn entry_count_raw_alias_keeps_contract() {
        let handle = new_map_handle();
        let key_a = string_handle("entry-a");
        let key_b = string_handle("entry-b");
        let value = string_handle("entry-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_a, value), 1);
        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_b, value), 1);
        assert_eq!(nyash_map_entry_count_i64(handle), 2);
        assert_eq!(nyash_map_entry_count_i64(0), 0);
    }

    #[test]
    fn capacity_raw_alias_keeps_observer_contract() {
        let handle = new_map_handle();
        let key_a = string_handle("cap-a");
        let key_b = string_handle("cap-b");
        let value = string_handle("cap-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_a, value), 1);
        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key_b, value), 1);
        assert!(nyash_map_cap_h(handle) >= nyash_map_entry_count_i64(handle));
        assert_eq!(nyash_map_cap_h(0), 0);
    }

    #[test]
    fn clear_raw_alias_keeps_contract() {
        let handle = new_map_handle();
        let key = string_handle("clear-key");
        let value = string_handle("clear-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key, value), 1);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 1);
        assert_eq!(nyash_map_clear_h(handle), 0);
        assert_eq!(nyash_map_entry_count_i64(handle), 0);
        assert_eq!(nyash_map_probe_hh_alias(handle, key), 0);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 0);
        assert_eq!(nyash_map_clear_h(0), 0);
    }

    #[test]
    fn delete_raw_alias_keeps_contract() {
        let handle = new_map_handle();
        let key = string_handle("delete-key");
        let value = string_handle("delete-value");

        assert_eq!(nyash_map_slot_store_hhh_alias(handle, key, value), 1);
        assert_eq!(nyash_map_entry_count_i64(handle), 1);
        assert_eq!(nyash_map_delete_hh_alias(handle, key), 1);
        assert_eq!(nyash_map_entry_count_i64(handle), 0);
        assert_eq!(nyash_map_probe_hh_alias(handle, key), 0);
        assert_eq!(nyash_map_slot_load_hh_alias(handle, key), 0);
        assert_eq!(nyash_runtime_data_has_hh(handle, key), 0);
        assert_eq!(nyash_map_delete_hh_alias(handle, key), 0);
        assert_eq!(nyash_map_delete_hh_alias(0, key), 0);
    }
}
