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
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]
HAKO_ALLOC = ROOT / "lang/src/hako_alloc/memory"
REPLACEMENT_FRONT = ROOT / "tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py"


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
    replacement = read_text(REPLACEMENT_FRONT)

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
            status="open" if page_arrays_are_arraybox else "unknown",
            evidence="page_box.hako",
            next_bridge="migrate hot page arrays from ArrayBox source to DirectArrayI64-backed storage when owner evidence selects it",
        ),
    ]


def report_dict(rows: list[CoverageRow]) -> dict[str, object]:
    page_box = read_text(hako_file("page_box.hako"))
    hot_core = read_text(hako_file("object_lifecycle_hot_core_box.hako"))
    replacement = read_text(REPLACEMENT_FRONT)
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
    size_class_single_bridge_supported = has_all(
        replacement,
        [
            "--replacement-front-match-hako-size-class",
            "hako_good_size",
            "hako_good_size_request_ceiling",
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
    return {
        "output_contract": "hako-mimalloc-algorithm-coverage-v0",
        "hako_alloc_root": str(HAKO_ALLOC.relative_to(ROOT)),
        "replacement_front": str(REPLACEMENT_FRONT.relative_to(ROOT)),
        "replacement_front_is_full_hako_algorithm": replacement_full_hako,
        "provider_activation": 0,
        "production_replacement_active": 0,
        "winner_claim": 0,
        "area_count": len(rows),
        "hako_model_area_count": sum(row.hako_model for row in rows),
        "replacement_front_area_count": sum(row.replacement_front for row in rows),
        "model_only_area_count": sum(1 for row in rows if row.status == "model_only"),
        "split_model_and_fixed_front_area_count": sum(
            1 for row in rows if row.status == "split_model_and_fixed_front"
        ),
        "open_area_count": sum(1 for row in rows if row.status == "open"),
        "size_class_policy_bridge_plan_v0": 1,
        "size_class_policy_product_bins_connected": 0,
        "size_class_policy_single_class_benchmark_bridge_supported": int(
            size_class_single_bridge_supported
        ),
        "size_class_policy_single_class_bridge_mode": "hako_good_size_request_ceiling"
        if size_class_single_bridge_supported
        else "none",
        "size_class_policy_next_bridge": "product_replacement_bins_pages",
        "page_model_hot_array_bridge_plan_v0": 1,
        "page_model_hot_array_access_plan_v0": 1,
        "page_model_hot_array_access_static_scan": 1,
        "page_model_hot_array_source_migration_selected": 0,
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
        "hotcore_replacement_consumer_enabled": 0,
        "hotcore_source_method_count": len(hotcore_methods),
        "hotcore_source_methods": ",".join(hotcore_methods) or "none",
        "hotcore_replacement_route": "not_consumed_by_replacement_front",
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
    args = parser.parse_args()

    data = report_dict(build_rows())
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        emit_text(data)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
