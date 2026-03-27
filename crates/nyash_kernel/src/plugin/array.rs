pub use super::array_compat::*;
pub use super::array_runtime_facade::*;
pub use super::array_substrate::*;

#[cfg(test)]
use super::handle_cache::with_array_box;

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::box_trait::NyashBox;
    use nyash_rust::boxes::array::ArrayBox;
    use nyash_rust::runtime::host_handles as handles;
    use std::sync::Arc;

    fn new_array_handle() -> i64 {
        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        handles::to_handle_arc(arr) as i64
    }

    #[test]
    fn legacy_set_h_returns_zero_but_applies_value() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 11), 1);

        // Legacy contract keeps return=0 even when applied.
        assert_eq!(nyash_array_set_h(handle, 0, 77), 0);
        assert_eq!(nyash_array_get_h(handle, 0), 77);
    }

    #[test]
    fn hi_hii_aliases_keep_fail_safe_contract() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 10), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), 10);

        assert_eq!(nyash_array_set_hii_alias(handle, 0, 33), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), 33);
        assert_eq!(nyash_array_has_hi_alias(handle, 0), 1);

        // Out-of-bounds keeps fail-safe return values.
        assert_eq!(nyash_array_get_hi_alias(handle, 3), 0);
        assert_eq!(nyash_array_set_hii_alias(handle, 3, 9), 0);
        assert_eq!(nyash_array_has_hi_alias(handle, 3), 0);
    }

    #[test]
    fn push_hi_alias_appends_integer_value() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_hi_alias(handle, 7), 1);
        assert_eq!(nyash_array_push_hi_alias(handle, 9), 2);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), 7);
        assert_eq!(nyash_array_get_hi_alias(handle, 1), 9);
    }

    #[test]
    fn slot_load_store_raw_aliases_keep_contract() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 10), 1);

        assert_eq!(nyash_array_slot_len_h_alias(handle), 1);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 0), 10);
        assert_eq!(nyash_array_slot_store_hii_alias(handle, 0, 44), 1);
        assert_eq!(nyash_array_slot_len_h_alias(handle), 1);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 0), 44);

        assert_eq!(nyash_array_slot_len_h_alias(0), 0);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 3), 0);
        assert_eq!(nyash_array_slot_store_hii_alias(handle, 3, 9), 0);
    }

    #[test]
    fn slot_rmw_add1_raw_alias_updates_integer_slot_and_returns_new_value() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 10), 1);

        assert_eq!(nyash_array_rmw_add1_hi_alias(handle, 0), 11);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 0), 11);
        assert_eq!(nyash_array_rmw_add1_hi_alias(handle, 3), 0);
    }

    #[test]
    fn string_len_raw_alias_reads_string_slot_directly() {
        let handle = new_array_handle();
        let string_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("length".to_string()),
        ) as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle), 1);
        assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 6);
        assert_eq!(nyash_array_string_len_hi_alias(handle, 3), 0);
    }

    #[test]
    fn string_indexof_raw_alias_reads_string_slot_directly() {
        let handle = new_array_handle();
        let hay_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line-seed".to_string()),
        ) as std::sync::Arc<dyn NyashBox>) as i64;
        let needle_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line".to_string()),
        ) as std::sync::Arc<dyn NyashBox>) as i64;
        let miss_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("none".to_string()),
        ) as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_set_his_alias(handle, 0, hay_handle), 1);
        assert_eq!(nyash_array_string_indexof_hih_alias(handle, 0, needle_handle), 0);
        assert_eq!(nyash_array_string_indexof_hih_alias(handle, 0, miss_handle), -1);
        assert_eq!(nyash_array_string_indexof_hih_alias(handle, 3, needle_handle), -1);
    }

    #[test]
    fn slot_append_raw_alias_keeps_contract() {
        let handle = new_array_handle();
        let string_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("slot-append".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_slot_append_hh_alias(handle, string_handle), 1);
        assert_eq!(nyash_array_slot_len_h_alias(handle), 1);
        assert_eq!(nyash_array_get_hh_alias(handle, 0), string_handle);
        assert_eq!(nyash_array_slot_append_hh_alias(0, string_handle), 0);
    }

    #[test]
    fn slot_reserve_and_grow_raw_aliases_keep_length_and_expand_capacity() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 1), 1);

        let before_cap = with_array_box(handle, |arr| arr.capacity()).unwrap_or(0);
        assert_eq!(nyash_array_slot_cap_h_alias(handle), before_cap as i64);
        assert_eq!(nyash_array_slot_reserve_hi_alias(handle, 8), 1);
        let after_reserve_cap = with_array_box(handle, |arr| arr.capacity()).unwrap_or(0);
        assert_eq!(
            nyash_array_slot_cap_h_alias(handle),
            after_reserve_cap as i64
        );
        assert!(after_reserve_cap >= before_cap);
        assert_eq!(nyash_array_length_h(handle), 1);

        assert_eq!(nyash_array_slot_grow_hi_alias(handle, 32), 1);
        let after_grow_cap = with_array_box(handle, |arr| arr.capacity()).unwrap_or(0);
        assert_eq!(nyash_array_slot_cap_h_alias(handle), after_grow_cap as i64);
        assert!(after_grow_cap >= 32);
        assert_eq!(nyash_array_length_h(handle), 1);
    }

    #[test]
    fn set_his_alias_sets_string_handle_value() {
        let handle = new_array_handle();
        assert_eq!(nyash_array_push_h(handle, 1), 1);
        let string_handle_a = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("ok".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let string_handle_b = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("ng".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle_a), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), string_handle_a);
        // Re-set same slot keeps alias contract and must expose the latest handle.
        assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle_b), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), string_handle_b);
    }
}
