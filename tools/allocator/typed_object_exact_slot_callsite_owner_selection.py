#!/usr/bin/env python3
"""Select one next owner from exact-slot callsite attribution evidence."""

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


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str) -> str:
    value = values.get(key)
    if value is None:
        raise SystemExit(f"missing {key}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--attribution-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    values = read_kv(args.attribution_report)
    require(values, "output_contract", "typed-object-exact-slot-callsite-attribution-v0")
    require(values, "summary", "ok")

    dominant_family = require_key(values, "dominant_family")
    if dominant_family == "object_lifecycle_facade":
        selected_owner = "object_lifecycle_facade_exact_slot_field_inventory"
        next_diagnostic = "object_lifecycle_facade_exact_slot_field_inventory"
        selected_reason = "dominant_family_object_lifecycle_facade"
    elif dominant_family == "page_model_hotpath":
        selected_owner = "page_model_exact_slot_followon_inventory"
        next_diagnostic = "page_model_exact_slot_followon_inventory"
        selected_reason = "dominant_family_page_model_hotpath"
    else:
        selected_owner = "exact_slot_family_specific_inventory"
        next_diagnostic = "exact_slot_family_specific_inventory"
        selected_reason = f"dominant_family_{dominant_family}"

    lines = [
        "output_contract=typed-object-exact-slot-callsite-owner-selection-v0",
        "input_contract=typed-object-exact-slot-callsite-attribution-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"dominant_family={dominant_family}",
        f"dominant_family_pct={require_key(values, 'dominant_family_pct')}",
        f"top_callsite_symbol={require_key(values, 'top_callsite_symbol')}",
        f"top_callsite_pct={require_key(values, 'top_callsite_pct')}",
        f"selected_owner={selected_owner}",
        f"selected_reason={selected_reason}",
        f"next_diagnostic={next_diagnostic}",
        "rejected_owner=page_model_followon_keeper",
        "rejected_reason=page_model_recently_optimized_and_not_dominant_family",
        "rejected_owner_1=generic_typed_field_residence_retry",
        "rejected_reason_1=no_new_positive_net_helper_delta_evidence",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
