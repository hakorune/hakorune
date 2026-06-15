#!/usr/bin/env python3
"""Classify the local known-receiver direct-call pilot seam."""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SOURCE_PROBE = ROOT / "tools/allocator/hako_local_page_receiver_candidate_probe.py"
SHADOW_PROBE = ROOT / "tools/allocator/hako_local_known_receiver_direct_call_shadow.py"
C_SHIM = ROOT / "lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc"
LOWERING_PLAN = ROOT / "lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc"


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


def contains(path: Path, needle: str) -> int:
    return int(needle in path.read_text(encoding="utf-8", errors="replace"))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate-report", type=Path, required=True)
    parser.add_argument("--shadow-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    candidate = read_kv(args.candidate_report)
    shadow = read_kv(args.shadow_report)
    require(candidate, "output_contract", "hako-local-page-receiver-candidate-probe-v0", "candidate")
    require(candidate, "page_pre_publication_call_count", "3", "candidate")
    require(candidate, "page_call_after_publication_count", "0", "candidate")
    require(candidate, "page_hosthandle_bypass_required", "0", "candidate")
    require(shadow, "output_contract", "hako-local-known-receiver-direct-call-shadow-v0", "shadow")
    require(shadow, "shadow_direct_call_candidate_count", "3", "shadow")
    require(shadow, "shadow_guard_satisfied", "1", "shadow")
    require(shadow, "storage_direct_count", "0", "shadow")
    require(shadow, "hosthandle_bypass_count", "0", "shadow")
    require(shadow, "arc_retirement_count", "0", "shadow")

    c_shim_route_consumer = contains(C_SHIM, "emit_user_box_method_lowering_plan_mir_call")
    c_shim_reads_route_source = contains(C_SHIM, 'strcmp(source, "user_box_method_routes")')
    c_shim_emits_target_symbol = contains(C_SHIM, "view.target_symbol, ab")
    c_shim_trace_consumer = contains(C_SHIM, "mir_call_user_box_method_emit")
    routeplan_direct_target = contains(
        LOWERING_PLAN,
        "lowering_plan_user_box_method_view_has_direct_target",
    )
    routeplan_same_module_required = contains(
        LOWERING_PLAN,
        "lowering_plan_user_box_method_view_requires_same_module_function_definition",
    )

    generic_seam_ready = int(
        c_shim_route_consumer
        and c_shim_reads_route_source
        and c_shim_emits_target_symbol
        and routeplan_direct_target
        and routeplan_same_module_required
    )

    lines = [
        "output_contract=hako-local-known-receiver-direct-call-pilot-v0",
        "source_evidence=296x-819,296x-818,296x-817",
        "target_front=object_lifecycle_body",
        "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "selected_shape=local_known_receiver_direct_call",
        "pilot_status=already_satisfied_existing_generic_route",
        "first_target_receiver=page",
        "first_target_call_count=3",
        "first_target_methods=acquire_usize,reuse",
        f"candidate_report={args.candidate_report.as_posix()}",
        f"shadow_report={args.shadow_report.as_posix()}",
        f"generic_routeplan_backend_seam_ready={generic_seam_ready}",
        f"c_shim_user_box_method_route_consumer={c_shim_route_consumer}",
        f"c_shim_reads_user_box_method_routes={c_shim_reads_route_source}",
        f"c_shim_emits_target_symbol_call={c_shim_emits_target_symbol}",
        f"c_shim_trace_consumer_present={c_shim_trace_consumer}",
        f"routeplan_direct_target_predicate_present={routeplan_direct_target}",
        f"routeplan_same_module_definition_required={routeplan_same_module_required}",
        "objectplan_pre_publication_shadow_used=1",
        "routeplan_backend_consumable_proof_used=1",
        "new_backend_lowering_code_added=0",
        "page_specific_rule_enabled=0",
        "method_name_special_case_enabled=0",
        "helper_symbol_inference_enabled=0",
        "storage_direct_enabled=0",
        "hosthandle_bypass_enabled=0",
        "arc_retirement_enabled=0",
        "product_default_changed=0",
        "measurement_required=1",
        "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001",
        "summary=ok" if generic_seam_ready else "summary=blocked",
    ]
    report = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    else:
        print(report, end="")
    return 0 if generic_seam_ready else 1


if __name__ == "__main__":
    raise SystemExit(main())
