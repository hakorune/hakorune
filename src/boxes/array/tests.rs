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
