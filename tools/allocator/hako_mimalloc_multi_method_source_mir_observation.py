#!/usr/bin/env python3
"""Select the next .hako mimalloc keeper from multi-method source/MIR reports."""

from __future__ import annotations

import argparse
from pathlib import Path


NEXT_ROW = "HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER-296X-001"


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


def keeper_for(report: dict[str, str]) -> tuple[str, str]:
    method = report.get("source_target_method", "")
    hot_context = report.get("method_hot_context", "")
    risk_kind = report.get("confirmed_risk_kind", "")
    if (
        method == "objectLifecycleSmallAlloc"
        and hot_context == "caller_repeated"
        and risk_kind == "array_access"
    ):
        return ("small_alloc_selected_page_cache_reuse", "box_count")
    if (
        method == "objectLifecycleReleaseBlock"
        and hot_context == "caller_repeated"
        and risk_kind == "array_access"
    ):
        return ("release_known_page_object_cache", "box_shape")
    if method == "selectPage" and hot_context == "direct_loop" and risk_kind == "array_access":
        return ("select_page_multi_page_loop_reduction", "box_count")
    return ("none", "none")


def score(report: dict[str, str]) -> tuple[int, int, int, int]:
    keeper, keeper_kind = keeper_for(report)
    if keeper == "small_alloc_selected_page_cache_reuse":
        keeper_rank = 30
    elif keeper_kind == "box_count":
        keeper_rank = 20
    elif keeper != "none":
        keeper_rank = 10
    else:
        keeper_rank = 0
    confirmed_rank = 100 if as_int(report, "source_risk_confirmed_in_mir") == 1 else 0
    array_rank = 10 if report.get("confirmed_risk_kind") == "array_access" else 0
    source_count = as_int(report, "source_array_access_count") + as_int(
        report, "source_loop_array_access_count"
    )
    return (confirmed_rank, keeper_rank, array_rank, source_count)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--join-report", type=Path, action="append", required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    reports: list[dict[str, str]] = []
    for path in args.join_report:
        report = read_kv(path)
        require(report, "output_contract", "hako-source-mir-shape-join-v1", str(path))
        reports.append(report)
    if not reports:
        raise SystemExit("--join-report must be passed at least once")

    confirmed = [report for report in reports if as_int(report, "source_risk_confirmed_in_mir") == 1]
    selected = max(reports, key=score)
    selected_keeper, selected_kind = keeper_for(selected)

    lines = [
        "output_contract=hako-mimalloc-multi-method-source-mir-observation-v0",
        "input_contract=hako-source-mir-shape-join-v1",
        f"method_count={len(reports)}",
        f"confirmed_source_mir_risk_count={len(confirmed)}",
        f"selected_method={selected.get('selected_method', '')}",
        f"selected_source_method={selected.get('source_target_method', '')}",
        f"selected_hot_context={selected.get('method_hot_context', '')}",
        f"selected_risk_kind={selected.get('confirmed_risk_kind', 'none')}",
        f"next_keeper={selected_keeper}",
        f"next_keeper_kind={selected_kind}",
        f"next_row={NEXT_ROW}",
        "winner_claim=0",
        "replacement_active=0",
        "summary=ok",
    ]
    report_text = "\n".join(lines) + "\n"
    if args.out is None:
        print(report_text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report_text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
