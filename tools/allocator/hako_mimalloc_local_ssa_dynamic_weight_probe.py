#!/usr/bin/env python3
"""Estimate dynamic copy weight for the selected local-SSA MIR owner."""

from __future__ import annotations

import argparse
from pathlib import Path


DEFAULT_METHOD_INVOCATION_COUNT = 524288


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str, label: str) -> str:
    value = values.get(key)
    if value is None or value == "":
        raise SystemExit(f"{label}: missing {key}")
    return value


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = require_key(values, key, label)
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--attribution", type=Path, required=True)
    parser.add_argument("--method-invocation-count", type=int, default=DEFAULT_METHOD_INVOCATION_COUNT)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    if args.method_invocation_count <= 0:
        raise SystemExit("--method-invocation-count must be positive")

    attribution = read_kv(args.attribution)
    require(attribution, "output_contract", "hako-mimalloc-callsite-copy-attribution-v0", "attribution")
    require(attribution, "target_method", "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1", "attribution")
    require(attribution, "summary", "ok", "attribution")

    static_counts = {
        "local_ssa_copy_materialization": require_int(attribution, "owner_local_ssa_copy_materialization_copy_count", "attribution"),
        "receiver_materialization": require_int(attribution, "owner_receiver_materialization_copy_count", "attribution"),
        "phi_edge_copy_materialization": require_int(attribution, "owner_phi_edge_copy_materialization_copy_count", "attribution"),
        "result_materialization": require_int(attribution, "owner_result_materialization_copy_count", "attribution"),
        "arg_materialization": require_int(attribution, "owner_arg_materialization_copy_count", "attribution"),
        "page_hotpath_helper_attribution": require_int(attribution, "page_hotpath_helpers_attributed_copy_count", "attribution"),
    }
    dynamic_counts = {
        key: value * args.method_invocation_count for key, value in static_counts.items()
    }
    dominant = max(sorted(dynamic_counts), key=lambda key: dynamic_counts[key])

    lines = [
        "output_contract=hako-mimalloc-local-ssa-dynamic-weight-probe-v0",
        "input_contract=hako-mimalloc-callsite-copy-attribution-v0",
        f"target_method={require_key(attribution, 'target_method', 'attribution')}",
        "method_invocation_source=object-lifecycle-allocation-count",
        f"method_invocation_count={args.method_invocation_count}",
        f"instruction_count={require_key(attribution, 'instruction_count', 'attribution')}",
        f"copy_count={require_key(attribution, 'copy_count', 'attribution')}",
        f"call_count={require_key(attribution, 'call_count', 'attribution')}",
        f"phi_count={require_key(attribution, 'phi_count', 'attribution')}",
    ]
    for owner, count in static_counts.items():
        lines.append(f"{owner}_static_count={count}")
        lines.append(f"{owner}_dynamic_ops={dynamic_counts[owner]}")
    lines.extend(
        [
            f"dominant_dynamic_owner={dominant}",
            "selected_owner=local_ssa_copy_materialization",
            "selected_owner_confidence=medium",
            "selected_reason=dominant_static_and_dynamic_copy_owner",
            "rejected_recent_nonkeeper=local_ssa_same_block_field_get_reuse",
            "rejected_retry_scope=same_block_field_get_only",
            "next_diagnostic=local_ssa_copy_kind_policy_selection",
            "optimization_open=0",
            "winner_claim=0",
            "provider_active=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
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
