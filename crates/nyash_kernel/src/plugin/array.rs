pub use super::array_compat::*;
pub use super::array_runtime_aliases::*;
pub use super::array_substrate::*;

#[cfg(test)]
use super::array_handle_cache::with_array_box;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nyash_string_kernel_slot_len_i_export;
    use nyash_rust::box_trait::{NyashBox, StringBox};
    use nyash_rust::boxes::array::ArrayBox;
    use nyash_rust::runtime::host_handles as handles;
    use std::sync::Arc;

    fn new_array_handle() -> i64 {
        let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        handles::to_handle_arc(arr) as i64
    }

    fn new_integer_handle(value: i64) -> i64 {
        let int_box: Arc<dyn NyashBox> = Arc::new(nyash_rust::box_trait::IntegerBox::new(value));
        handles::to_handle_arc(int_box) as i64
    }

    fn new_string_handle(value: &str) -> i64 {
        let string_box: Arc<dyn NyashBox> = Arc::new(StringBox::new(value.to_string()));
        handles::to_handle_arc(string_box) as i64
    }

    fn storage_tag(handle: i64) -> Option<String> {
        with_array_box(handle, |arr| format!("{arr:?}"))
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
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("inline_i64")));
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
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle), 1);
        assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 6);
        assert_eq!(nyash_array_string_len_hi_alias(handle, 3), 0);
    }

    #[test]
    fn kernel_slot_store_alias_writes_string_slot_without_publish_handle() {
        let handle = new_array_handle();
        let lhs_h = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line-seed-abcdef".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let rhs_h = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("xy".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            crate::nyash_string_kernel_slot_concat_hh_export(&mut slot, lhs_h, rhs_h),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 18);
        assert_eq!(
            nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 18);
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 0);
    }

    #[test]
    fn kernel_slot_concat_by_index_reads_string_slot_directly() {
        let handle = new_array_handle();
        let seed_h = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line-seed".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let suffix = std::ffi::CString::new("ln").expect("suffix");
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(nyash_array_set_his_alias(handle, 0, seed_h), 1);
        assert_eq!(
            crate::nyash_array_kernel_slot_concat_his_alias(&mut slot, handle, 0, suffix.as_ptr()),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 11);
        assert_eq!(
            nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 0);
        assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 11);
    }

    #[test]
    fn kernel_slot_insert_by_index_reads_string_slot_directly() {
        let handle = new_array_handle();
        let seed_h = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line-seed".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let middle = std::ffi::CString::new("xx").expect("middle");
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(nyash_array_set_his_alias(handle, 0, seed_h), 1);
        assert_eq!(
            crate::nyash_array_kernel_slot_insert_hisi_alias(
                &mut slot,
                handle,
                0,
                middle.as_ptr(),
                4,
            ),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 11);
        assert_eq!(
            nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 0);

        let stored = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                items
                    .first()
                    .and_then(|item| item.as_str_fast())
                    .map(str::to_string)
            })
        })
        .flatten();
        assert_eq!(stored.as_deref(), Some("linexx-seed"));
    }

    #[test]
    fn insert_mid_store_by_index_mutates_string_slot_directly() {
        let handle = new_array_handle();
        let middle = std::ffi::CString::new("xx").expect("middle");

        assert_eq!(
            with_array_box(handle, |arr| arr.slot_store_box_raw(
                0,
                Box::new(nyash_rust::box_trait::StringBox::new("line-seed"))
            ))
            .unwrap_or(false),
            true
        );
        let before_ptr = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                items
                    .first()
                    .and_then(|item| {
                        item.as_any()
                            .downcast_ref::<nyash_rust::box_trait::StringBox>()
                    })
                    .map(|value| value as *const nyash_rust::box_trait::StringBox as usize)
            })
        })
        .flatten();
        assert_eq!(
            crate::nyash_array_string_insert_mid_store_hisi_alias(handle, 0, middle.as_ptr(), 4,),
            1
        );
        let stored = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                let value = items.first().and_then(|item| {
                    item.as_any()
                        .downcast_ref::<nyash_rust::box_trait::StringBox>()
                });
                value.map(|value| {
                    (
                        value as *const nyash_rust::box_trait::StringBox as usize,
                        value.value.clone(),
                    )
                })
            })
        })
        .flatten();
        assert_eq!(
            stored.as_ref().map(|(_, text)| text.as_str()),
            Some("linexx-seed")
        );
        assert_eq!(stored.map(|(ptr, _)| ptr), before_ptr);
    }

    #[test]
    fn insert_mid_store_by_index_materializes_alias_slot_without_mutating_source() {
        let handle = new_array_handle();
        let seed_h = new_string_handle("line-seed");
        let middle = std::ffi::CString::new("xx").expect("middle");

        assert_eq!(nyash_array_set_his_alias(handle, 0, seed_h), 1);
        let before_is_alias = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                items.first().is_some_and(|item| {
                    item.as_any()
                        .downcast_ref::<crate::plugin::value_codec::BorrowedHandleBox>()
                        .is_some()
                })
            })
        })
        .unwrap_or(false);
        assert!(before_is_alias);

        assert_eq!(
            crate::nyash_array_string_insert_mid_store_hisi_alias(handle, 0, middle.as_ptr(), 4,),
            1
        );

        let stored = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                items.first().and_then(|item| {
                    item.as_any()
                        .downcast_ref::<StringBox>()
                        .map(|value| value.value.clone())
                })
            })
        })
        .flatten();
        assert_eq!(stored.as_deref(), Some("linexx-seed"));

        let source = handles::get(seed_h as u64).and_then(|source| {
            source
                .as_ref()
                .as_any()
                .downcast_ref::<StringBox>()
                .map(|value| value.value.clone())
        });
        assert_eq!(source.as_deref(), Some("line-seed"));
    }

    #[test]
    fn kernel_slot_store_existing_string_slot_keeps_borrowed_alias_wrapper() {
        let handle = new_array_handle();
        let seed_h = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line-seed-abcdef".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let rhs_h = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("xy".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(nyash_array_set_his_alias(handle, 0, seed_h), 1);
        assert_eq!(
            crate::nyash_string_kernel_slot_concat_hh_export(&mut slot, seed_h, rhs_h),
            1
        );
        assert_eq!(
            nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 0);
        assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 18);

        let kept = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                let item = items.first().expect("stored string slot");
                item.as_any()
                    .downcast_ref::<crate::plugin::value_codec::BorrowedHandleBox>()
                    .map(|alias| {
                        (
                            alias.borrowed_handle_source_fast().is_none(),
                            alias.as_str_fast().map(str::to_string),
                        )
                    })
            })
        })
        .flatten()
        .expect("borrowed alias slot");
        assert!(kept.0);
        assert_eq!(kept.1.as_deref(), Some("line-seed-abcdefxy"));
    }

    #[test]
    fn kernel_slot_store_existing_string_box_overwrites_in_place() {
        let handle = new_array_handle();
        let lhs_h = new_string_handle("line-seed");
        let suffix = std::ffi::CString::new("xy").expect("CString");
        let mut slot = crate::plugin::KernelTextSlot::empty();

        with_array_box(handle, |arr| {
            arr.with_items_write(|items| {
                items.push(Box::new(StringBox::new("seed-old".to_string())) as Box<dyn NyashBox>);
            });
        })
        .expect("array write");

        let before = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                let item = items.first().expect("stored string slot");
                (
                    item.box_id(),
                    item.as_any()
                        .downcast_ref::<StringBox>()
                        .map(|s| s.value.clone()),
                )
            })
        })
        .expect("array read");
        assert_eq!(before.1.as_deref(), Some("seed-old"));

        assert_eq!(
            crate::nyash_string_kernel_slot_concat_hs_export(&mut slot, lhs_h, suffix.as_ptr()),
            1
        );
        assert_eq!(
            slot.state(),
            crate::plugin::KernelTextSlotState::DeferredConstSuffix
        );
        assert_eq!(
            nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(slot.state(), crate::plugin::KernelTextSlotState::Empty);

        let after = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                let item = items.first().expect("stored string slot");
                (
                    item.box_id(),
                    item.as_any()
                        .downcast_ref::<StringBox>()
                        .map(|s| s.value.clone()),
                )
            })
        })
        .expect("array read");
        assert_eq!(after.0, before.0);
        assert_eq!(after.1.as_deref(), Some("line-seedxy"));
    }

    #[test]
    fn kernel_slot_const_suffix_store_existing_alias_keeps_borrowed_wrapper() {
        let handle = new_array_handle();
        let seed_h = new_string_handle("line-seed");
        let suffix = std::ffi::CString::new("xy").expect("CString");
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(nyash_array_set_his_alias(handle, 0, seed_h), 1);
        assert_eq!(
            crate::nyash_string_kernel_slot_concat_hs_export(&mut slot, seed_h, suffix.as_ptr()),
            1
        );
        assert_eq!(
            slot.state(),
            crate::plugin::KernelTextSlotState::DeferredConstSuffix
        );
        assert_eq!(
            nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(slot.state(), crate::plugin::KernelTextSlotState::Empty);
        assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 11);

        let kept = with_array_box(handle, |arr| {
            arr.with_items_read(|items| {
                let item = items.first().expect("stored string slot");
                item.as_any()
                    .downcast_ref::<crate::plugin::value_codec::BorrowedHandleBox>()
                    .map(|alias| {
                        (
                            alias.borrowed_handle_source_fast().is_none(),
                            alias.as_str_fast().map(str::to_string),
                        )
                    })
            })
        })
        .flatten()
        .expect("borrowed alias slot");
        assert!(kept.0);
        assert_eq!(kept.1.as_deref(), Some("line-seedxy"));
    }

    #[test]
    fn kernel_slot_const_suffix_store_alias_writes_string_slot_without_publish_handle() {
        let handle = new_array_handle();
        let lhs_h = new_string_handle("line-seed");
        let suffix = std::ffi::CString::new("xy").expect("CString");
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            crate::nyash_string_kernel_slot_concat_hs_export(&mut slot, lhs_h, suffix.as_ptr()),
            1
        );
        assert_eq!(
            slot.state(),
            crate::plugin::KernelTextSlotState::DeferredConstSuffix
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 11);
        assert_eq!(
            nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 11);
        assert_eq!(slot.state(), crate::plugin::KernelTextSlotState::Empty);
    }

    #[test]
    fn string_indexof_raw_alias_reads_string_slot_directly() {
        let handle = new_array_handle();
        let hay_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line-seed".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let needle_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("line".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;
        let miss_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new("none".to_string()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_set_his_alias(handle, 0, hay_handle), 1);
        assert_eq!(
            nyash_array_string_indexof_hih_alias(handle, 0, needle_handle),
            0
        );
        assert_eq!(
            nyash_array_string_indexof_hih_alias(handle, 0, miss_handle),
            -1
        );
        assert_eq!(
            nyash_array_string_indexof_hih_alias(handle, 3, needle_handle),
            -1
        );
    }

    #[test]
    fn string_indexof_raw_alias_keeps_empty_needle_fail_safe_contract() {
        let handle = new_array_handle();
        let empty_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::StringBox::new(String::new()),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(
            nyash_array_string_indexof_hih_alias(handle, 0, empty_handle),
            0
        );
        assert_eq!(nyash_array_string_indexof_hih_alias(0, 0, empty_handle), 0);
        assert_eq!(
            nyash_array_string_indexof_hih_alias(handle, -1, empty_handle),
            0
        );
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
    fn slot_append_raw_alias_births_inline_i64_lane_for_integer_values() {
        let handle = new_array_handle();
        let int_handle = new_integer_handle(7);
        let next_int_handle = new_integer_handle(9);

        assert_eq!(nyash_array_slot_append_hh_alias(handle, int_handle), 1);
        assert_eq!(nyash_array_slot_append_hh_alias(handle, next_int_handle), 2);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 0), 7);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 1), 9);
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("inline_i64")));
    }

    #[test]
    fn slot_store_any_bool_handle_births_inline_bool_lane() {
        let handle = new_array_handle();
        let bool_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::BoolBox::new(true),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_slot_store_hih_alias(handle, 0, bool_handle), 1);
        assert_eq!(nyash_array_slot_len_h_alias(handle), 1);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 0), 1);
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("inline_bool")));
    }

    #[test]
    fn slot_append_raw_alias_births_inline_bool_lane_for_bool_values() {
        let handle = new_array_handle();
        let bool_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::box_trait::BoolBox::new(true),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_slot_append_hh_alias(handle, bool_handle), 1);
        assert_eq!(nyash_array_slot_load_hi_alias(handle, 0), 1);
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("inline_bool")));
    }

    #[test]
    fn slot_store_any_float_handle_births_inline_f64_lane() {
        let handle = new_array_handle();
        let float_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::boxes::FloatBox::new(1.25),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_slot_store_hih_alias(handle, 0, float_handle), 1);
        assert_eq!(nyash_array_slot_len_h_alias(handle), 1);
        let got = nyash_array_slot_load_hi_alias(handle, 0);
        assert!(got > 0, "expected encoded FloatBox handle, got {got}");
        let got_text = nyash_rust::runtime::host_handles::with_handle(got as u64, |obj| {
            obj.map(|obj| obj.to_string_box().value)
        });
        assert_eq!(got_text.as_deref(), Some("1.25"));
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("inline_f64")));
    }

    #[test]
    fn slot_append_raw_alias_births_inline_f64_lane_for_float_values() {
        let handle = new_array_handle();
        let float_handle = nyash_rust::runtime::host_handles::to_handle_arc(std::sync::Arc::new(
            nyash_rust::boxes::FloatBox::new(2.5),
        )
            as std::sync::Arc<dyn NyashBox>) as i64;

        assert_eq!(nyash_array_slot_append_hh_alias(handle, float_handle), 1);
        let got = nyash_array_slot_load_hi_alias(handle, 0);
        assert!(got > 0, "expected encoded FloatBox handle, got {got}");
        let got_text = nyash_rust::runtime::host_handles::with_handle(got as u64, |obj| {
            obj.map(|obj| obj.to_string_box().value)
        });
        assert_eq!(got_text.as_deref(), Some("2.5"));
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("inline_f64")));
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
        assert_eq!(nyash_array_push_hi_alias(handle, 1), 1);
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("inline_i64")));
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
        assert!(storage_tag(handle)
            .as_deref()
            .is_some_and(|text| text.contains("boxed")));
        // Re-set same slot keeps alias contract and must expose the latest handle.
        assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle_b), 1);
        assert_eq!(nyash_array_get_hi_alias(handle, 0), string_handle_b);
    }
}
