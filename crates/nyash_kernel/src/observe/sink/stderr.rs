use super::super::backend;
use super::super::contract;
use std::fmt::Write as _;

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
    let mut store_array_str_fields = contract::STORE_ARRAY_STR_SUMMARY_FIELDS.into_iter();
    let store_array_str_total = store_array_str_fields
        .next()
        .expect("store.array.str total field");
    let mut store_array_str_line = format!(
        "[perf/counter][{}] total={}",
        contract::STORE_ARRAY_STR,
        store_array_str_total.read(&snapshot)
    );
    for field in store_array_str_fields {
        let _ = write!(
            &mut store_array_str_line,
            " {}={}",
            field.name,
            field.read(&snapshot)
        );
    }
    eprintln!("{}", store_array_str_line);
    eprintln!(
        "[perf/counter][{}] total={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::CONST_SUFFIX,
        snapshot[14],
        contract::CONST_SUFFIX_CACHED_HANDLE_HIT,
        snapshot[15],
        contract::CONST_SUFFIX_TEXT_CACHE_RELOAD,
        snapshot[16],
        contract::CONST_SUFFIX_FREEZE_FALLBACK,
        snapshot[17],
        contract::CONST_SUFFIX_EMPTY_RETURN,
        snapshot[18],
        contract::CONST_SUFFIX_CACHED_FAST_STR_HIT,
        snapshot[19],
        contract::CONST_SUFFIX_CACHED_SPAN_HIT,
        snapshot[20],
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
    for (name, value) in [
        (contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_TOTAL, snapshot[27]),
        (contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_VIEW1, snapshot[28]),
        (
            contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES2,
            snapshot[29],
        ),
        (
            contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES3,
            snapshot[30],
        ),
        (
            contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_PIECES4,
            snapshot[31],
        ),
        (
            contract::BIRTH_BACKEND_FREEZE_TEXT_PLAN_OWNED_TMP,
            snapshot[32],
        ),
        (contract::BIRTH_BACKEND_STRING_BOX_NEW_TOTAL, snapshot[33]),
        (contract::BIRTH_BACKEND_STRING_BOX_NEW_BYTES, snapshot[34]),
        (contract::BIRTH_BACKEND_STRING_BOX_CTOR_TOTAL, snapshot[35]),
        (contract::BIRTH_BACKEND_STRING_BOX_CTOR_BYTES, snapshot[36]),
        (contract::BIRTH_BACKEND_ARC_WRAP_TOTAL, snapshot[37]),
        (
            contract::BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_TOTAL,
            snapshot[38],
        ),
        (
            contract::BIRTH_BACKEND_OBJECTIZE_STABLE_BOX_NOW_BYTES,
            snapshot[39],
        ),
        (contract::BIRTH_BACKEND_HANDLE_ISSUE_TOTAL, snapshot[40]),
        (
            contract::BIRTH_BACKEND_ISSUE_FRESH_HANDLE_TOTAL,
            snapshot[41],
        ),
        (
            contract::BIRTH_BACKEND_MATERIALIZE_OWNED_TOTAL,
            snapshot[42],
        ),
        (
            contract::BIRTH_BACKEND_MATERIALIZE_OWNED_BYTES,
            snapshot[43],
        ),
        (contract::BIRTH_BACKEND_GC_ALLOC_CALLED, snapshot[44]),
        (contract::BIRTH_BACKEND_GC_ALLOC_BYTES, snapshot[45]),
        (contract::BIRTH_BACKEND_GC_ALLOC_SKIPPED, snapshot[46]),
        (
            contract::BIRTH_BACKEND_CARRIER_KIND_STABLE_BOX,
            snapshot[113],
        ),
        (
            contract::BIRTH_BACKEND_CARRIER_KIND_SOURCE_KEEP,
            snapshot[114],
        ),
        (
            contract::BIRTH_BACKEND_CARRIER_KIND_OWNED_BYTES,
            snapshot[115],
        ),
        (contract::BIRTH_BACKEND_CARRIER_KIND_HANDLE, snapshot[116]),
        (
            contract::BIRTH_BACKEND_PUBLISH_REASON_EXTERNAL_BOUNDARY,
            snapshot[117],
        ),
        (
            contract::BIRTH_BACKEND_PUBLISH_REASON_NEED_STABLE_OBJECT,
            snapshot[118],
        ),
        (
            contract::BIRTH_BACKEND_PUBLISH_REASON_GENERIC_FALLBACK,
            snapshot[119],
        ),
        (
            contract::BIRTH_BACKEND_PUBLISH_REASON_EXPLICIT_API,
            snapshot[120],
        ),
    ] {
        let _ = write!(&mut birth_backend_line, " {}={}", name, value);
    }
    for field in contract::BIRTH_BACKEND_PUBLISH_BOUNDARY_SLOT_SUMMARY_FIELDS {
        let _ = write!(
            &mut birth_backend_line,
            " {}={}",
            field.name,
            field.read(&snapshot)
        );
    }
    for field in contract::BIRTH_BACKEND_SITE_SUMMARY_FIELDS {
        let _ = write!(
            &mut birth_backend_line,
            " {}={}",
            field.name,
            field.read(&snapshot)
        );
    }
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
    for field in contract::STR_SUBSTRING_ROUTE_SUMMARY_FIELDS {
        let _ = write!(
            &mut str_substring_route_line,
            " {}={}",
            field.name,
            field.read(&snapshot)
        );
    }
    let _ = write!(
        &mut str_substring_route_line,
        " {}={}",
        contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_UNCLASSIFIED,
        str_substring_slow_plan_unclassified
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
    for field in contract::BORROWED_ALIAS_SUMMARY_FIELDS {
        let _ = write!(
            &mut borrowed_alias_line,
            " {}={}",
            field.name,
            field.read(&snapshot)
        );
    }
    eprintln!("{}", borrowed_alias_line);
}
