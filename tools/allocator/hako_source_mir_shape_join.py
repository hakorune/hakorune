#!/usr/bin/env python3
"""Join hako_check source perf-surface and MIR method shape reports."""

from __future__ import annotations

import argparse
from pathlib import Path


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


def choose_diagnostic(source: dict[str, str], mir: dict[str, str]) -> tuple[int, str, str]:
    source_loop_array = (
        as_int(source, "loop_array_get_count") + as_int(source, "loop_array_length_count")
    )
    mir_array = as_int(mir, "array_get_call_count") + as_int(mir, "array_length_call_count")
    if source_loop_array > 0 and mir_array > 0:
        return (
            1,
            "array_access",
            "keeper_candidate_from_confirmed_source_mir_array_access",
        )

    source_loop_field = as_int(source, "loop_field_get_count") + as_int(
        source, "loop_field_set_count"
    )
    mir_field = as_int(mir, "field_get_count") + as_int(mir, "field_set_count")
    if source_loop_field > 0 and mir_field > 0:
        return (
            1,
            "field_access",
            "keeper_candidate_from_confirmed_source_mir_field_access",
        )

    source_loop_call = as_int(source, "loop_method_call_count")
    mir_call = as_int(mir, "call_count")
    if source_loop_call > 0 and mir_call > 0:
        return (
            1,
            "method_call",
            "keeper_candidate_from_confirmed_source_mir_method_call",
        )

    return (0, "none", "mir_shape_not_confirmed_refresh_source_surface")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-report", type=Path, required=True)
    parser.add_argument("--mir-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = read_kv(args.source_report)
    mir = read_kv(args.mir_report)
    require(source, "output_contract", "hako-check-perf-surface-v1", "source")
    require(mir, "output_contract", "hako-mir-method-shape-v0", "mir")
    source_method = source.get("target_method", "")
    selected_method = mir.get("selected_method", source_method)
    confirmed, confirmed_kind, next_diagnostic = choose_diagnostic(source, mir)

    lines = [
        "output_contract=hako-source-mir-shape-join-v0",
        "source_contract=hako-check-perf-surface-v1",
        "mir_contract=hako-mir-method-shape-v0",
        f"selected_method={selected_method}",
        f"source_target_method={source_method}",
        f"source_loop_array_access_count={as_int(source, 'loop_array_get_count') + as_int(source, 'loop_array_length_count')}",
        f"mir_array_access_count={as_int(mir, 'array_get_call_count') + as_int(mir, 'array_length_call_count')}",
        f"source_loop_field_access_count={as_int(source, 'loop_field_get_count') + as_int(source, 'loop_field_set_count')}",
        f"mir_field_access_count={as_int(mir, 'field_get_count') + as_int(mir, 'field_set_count')}",
        f"source_loop_method_call_count={as_int(source, 'loop_method_call_count')}",
        f"mir_call_count={as_int(mir, 'call_count')}",
        f"source_risk_confirmed_in_mir={confirmed}",
        f"confirmed_risk_kind={confirmed_kind}",
        f"next_diagnostic={next_diagnostic}",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
