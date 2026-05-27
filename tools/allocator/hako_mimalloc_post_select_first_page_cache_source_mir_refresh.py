#!/usr/bin/env python3
"""Refresh keeper selection after the select first-page cache measurement."""

from __future__ import annotations

import argparse
from pathlib import Path


NEXT_ROW = "HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001"


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


def find_report(reports: list[dict[str, str]], source_method: str) -> dict[str, str]:
    for report in reports:
        if report.get("source_target_method") == source_method:
            return report
    return {}


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
        "hako-mimalloc-post-select-first-page-cache-keeper-measurement-v0",
        "measurement",
    )

    reports = []
    for path in args.join_report:
        report = read_kv(path)
        require(report, "output_contract", "hako-source-mir-shape-join-v1", str(path))
        reports.append(report)

    select_fast = as_int(measurement, "select_page_single_fast_path_count")
    select_fallback = as_int(measurement, "select_page_single_fallback_count")
    release_fast = as_int(measurement, "release_known_page_fast_path_count")
    release_fallback = as_int(measurement, "release_known_page_fallback_count")

    selected = find_report(reports, "selectSinglePageFastPath")
    if not selected:
        selected = find_report(reports, "objectLifecycleSmallAlloc")
    if not selected:
        selected = reports[0] if reports else {}

    if (
        select_fast > 0
        and select_fallback == 0
        and selected.get("source_target_method") == "selectSinglePageFastPath"
        and selected.get("confirmed_risk_kind") == "field_access"
    ):
        next_keeper = "select_single_page_active_field_fast_path"
        next_kind = "box_count"
        selected_reason = "active_single_page_select_field_method_surface"
    else:
        next_keeper = "none"
        next_kind = "none"
        selected_reason = "none"

    inactive_select_loop_risk = 1 if select_fast > 0 and select_fallback == 0 else 0
    inactive_release_lookup_risk = 1 if release_fast > 0 and release_fallback == 0 else 0
    confirmed_count = sum(as_int(report, "source_risk_confirmed_in_mir") for report in reports)

    lines = [
        "output_contract=hako-mimalloc-post-select-first-page-cache-source-mir-refresh-v0",
        "input_contract=hako-mimalloc-post-select-first-page-cache-keeper-measurement-v0",
        f"method_count={len(reports)}",
        f"confirmed_source_mir_risk_count={confirmed_count}",
        f"select_page_single_fast_path_count={select_fast}",
        f"select_page_single_fallback_count={select_fallback}",
        f"release_known_page_fast_path_count={release_fast}",
        f"release_known_page_fallback_count={release_fallback}",
        f"inactive_select_loop_risk={inactive_select_loop_risk}",
        f"inactive_release_lookup_risk={inactive_release_lookup_risk}",
        f"selected_reason={selected_reason}",
        f"selected_method={selected.get('selected_method', '')}",
        f"selected_source_method={selected.get('source_target_method', '')}",
        f"selected_hot_context={selected.get('method_hot_context', '')}",
        f"selected_risk_kind={selected.get('confirmed_risk_kind', 'none')}",
        f"selected_mir_call_count={selected.get('mir_call_count', '0')}",
        f"selected_mir_field_access_count={selected.get('mir_field_access_count', '0')}",
        f"next_keeper={next_keeper}",
        f"next_keeper_kind={next_kind}",
        f"next_row={NEXT_ROW}",
        "winner_claim=0",
        "replacement_active=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
