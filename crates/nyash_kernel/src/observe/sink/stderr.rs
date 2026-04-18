use super::super::backend;
use super::super::contract;
use std::fmt::Write as _;

pub(crate) fn emit_summary_to_stderr() {
    let snapshot = backend::snapshot();
    let str_concat2_classified =
        snapshot[49] + snapshot[50] + snapshot[51] + snapshot[52] + snapshot[53];
    let str_concat2_unclassified = snapshot[47].saturating_sub(str_concat2_classified);
    let str_len_classified = snapshot[56] + snapshot[57] + snapshot[58];
    let str_len_unclassified = snapshot[54].saturating_sub(str_len_classified);
    let str_substring_slow_plan_classified =
        snapshot[68] + snapshot[69] + snapshot[70] + snapshot[71];
    let str_substring_slow_plan_unclassified =
        snapshot[67].saturating_sub(str_substring_slow_plan_classified);
    let piecewise_subrange_classified = snapshot[104]
        + snapshot[105]
        + snapshot[106]
        + snapshot[107]
        + snapshot[108]
        + snapshot[109]
        + snapshot[110]
        + snapshot[111]
        + snapshot[112];
    let piecewise_subrange_unclassified =
        snapshot[102].saturating_sub(piecewise_subrange_classified);
    let mut store_array_str_line = format!(
        "[perf/counter][{}] total={}",
        contract::STORE_ARRAY_STR,
        snapshot[0]
    );
    for (name, value) in [
        (contract::STORE_ARRAY_STR_CACHE_HIT, snapshot[1]),
        (contract::STORE_ARRAY_STR_CACHE_MISS_HANDLE, snapshot[2]),
        (contract::STORE_ARRAY_STR_CACHE_MISS_EPOCH, snapshot[3]),
        (contract::STORE_ARRAY_STR_RETARGET_HIT, snapshot[4]),
        (
            contract::STORE_ARRAY_STR_LATEST_FRESH_RETARGET_HIT,
            snapshot[5],
        ),
        (contract::STORE_ARRAY_STR_SOURCE_STORE, snapshot[6]),
        (
            contract::STORE_ARRAY_STR_LATEST_FRESH_SOURCE_STORE,
            snapshot[7],
        ),
        (contract::STORE_ARRAY_STR_NON_STRING_SOURCE, snapshot[8]),
        (contract::STORE_ARRAY_STR_EXISTING_SLOT, snapshot[9]),
        (contract::STORE_ARRAY_STR_APPEND_SLOT, snapshot[10]),
        (contract::STORE_ARRAY_STR_SOURCE_STRING_BOX, snapshot[11]),
        (contract::STORE_ARRAY_STR_SOURCE_STRING_VIEW, snapshot[12]),
        (contract::STORE_ARRAY_STR_SOURCE_MISSING, snapshot[13]),
        (
            contract::STORE_ARRAY_STR_PLAN_SOURCE_KIND_STRING_LIKE,
            snapshot[89],
        ),
        (
            contract::STORE_ARRAY_STR_PLAN_SOURCE_KIND_OTHER_OBJECT,
            snapshot[90],
        ),
        (
            contract::STORE_ARRAY_STR_PLAN_SOURCE_KIND_MISSING,
            snapshot[91],
        ),
        (
            contract::STORE_ARRAY_STR_PLAN_SLOT_KIND_BORROWED_ALIAS,
            snapshot[92],
        ),
        (contract::STORE_ARRAY_STR_PLAN_SLOT_KIND_OTHER, snapshot[93]),
        (
            contract::STORE_ARRAY_STR_PLAN_ACTION_RETARGET_ALIAS,
            snapshot[94],
        ),
        (
            contract::STORE_ARRAY_STR_PLAN_ACTION_STORE_FROM_SOURCE,
            snapshot[95],
        ),
        (
            contract::STORE_ARRAY_STR_PLAN_ACTION_NEED_STABLE_OBJECT,
            snapshot[96],
        ),
        (
            contract::STORE_ARRAY_STR_REASON_SOURCE_KIND_VIA_OBJECT,
            snapshot[97],
        ),
        (
            contract::STORE_ARRAY_STR_REASON_RETARGET_KEEP_SOURCE_ARC,
            snapshot[98],
        ),
        (
            contract::STORE_ARRAY_STR_REASON_RETARGET_KEEP_SOURCE_ARC_PTR_EQ_HIT,
            snapshot[99],
        ),
        (
            contract::STORE_ARRAY_STR_REASON_RETARGET_KEEP_SOURCE_ARC_PTR_EQ_MISS,
            snapshot[100],
        ),
        (
            contract::STORE_ARRAY_STR_REASON_RETARGET_ALIAS_UPDATE,
            snapshot[101],
        ),
        (
            contract::STORE_ARRAY_STR_LOOKUP_REGISTRY_SLOT_READ,
            snapshot[121],
        ),
        (
            contract::STORE_ARRAY_STR_LOOKUP_CALLER_LATEST_FRESH_TAG,
            snapshot[122],
        ),
    ] {
        let _ = write!(&mut store_array_str_line, " {}={}", name, value);
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
        contract::BIRTH_PLACEMENT_RETURN_HANDLE,
        snapshot[21],
        contract::BIRTH_PLACEMENT_BORROW_VIEW,
        snapshot[22],
        contract::BIRTH_PLACEMENT_FREEZE_OWNED,
        snapshot[23],
        contract::BIRTH_PLACEMENT_FRESH_HANDLE,
        snapshot[24],
        contract::BIRTH_PLACEMENT_MATERIALIZE_OWNED,
        snapshot[25],
        contract::BIRTH_PLACEMENT_STORE_FROM_SOURCE,
        snapshot[26],
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
        (
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_TOTAL,
            snapshot[123],
        ),
        (
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_MATERIALIZE_OWNED_BYTES,
            snapshot[124],
        ),
        (
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_OBJECTIZE_BOX_TOTAL,
            snapshot[125],
        ),
        (
            contract::BIRTH_BACKEND_SITE_STRING_CONCAT_HH_PUBLISH_HANDLE_TOTAL,
            snapshot[126],
        ),
        (
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_TOTAL,
            snapshot[127],
        ),
        (
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_MATERIALIZE_OWNED_BYTES,
            snapshot[128],
        ),
        (
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_OBJECTIZE_BOX_TOTAL,
            snapshot[129],
        ),
        (
            contract::BIRTH_BACKEND_SITE_STRING_SUBSTRING_CONCAT_HHII_PUBLISH_HANDLE_TOTAL,
            snapshot[130],
        ),
        (
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_TOTAL,
            snapshot[131],
        ),
        (
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_MATERIALIZE_OWNED_BYTES,
            snapshot[132],
        ),
        (
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_OBJECTIZE_BOX_TOTAL,
            snapshot[133],
        ),
        (
            contract::BIRTH_BACKEND_SITE_CONST_SUFFIX_PUBLISH_HANDLE_TOTAL,
            snapshot[134],
        ),
        (
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_TOTAL,
            snapshot[135],
        ),
        (
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_MATERIALIZE_OWNED_BYTES,
            snapshot[136],
        ),
        (
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_OBJECTIZE_BOX_TOTAL,
            snapshot[137],
        ),
        (
            contract::BIRTH_BACKEND_SITE_FREEZE_TEXT_PLAN_PIECES3_PUBLISH_HANDLE_TOTAL,
            snapshot[138],
        ),
    ] {
        let _ = write!(&mut birth_backend_line, " {}={}", name, value);
    }
    eprintln!("{}", birth_backend_line);
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::STR_CONCAT2_ROUTE,
        contract::STR_CONCAT2_ROUTE_TOTAL,
        snapshot[47],
        contract::STR_CONCAT2_ROUTE_DISPATCH_HIT,
        snapshot[48],
        contract::STR_CONCAT2_ROUTE_FAST_STR_OWNED,
        snapshot[49],
        contract::STR_CONCAT2_ROUTE_FAST_STR_RETURN_HANDLE,
        snapshot[50],
        contract::STR_CONCAT2_ROUTE_SPAN_FREEZE,
        snapshot[51],
        contract::STR_CONCAT2_ROUTE_SPAN_RETURN_HANDLE,
        snapshot[52],
        contract::STR_CONCAT2_ROUTE_MATERIALIZE_FALLBACK,
        snapshot[53],
        contract::STR_CONCAT2_ROUTE_UNCLASSIFIED,
        str_concat2_unclassified,
    );
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::STR_LEN_ROUTE,
        contract::STR_LEN_ROUTE_TOTAL,
        snapshot[54],
        contract::STR_LEN_ROUTE_DISPATCH_HIT,
        snapshot[55],
        contract::STR_LEN_ROUTE_FAST_STR_HIT,
        snapshot[56],
        contract::STR_LEN_ROUTE_FALLBACK_HIT,
        snapshot[57],
        contract::STR_LEN_ROUTE_MISS,
        snapshot[58],
        contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FAST_STR_HIT,
        snapshot[59],
        contract::STR_LEN_ROUTE_LATEST_FRESH_HANDLE_FALLBACK_HIT,
        snapshot[60],
        contract::STR_LEN_ROUTE_UNCLASSIFIED,
        str_len_unclassified,
    );
    let mut str_substring_route_line = format!("[perf/counter][{}]", contract::STR_SUBSTRING_ROUTE);
    for (name, value) in [
        (contract::STR_SUBSTRING_ROUTE_TOTAL, snapshot[61]),
        (
            contract::STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_HANDLE_HIT,
            snapshot[62],
        ),
        (
            contract::STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_REISSUE_HIT,
            snapshot[63],
        ),
        (
            contract::STR_SUBSTRING_ROUTE_VIEW_ARC_CACHE_MISS,
            snapshot[64],
        ),
        (contract::STR_SUBSTRING_ROUTE_FAST_CACHE_HIT, snapshot[65]),
        (contract::STR_SUBSTRING_ROUTE_DISPATCH_HIT, snapshot[66]),
        (contract::STR_SUBSTRING_ROUTE_SLOW_PLAN, snapshot[67]),
        (
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_HANDLE,
            snapshot[68],
        ),
        (
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_RETURN_EMPTY,
            snapshot[69],
        ),
        (
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_FREEZE_SPAN,
            snapshot[70],
        ),
        (
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_VIEW_SPAN,
            snapshot[71],
        ),
        (
            contract::STR_SUBSTRING_ROUTE_SLOW_PLAN_UNCLASSIFIED,
            str_substring_slow_plan_unclassified,
        ),
    ] {
        let _ = write!(&mut str_substring_route_line, " {}={}", name, value);
    }
    eprintln!("{}", str_substring_route_line);
    eprintln!(
        "[perf/counter][{}] {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={} {}={}",
        contract::PIECEWISE_SUBRANGE,
        contract::PIECEWISE_SUBRANGE_TOTAL,
        snapshot[102],
        contract::PIECEWISE_SUBRANGE_SINGLE_SESSION_HIT,
        snapshot[103],
        contract::PIECEWISE_SUBRANGE_FALLBACK_INSERT,
        snapshot[104],
        contract::PIECEWISE_SUBRANGE_EMPTY_RETURN,
        snapshot[105],
        contract::PIECEWISE_SUBRANGE_PREFIX_ONLY,
        snapshot[106],
        contract::PIECEWISE_SUBRANGE_MIDDLE_ONLY,
        snapshot[107],
        contract::PIECEWISE_SUBRANGE_SUFFIX_ONLY,
        snapshot[108],
        contract::PIECEWISE_SUBRANGE_PREFIX_MIDDLE,
        snapshot[109],
        contract::PIECEWISE_SUBRANGE_MIDDLE_SUFFIX,
        snapshot[110],
        contract::PIECEWISE_SUBRANGE_PREFIX_SUFFIX,
        snapshot[111],
        contract::PIECEWISE_SUBRANGE_ALL_THREE,
        snapshot[112],
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
    for (name, value) in [
        (contract::BORROWED_ALIAS_TO_STRING_BOX, snapshot[72]),
        (contract::BORROWED_ALIAS_EQUALS, snapshot[73]),
        (contract::BORROWED_ALIAS_CLONE_BOX, snapshot[74]),
        (
            contract::BORROWED_ALIAS_TO_STRING_BOX_LATEST_FRESH,
            snapshot[75],
        ),
        (contract::BORROWED_ALIAS_EQUALS_LATEST_FRESH, snapshot[76]),
        (
            contract::BORROWED_ALIAS_CLONE_BOX_LATEST_FRESH,
            snapshot[77],
        ),
        (contract::BORROWED_ALIAS_BORROWED_SOURCE_FAST, snapshot[78]),
        (contract::BORROWED_ALIAS_AS_STR_FAST, snapshot[79]),
        (
            contract::BORROWED_ALIAS_AS_STR_FAST_LIVE_SOURCE,
            snapshot[80],
        ),
        (
            contract::BORROWED_ALIAS_AS_STR_FAST_STALE_SOURCE,
            snapshot[81],
        ),
        (
            contract::BORROWED_ALIAS_ARRAY_LEN_BY_INDEX_LATEST_FRESH,
            snapshot[82],
        ),
        (
            contract::BORROWED_ALIAS_ARRAY_INDEXOF_BY_INDEX_LATEST_FRESH,
            snapshot[83],
        ),
        (contract::BORROWED_ALIAS_ENCODE_EPOCH_HIT, snapshot[84]),
        (contract::BORROWED_ALIAS_ENCODE_PTR_EQ_HIT, snapshot[85]),
        (contract::BORROWED_ALIAS_ENCODE_TO_HANDLE_ARC, snapshot[86]),
        (
            contract::BORROWED_ALIAS_ENCODE_TO_HANDLE_ARC_ARRAY_GET_INDEX,
            snapshot[87],
        ),
        (
            contract::BORROWED_ALIAS_ENCODE_TO_HANDLE_ARC_MAP_RUNTIME_DATA_GET_ANY,
            snapshot[88],
        ),
    ] {
        let _ = write!(&mut borrowed_alias_line, " {}={}", name, value);
    }
    eprintln!("{}", borrowed_alias_line);
}
