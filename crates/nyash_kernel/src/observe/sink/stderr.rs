use super::super::backend;
use super::super::contract;
use std::fmt::Write as _;

fn append_counter_value(line: &mut String, name: &str, value: u64) {
    let _ = write!(line, " {}={}", name, value);
}

fn append_counter_values(
    line: &mut String,
    values: impl IntoIterator<Item = contract::SnapshotCounterValue>,
) {
    for value in values {
        append_counter_value(line, value.name, value.value);
    }
}

fn append_snapshot_fields(
    line: &mut String,
    snapshot: &[u64],
    fields: impl IntoIterator<Item = contract::SnapshotCounterField>,
) {
    for field in fields {
        append_counter_value(line, field.name, field.read(snapshot));
    }
}

pub(crate) fn emit_summary_to_stderr() {
    let snapshot = backend::snapshot();
    let str_concat2_classified = contract::STR_CONCAT2_ROUTE_FAST_STR_OWNED_FIELD.read(&snapshot)
        + contract::STR_CONCAT2_ROUTE_FAST_STR_RETURN_HANDLE_FIELD.read(&snapshot)
        + contract::STR_CONCAT2_ROUTE_SPAN_FREEZE_FIELD.read(&snapshot)
        + contract::STR_CONCAT2_ROUTE_SPAN_RETURN_HANDLE_FIELD.read(&snapshot)
        + contract::STR_CONCAT2_ROUTE_MATERIALIZE_FALLBACK_FIELD.read(&snapshot);
    let str_concat2_unclassified = contract::STR_CONCAT2_ROUTE_TOTAL_FIELD
        .read(&snapshot)
        .saturating_sub(str_concat2_classified);
    let str_len_classified = contract::STR_LEN_ROUTE_FAST_STR_HIT_FIELD.read(&snapshot)
        + contract::STR_LEN_ROUTE_FALLBACK_HIT_FIELD.read(&snapshot)
        + contract::STR_LEN_ROUTE_MISS_FIELD.read(&snapshot);
    let str_len_unclassified = contract::STR_LEN_ROUTE_TOTAL_FIELD
        .read(&snapshot)
        .saturating_sub(str_len_classified);
    let str_substring_slow_plan_classified =
        contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_HANDLE_FIELD.read(&snapshot)
            + contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_EMPTY_FIELD.read(&snapshot)
            + contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_FREEZE_SPAN_FIELD.read(&snapshot)
            + contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_VIEW_SPAN_FIELD.read(&snapshot);
    let str_substring_slow_plan_unclassified = contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_FIELD
        .read(&snapshot)
        .saturating_sub(str_substring_slow_plan_classified);
    let piecewise_subrange_classified = contract::PIECEWISE_SUBRANGE_FALLBACK_INSERT_FIELD
        .read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_EMPTY_RETURN_FIELD.read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_PREFIX_ONLY_FIELD.read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_MIDDLE_ONLY_FIELD.read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_SUFFIX_ONLY_FIELD.read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_PREFIX_MIDDLE_FIELD.read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_MIDDLE_SUFFIX_FIELD.read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_PREFIX_SUFFIX_FIELD.read(&snapshot)
        + contract::PIECEWISE_SUBRANGE_ALL_THREE_FIELD.read(&snapshot);
    let piecewise_subrange_unclassified = contract::PIECEWISE_SUBRANGE_TOTAL_FIELD
        .read(&snapshot)
        .saturating_sub(piecewise_subrange_classified);
    let mut store_array_str_line = format!(
        "[perf/counter][{}] total={}",
        contract::STORE_ARRAY_STR,
        contract::store_array_str_total(&snapshot)
    );
    append_counter_values(
        &mut store_array_str_line,
        contract::store_array_str_detail_values(&snapshot),
    );
    eprintln!("{}", store_array_str_line);
    let mut const_suffix_line = format!(
        "[perf/counter][{}] total={}",
        contract::CONST_SUFFIX,
        contract::CONST_SUFFIX_TOTAL_FIELD.read(&snapshot)
    );
    append_snapshot_fields(
        &mut const_suffix_line,
        &snapshot,
        contract::CONST_SUFFIX_SUMMARY_FIELDS.into_iter().skip(1),
    );
    eprintln!("{}", const_suffix_line);
    let mut birth_placement_line = format!("[perf/counter][{}]", contract::BIRTH_PLACEMENT);
    append_snapshot_fields(
        &mut birth_placement_line,
        &snapshot,
        contract::BIRTH_PLACEMENT_SUMMARY_FIELDS,
    );
    eprintln!("{}", birth_placement_line);
    let mut birth_backend_line = format!("[perf/counter][{}]", contract::BIRTH_BACKEND);
    append_snapshot_fields(
        &mut birth_backend_line,
        &snapshot,
        contract::BIRTH_BACKEND_CORE_SUMMARY_FIELDS,
    );
    append_snapshot_fields(
        &mut birth_backend_line,
        &snapshot,
        contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_SUMMARY_FIELDS,
    );
    append_snapshot_fields(
        &mut birth_backend_line,
        &snapshot,
        contract::BIRTH_BACKEND_SITE_SUMMARY_FIELDS,
    );
    eprintln!("{}", birth_backend_line);
    let mut str_concat2_route_line = format!("[perf/counter][{}]", contract::STR_CONCAT2_ROUTE);
    append_snapshot_fields(
        &mut str_concat2_route_line,
        &snapshot,
        contract::STR_CONCAT2_ROUTE_SUMMARY_FIELDS,
    );
    append_counter_value(
        &mut str_concat2_route_line,
        contract::STR_CONCAT2_ROUTE_UNCLASSIFIED,
        str_concat2_unclassified,
    );
    eprintln!("{}", str_concat2_route_line);
    let mut str_len_route_line = format!("[perf/counter][{}]", contract::STR_LEN_ROUTE);
    append_snapshot_fields(
        &mut str_len_route_line,
        &snapshot,
        contract::STR_LEN_ROUTE_SUMMARY_FIELDS,
    );
    append_counter_value(
        &mut str_len_route_line,
        contract::STR_LEN_ROUTE_UNCLASSIFIED,
        str_len_unclassified,
    );
    eprintln!("{}", str_len_route_line);
    let mut str_substring_route_line = format!("[perf/counter][{}]", contract::STR_SUBSTRING_ROUTE);
    append_snapshot_fields(
        &mut str_substring_route_line,
        &snapshot,
        contract::STR_SUBSTRING_ROUTE_SUMMARY_FIELDS,
    );
    append_counter_value(
        &mut str_substring_route_line,
        contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_UNCLASSIFIED,
        str_substring_slow_plan_unclassified,
    );
    eprintln!("{}", str_substring_route_line);
    let mut piecewise_subrange_line = format!("[perf/counter][{}]", contract::PIECEWISE_SUBRANGE);
    append_snapshot_fields(
        &mut piecewise_subrange_line,
        &snapshot,
        contract::PIECEWISE_SUBRANGE_SUMMARY_FIELDS,
    );
    append_counter_value(
        &mut piecewise_subrange_line,
        contract::PIECEWISE_SUBRANGE_UNCLASSIFIED,
        piecewise_subrange_unclassified,
    );
    eprintln!("{}", piecewise_subrange_line);
    let stable_box_demand = nyash_rust::runtime::host_handles::perf_observe_snapshot();
    let stable_box_demand_values = stable_box_demand.ordered_values();
    let mut stable_box_demand_line = format!("[perf/counter][{}]", contract::STABLE_BOX_DEMAND);
    append_counter_values(
        &mut stable_box_demand_line,
        contract::stable_box_demand_values(&stable_box_demand_values),
    );
    eprintln!("{}", stable_box_demand_line);
    let mut borrowed_alias_line = format!("[perf/counter][{}]", contract::BORROWED_ALIAS);
    append_snapshot_fields(
        &mut borrowed_alias_line,
        &snapshot,
        contract::BORROWED_ALIAS_SUMMARY_FIELDS,
    );
    eprintln!("{}", borrowed_alias_line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_helpers_preserve_counter_format_order() {
        let mut snapshot = vec![0; contract::SNAPSHOT_COUNTER_LEN];
        snapshot[contract::CONST_SUFFIX_TOTAL_FIELD.snapshot_index] = 7;
        snapshot[contract::CONST_SUFFIX_CACHED_HANDLE_HIT_FIELD.snapshot_index] = 3;

        let mut line = String::from("[perf/counter][unit]");
        append_snapshot_fields(
            &mut line,
            &snapshot,
            [
                contract::CONST_SUFFIX_TOTAL_FIELD,
                contract::CONST_SUFFIX_CACHED_HANDLE_HIT_FIELD,
            ],
        );
        append_counter_value(&mut line, "tail", 11);

        assert_eq!(
            line,
            "[perf/counter][unit] const_suffix=7 cached_handle_hit=3 tail=11"
        );

        let demand_snapshot = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let mut demand_line = String::from("[perf/counter][stable_box_demand]");
        append_counter_values(
            &mut demand_line,
            contract::stable_box_demand_values(&demand_snapshot),
        );

        assert_eq!(
            demand_line,
            "[perf/counter][stable_box_demand] object_get_latest_fresh=1 object_with_handle_latest_fresh=2 object_pair_latest_fresh=3 object_triple_latest_fresh=4 text_read_handle_latest_fresh=5 text_read_pair_latest_fresh=6 text_read_triple_latest_fresh=7 object_with_handle_array_store_str_source_latest_fresh=8 object_with_handle_substring_plan_latest_fresh=9 object_with_handle_decode_array_fast_latest_fresh=10 object_with_handle_decode_any_arg_latest_fresh=11 object_with_handle_decode_any_index_latest_fresh=12"
        );
    }
}
