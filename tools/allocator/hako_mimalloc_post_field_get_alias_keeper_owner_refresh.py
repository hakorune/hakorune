#!/usr/bin/env python3
"""Select the next object-lifecycle owner after the field_get alias keeper."""

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
        raise SystemExit(f"{label}: {key} must be integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--taxonomy", type=Path, required=True)
    parser.add_argument("--attribution", type=Path, required=True)
    parser.add_argument("--dynamic-weight", type=Path, required=True)
    parser.add_argument("--position", type=Path, required=True)
    parser.add_argument("--post", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    taxonomy = read_kv(args.taxonomy)
    attribution = read_kv(args.attribution)
    dynamic = read_kv(args.dynamic_weight)
    position = read_kv(args.position)
    post = read_kv(args.post)

    require(taxonomy, "output_contract", "hako-mimalloc-object-lifecycle-body-timing-gap-taxonomy-v0", "taxonomy")
    require(taxonomy, "gap_owner", "compiler_lowering", "taxonomy")
    require(attribution, "output_contract", "hako-mimalloc-callsite-copy-attribution-v0", "attribution")
    require(dynamic, "output_contract", "hako-mimalloc-local-ssa-dynamic-weight-probe-v0", "dynamic")
    require(position, "output_contract", "hako-mimalloc-local-ssa-copy-position-probe-v0", "position")
    require(post, "output_contract", "hako-mimalloc-field-get-alias-keeper-post-probe-v0", "post")
    require(post, "forwarding_candidate_copy_count", "0", "post")

    dominant_copy_owner = attribution.get("dominant_copy_owner", "unknown")
    dominant_dynamic_owner = dynamic.get("dominant_dynamic_owner", "unknown")
    dominant_position = position.get("dominant_position", "unknown")
    dominant_route_role = position.get("dominant_route_carrier_role", "unknown")
    expression_count = require_int(post, "expression_materialization_copy_count", "post")
    copy_count = require_int(post, "copy_count", "post")

    selected_next_owner = "post_keeper_owner_unclear"
    confidence = "low"
    next_task = "post_keeper_owner_repeat"
    if (
        dominant_copy_owner == "result_materialization"
        and dominant_dynamic_owner == "page_hotpath_helper_attribution"
    ):
        selected_next_owner = "page_hotpath_helper_result_materialization_copy_chain"
        confidence = "medium"
        next_task = "page_hotpath_helper_result_materialization_inventory"

    lines = [
        "output_contract=hako-mimalloc-post-field-get-alias-keeper-owner-refresh-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        f"hako_body_elapsed_ns={taxonomy.get('hako_body_elapsed_ns', '0')}",
        f"c_body_elapsed_ns={taxonomy.get('c_body_elapsed_ns', '0')}",
        f"body_elapsed_ratio={taxonomy.get('body_elapsed_ratio', '0')}",
        f"gap_owner={taxonomy.get('gap_owner', 'unknown')}",
        f"copy_count={copy_count}",
        f"expression_materialization_copy_count={expression_count}",
        f"dominant_copy_owner={dominant_copy_owner}",
        f"dominant_dynamic_owner={dominant_dynamic_owner}",
        f"dominant_position={dominant_position}",
        f"dominant_route_carrier_role={dominant_route_role}",
        f"page_hotpath_helpers_call_count={attribution.get('page_hotpath_helpers_call_count', '0')}",
        f"page_hotpath_helpers_attributed_copy_count={attribution.get('page_hotpath_helpers_attributed_copy_count', '0')}",
        f"result_materialization_copy_count={attribution.get('owner_result_materialization_copy_count', '0')}",
        f"selected_next_owner={selected_next_owner}",
        f"selected_owner_confidence={confidence}",
        f"next_task={next_task}",
        "implementation_started=0",
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
