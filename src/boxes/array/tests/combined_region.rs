use super::*;

#[test]
fn combined_region_all_hit_appends_each_observer_period() {
    let array = ArrayBox::new();
    for idx in 0..4 {
        assert!(array.slot_store_text_raw(idx, "line-seed".to_string()));
    }

    assert_eq!(
        array.slot_text_lenhalf_insert_mid_periodic_indexof_suffix_region_byte_boundary_safe_raw(
            4, 4, "xx", 1, 4, "line", "ln",
        ),
        Some(4)
    );

    for idx in 0..4 {
        let value = array.slot_with_text_raw(idx, str::to_owned).unwrap();
        assert!(value.contains("line"), "idx={idx} value={value}");
        assert!(value.ends_with("lnlnlnln"), "idx={idx} value={value}");
    }
}

#[test]
fn combined_region_all_hit_guard_keeps_miss_fallback_behavior() {
    let array = ArrayBox::new();
    assert!(array.slot_store_text_raw(0, "line-seed".to_string()));
    assert!(array.slot_store_text_raw(1, "seed-only".to_string()));

    assert_eq!(
        array.slot_text_lenhalf_insert_mid_periodic_indexof_suffix_region_byte_boundary_safe_raw(
            2, 2, "xx", 1, 2, "line", "ln",
        ),
        Some(2)
    );

    let hit = array.slot_with_text_raw(0, str::to_owned).unwrap();
    let miss = array.slot_with_text_raw(1, str::to_owned).unwrap();
    assert!(hit.ends_with("lnln"), "hit={hit}");
    assert!(!miss.ends_with("ln"), "miss={miss}");
}
