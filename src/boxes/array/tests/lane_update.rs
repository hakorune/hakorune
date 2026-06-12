use super::*;

#[test]
fn slot_update_text_mutates_text_lane_without_boxing() {
    let array = ArrayBox::new();
    assert!(array.slot_store_text_raw(0, "line".to_string()));
    assert_eq!(
        array.slot_update_text_raw(0, |value| {
            value.push_str("-seed");
            value.len()
        }),
        Some(9)
    );
    assert!(array.uses_text_slots());
    assert_eq!(
        array.slot_with_text_raw(0, str::to_owned).as_deref(),
        Some("line-seed")
    );
}

#[test]
fn slot_update_text_resident_first_reports_existing_text_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_text_raw(0, "line".to_string()));

    assert_eq!(
        array.slot_update_text_resident_first_raw(0, |value| {
            value.push_str("-seed");
            value.len()
        }),
        Some((9, true))
    );
    assert!(array.uses_text_slots());
    assert_eq!(
        array.slot_with_text_raw(0, str::to_owned).as_deref(),
        Some("line-seed")
    );
}

#[test]
fn slot_update_text_resident_raw_does_not_promote_boxed_string_lane() {
    let array =
        ArrayBox::new_with_elements(vec![Box::new(StringBox::new("line")) as Box<dyn NyashBox>]);

    assert!(!array.uses_text_slots());
    assert_eq!(
        array.slot_update_text_resident_raw(0, |value| value.len()),
        None
    );
    assert!(!array.uses_text_slots());
    assert_eq!(array.get_index_i64(0).to_string_box().value, "line");
}

#[test]
fn slot_update_text_raw_mutates_mixed_boxed_string_slot() {
    let array = ArrayBox::new_with_elements(vec![
        Box::new(StringBox::new("line")) as Box<dyn NyashBox>,
        Box::new(IntegerBox::new(7)) as Box<dyn NyashBox>,
    ]);

    assert_eq!(
        array.slot_update_text_resident_first_raw(0, |value| {
            value.push_str("-seed");
            value.len()
        }),
        Some((9, false))
    );
    assert!(!array.uses_text_slots());
    assert_eq!(
        array.slot_with_text_raw(0, str::to_owned).as_deref(),
        Some("line-seed")
    );
    assert_eq!(array.slot_load_i64_raw(1), Some(7));
}

#[test]
fn slot_update_text_raw_misses_mixed_boxed_non_string_slot() {
    let array = ArrayBox::new_with_elements(vec![
        Box::new(StringBox::new("line")) as Box<dyn NyashBox>,
        Box::new(IntegerBox::new(7)) as Box<dyn NyashBox>,
    ]);

    assert_eq!(
        array.slot_update_text_resident_first_raw(1, |value| {
            value.push_str("-seed");
            value.len()
        }),
        None
    );
    assert!(!array.uses_text_slots());
    assert_eq!(array.slot_load_i64_raw(1), Some(7));
    assert_eq!(
        array.slot_update_text_resident_first_raw(-1, |value| value.len()),
        None
    );
}

#[test]
fn slot_insert_const_mid_lenhalf_raw_mutates_text_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_text_raw(0, "abcd".to_string()));

    assert_eq!(array.slot_insert_const_mid_lenhalf_raw(0, "XY"), Some(6));
    assert!(array.uses_text_slots());
    assert_eq!(
        array.slot_with_text_raw(0, str::to_owned).as_deref(),
        Some("abXYcd")
    );
}

#[test]
fn slot_insert_const_mid_lenhalf_raw_mutates_mixed_boxed_string_slot() {
    let array = ArrayBox::new_with_elements(vec![
        Box::new(StringBox::new("abcd")) as Box<dyn NyashBox>,
        Box::new(IntegerBox::new(7)) as Box<dyn NyashBox>,
    ]);

    assert_eq!(array.slot_insert_const_mid_lenhalf_raw(0, "XY"), Some(6));
    assert!(!array.uses_text_slots());
    assert_eq!(
        array.slot_with_text_raw(0, str::to_owned).as_deref(),
        Some("abXYcd")
    );
    assert_eq!(array.slot_load_i64_raw(1), Some(7));
}

#[test]
fn slot_insert_box_raw_preserves_inline_i64_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_i64_raw(0, 10));
    assert!(array.slot_store_i64_raw(1, 30));

    assert!(array.slot_insert_box_raw(1, Box::new(IntegerBox::new(20))));
    assert!(array.uses_inline_i64_slots());
    assert_eq!(array.slot_load_i64_raw(0), Some(10));
    assert_eq!(array.slot_load_i64_raw(1), Some(20));
    assert_eq!(array.slot_load_i64_raw(2), Some(30));
}

#[test]
fn slot_insert_box_raw_preserves_text_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_text_raw(0, "Alpha".to_string()));
    assert!(array.slot_store_text_raw(1, "Gamma".to_string()));

    assert!(array.slot_insert_box_raw(1, Box::new(StringBox::new("Beta"))));
    assert!(array.uses_text_slots());
    assert_eq!(
        array.slot_with_text_raw(0, str::to_owned).as_deref(),
        Some("Alpha")
    );
    assert_eq!(
        array.slot_with_text_raw(1, str::to_owned).as_deref(),
        Some("Beta")
    );
    assert_eq!(
        array.slot_with_text_raw(2, str::to_owned).as_deref(),
        Some("Gamma")
    );
}
