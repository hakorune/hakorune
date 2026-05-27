#!/usr/bin/env python3
"""Rank active .hako mimalloc owners from measurement counters and MIR shape."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path


REJECTED_KEEPER = "select_single_page_active_field_fast_path"
NEXT_ROW = "HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER-296X-001"


@dataclass(frozen=True)
class OwnerRank:
    source_method: str
    selected_method: str
    active_count: int
    mir_call_count: int
    mir_field_count: int
    mir_array_count: int
    score: int
    risk_kind: str
    reason: str


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key)
    if text is None or text == "":
        return 0
    try:
        return int(text)
    except ValueError:
        return 0


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def active_count_for(method: str, measurement: dict[str, str]) -> tuple[int, str]:
    allocation_count = as_int(measurement, "allocation_count")
    free_count = as_int(measurement, "free_count")
    select_fast = as_int(measurement, "select_page_single_fast_path_count")
    select_fallback = as_int(measurement, "select_page_single_fallback_count")
    release_fast = as_int(measurement, "release_known_page_fast_path_count")
    release_fallback = as_int(measurement, "release_known_page_fallback_count")

    if method == "objectLifecycleSmallAlloc":
        return (allocation_count or select_fast, "allocation_count")
    if method == "selectPage":
        return (select_fast + select_fallback, "select_invocation_count")
    if method == "selectSinglePageFastPath":
        return (select_fast, "select_single_fast_path_count")
    if method == "objectLifecycleReleaseBlock":
        return (free_count or release_fast, "free_count")
    if method == "objectLifecycleReleaseDirectCachedPage":
        return (release_fast, "release_direct_fast_path_count")
    if method == "objectLifecycleReleaseKnownPageIndex":
        return (release_fallback, "release_lookup_fallback_count")
    return (0, "unknown")


def risk_kind_for(report: dict[str, str]) -> str:
    mir_call = as_int(report, "mir_call_count")
    mir_field = as_int(report, "mir_field_access_count")
    mir_array = as_int(report, "mir_array_access_count")
    if mir_call >= mir_field and mir_call >= mir_array:
        return "method_call_surface"
    if mir_array >= mir_field:
        return "array_access"
    return "field_access"


def rank_for(report: dict[str, str], measurement: dict[str, str]) -> OwnerRank:
    source_method = report.get("source_target_method", "")
    active_count, active_source = active_count_for(source_method, measurement)
    mir_call = as_int(report, "mir_call_count")
    mir_field = as_int(report, "mir_field_access_count")
    mir_array = as_int(report, "mir_array_access_count")
    shape_weight = mir_call * 3 + mir_array * 4 + mir_field
    score = active_count * shape_weight
    risk_kind = risk_kind_for(report)
    reason = f"{active_source}:active={active_count};shape_weight={shape_weight}"
    return OwnerRank(
        source_method=source_method,
        selected_method=report.get("selected_method", ""),
        active_count=active_count,
        mir_call_count=mir_call,
        mir_field_count=mir_field,
        mir_array_count=mir_array,
        score=score,
        risk_kind=risk_kind,
        reason=reason,
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measurement-report", type=Path, required=True)
    parser.add_argument("--join-report", type=Path, action="append", required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    measurement = read_kv(args.measurement_report)
    require(
        measurement,
        "output_contract",
        "hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0",
        "measurement",
    )

    reports: list[dict[str, str]] = []
    for path in args.join_report:
        report = read_kv(path)
        require(report, "output_contract", "hako-source-mir-shape-join-v1", str(path))
        reports.append(report)
    if not reports:
        raise SystemExit("--join-report must be passed at least once")

    ranks = sorted((rank_for(report, measurement) for report in reports), key=lambda item: item.score, reverse=True)
    active = [rank for rank in ranks if rank.active_count > 0]
    inactive = [rank for rank in ranks if rank.active_count == 0]
    selected = active[0] if active else ranks[0]

    if (
        selected.source_method == "objectLifecycleSmallAlloc"
        and as_int(measurement, "select_page_single_fallback_count") == 0
    ):
        next_keeper = "small_alloc_direct_single_page_select_fast_path"
        next_kind = "box_count"
        selected_next_kind = "keeper"
        confidence = "medium"
        selected_reason = "top_active_owner_can_bypass_selectPage_wrapper_for_single_page_workload"
    else:
        next_keeper = "none"
        next_kind = "none"
        selected_next_kind = "mir_lowering"
        confidence = "low"
        selected_reason = "no_narrow_keeper_selected_from_active_rank"

    lines = [
        "output_contract=hako-mimalloc-hot-owner-rank-v0",
        "input_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0",
        f"method_count={len(ranks)}",
        f"active_method_count={len(active)}",
        f"inactive_surface_count={len(inactive)}",
        f"rejected_keeper={REJECTED_KEEPER}",
        "rejected_keeper_reason=measured_regression_row102",
    ]
    for idx, rank in enumerate(ranks[:6]):
        prefix = f"active_method_rank_{idx}"
        lines.extend(
            [
                f"{prefix}={rank.selected_method}",
                f"{prefix}_source_method={rank.source_method}",
                f"{prefix}_active_count={rank.active_count}",
                f"{prefix}_mir_call_count={rank.mir_call_count}",
                f"{prefix}_mir_field_access_count={rank.mir_field_count}",
                f"{prefix}_mir_array_access_count={rank.mir_array_count}",
                f"{prefix}_score={rank.score}",
                f"{prefix}_risk_kind={rank.risk_kind}",
                f"{prefix}_reason={rank.reason}",
            ]
        )
    lines.extend(
        [
            f"selected_owner={selected.selected_method}",
            f"selected_source_method={selected.source_method}",
            f"selected_risk_kind={selected.risk_kind}",
            f"selected_reason={selected_reason}",
            f"selected_next_kind={selected_next_kind}",
            f"next_keeper={next_keeper}",
            f"next_keeper_kind={next_kind}",
            f"confidence={confidence}",
            f"next_row={NEXT_ROW}",
            "winner_claim=0",
            "replacement_active=0",
            "summary=ok",
        ]
    )

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
