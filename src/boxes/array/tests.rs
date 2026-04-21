use super::*;

#[test]
fn slot_store_i64_births_inline_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_i64_raw(0, 10));
    assert!(array.uses_inline_i64_slots());
    assert_eq!(array.slot_load_i64_raw(0), Some(10));
    assert_eq!(array.get_index_i64(0).to_string_box().value, "10");
}

#[test]
fn slot_store_box_promotes_inline_lane_to_boxed() {
    let array = ArrayBox::new();
    assert!(array.slot_store_i64_raw(0, 10));
    assert!(array.uses_inline_i64_slots());

    assert!(array.slot_store_box_raw(0, Box::new(StringBox::new("hello"))));
    assert!(!array.uses_inline_i64_slots());
    assert_eq!(array.get_index_i64(0).to_string_box().value, "hello");
    assert_eq!(array.slot_rmw_add1_i64_raw(0), None);
}

#[test]
fn slot_store_bool_births_inline_bool_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_bool_raw(0, true));
    assert!(array.uses_inline_bool_slots());
    assert_eq!(array.slot_load_i64_raw(0), Some(1));
    assert_eq!(array.get_index_i64(0).to_string_box().value, "true");
}

#[test]
fn slot_store_box_preserves_inline_bool_lane_for_bool_values() {
    let array = ArrayBox::new();
    assert!(array.slot_store_bool_raw(0, true));
    assert!(array.uses_inline_bool_slots());

    assert!(array.slot_store_box_raw(0, Box::new(BoolBox::new(false))));
    assert!(array.uses_inline_bool_slots());
    assert_eq!(array.get_index_i64(0).to_string_box().value, "false");
}

#[test]
fn slot_store_f64_births_inline_f64_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_f64_raw(0, 1.25));
    assert!(array.uses_inline_f64_slots());
    assert_eq!(array.slot_load_i64_raw(0), None);
    assert_eq!(array.get_index_i64(0).to_string_box().value, "1.25");
}

#[test]
fn slot_store_box_preserves_inline_f64_lane_for_float_values() {
    let array = ArrayBox::new();
    assert!(array.slot_store_f64_raw(0, 1.25));
    assert!(array.uses_inline_f64_slots());

    assert!(array.slot_store_box_raw(0, Box::new(FloatBox::new(2.5))));
    assert!(array.uses_inline_f64_slots());
    assert_eq!(array.get_index_i64(0).to_string_box().value, "2.5");
}

#[test]
fn slot_store_text_births_text_lane() {
    let array = ArrayBox::new();
    assert!(array.slot_store_text_raw(0, "hello".to_string()));
    assert!(array.uses_text_slots());
    assert_eq!(
        array.slot_with_text_raw(0, str::to_owned).as_deref(),
        Some("hello")
    );
    assert_eq!(array.slot_text_len_raw(0), Some(5));
    assert_eq!(array.get_index_i64(0).to_string_box().value, "hello");
}

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
fn generic_box_store_degrades_text_lane_to_boxed_for_mixed_value() {
    let array = ArrayBox::new();
    assert!(array.slot_store_text_raw(0, "hello".to_string()));
    assert!(array.uses_text_slots());

    assert!(array.slot_store_box_raw(1, Box::new(IntegerBox::new(7))));
    assert!(!array.uses_text_slots());
    assert_eq!(array.get_index_i64(0).to_string_box().value, "hello");
    assert_eq!(array.slot_load_i64_raw(1), Some(7));
}
