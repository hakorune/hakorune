use super::super::*;
use crate::c_string::cstring;
use crate::test_support::{handle_registry_test_lock, with_env_var};
use nyash_rust::{box_trait::NyashBox, boxes::array::ArrayBox, runtime::host_handles as handles};
use std::sync::Arc;

#[test]
fn string_kernel_slot_concat_hs_len_publish_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let lhs_h = string_handle("line-seed");
        let suffix = cstring("::tail");
        let direct_h = nyash_string_concat_hs_export(lhs_h, suffix.as_ptr());
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            nyash_string_kernel_slot_concat_hs_export(&mut slot, lhs_h, suffix.as_ptr()),
            1
        );
        assert_eq!(
            slot.state(),
            crate::plugin::KernelTextSlotState::DeferredConstSuffix
        );
        assert_eq!(
            nyash_string_kernel_slot_len_i_export(&slot),
            nyash_string_len_h(direct_h)
        );

        let helper_h = nyash_string_kernel_slot_publish_h_export(&mut slot);
        assert!(helper_h > 0);
        assert_eq!(
            decode_string_like_handle(helper_h),
            decode_string_like_handle(direct_h)
        );
    });
}

#[test]
fn string_kernel_slot_insert_hsi_len_publish_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let source_h = string_handle("line-seed");
        let middle = cstring("xx");
        let direct_h = nyash_string_insert_hsi_export(source_h, middle.as_ptr(), 4);
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            nyash_string_kernel_slot_insert_hsi_export(&mut slot, source_h, middle.as_ptr(), 4),
            1
        );
        assert_eq!(
            nyash_string_kernel_slot_len_i_export(&slot),
            nyash_string_len_h(direct_h)
        );

        let helper_h = nyash_string_kernel_slot_publish_h_export(&mut slot);
        assert!(helper_h > 0);
        assert_eq!(
            decode_string_like_handle(helper_h),
            decode_string_like_handle(direct_h)
        );
    });
}

#[test]
fn string_kernel_slot_piecewise_substring_publish_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let source_h = string_handle("prefix-suffix");
        let middle = cstring("::mid::");
        let direct_h = nyash_string_substring_hii_export(
            nyash_string_piecewise_subrange_hsiii_export(source_h, middle.as_ptr(), 6, 3, 16),
            1,
            10,
        );
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            nyash_string_kernel_slot_piecewise_subrange_hsiii_export(
                &mut slot,
                source_h,
                middle.as_ptr(),
                6,
                3,
                16,
            ),
            1
        );
        assert_eq!(
            nyash_string_kernel_slot_substring_hii_in_place_export(&mut slot, 1, 10),
            1
        );

        let helper_h = nyash_string_kernel_slot_publish_h_export(&mut slot);
        assert!(helper_h > 0);
        assert_eq!(
            decode_string_like_handle(helper_h),
            decode_string_like_handle(direct_h)
        );
        assert_eq!(nyash_string_len_h(helper_h), nyash_string_len_h(direct_h));
    });
}

#[test]
fn string_kernel_slot_piecewise_subrange_store_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let handle = handles::to_handle_arc(array) as i64;
        let source_h = string_handle("prefix-suffix");
        let middle = cstring("::mid::");
        let direct_h =
            nyash_string_piecewise_subrange_hsiii_export(source_h, middle.as_ptr(), 6, 3, 16);
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            nyash_string_kernel_slot_piecewise_subrange_hsiii_export(
                &mut slot,
                source_h,
                middle.as_ptr(),
                6,
                3,
                16,
            ),
            1
        );
        assert_eq!(
            nyash_string_kernel_slot_len_i_export(&slot),
            nyash_string_len_h(direct_h)
        );
        assert_eq!(
            crate::nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(
            crate::nyash_array_string_len_hi_alias(handle, 0),
            nyash_string_len_h(direct_h)
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 0);
        assert_eq!(
            decode_string_like_handle(crate::nyash_array_get_hi_alias(handle, 0)).as_deref(),
            decode_string_like_handle(direct_h).as_deref()
        );
    });
}

#[test]
fn string_kernel_slot_concat_len_publish_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let lhs_h = string_handle("line-seed-abcdef");
        let rhs_h = string_handle("xy");
        let direct_h = nyash_string_concat_hh_export(lhs_h, rhs_h);
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            nyash_string_kernel_slot_concat_hh_export(&mut slot, lhs_h, rhs_h),
            1
        );
        assert_eq!(
            nyash_string_kernel_slot_len_i_export(&slot),
            nyash_string_len_h(direct_h)
        );

        let helper_h = nyash_string_kernel_slot_publish_h_export(&mut slot);
        assert!(helper_h > 0);
        assert_eq!(
            decode_string_like_handle(helper_h),
            decode_string_like_handle(direct_h)
        );
    });
}

#[test]
fn string_kernel_slot_concat_hs_store_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let handle = handles::to_handle_arc(array) as i64;
        let lhs_h = string_handle("line-seed");
        let suffix = cstring("xy");
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            nyash_string_kernel_slot_concat_hs_export(&mut slot, lhs_h, suffix.as_ptr()),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 11);
        assert_eq!(
            crate::nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(crate::nyash_array_string_len_hi_alias(handle, 0), 11);
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 0);
    });
}

#[test]
fn string_kernel_slot_insert_hsi_store_contract() {
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let handle = handles::to_handle_arc(array) as i64;
        let source_h = string_handle("line-seed");
        let middle = cstring("xx");
        let mut slot = crate::plugin::KernelTextSlot::empty();

        assert_eq!(
            nyash_string_kernel_slot_insert_hsi_export(&mut slot, source_h, middle.as_ptr(), 4),
            1
        );
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 11);
        assert_eq!(
            crate::nyash_array_kernel_slot_store_hi_alias(handle, 0, &mut slot),
            1
        );
        assert_eq!(crate::nyash_array_string_len_hi_alias(handle, 0), 11);
        assert_eq!(nyash_string_kernel_slot_len_i_export(&slot), 0);
    });
}

#[test]
fn string_kernel_slot_capture_piecewise_loop_publish_contract() {
    let _guard = handle_registry_test_lock();
    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let source_h = string_handle("line-seed-abcdef");
        let middle = cstring("xx");
        let middle_text = middle.to_str().expect("middle text");
        let split = 8;
        let start = 1;
        let end = 17;
        let mut current = crate::plugin::KernelTextSlot::empty();
        let mut next = crate::plugin::KernelTextSlot::empty();
        let mut expected = "line-seed-abcdef".to_string();

        assert_eq!(
            nyash_string_kernel_slot_capture_h_export(&mut current, source_h),
            1
        );

        for _ in 0..4 {
            let split_idx = split.min(expected.len() as i64) as usize;
            let mut inserted = String::with_capacity(expected.len() + middle_text.len());
            inserted.push_str(&expected[..split_idx]);
            inserted.push_str(middle_text);
            inserted.push_str(&expected[split_idx..]);
            let (slice_start, slice_end) =
                crate::exports::string_view::clamp_i64_range(inserted.len(), start, end);
            expected = inserted[slice_start..slice_end].to_string();
            assert_eq!(
                nyash_string_kernel_slot_piecewise_subrange_ssiii_export(
                    &mut next,
                    &current,
                    middle.as_ptr(),
                    split,
                    start,
                    end,
                ),
                1
            );
            std::mem::swap(&mut current, &mut next);
            next.clear();
        }

        let helper_h = nyash_string_kernel_slot_publish_h_export(&mut current);
        assert!(helper_h > 0);
        assert_eq!(
            decode_string_like_handle(helper_h).as_deref(),
            Some(expected.as_str())
        );
        assert_eq!(nyash_string_len_h(helper_h), expected.len() as i64);
    });
}
