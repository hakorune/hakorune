#!/usr/bin/env python3
"""Build a DirectSlotLease selected-method inventory from prior residence evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


DEFAULT_METHOD = "HakoAllocPageModel.acquire_usize/1"


def parse_key_values(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key.strip()] = value.strip()
    return values


def require_int(values: dict[str, str], key: str, source: Path) -> int:
    try:
        return int(values[key])
    except KeyError as exc:
        raise SystemExit(f"missing {key} in {source}") from exc
    except ValueError as exc:
        raise SystemExit(f"non-integer {key}={values.get(key)!r} in {source}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--resident-plan", type=Path, required=True)
    parser.add_argument("--feasibility-closeout", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    plan = parse_key_values(args.resident_plan)
    closeout = parse_key_values(args.feasibility_closeout)

    candidate_gets = require_int(plan, "eligible_field_get_count", args.resident_plan)
    candidate_sets = require_int(plan, "eligible_field_set_count", args.resident_plan)
    resident_fields = require_int(plan, "resident_field_key_count", args.resident_plan)
    prior_inserted_loads = require_int(
        closeout, "inserted_helper_load_count", args.feasibility_closeout
    )
    prior_inserted_writebacks = require_int(
        closeout, "inserted_helper_writeback_count", args.feasibility_closeout
    )
    prior_net = require_int(closeout, "net_helper_call_delta", args.feasibility_closeout)

    erased = candidate_gets + candidate_sets
    # A DirectSlotLease plan is not a helper-backed ResidentScalar plan. The
    # lease token is a representation fact for the selected region; this
    # inventory keeps C ABI helper additions at zero until a later guard opens
    # lowering.
    lease_acquire_count = resident_fields
    materialization_helper_count = 0
    planned_added_helper_ops = 0
    planned_net = erased - planned_added_helper_ops

    lines = [
        "output_contract=direct-slot-lease-selected-method-inventory-v0",
        "input_contract=direct-slot-lease-compiler-plan-inventory-selection-v0",
        f"selected_method={args.method}",
        "selected_storage_backend=pinned_arena_exact",
        "selected_storage_classes=i64|u64|handle",
        f"candidate_exact_slot_get_count={candidate_gets}",
        f"candidate_exact_slot_set_count={candidate_sets}",
        f"candidate_exact_slot_helper_count={erased}",
        f"resident_field_key_count={resident_fields}",
        f"lease_acquire_count={lease_acquire_count}",
        "lease_acquire_c_abi_helper_count=0",
        f"materialization_helper_count={materialization_helper_count}",
        f"planned_erased_helper_ops={erased}",
        f"planned_added_helper_ops={planned_added_helper_ops}",
        f"planned_net_helper_delta={planned_net}",
        f"planned_net_helper_delta_positive={1 if planned_net > 0 else 0}",
        f"prior_resident_scalar_inserted_helper_load_count={prior_inserted_loads}",
        f"prior_resident_scalar_inserted_helper_writeback_count={prior_inserted_writebacks}",
        f"prior_resident_scalar_net_helper_call_delta={prior_net}",
        "barrier_policy=guard_surface_required_before_lowering",
        "lowering_open=0",
        "native_direct_open=0",
        "direct_load_store_open=0",
        "selected_next=direct_slot_lease_lowering_guard_surface",
        "implementation_open=0",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
