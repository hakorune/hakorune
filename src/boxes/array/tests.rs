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

#[test]
fn invoke_surface_routes_insert_remove_clear_contains_indexof_join_and_length_alias() {
    let array = ArrayBox::new();
    assert!(matches!(
        array
            .invoke_surface(
                ArrayMethodId::Push,
                vec![Box::new(IntegerBox::new(10)) as Box<dyn NyashBox>],
            )
            .unwrap(),
        ArraySurfaceInvokeResult::Void
    ));

    assert!(matches!(
        array
            .invoke_surface(
                ArrayMethodId::Set,
                vec![
                    Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>,
                    Box::new(IntegerBox::new(11)) as Box<dyn NyashBox>,
                ],
            )
            .unwrap(),
        ArraySurfaceInvokeResult::Void
    ));

    let get = array
        .invoke_surface(
            ArrayMethodId::Get,
            vec![Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>],
        )
        .unwrap();
    match get {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "11");
        }
        ArraySurfaceInvokeResult::Void => panic!("get must return a value"),
    }

    let insert_result = array
        .invoke_surface(
            ArrayMethodId::Insert,
            vec![
                Box::new(IntegerBox::new(1)) as Box<dyn NyashBox>,
                Box::new(StringBox::new("Alpha")) as Box<dyn NyashBox>,
            ],
        )
        .unwrap();
    assert!(matches!(insert_result, ArraySurfaceInvokeResult::Void));

    let length = array
        .invoke_surface(ArrayMethodId::from_name("size").unwrap(), vec![])
        .unwrap();
    match length {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "2");
        }
        ArraySurfaceInvokeResult::Void => panic!("length must return a value"),
    }

    let contains = array
        .invoke_surface(
            ArrayMethodId::Contains,
            vec![Box::new(StringBox::new("Alpha")) as Box<dyn NyashBox>],
        )
        .unwrap();
    match contains {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "true");
        }
        ArraySurfaceInvokeResult::Void => panic!("contains must return a value"),
    }

    let index = array
        .invoke_surface(
            ArrayMethodId::IndexOf,
            vec![Box::new(StringBox::new("Alpha")) as Box<dyn NyashBox>],
        )
        .unwrap();
    match index {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "1");
        }
        ArraySurfaceInvokeResult::Void => panic!("indexOf must return a value"),
    }

    let joined = array
        .invoke_surface(
            ArrayMethodId::Join,
            vec![Box::new(StringBox::new("|")) as Box<dyn NyashBox>],
        )
        .unwrap();
    match joined {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "11|Alpha");
        }
        ArraySurfaceInvokeResult::Void => panic!("join must return a value"),
    }

    let slice = array
        .invoke_surface(
            ArrayMethodId::Slice,
            vec![
                Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>,
                Box::new(IntegerBox::new(1)) as Box<dyn NyashBox>,
            ],
        )
        .unwrap();
    match slice {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "[11]");
        }
        ArraySurfaceInvokeResult::Void => panic!("slice must return a value"),
    }

    let removed = array
        .invoke_surface(
            ArrayMethodId::Remove,
            vec![Box::new(IntegerBox::new(1)) as Box<dyn NyashBox>],
        )
        .unwrap();
    match removed {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "Alpha");
        }
        ArraySurfaceInvokeResult::Void => panic!("remove must return a value"),
    }

    let popped = array.invoke_surface(ArrayMethodId::Pop, vec![]).unwrap();
    match popped {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "11");
        }
        ArraySurfaceInvokeResult::Void => panic!("pop must return a value"),
    }

    let cleared = array.invoke_surface(ArrayMethodId::Clear, vec![]).unwrap();
    assert!(matches!(cleared, ArraySurfaceInvokeResult::Void));

    let length_after_clear = array.invoke_surface(ArrayMethodId::Length, vec![]).unwrap();
    match length_after_clear {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "0");
        }
        ArraySurfaceInvokeResult::Void => panic!("length after clear must return a value"),
    }
}

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

#[test]
fn array_visible_read_shares_self_identity_without_clone_recursion() {
    let array = ArrayBox::new();
    array.push(array.share_box());

    let got = array.get_index_i64(0);

    assert_eq!(got.type_name(), "ArrayBox");
}

#[test]
fn array_clone_shares_self_identity_without_clone_recursion() {
    let array = ArrayBox::new();
    array.push(array.share_box());

    let cloned = array.clone_box();

    assert_eq!(cloned.type_name(), "ArrayBox");
}

#[test]
fn map_clone_shares_nested_collection_identity_without_clone_recursion() {
    let array = ArrayBox::new();
    let map = crate::boxes::MapBox::new();
    array.push(map.share_box());
    map.set(Box::new(StringBox::new("array")), array.share_box());

    let cloned = map.clone_box();

    assert_eq!(cloned.type_name(), "MapBox");
}
