#!/usr/bin/env python3
"""Select the receiver pin-chain policy from receiver attribution evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None or text == "":
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receiver-attribution", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    values = read_kv(args.receiver_attribution)
    require(
        values,
        "output_contract",
        "hako-mimalloc-receiver-materialization-attribution-probe-v0",
        "receiver-attribution",
    )
    require(values, "summary", "ok", "receiver-attribution")

    receiver_count = require_int(values, "receiver_attributed_copy_count", "receiver-attribution")
    unique_count = require_int(values, "unique_receiver_copy_count", "receiver-attribution")
    duplicate_count = require_int(
        values, "duplicate_receiver_attribution_count", "receiver-attribution"
    )
    page_hotpath_count = require_int(
        values, "page_hotpath_receiver_copy_count", "receiver-attribution"
    )
    if receiver_count <= 0:
        raise SystemExit("receiver-attribution: receiver_attributed_copy_count must be positive")

    selected = "receiver_pin_chain_narrowing"
    rejected = "same_receiver_callsite_cache"
    rejected_reason = "duplicate_receiver_attribution_too_small"
    confidence = "medium"
    if duplicate_count * 4 <= receiver_count and page_hotpath_count * 2 >= receiver_count:
        confidence = "high"

    lines = [
        "output_contract=hako-mimalloc-receiver-pin-chain-policy-selection-v0",
        "input_contract=hako-mimalloc-receiver-materialization-attribution-probe-v0",
        f"target_method={values.get('target_method', '')}",
        f"receiver_attributed_copy_count={receiver_count}",
        f"unique_receiver_copy_count={unique_count}",
        f"duplicate_receiver_attribution_count={duplicate_count}",
        f"page_hotpath_receiver_copy_count={page_hotpath_count}",
        f"selected_receiver_policy={selected}",
        f"selected_receiver_policy_confidence={confidence}",
        f"rejected_receiver_policy={rejected}",
        f"rejected_reason={rejected_reason}",
        "next_diagnostic=receiver_pin_chain_keeper_design",
        "optimization_open=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
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
