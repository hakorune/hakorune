#!/usr/bin/env python3
"""Build a report-only shadow for local known receiver direct-call candidates."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    out: dict[str, str] = {}
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        out[key] = value
    return out


def as_int(values: dict[str, str], key: str) -> int:
    try:
        return int(values.get(key, "0"))
    except ValueError:
        return 0


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--probe-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    probe = read_kv(args.probe_report)
    require(probe, "output_contract", "hako-local-page-receiver-candidate-probe-v0")
    require(probe, "summary", "ok")

    acquire_count = as_int(probe, "page_acquire_usize_call_count")
    reuse_count = as_int(probe, "page_reuse_call_count")
    pre_publication_count = as_int(probe, "page_pre_publication_call_count")

    guard_satisfied = int(
        probe.get("page_from_queue_selection") == "1"
        and probe.get("page_type_known") == "1"
        and as_int(probe, "page_method_surface_known_count") >= 2
        and as_int(probe, "page_call_after_publication_count") == 0
        and as_int(probe, "page_dynamic_api_required_count") == 0
        and as_int(probe, "page_plugin_or_extern_escape_count") == 0
        and as_int(probe, "page_task_boundary_escape_count") == 0
        and as_int(probe, "page_storage_direct_required") == 0
        and as_int(probe, "page_hosthandle_bypass_required") == 0
    )

    candidate_count = pre_publication_count if guard_satisfied else 0

    lines = [
        "output_contract=hako-local-known-receiver-direct-call-shadow-v0",
        "source_evidence=296x-818,296x-817",
        "target_front=object_lifecycle_body",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "shadow_kind=report_only",
        "selected_shape=local_known_receiver_direct_call",
        "first_target_receiver=page",
        f"shadow_guard_satisfied={guard_satisfied}",
        f"shadow_direct_call_candidate_count={candidate_count}",
        f"shadow_page_acquire_usize_count={acquire_count if guard_satisfied else 0}",
        f"shadow_page_reuse_count={reuse_count if guard_satisfied else 0}",
        "shadow_route_kind=pre_publication_known_receiver_method_call",
        "shadow_rule_source=objectplan_pre_publication_plus_known_receiver_surface",
        "receiver_name_rule_enabled=0",
        "method_name_rule_enabled=0",
        "helper_symbol_inference_enabled=0",
        "storage_direct_count=0",
        "hosthandle_bypass_count=0",
        "arc_retirement_count=0",
        "routeplan_backend_consumable_proof_required_before_lowering=1",
        "shadow_plan_behavior_changed=0",
        "product_default_changed=0",
        "pilot_implementation_candidate=1" if candidate_count == 3 else "pilot_implementation_candidate=0",
        "summary=ok" if candidate_count == 3 else "summary=blocked",
    ]
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
