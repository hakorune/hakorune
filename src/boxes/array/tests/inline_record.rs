use super::*;

#[test]
fn inline_record_probe_builds_explicit_probe_array() {
    let array = ArrayInlineRecordProbe::build(
        11,
        vec![
            ArrayInlineRecordColumn::i64(vec![1, 2, 3]),
            ArrayInlineRecordColumn::bool_values(vec![true, false, true]),
        ],
    )
    .expect("probe columns must have equal row counts");

    assert!(array.uses_inline_record_slots());
    assert_eq!(array.inline_record_layout_id(), Some(11));
    assert_eq!(array.len(), 3);
    assert_eq!(
        array.get_index_i64(0).to_string_box().value,
        "[array/inline-record/unmaterialized] record value materialization is not enabled"
    );
}

#[test]
fn inline_record_probe_rejects_ragged_columns() {
    let array = ArrayInlineRecordProbe::build(
        12,
        vec![
            ArrayInlineRecordColumn::i64(vec![1, 2]),
            ArrayInlineRecordColumn::bool_values(vec![true]),
        ],
    );

    assert!(array.is_none());
}

#[test]
fn inline_record_plan_probe_builds_integer_lane_array() {
    let plan = inline_record_probe_plan(
        23,
        vec![TypedObjectFieldStorage::I64, TypedObjectFieldStorage::USize],
    );
    let array =
        ArrayInlineRecordPlanProbe::build_integer_lane_array(&plan, vec![vec![10, 20], vec![3, 4]])
            .expect("integer-lane storage plan must build a probe array");

    assert!(array.uses_inline_record_slots());
    assert_eq!(array.inline_record_layout_id(), Some(23));
    assert_eq!(array.len(), 2);
    assert_eq!(
        array.get_index_i64(0).to_string_box().value,
        "[array/inline-record/unmaterialized] record value materialization is not enabled"
    );
}

#[test]
fn inline_record_plan_probe_rejects_handle_columns() {
    let plan = inline_record_probe_plan(24, vec![TypedObjectFieldStorage::Handle]);
    let array = ArrayInlineRecordPlanProbe::build_integer_lane_array(&plan, vec![vec![1, 2]]);

    assert!(array.is_none());
}

#[test]
fn inline_record_autouse_pilot_reads_i64_columns_without_materializing() {
    let array = ArrayBox::new_with_inline_record_i64_columns_for_compiler_autouse(
        31,
        vec![vec![10, 20], vec![100, 200]],
    )
    .expect("equal-height integer columns must build C209 pilot storage");

    assert!(array.uses_inline_record_slots());
    assert_eq!(array.inline_record_layout_id(), Some(31));
    assert_eq!(array.inline_record_load_i64_column_raw(31, 0, 0), Some(10));
    assert_eq!(array.inline_record_load_i64_column_raw(31, 1, 0), Some(20));
    assert_eq!(array.inline_record_load_i64_column_raw(31, 0, 1), Some(100));
    assert_eq!(array.inline_record_load_i64_column_raw(30, 0, 0), None);
    assert_eq!(array.inline_record_load_i64_column_raw(31, 2, 0), None);
    assert_eq!(array.inline_record_load_i64_column_raw(31, 0, 2), None);
    assert_eq!(array.slot_load_i64_raw(0), None);
    assert_eq!(
        array.get_index_i64(0).to_string_box().value,
        "[array/inline-record/unmaterialized] record value materialization is not enabled"
    );
}

#[test]
fn inline_record_autouse_pilot_rejects_ragged_i64_columns() {
    let array = ArrayBox::new_with_inline_record_i64_columns_for_compiler_autouse(
        32,
        vec![vec![10, 20], vec![100]],
    );

    assert!(array.is_none());
}

#[test]
fn aligned_small_metadata_packed_store_pilot_reads_metadata_columns() {
    let layout_id = 41;
    let ptr_column = 0;
    let alignment_column = 1;
    let padded_size_column = 2;
    let array = ArrayBox::new_with_inline_record_i64_columns_for_compiler_autouse(
        layout_id,
        vec![vec![1001, 1002], vec![8, 16], vec![64, 128]],
    )
    .expect("aligned-small metadata columns must build packed pilot storage");

    assert!(array.uses_inline_record_slots());
    assert_eq!(array.inline_record_layout_id(), Some(layout_id));
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 0, ptr_column),
        Some(1001)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 1, ptr_column),
        Some(1002)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 0, alignment_column),
        Some(8)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 1, alignment_column),
        Some(16)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 0, padded_size_column),
        Some(64)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 1, padded_size_column),
        Some(128)
    );
    assert_eq!(
        array.get_index_i64(0).to_string_box().value,
        "[array/inline-record/unmaterialized] record value materialization is not enabled"
    );
}

#[test]
fn huge_page_metadata_packed_store_pilot_reads_metadata_columns() {
    let layout_id = 42;
    let page_id_column = 0;
    let ptr_column = 1;
    let requested_size_column = 2;
    let committed_size_column = 3;
    let live_column = 4;
    let array = ArrayBox::new_with_inline_record_i64_columns_for_compiler_autouse(
        layout_id,
        vec![
            vec![70, 71],
            vec![8001, 8002],
            vec![4096, 8192],
            vec![4096, 8192],
            vec![1, 0],
        ],
    )
    .expect("huge-page metadata columns must build packed pilot storage");

    assert!(array.uses_inline_record_slots());
    assert_eq!(array.inline_record_layout_id(), Some(layout_id));
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 0, page_id_column),
        Some(70)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 1, page_id_column),
        Some(71)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 0, ptr_column),
        Some(8001)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 0, requested_size_column),
        Some(4096)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 1, requested_size_column),
        Some(8192)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 0, committed_size_column),
        Some(4096)
    );
    assert_eq!(
        array.inline_record_load_i64_column_raw(layout_id, 1, live_column),
        Some(0)
    );
    assert_eq!(
        array.get_index_i64(0).to_string_box().value,
        "[array/inline-record/unmaterialized] record value materialization is not enabled"
    );
}

#[test]
fn inline_record_storage_reports_len_capacity_and_debug_kind() {
    let array = inline_record_test_array();

    assert!(array.uses_inline_record_slots());
    assert_eq!(array.inline_record_layout_id(), Some(7));
    assert_eq!(array.len(), 2);
    assert_eq!(array.slot_load_i64_raw(0), None);
    assert!(array.capacity() >= 2);
    assert!(format!("{array:?}").contains("inline_record"));
}

#[test]
fn inline_record_storage_keeps_visible_materialization_boundary() {
    let array = inline_record_test_array();

    assert_eq!(
        array.get_index_i64(0).to_string_box().value,
        "[array/inline-record/unmaterialized] record value materialization is not enabled"
    );
    assert_eq!(array.slot_append_box_raw(Box::new(IntegerBox::new(30))), -1);
    assert!(!array.slot_store_i64_raw(0, 99));
    assert_eq!(array.len(), 2);
    assert!(array.uses_inline_record_slots());
}

#[test]
fn inline_record_storage_clone_clear_and_slice_preserve_internal_shape() {
    let array = inline_record_test_array();
    let cloned = array.clone();

    assert!(cloned.equals(&array).value);
    let sliced = array.slice(Box::new(IntegerBox::new(1)), Box::new(IntegerBox::new(2)));
    let sliced = sliced.as_any().downcast_ref::<ArrayBox>().unwrap();
    assert!(sliced.uses_inline_record_slots());
    assert_eq!(sliced.inline_record_layout_id(), Some(7));
    assert_eq!(sliced.len(), 1);

    array.clear();
    assert!(array.uses_inline_record_slots());
    assert_eq!(array.len(), 0);
}
