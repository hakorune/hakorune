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


def source_method_prefix(source: dict[str, str], source_method: str) -> str:
    for key, value in source.items():
        if key.startswith("target_method_") and key.count("_") == 2 and value == source_method:
            return f"{key}_"
    return ""


def source_count(source: dict[str, str], source_method: str, suffix: str, fallback: str) -> int:
    prefix = source_method_prefix(source, source_method)
    if prefix:
        return as_int(source, f"{prefix}{suffix}")
    return as_int(source, fallback)


def source_counts(source: dict[str, str], source_method: str) -> dict[str, int]:
    loop_array = as_int(source, "loop_array_get_count") + as_int(source, "loop_array_length_count")
    loop_field = as_int(source, "loop_field_get_count") + as_int(source, "loop_field_set_count")
    return {
        "loop_array": loop_array,
        "loop_field": loop_field,
        "loop_call": as_int(source, "loop_method_call_count"),
        "method_call": source_count(
            source, source_method, "method_call_count", "method_call_count"
        ),
        "field_get": source_count(source, source_method, "field_get_count", "field_get_count"),
        "field_set": source_count(source, source_method, "field_set_count", "field_set_count"),
        "array": source_count(source, source_method, "array_access_count", "array_access_count"),
    }


def mir_counts(mir: dict[str, str]) -> dict[str, int]:
    return {
        "array": as_int(mir, "array_get_call_count") + as_int(mir, "array_length_call_count"),
        "field": as_int(mir, "field_get_count") + as_int(mir, "field_set_count"),
        "call": as_int(mir, "call_count"),
    }


def choose_hot_context(
    requested: str, source_shape: dict[str, int], mir_shape: dict[str, int]
) -> str:
    if requested != "auto":
        return requested
    if (
        source_shape["loop_array"] > 0
        or source_shape["loop_field"] > 0
        or source_shape["loop_call"] > 0
    ):
        return "direct_loop"
    if (
        (source_shape["array"] > 0 and mir_shape["array"] > 0)
        or (
            source_shape["field_get"] + source_shape["field_set"] > 0
            and mir_shape["field"] > 0
        )
        or (source_shape["method_call"] > 0 and mir_shape["call"] > 0)
    ):
        return "caller_repeated"
    return "unknown"


def choose_diagnostic(
    source_shape: dict[str, int], mir_shape: dict[str, int], hot_context: str
) -> tuple[int, str, str]:
    if hot_context == "direct_loop":
        if source_shape["loop_array"] > 0 and mir_shape["array"] > 0:
            return (
                1,
                "array_access",
                "keeper_candidate_from_confirmed_source_mir_array_access",
            )
        if source_shape["loop_field"] > 0 and mir_shape["field"] > 0:
            return (
                1,
                "field_access",
                "keeper_candidate_from_confirmed_source_mir_field_access",
            )
        if source_shape["loop_call"] > 0 and mir_shape["call"] > 0:
            return (
                1,
                "method_call",
                "keeper_candidate_from_confirmed_source_mir_method_call",
            )

    if hot_context == "caller_repeated":
        if source_shape["array"] > 0 and mir_shape["array"] > 0:
            return (
                1,
                "array_access",
                "keeper_candidate_from_confirmed_caller_repeated_array_access",
            )
        if source_shape["field_get"] + source_shape["field_set"] > 0 and mir_shape["field"] > 0:
            return (
                1,
                "field_access",
                "keeper_candidate_from_confirmed_caller_repeated_field_access",
            )
        if source_shape["method_call"] > 0 and mir_shape["call"] > 0:
            return (
                1,
                "method_call",
                "keeper_candidate_from_confirmed_caller_repeated_method_call",
            )

    return (0, "none", "mir_shape_not_confirmed_refresh_source_surface")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-report", type=Path, required=True)
    parser.add_argument("--mir-report", type=Path, required=True)
    parser.add_argument("--contract-version", choices=("v0", "v1"), default="v1")
    parser.add_argument(
        "--method-hot-context",
        choices=("auto", "direct_loop", "caller_repeated", "unknown"),
        default="auto",
    )
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    source = read_kv(args.source_report)
    mir = read_kv(args.mir_report)
    require(source, "output_contract", "hako-check-perf-surface-v1", "source")
    require(mir, "output_contract", "hako-mir-method-shape-v0", "mir")
    source_method = source.get("target_method", "")
    selected_method = mir.get("selected_method", source_method)
    source_shape = source_counts(source, source_method)
    mir_shape = mir_counts(mir)
    hot_context = choose_hot_context(args.method_hot_context, source_shape, mir_shape)
    confirmed, confirmed_kind, next_diagnostic = choose_diagnostic(
        source_shape, mir_shape, hot_context
    )

    lines = [
        f"output_contract=hako-source-mir-shape-join-{args.contract_version}",
        "source_contract=hako-check-perf-surface-v1",
        "mir_contract=hako-mir-method-shape-v0",
        f"selected_method={selected_method}",
        f"source_target_method={source_method}",
        f"source_loop_array_access_count={source_shape['loop_array']}",
        f"mir_array_access_count={mir_shape['array']}",
        f"source_loop_field_access_count={source_shape['loop_field']}",
        f"mir_field_access_count={mir_shape['field']}",
        f"source_loop_method_call_count={source_shape['loop_call']}",
        f"mir_call_count={mir_shape['call']}",
    ]
    if args.contract_version == "v1":
        lines.extend(
            [
                f"method_hot_context={hot_context}",
                f"source_method_call_count={source_shape['method_call']}",
                f"source_field_get_count={source_shape['field_get']}",
                f"source_field_set_count={source_shape['field_set']}",
                f"source_array_access_count={source_shape['array']}",
            ]
        )
    lines.extend(
        [
            f"source_risk_confirmed_in_mir={confirmed}",
            f"confirmed_risk_kind={confirmed_kind}",
            f"next_diagnostic={next_diagnostic}",
            "summary=ok",
        ]
    )
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
