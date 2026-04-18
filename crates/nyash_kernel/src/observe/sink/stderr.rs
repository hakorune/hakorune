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
    eprintln!(
        "[perf/counter][{}] total={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::CONST_SUFFIX,
        contract::CONST_SUFFIX_TOTAL_FIELD.read(&snapshot),
        contract::CONST_SUFFIX_CACHED_HANDLE_HIT_FIELD.name,
        contract::CONST_SUFFIX_CACHED_HANDLE_HIT_FIELD.read(&snapshot),
        contract::CONST_SUFFIX_TEXT_CACHE_RELOAD_FIELD.name,
        contract::CONST_SUFFIX_TEXT_CACHE_RELOAD_FIELD.read(&snapshot),
        contract::CONST_SUFFIX_FREEZE_FALLBACK_FIELD.name,
        contract::CONST_SUFFIX_FREEZE_FALLBACK_FIELD.read(&snapshot),
        contract::CONST_SUFFIX_EMPTY_RETURN_FIELD.name,
        contract::CONST_SUFFIX_EMPTY_RETURN_FIELD.read(&snapshot),
        contract::CONST_SUFFIX_CACHED_FAST_STR_HIT_FIELD.name,
        contract::CONST_SUFFIX_CACHED_FAST_STR_HIT_FIELD.read(&snapshot),
        contract::CONST_SUFFIX_CACHED_SPAN_HIT_FIELD.name,
        contract::CONST_SUFFIX_CACHED_SPAN_HIT_FIELD.read(&snapshot),
    );
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::BIRTH_PLACEMENT,
        contract::BIRTH_PLACEMENT_RETURN_HANDLE_FIELD.name,
        contract::BIRTH_PLACEMENT_RETURN_HANDLE_FIELD.read(&snapshot),
        contract::BIRTH_PLACEMENT_BORROW_VIEW_FIELD.name,
        contract::BIRTH_PLACEMENT_BORROW_VIEW_FIELD.read(&snapshot),
        contract::BIRTH_PLACEMENT_FREEZE_OWNED_FIELD.name,
        contract::BIRTH_PLACEMENT_FREEZE_OWNED_FIELD.read(&snapshot),
        contract::BIRTH_PLACEMENT_FRESH_HANDLE_FIELD.name,
        contract::BIRTH_PLACEMENT_FRESH_HANDLE_FIELD.read(&snapshot),
        contract::BIRTH_PLACEMENT_MATERIALIZE_OWNED_FIELD.name,
        contract::BIRTH_PLACEMENT_MATERIALIZE_OWNED_FIELD.read(&snapshot),
        contract::BIRTH_PLACEMENT_STORE_FROM_SOURCE_FIELD.name,
        contract::BIRTH_PLACEMENT_STORE_FROM_SOURCE_FIELD.read(&snapshot),
    );
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
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::STR_CONCAT2_ROUTE,
        contract::STR_CONCAT2_ROUTE_TOTAL_FIELD.name,
        contract::STR_CONCAT2_ROUTE_TOTAL_FIELD.read(&snapshot),
        contract::STR_CONCAT2_ROUTE_DISPATCH_HIT_FIELD.name,
        contract::STR_CONCAT2_ROUTE_DISPATCH_HIT_FIELD.read(&snapshot),
        contract::STR_CONCAT2_ROUTE_FAST_STR_OWNED_FIELD.name,
        contract::STR_CONCAT2_ROUTE_FAST_STR_OWNED_FIELD.read(&snapshot),
        contract::STR_CONCAT2_ROUTE_FAST_STR_RETURN_HANDLE_FIELD.name,
        contract::STR_CONCAT2_ROUTE_FAST_STR_RETURN_HANDLE_FIELD.read(&snapshot),
        contract::STR_CONCAT2_ROUTE_SPAN_FREEZE_FIELD.name,
        contract::STR_CONCAT2_ROUTE_SPAN_FREEZE_FIELD.read(&snapshot),
        contract::STR_CONCAT2_ROUTE_SPAN_RETURN_HANDLE_FIELD.name,
        contract::STR_CONCAT2_ROUTE_SPAN_RETURN_HANDLE_FIELD.read(&snapshot),
        contract::STR_CONCAT2_ROUTE_MATERIALIZE_FALLBACK_FIELD.name,
        contract::STR_CONCAT2_ROUTE_MATERIALIZE_FALLBACK_FIELD.read(&snapshot),
        contract::STR_CONCAT2_ROUTE_UNCLASSIFIED,
        str_concat2_unclassified,
    );
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::STR_LEN_ROUTE,
        contract::STR_LEN_ROUTE_TOTAL_FIELD.name,
        contract::STR_LEN_ROUTE_TOTAL_FIELD.read(&snapshot),
        contract::STR_LEN_ROUTE_DISPATCH_HIT_FIELD.name,
        contract::STR_LEN_ROUTE_DISPATCH_HIT_FIELD.read(&snapshot),
        contract::STR_LEN_ROUTE_FAST_STR_HIT_FIELD.name,
        contract::STR_LEN_ROUTE_FAST_STR_HIT_FIELD.read(&snapshot),
        contract::STR_LEN_ROUTE_FALLBACK_HIT_FIELD.name,
        contract::STR_LEN_ROUTE_FALLBACK_HIT_FIELD.read(&snapshot),
        contract::STR_LEN_ROUTE_MISS_FIELD.name,
        contract::STR_LEN_ROUTE_MISS_FIELD.read(&snapshot),
        contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FAST_STR_HIT_FIELD.name,
        contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FAST_STR_HIT_FIELD.read(&snapshot),
        contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FALLBACK_HIT_FIELD.name,
        contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FALLBACK_HIT_FIELD.read(&snapshot),
        contract::STR_LEN_ROUTE_UNCLASSIFIED,
        str_len_unclassified,
    );
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
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::PIECEWISE_SUBRANGE,
        contract::PIECEWISE_SUBRANGE_TOTAL_FIELD.name,
        contract::PIECEWISE_SUBRANGE_TOTAL_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_SINGLE_SESSION_HIT_FIELD.name,
        contract::PIECEWISE_SUBRANGE_SINGLE_SESSION_HIT_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_FALLBACK_INSERT_FIELD.name,
        contract::PIECEWISE_SUBRANGE_FALLBACK_INSERT_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_EMPTY_RETURN_FIELD.name,
        contract::PIECEWISE_SUBRANGE_EMPTY_RETURN_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_PREFIX_ONLY_FIELD.name,
        contract::PIECEWISE_SUBRANGE_PREFIX_ONLY_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_MIDDLE_ONLY_FIELD.name,
        contract::PIECEWISE_SUBRANGE_MIDDLE_ONLY_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_SUFFIX_ONLY_FIELD.name,
        contract::PIECEWISE_SUBRANGE_SUFFIX_ONLY_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_PREFIX_MIDDLE_FIELD.name,
        contract::PIECEWISE_SUBRANGE_PREFIX_MIDDLE_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_MIDDLE_SUFFIX_FIELD.name,
        contract::PIECEWISE_SUBRANGE_MIDDLE_SUFFIX_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_PREFIX_SUFFIX_FIELD.name,
        contract::PIECEWISE_SUBRANGE_PREFIX_SUFFIX_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_ALL_THREE_FIELD.name,
        contract::PIECEWISE_SUBRANGE_ALL_THREE_FIELD.read(&snapshot),
        contract::PIECEWISE_SUBRANGE_UNCLASSIFIED,
        piecewise_subrange_unclassified,
    );
    let stable_box_demand = nyash_rust::runtime::host_handles::perf_observe_snapshot();
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::STABLE_BOX_DEMAND,
        contract::STABLE_BOX_DEMAND_OBJECT_GET_LATEST_FRESH,
        stable_box_demand[0],
        contract::STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_LATEST_FRESH,
        stable_box_demand[1],
        contract::STABLE_BOX_DEMAND_OBJECT_PAIR_LATEST_FRESH,
        stable_box_demand[2],
        contract::STABLE_BOX_DEMAND_OBJECT_TRIPLE_LATEST_FRESH,
        stable_box_demand[3],
        contract::STABLE_BOX_DEMAND_TEXT_READ_HANDLE_LATEST_FRESH,
        stable_box_demand[4],
        contract::STABLE_BOX_DEMAND_TEXT_READ_PAIR_LATEST_FRESH,
        stable_box_demand[5],
        contract::STABLE_BOX_DEMAND_TEXT_READ_TRIPLE_LATEST_FRESH,
        stable_box_demand[6],
        contract::STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_ARRAY_STORE_STR_SOURCE_LATEST_FRESH,
        stable_box_demand[7],
        contract::STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_SUBSTRING_PLAN_LATEST_FRESH,
        stable_box_demand[8],
        contract::STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_DECODE_ARRAY_FAST_LATEST_FRESH,
        stable_box_demand[9],
        contract::STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_DECODE_ANY_ARG_LATEST_FRESH,
        stable_box_demand[10],
        contract::STABLE_BOX_DEMAND_OBJECT_WITH_HANDLE_DECODE_ANY_INDEX_LATEST_FRESH,
        stable_box_demand[11],
    );
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
    }
}
