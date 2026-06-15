#!/usr/bin/env python3
"""Inventory runtime/object boundaries after mimalloc body timing precision."""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


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


def repo_text(rel: str) -> str:
    path = ROOT / rel
    if not path.is_file():
        raise SystemExit(f"missing repo file: {rel}")
    return path.read_text(encoding="utf-8", errors="replace")


def contains(rel: str, needle: str) -> bool:
    return needle in repo_text(rel)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--precision-card", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    precision = read_kv(args.precision_card)
    require(precision, "output_contract", "hako-mimalloc-body-timing-precision-v0", "precision")
    require(precision, "source_evidence", "296x-701", "precision")
    require(precision, "timer_family_matched", "0", "precision")
    require(precision, "measurement_boundary_confidence", "low", "precision")
    require(precision, "summary", "ok", "precision")

    box_callable_visible = contains("src/box_callable/registry.rs", "pub struct BoxCallableRegistry")
    route_plan_visible = contains("src/box_callable/route_plan.rs", "pub enum MethodCallRoutePlan")
    object_handle_visible = contains("src/runtime/object_identity.rs", "pub struct ObjectHandle")
    host_handle_arc_visible = contains("src/runtime/host_handles.rs", "StableBox(Arc<dyn NyashBox>)")
    arc_carrier_visible = contains("src/runtime/box_object_model.rs", "ArcDynNyashBox")
    env_timer_visible = contains(
        "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako",
        "env.now_ms()",
    )

    lines = [
        "output_contract=hako-mimalloc-runtime-boundary-inventory-v0",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "source_evidence=296x-702",
        f"body_elapsed_ratio_raw={require_key(precision, 'body_elapsed_ratio_raw', 'precision')}",
        "measurement_boundary_confidence=low",
        f"box_method_boundary_visible={1 if box_callable_visible and route_plan_visible else 0}",
        "routeplan_slow_dynamic_hit_count=unknown",
        f"object_refcount_boundary_visible={1 if arc_carrier_visible else 0}",
        f"host_handle_boundary_visible={1 if object_handle_visible and host_handle_arc_visible else 0}",
        f"runtime_helper_call_boundary_visible={1 if env_timer_visible else 0}",
        f"generated_runtime_boundary_visible={1 if env_timer_visible else 0}",
        "selected_owner=none",
        "selected_owner_confidence=low",
        "owner_reason=mixed_runtime_boundary_visible_but_measurement_boundary_low_confidence",
        "closed_world_routeplan_allowed=0",
        "exact_aot_specialization_selected=0",
        "implementation_started=0",
        "compiler_lowering_changed=0",
        "runtime_object_changed=0",
        "product_default_changed=0",
        "startup_lane_reopened=0",
        "source_hako_changed=0",
        "winner_claim=0",
        "next_task=body_timer_alignment_or_boundary_probe",
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
