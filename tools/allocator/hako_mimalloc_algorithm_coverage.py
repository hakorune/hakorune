#!/usr/bin/env python3
"""Report current .hako mimalloc algorithm coverage.

This is a read-only inventory tool. It separates:

- `.hako` hako_alloc policy/model coverage
- benchmark-only replacement-front execution coverage

It does not run benchmarks, choose keepers, or claim allocator readiness.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass, replace
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]
HAKO_ALLOC = ROOT / "lang/src/hako_alloc/memory"
REPLACEMENT_FRONT = ROOT / "tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py"
REPLACEMENT_TEMPLATES = ROOT / "tools/allocator/replacement_front_templates.py"


@dataclass(frozen=True)
class CoverageRow:
    area: str
    hako_model: int
    replacement_front: int
    status: str
    evidence: str
    next_bridge: str


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        return ""


def read_kv_report(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    data: dict[str, str] = {}
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except FileNotFoundError:
        return {}
    for line in lines:
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        data[key.strip()] = value.strip()
    return data


def read_fastpath_counts(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    try:
        text = path.read_text(encoding="utf-8")
    except FileNotFoundError:
        return {}
    try:
        payload = json.loads(text)
    except json.JSONDecodeError:
        return read_kv_report(path)
    counts = payload.get("counts") if isinstance(payload, dict) else None
    if not isinstance(counts, dict):
        return {}
    return {str(key): str(value) for key, value in counts.items()}


def int_field(data: dict[str, str], key: str, default: int = 0) -> int:
    try:
        return int(data.get(key, str(default)))
    except ValueError:
        return default


def str_field(data: dict[str, str], key: str, default: str = "0") -> str:
    value = data.get(key, default)
    return value if value else default


def has_file(path: Path) -> bool:
    return path.exists() and path.is_file()


def has_all(text: str, needles: Iterable[str]) -> bool:
    return all(needle in text for needle in needles)


def hako_file(name: str) -> Path:
    return HAKO_ALLOC / name


def count_member_calls(text: str, field: str, method: str) -> int:
    """Count direct `field.method(` and `me.field.method(` source calls.

    This is a static readiness scan, not semantic alias analysis. The leading
    boundary avoids counting `free.set(...)` inside `local_free.set(...)`.
    """

    pattern = rf"(?<![A-Za-z0-9_])(?:me\.)?{re.escape(field)}\.{method}\s*\("
    return len(re.findall(pattern, text))


def build_rows() -> list[CoverageRow]:
    page_box = read_text(hako_file("page_box.hako"))
    hot_core = read_text(hako_file("object_lifecycle_hot_core_box.hako"))
    size_class = read_text(hako_file("size_class_box.hako"))
    page_map = read_text(hako_file("page_map_box.hako"))
    realloc_same = read_text(hako_file("page_map_realloc_same_class_box.hako"))
    realloc_grow = read_text(hako_file("page_map_realloc_alloc_copy_release_box.hako"))
    remote_policy = read_text(hako_file("remote_free_policy_box.hako"))
    osvm_source = read_text(hako_file("osvm_page_source_pilot_box.hako"))
    huge_model = read_text(hako_file("huge_page_model_box.hako"))
    replacement = read_text(REPLACEMENT_FRONT) + "\n" + read_text(REPLACEMENT_TEMPLATES)

    fixed_slot_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_SLOT_SIZE",
            "direct_alloc_fast",
            "direct_free_local",
            "free_stack",
        ],
    )
    tls_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_THREAD_LOCAL",
            "remote_free_to_owner",
            "arena_registry",
        ],
    )
    inplace_realloc = has_all(
        replacement,
        [
            "realloc_inplace_count",
            "if (size <= HAKO_REPLACEMENT_SLOT_SIZE)",
        ],
    )

    direct_array_source = "DirectArrayI64" in page_box
    page_arrays_are_arraybox = has_all(
        page_box,
        [
            "free: ArrayBox",
            "local_free: ArrayBox",
            "block_used: ArrayBox",
        ],
    )

    return [
        CoverageRow(
            area="size_class_policy",
            hako_model=int(has_all(size_class, ["size_to_bin", "bin_size", "huge_bin"])),
            replacement_front=0,
            status="model_only",
            evidence="size_class_box.hako",
            next_bridge="connect size_class_policy to replacement bins/pages",
        ),
        CoverageRow(
            area="page_local_free_stack",
            hako_model=int(has_all(page_box, ["free_top", "acquireFreshSmall", "block_used"])),
            replacement_front=int(fixed_slot_front),
            status="split_model_and_fixed_front",
            evidence="page_box.hako + generated fixed-slot front",
            next_bridge="replace fixed one-size front with page/bin-backed route or prove selected fixture remains fixed-slot only",
        ),
        CoverageRow(
            area="same_thread_local_free",
            hako_model=int(has_all(page_box, ["local_free_top", "releaseLocalKnownLive"])),
            replacement_front=int(fixed_slot_front),
            status="split_model_and_fixed_front",
            evidence="page_box.hako + direct_free_local",
            next_bridge="connect PageModel release/local_free semantics to replacement free route",
        ),
        CoverageRow(
            area="object_lifecycle_hot_core",
            hako_model=int(has_all(hot_core, ["objectLifecycleSmallAlloc", "objectLifecycleReleaseBlock"])),
            replacement_front=0,
            status="model_only",
            evidence="object_lifecycle_hot_core_box.hako",
            next_bridge="consume HotCore/PageModel plans in replacement-front lowering",
        ),
        CoverageRow(
            area="page_map_lookup",
            hako_model=int(has_all(page_map, ["register", "lookup", "unregister"])),
            replacement_front=0,
            status="model_only",
            evidence="page_map_box.hako",
            next_bridge="connect pointer ownership lookup to product replacement route",
        ),
        CoverageRow(
            area="realloc_same_class",
            hako_model=int(has_file(hako_file("page_map_realloc_same_class_box.hako")) and "realloc" in realloc_same.lower()),
            replacement_front=int(inplace_realloc),
            status="split_model_and_fixed_front",
            evidence="page_map_realloc_same_class_box.hako + fixed-slot inplace realloc",
            next_bridge="connect requested-size/slot-class proof to general page-map realloc",
        ),
        CoverageRow(
            area="realloc_grow_copy_release",
            hako_model=int(has_file(hako_file("page_map_realloc_alloc_copy_release_box.hako")) and "copy" in realloc_grow.lower()),
            replacement_front=int("memcpy(next, ptr, copy_size)" in replacement),
            status="split_model_and_fixed_front",
            evidence="page_map_realloc_alloc_copy_release_box.hako + replacement memcpy fallback",
            next_bridge="connect hako realloc grow route to replacement bins/pages",
        ),
        CoverageRow(
            area="remote_free_policy",
            hako_model=int(has_file(hako_file("remote_free_policy_box.hako")) and "remote" in remote_policy.lower()),
            replacement_front=int(tls_front),
            status="split_model_and_fixed_front",
            evidence="remote_free_policy_box.hako + thread-local replacement remote queue",
            next_bridge="align .hako remote-free policy with replacement arena registry route",
        ),
        CoverageRow(
            area="osvm_page_source",
            hako_model=int(has_file(hako_file("osvm_page_source_pilot_box.hako")) and "osvm" in osvm_source.lower()),
            replacement_front=0,
            status="model_only",
            evidence="osvm_page_source_pilot_box.hako",
            next_bridge="connect page source to product allocator, not benchmark-only fixed slots",
        ),
        CoverageRow(
            area="huge_allocation_route",
            hako_model=int(has_file(hako_file("huge_page_model_box.hako")) and "huge" in huge_model.lower()),
            replacement_front=0,
            status="model_only",
            evidence="huge_page_model_box.hako",
            next_bridge="connect huge threshold/page model to replacement route",
        ),
        CoverageRow(
            area="directarray_source_storage",
            hako_model=int(direct_array_source),
            replacement_front=0,
            status="open" if page_arrays_are_arraybox else "source_migrated",
            evidence="page_box.hako",
            next_bridge="migrate hot page arrays from ArrayBox source to DirectArrayI64-backed storage when owner evidence selects it"
            if page_arrays_are_arraybox
            else "measure migrated DirectArrayI64 source route",
        ),
    ]


def report_dict(
    rows: list[CoverageRow],
    *,
    benchmark_report: Path | None = None,
    fastpath_report: Path | None = None,
    perf_attribution_report: Path | None = None,
) -> dict[str, object]:
    page_box = read_text(hako_file("page_box.hako"))
    hot_core = read_text(hako_file("object_lifecycle_hot_core_box.hako"))
    page_map = read_text(hako_file("page_map_box.hako"))
    page_map_release = read_text(hako_file("page_map_release_box.hako"))
    realloc_same = read_text(hako_file("page_map_realloc_same_class_box.hako"))
    realloc_grow = read_text(hako_file("page_map_realloc_alloc_copy_release_box.hako"))
    huge_model = read_text(hako_file("huge_page_model_box.hako"))
    osvm_source = read_text(hako_file("osvm_page_source_pilot_box.hako"))
    replacement = read_text(REPLACEMENT_FRONT) + "\n" + read_text(REPLACEMENT_TEMPLATES)
    hot_array_fields = ["free", "local_free", "block_used"]
    hot_array_ops = {
        name: {
            "get": count_member_calls(page_box, name, "get"),
            "set": count_member_calls(page_box, name, "set"),
            "push": count_member_calls(page_box, name, "push"),
        }
        for name in hot_array_fields
    }
    hot_array_get_count = sum(ops["get"] for ops in hot_array_ops.values())
    hot_array_set_count = sum(ops["set"] for ops in hot_array_ops.values())
    hot_array_access_count = hot_array_get_count + hot_array_set_count
    hot_array_push_count = sum(ops["push"] for ops in hot_array_ops.values())
    hot_array_arraybox_fields = [
        name for name in hot_array_fields if f"{name}: ArrayBox" in page_box
    ]
    hot_array_direct_fields = [
        name for name in hot_array_fields if f"{name}: DirectArrayI64" in page_box
    ]
    hot_array_source_type_ready = int(not hot_array_arraybox_fields and len(hot_array_direct_fields) == len(hot_array_fields))
    hot_array_birth_contract_ready = int(
        hot_array_source_type_ready
        and has_all(page_box, ["new DirectArrayI64", ".set("])
        and hot_array_push_count == 0
    )
    hot_array_source_migration_selected = int(
        hot_array_source_type_ready and hot_array_birth_contract_ready
    )
    if hot_array_source_type_ready:
        migration_blocker = "none" if hot_array_birth_contract_ready else "directarray_i64_birth_contract_unverified"
    elif hot_array_push_count:
        migration_blocker = "push_or_initialized_len_contract"
    else:
        migration_blocker = "field_type_and_birth_contract_unverified"
    hotcore_methods = [
        method
        for method in ("objectLifecycleSmallAlloc", "objectLifecycleReleaseBlock")
        if method in hot_core
    ]
    hotcore_small_alloc_calls_acquire_fresh_small = int(
        "page.acquireFreshSmall(" in hot_core
    )
    hotcore_release_calls_release_local_known_live = int(
        "page.releaseLocalKnownLive(" in hot_core
    )
    page_model_hot_methods_ready = int(
        has_all(page_box, ["acquireFreshSmall", "releaseLocalKnownLive"])
    )
    page_map_source_ready = int(
        has_all(page_map, ["findIndex", "register", "lookup", "unregister"])
    )
    page_map_release_source_ready = int(
        has_all(page_map_release, ["releasePtr", "page_map.lookup", "page.releaseLocal", "page_map.unregister"])
    )
    realloc_same_class_source_ready = int(
        has_all(realloc_same, ["tryReallocSameClass", "page_map.lookup", "blockIsLive", "requested_size > page.block_size"])
    )
    realloc_grow_copy_release_source_ready = int(
        has_all(realloc_grow, ["page_map.lookup", "copy", "page_map.register"])
    )
    huge_page_source_ready = int(
        has_all(huge_model, ["register", "lookup", "huge"])
        or has_all(huge_model, ["allocateHuge", "markReleased", "requestedSizeFor"])
    )
    osvm_page_source_pilot_ready = int(
        has_all(osvm_source, ["osvm", "page"]) and has_file(hako_file("osvm_page_source_pilot_box.hako"))
    )
    size_class_single_bridge_supported = has_all(
        replacement,
        [
            "--replacement-front-match-hako-size-class",
            "hako_good_size",
            "hako_good_size_request_ceiling",
        ],
    )
    page_bins_bridge_supported = has_all(
        replacement,
        [
            "--replacement-front-page-bins-mode",
            "page_shaped",
            "HakoReplacement",
            "Page",
            "benchmark_page_bins",
        ],
    )
    locked_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_LOCKED",
            "lock_arena",
            "pthread_mutex_lock(&arena_lock)",
        ],
    )
    tls_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_THREAD_LOCAL",
            "remote_free_to_owner",
            "arena_registry",
        ],
    )
    replacement_full_hako = int(
        all(row.replacement_front for row in rows if row.area in {
            "size_class_policy",
            "page_local_free_stack",
            "same_thread_local_free",
            "object_lifecycle_hot_core",
            "page_map_lookup",
        })
    )
    benchmark = read_kv_report(benchmark_report)
    fastpath = read_fastpath_counts(fastpath_report)
    perf_attribution = read_kv_report(perf_attribution_report)
    benchmark_report_consumed = int(bool(benchmark))
    fastpath_report_consumed = int(bool(fastpath))
    perf_attribution_report_consumed = int(bool(perf_attribution))
    benchmark_subject = "none"
    benchmark_subject_prefix = ""
    for prefix in ("subject_2", "subject_3", "subject_4"):
        subject_id = benchmark.get(f"{prefix}_id") or benchmark.get(f"{prefix}_name")
        if subject_id == "hakorune_replacement_front_ldpreload":
            benchmark_subject = subject_id
            benchmark_subject_prefix = prefix
            break
    if (
        benchmark_report_consumed
        and benchmark_subject == "none"
        and int_field(benchmark, "replacement_front_page_bins_consumer_enabled", 0)
    ):
        benchmark_subject = "hakorune_replacement_front_ldpreload"
    page_bins_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_page_bins_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_page_bins_consumer_enabled",
        0,
    )
    page_bins_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_page_bins_route"
        if benchmark_subject_prefix
        else "replacement_front_page_bins_route",
        "not_consumed",
    )
    page_bins_lookup_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_page_bins_lookup_route"
        if benchmark_subject_prefix
        else "replacement_front_page_bins_lookup_route",
        "not_recorded" if page_bins_consumer_enabled else "not_consumed",
    )
    product_bins_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_product_bins_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_product_bins_consumer_enabled",
        0,
    )
    product_bins_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_product_bins_route"
        if benchmark_subject_prefix
        else "replacement_front_product_bins_route",
        "not_consumed",
    )
    product_pages_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_product_pages_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_product_pages_consumer_enabled",
        0,
    )
    product_pages_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_product_pages_route"
        if benchmark_subject_prefix
        else "replacement_front_product_pages_route",
        "not_consumed",
    )
    algorithm_shape = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_algorithm_shape"
        if benchmark_subject_prefix
        else "replacement_front_algorithm_shape",
        "not_consumed",
    )
    hotcore_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_hotcore_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_hotcore_consumer_enabled",
        0,
    )
    hotcore_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_hotcore_route"
        if benchmark_subject_prefix
        else "replacement_front_hotcore_route",
        "not_consumed_by_replacement_front",
    )
    hotcore_median_ops_per_sec = str_field(
        benchmark,
        f"{benchmark_subject_prefix}_throughput_median_ops_per_sec"
        if benchmark_subject_prefix
        else "throughput_median_ops_per_sec",
        "0",
    )
    hotcore_measurement_reported = int(
        hotcore_consumer_enabled and hotcore_median_ops_per_sec != "0"
    )
    fastpath_direct_array_plan_count = int_field(
        fastpath, "direct_array_access_plan_count", 0
    )
    fastpath_route_decision_count = int_field(fastpath, "route_decision_count", 0)
    fastpath_fast_selected_count = int_field(
        fastpath, "route_decision_fast_selected_count", 0
    )
    fastpath_slow_selected_count = int_field(
        fastpath, "route_decision_slow_selected_count", 0
    )
    fastpath_generic_dispatch_count = int_field(
        fastpath, "generic_method_dispatch_count", 0
    )
    fastpath_dynamic_route_count = int_field(fastpath, "dynamic_route_count", 0)
    fastpath_boxed_fallback_count = int_field(fastpath, "boxed_fallback_count", 0)
    fastpath_clean = int_field(fastpath, "clean", 0)
    page_model_hot_array_source_route_measured = int(
        fastpath_report_consumed
        and hot_array_access_count > 0
        and fastpath_direct_array_plan_count >= hot_array_access_count
        and fastpath_route_decision_count > 0
        and fastpath_fast_selected_count == fastpath_route_decision_count
        and fastpath_slow_selected_count == 0
        and fastpath_generic_dispatch_count == 0
        and fastpath_dynamic_route_count == 0
        and fastpath_boxed_fallback_count == 0
        and fastpath_clean == 1
    )
    if page_model_hot_array_source_route_measured:
        hot_array_route_measurement_blocker = "none"
        hot_array_route_next_bridge = "perf_delta_measurement"
    elif fastpath_report_consumed:
        hot_array_route_measurement_blocker = "directarray_route_not_clean"
        hot_array_route_next_bridge = "fix_or_explain_directarray_route_miss"
    else:
        hot_array_route_measurement_blocker = "fastpath_report_not_consumed"
        hot_array_route_next_bridge = "run_hako_check_fastpath_explain"
    hotcore_page_model_source_ready = int(
        len(hotcore_methods) == 2
        and hotcore_small_alloc_calls_acquire_fresh_small
        and hotcore_release_calls_release_local_known_live
        and page_model_hot_methods_ready
        and hot_array_source_migration_selected
    )
    hotcore_replacement_shape_ready = int(hotcore_page_model_source_ready)
    if hotcore_consumer_enabled:
        hotcore_bridge_blocker = "none"
        hotcore_next_bridge = (
            "select_next_structural_owner"
            if hotcore_measurement_reported
            else "measure_hotcore_replacement_consumer"
        )
    elif hotcore_replacement_shape_ready:
        hotcore_bridge_blocker = "consumer_not_enabled"
        hotcore_next_bridge = "replacement_front_consume_hotcore_page_model"
    else:
        hotcore_bridge_blocker = "source_shape_not_ready"
        hotcore_next_bridge = "fix_hotcore_page_model_source_shape"
    product_pages_source_ready = int(
        page_map_source_ready
        and page_map_release_source_ready
        and realloc_same_class_source_ready
        and page_model_hot_methods_ready
    )
    product_pages_full_source_ready = int(
        product_pages_source_ready
        and realloc_grow_copy_release_source_ready
        and huge_page_source_ready
        and osvm_page_source_pilot_ready
    )
    if product_pages_consumer_enabled:
        product_pages_bridge_blocker = "none"
        product_pages_next_bridge = "measure_product_pages_consumer"
    elif product_pages_source_ready:
        product_pages_bridge_blocker = "consumer_not_enabled"
        product_pages_next_bridge = "page_map_backed_replacement_front_plan"
    else:
        product_pages_bridge_blocker = "source_shape_not_ready"
        product_pages_next_bridge = "fix_product_pages_source_shape"
    structural_owner_refresh_required = int(
        hotcore_measurement_reported
        and hotcore_next_bridge == "select_next_structural_owner"
    )
    page_model_hot_array_measurement_ready = int(
        structural_owner_refresh_required and hot_array_source_migration_selected
    )
    perf_delta_plan = int_field(
        perf_attribution, "page_model_hot_array_perf_delta_measurement_plan_v0", 0
    )
    perf_delta_ready = int_field(
        perf_attribution, "page_model_hot_array_perf_delta_ready", 0
    )
    perf_delta_blocker = str_field(
        perf_attribution,
        "page_model_hot_array_perf_delta_blocker",
        "perf_attribution_report_not_consumed"
        if not perf_attribution_report_consumed
        else "unknown",
    )
    perf_delta_next_bridge = str_field(
        perf_attribution,
        "page_model_hot_array_perf_delta_next_bridge",
        "run_hako_mimalloc_direct_exact_app_perf_asm"
        if not perf_attribution_report_consumed
        else "inspect_perf_attribution",
    )
    product_pages_non_linear_owner_candidate_ready = int(
        structural_owner_refresh_required
        and product_pages_source_ready
        and not product_pages_consumer_enabled
    )
    if page_model_hot_array_measurement_ready:
        structural_owner_selected = "page_model_hot_array_source_route_measurement"
        structural_owner_reason = "hotcore_measured_and_directarray_source_ready"
        structural_owner_next_action = (
            "measure_page_model_hot_array_perf_delta"
            if page_model_hot_array_source_route_measured
            else "measure_page_model_hot_array_source_route"
        )
        if page_model_hot_array_source_route_measured and perf_attribution_report_consumed:
            structural_owner_next_action = (
                "select_next_perf_owner" if perf_delta_ready else perf_delta_next_bridge
            )
    elif product_pages_non_linear_owner_candidate_ready:
        structural_owner_selected = "product_pages_bridge_non_linear_owner_lookup"
        structural_owner_reason = "hotcore_measured_and_product_pages_source_ready"
        structural_owner_next_action = "design_non_linear_product_pages_bridge"
    elif structural_owner_refresh_required:
        structural_owner_selected = "none"
        structural_owner_reason = "no_source_ready_structural_owner"
        structural_owner_next_action = "fix_source_shape_before_next_probe"
    else:
        structural_owner_selected = "none"
        structural_owner_reason = "hotcore_measurement_not_reported"
        structural_owner_next_action = "measure_hotcore_replacement_consumer"
    refreshed_rows: list[CoverageRow] = []
    for row in rows:
        if row.area == "size_class_policy" and product_bins_consumer_enabled:
            row = replace(
                row,
                replacement_front=1,
                status="split_model_and_fixed_front",
                evidence="size_class_box.hako + benchmark replacement-front size-class bridge",
                next_bridge="measure current size-class bridge or connect product pages",
            )
        if row.area == "object_lifecycle_hot_core" and hotcore_consumer_enabled:
            row = replace(
                row,
                replacement_front=1,
                status="split_model_and_fixed_front",
                evidence=(
                    "object_lifecycle_hot_core_box.hako + benchmark HotCore/PageModel front"
                ),
                next_bridge=hotcore_next_bridge,
            )
        refreshed_rows.append(row)
    rows = refreshed_rows
    return {
        "output_contract": "hako-mimalloc-algorithm-coverage-v0",
        "hako_alloc_root": str(HAKO_ALLOC.relative_to(ROOT)),
        "replacement_front": str(REPLACEMENT_FRONT.relative_to(ROOT)),
        "replacement_front_is_full_hako_algorithm": replacement_full_hako,
        "provider_activation": 0,
        "production_replacement_active": 0,
        "winner_claim": 0,
        "benchmark_report": str(benchmark_report) if benchmark_report is not None else "none",
        "benchmark_report_consumed": benchmark_report_consumed,
        "benchmark_replacement_subject": benchmark_subject,
        "fastpath_report": str(fastpath_report) if fastpath_report is not None else "none",
        "fastpath_report_consumed": fastpath_report_consumed,
        "perf_attribution_report": str(perf_attribution_report)
        if perf_attribution_report is not None
        else "none",
        "perf_attribution_report_consumed": perf_attribution_report_consumed,
        "area_count": len(rows),
        "hako_model_area_count": sum(row.hako_model for row in rows),
        "replacement_front_area_count": sum(row.replacement_front for row in rows),
        "model_only_area_count": sum(1 for row in rows if row.status == "model_only"),
        "split_model_and_fixed_front_area_count": sum(
            1 for row in rows if row.status == "split_model_and_fixed_front"
        ),
        "open_area_count": sum(1 for row in rows if row.status == "open"),
        "size_class_policy_bridge_plan_v0": 1,
        "size_class_policy_product_bins_connected": product_bins_consumer_enabled,
        "size_class_policy_single_class_benchmark_bridge_supported": int(
            size_class_single_bridge_supported
        ),
        "size_class_policy_single_class_bridge_mode": "hako_good_size_request_ceiling"
        if size_class_single_bridge_supported
        else "none",
        "size_class_policy_next_bridge": "product_replacement_bins_pages",
        "replacement_front_page_bins_plan_v0": 1,
        "replacement_front_page_bins_supported": int(page_bins_bridge_supported),
        "replacement_front_page_bins_consumer_enabled": page_bins_consumer_enabled,
        "replacement_front_page_bins_route": page_bins_route,
        "replacement_front_page_bins_lookup_route": page_bins_lookup_route,
        "replacement_front_page_bins_owner": "benchmark_only",
        "replacement_front_page_bins_product_claim": 0,
        "replacement_front_benchmark_algorithm_shape": algorithm_shape,
        "replacement_front_product_bins_consumer_enabled": product_bins_consumer_enabled,
        "replacement_front_product_bins_route": product_bins_route,
        "replacement_front_product_pages_bridge_plan_v0": 1,
        "replacement_front_product_pages_bridge_report_only": 1,
        "replacement_front_product_pages_consumer_enabled": product_pages_consumer_enabled,
        "replacement_front_product_pages_route": product_pages_route,
        "replacement_front_product_pages_source_ready": product_pages_source_ready,
        "replacement_front_product_pages_full_source_ready": product_pages_full_source_ready,
        "replacement_front_product_pages_bridge_blocker": product_pages_bridge_blocker,
        "replacement_front_product_pages_next_bridge": product_pages_next_bridge,
        "page_map_source_ready": page_map_source_ready,
        "page_map_release_source_ready": page_map_release_source_ready,
        "realloc_same_class_source_ready": realloc_same_class_source_ready,
        "realloc_grow_copy_release_source_ready": realloc_grow_copy_release_source_ready,
        "huge_page_source_ready": huge_page_source_ready,
        "osvm_page_source_pilot_ready": osvm_page_source_pilot_ready,
        "replacement_front_locked_global_multithread_supported": int(locked_front),
        "replacement_front_thread_local_multithread_supported": int(tls_front),
        "replacement_front_multithread_claim": 0,
        "structural_owner_selection_plan_v0": 1,
        "structural_owner_refresh_required": structural_owner_refresh_required,
        "structural_owner_selected": structural_owner_selected,
        "structural_owner_selected_reason": structural_owner_reason,
        "structural_owner_next_action": structural_owner_next_action,
        "structural_owner_candidate_0": "page_model_hot_array_source_route_measurement",
        "structural_owner_candidate_0_ready": page_model_hot_array_measurement_ready,
        "structural_owner_candidate_1": "product_pages_bridge_non_linear_owner_lookup",
        "structural_owner_candidate_1_ready": product_pages_non_linear_owner_candidate_ready,
        "page_model_hot_array_bridge_plan_v0": 1,
        "page_model_hot_array_access_plan_v0": 1,
        "page_model_hot_array_access_static_scan": 1,
        "page_model_hot_array_source_migration_selected": hot_array_source_migration_selected,
        "page_model_hot_array_source_type_ready": hot_array_source_type_ready,
        "page_model_hot_array_birth_contract_ready": hot_array_birth_contract_ready,
        "page_model_hot_array_source_migration_blocker": migration_blocker,
        "page_model_hot_array_next_bridge": "directarray_i64_field_type_and_birth_fixture"
        if migration_blocker != "none"
        else "source_migration_measurement",
        "page_model_hot_array_candidate_type": "DirectArrayI64",
        "page_model_hot_array_directarray_supported_ops": "get,set",
        "page_model_hot_array_directarray_missing_ops": "push_or_birth_with_initialized_len"
        if hot_array_push_count
        else "none",
        "page_model_hot_array_source_route_measurement_plan_v0": 1,
        "page_model_hot_array_source_route_measured": page_model_hot_array_source_route_measured,
        "page_model_hot_array_source_route_measurement_blocker": hot_array_route_measurement_blocker,
        "page_model_hot_array_source_route_next_bridge": hot_array_route_next_bridge,
        "page_model_hot_array_fastpath_direct_array_plan_count": fastpath_direct_array_plan_count,
        "page_model_hot_array_fastpath_route_decision_count": fastpath_route_decision_count,
        "page_model_hot_array_fastpath_fast_selected_count": fastpath_fast_selected_count,
        "page_model_hot_array_fastpath_slow_selected_count": fastpath_slow_selected_count,
        "page_model_hot_array_perf_delta_measurement_plan_v0": perf_delta_plan,
        "page_model_hot_array_perf_delta_ready": perf_delta_ready,
        "page_model_hot_array_perf_delta_blocker": perf_delta_blocker,
        "page_model_hot_array_perf_delta_next_bridge": perf_delta_next_bridge,
        "perf_top_symbol": str_field(perf_attribution, "top_symbol", "none"),
        "perf_top_symbol_percent": str_field(perf_attribution, "top_symbol_percent", "0.00"),
        "perf_symbol_collapse_detected": int_field(
            perf_attribution, "symbol_collapse_detected", 0
        ),
        "perf_symbol_attribution_available": int_field(
            perf_attribution, "symbol_attribution_available", 0
        ),
        "perf_instruction_attribution_available": int_field(
            perf_attribution, "instruction_attribution_available", 0
        ),
        "perf_annotate_nonzero_instruction_count": int_field(
            perf_attribution, "annotate_nonzero_instruction_count", 0
        ),
        "perf_top_instruction_percent": str_field(
            perf_attribution, "top_instruction_percent", "0.00"
        ),
        "perf_top_instruction_mnemonic": str_field(
            perf_attribution, "top_instruction_mnemonic", "none"
        ),
        "perf_top_instruction_category": str_field(
            perf_attribution, "top_instruction_category", "none"
        ),
        "perf_hot_instruction_report_count": int_field(
            perf_attribution, "hot_instruction_report_count", 0
        ),
        "perf_hot_instruction_0_category": str_field(
            perf_attribution, "hot_instruction_0_category", "none"
        ),
        "perf_hot_instruction_0_asm": str_field(
            perf_attribution, "hot_instruction_0_asm", "none"
        ),
        "page_model_hot_array_seed_push_blocker": int(hot_array_push_count > 0),
        "page_model_hot_array_field_count": len(hot_array_fields),
        "page_model_hot_array_arraybox_field_count": len(hot_array_arraybox_fields),
        "page_model_hot_array_directarray_field_count": len(hot_array_direct_fields),
        "page_model_hot_array_arraybox_fields": ",".join(hot_array_arraybox_fields) or "none",
        "page_model_hot_array_directarray_fields": ",".join(hot_array_direct_fields) or "none",
        "page_model_hot_array_get_count": hot_array_get_count,
        "page_model_hot_array_set_count": hot_array_set_count,
        "page_model_hot_array_push_count": hot_array_push_count,
        "page_model_hot_array_op_summary": ",".join(
            f"{name}:get={ops['get']}:set={ops['set']}:push={ops['push']}"
            for name, ops in hot_array_ops.items()
        ),
        "hotcore_replacement_bridge_plan_v0": 1,
        "hotcore_replacement_bridge_report_only": 1,
        "hotcore_replacement_consumer_enabled": hotcore_consumer_enabled,
        "hotcore_replacement_shape_ready": hotcore_replacement_shape_ready,
        "hotcore_replacement_bridge_blocker": hotcore_bridge_blocker,
        "hotcore_replacement_next_bridge": hotcore_next_bridge,
        "hotcore_replacement_measurement_reported": hotcore_measurement_reported,
        "hotcore_replacement_median_ops_per_sec": hotcore_median_ops_per_sec,
        "hotcore_page_model_source_ready": hotcore_page_model_source_ready,
        "hotcore_small_alloc_calls_acquire_fresh_small": hotcore_small_alloc_calls_acquire_fresh_small,
        "hotcore_release_calls_release_local_known_live": hotcore_release_calls_release_local_known_live,
        "page_model_hot_methods_ready": page_model_hot_methods_ready,
        "hotcore_source_method_count": len(hotcore_methods),
        "hotcore_source_methods": ",".join(hotcore_methods) or "none",
        "hotcore_replacement_route": hotcore_route,
        "rows": [row.__dict__ for row in rows],
    }


def emit_text(data: dict[str, object]) -> None:
    for key in [
        "output_contract",
        "hako_alloc_root",
        "replacement_front",
        "replacement_front_is_full_hako_algorithm",
        "provider_activation",
        "production_replacement_active",
        "winner_claim",
        "benchmark_report",
        "benchmark_report_consumed",
        "benchmark_replacement_subject",
        "fastpath_report",
        "fastpath_report_consumed",
        "perf_attribution_report",
        "perf_attribution_report_consumed",
        "area_count",
        "hako_model_area_count",
        "replacement_front_area_count",
        "model_only_area_count",
        "split_model_and_fixed_front_area_count",
        "open_area_count",
        "size_class_policy_bridge_plan_v0",
        "size_class_policy_product_bins_connected",
        "size_class_policy_single_class_benchmark_bridge_supported",
        "size_class_policy_single_class_bridge_mode",
        "size_class_policy_next_bridge",
        "replacement_front_page_bins_plan_v0",
        "replacement_front_page_bins_supported",
        "replacement_front_page_bins_consumer_enabled",
        "replacement_front_page_bins_route",
        "replacement_front_page_bins_lookup_route",
        "replacement_front_page_bins_owner",
        "replacement_front_page_bins_product_claim",
        "replacement_front_benchmark_algorithm_shape",
        "replacement_front_product_bins_consumer_enabled",
        "replacement_front_product_bins_route",
        "replacement_front_product_pages_bridge_plan_v0",
        "replacement_front_product_pages_bridge_report_only",
        "replacement_front_product_pages_consumer_enabled",
        "replacement_front_product_pages_route",
        "replacement_front_product_pages_source_ready",
        "replacement_front_product_pages_full_source_ready",
        "replacement_front_product_pages_bridge_blocker",
        "replacement_front_product_pages_next_bridge",
        "page_map_source_ready",
        "page_map_release_source_ready",
        "realloc_same_class_source_ready",
        "realloc_grow_copy_release_source_ready",
        "huge_page_source_ready",
        "osvm_page_source_pilot_ready",
        "replacement_front_locked_global_multithread_supported",
        "replacement_front_thread_local_multithread_supported",
        "replacement_front_multithread_claim",
        "structural_owner_selection_plan_v0",
        "structural_owner_refresh_required",
        "structural_owner_selected",
        "structural_owner_selected_reason",
        "structural_owner_next_action",
        "structural_owner_candidate_0",
        "structural_owner_candidate_0_ready",
        "structural_owner_candidate_1",
        "structural_owner_candidate_1_ready",
        "page_model_hot_array_bridge_plan_v0",
        "page_model_hot_array_access_plan_v0",
        "page_model_hot_array_access_static_scan",
        "page_model_hot_array_source_migration_selected",
        "page_model_hot_array_source_type_ready",
        "page_model_hot_array_birth_contract_ready",
        "page_model_hot_array_source_migration_blocker",
        "page_model_hot_array_next_bridge",
        "page_model_hot_array_candidate_type",
        "page_model_hot_array_directarray_supported_ops",
        "page_model_hot_array_directarray_missing_ops",
        "page_model_hot_array_source_route_measurement_plan_v0",
        "page_model_hot_array_source_route_measured",
        "page_model_hot_array_source_route_measurement_blocker",
        "page_model_hot_array_source_route_next_bridge",
        "page_model_hot_array_fastpath_direct_array_plan_count",
        "page_model_hot_array_fastpath_route_decision_count",
        "page_model_hot_array_fastpath_fast_selected_count",
        "page_model_hot_array_fastpath_slow_selected_count",
        "page_model_hot_array_perf_delta_measurement_plan_v0",
        "page_model_hot_array_perf_delta_ready",
        "page_model_hot_array_perf_delta_blocker",
        "page_model_hot_array_perf_delta_next_bridge",
        "perf_top_symbol",
        "perf_top_symbol_percent",
        "perf_symbol_collapse_detected",
        "perf_symbol_attribution_available",
        "perf_instruction_attribution_available",
        "perf_annotate_nonzero_instruction_count",
        "perf_top_instruction_percent",
        "perf_top_instruction_mnemonic",
        "perf_top_instruction_category",
        "perf_hot_instruction_report_count",
        "perf_hot_instruction_0_category",
        "perf_hot_instruction_0_asm",
        "page_model_hot_array_seed_push_blocker",
        "page_model_hot_array_field_count",
        "page_model_hot_array_arraybox_field_count",
        "page_model_hot_array_directarray_field_count",
        "page_model_hot_array_arraybox_fields",
        "page_model_hot_array_directarray_fields",
        "page_model_hot_array_get_count",
        "page_model_hot_array_set_count",
        "page_model_hot_array_push_count",
        "page_model_hot_array_op_summary",
        "hotcore_replacement_bridge_plan_v0",
        "hotcore_replacement_bridge_report_only",
        "hotcore_replacement_consumer_enabled",
        "hotcore_replacement_shape_ready",
        "hotcore_replacement_bridge_blocker",
        "hotcore_replacement_next_bridge",
        "hotcore_replacement_measurement_reported",
        "hotcore_replacement_median_ops_per_sec",
        "hotcore_page_model_source_ready",
        "hotcore_small_alloc_calls_acquire_fresh_small",
        "hotcore_release_calls_release_local_known_live",
        "page_model_hot_methods_ready",
        "hotcore_source_method_count",
        "hotcore_source_methods",
        "hotcore_replacement_route",
    ]:
        print(f"{key}={data[key]}")

    print("")
    print("area_status:")
    for row in data["rows"]:  # type: ignore[index]
        print(
            "{area} hako_model={hako_model} replacement_front={replacement_front} "
            "status={status} evidence={evidence} next_bridge={next_bridge}".format(**row)
        )

    print("")
    print("summary=ok")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit JSON instead of text")
    parser.add_argument(
        "--benchmark-report",
        type=Path,
        help=(
            "optional hakozuna mixed-ws compare report to overlay executed "
            "benchmark-only replacement-front route fields"
        ),
    )
    parser.add_argument(
        "--fastpath-report",
        type=Path,
        help=(
            "optional hako_check fastpath-explain JSON/KV report to overlay "
            "DirectArray source-route measurement fields"
        ),
    )
    parser.add_argument(
        "--perf-attribution-report",
        type=Path,
        help=(
            "optional hako-mimalloc-perf-attribution report to overlay "
            "PageModel hot-array perf-delta readiness fields"
        ),
    )
    args = parser.parse_args()

    data = report_dict(
        build_rows(),
        benchmark_report=args.benchmark_report,
        fastpath_report=args.fastpath_report,
        perf_attribution_report=args.perf_attribution_report,
    )
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        emit_text(data)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
