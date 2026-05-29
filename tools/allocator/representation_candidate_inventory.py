#!/usr/bin/env python3
"""Normalize representation/direct-lowering candidates into one inventory."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key.strip()] = value.strip()
    return values


def require(values: dict[str, str], key: str, expected: str, source: Path) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{source}: {key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str, source: Path) -> str:
    value = values.get(key)
    if value is None:
        raise SystemExit(f"{source}: missing {key}")
    return value


def int_value(values: dict[str, str], key: str, source: Path) -> int:
    return int(require_key(values, key, source))


def float_value(values: dict[str, str], key: str, source: Path) -> str:
    # Keep the original decimal formatting in the report.
    value = require_key(values, key, source)
    float(value)
    return value


@dataclass(frozen=True)
class Candidate:
    family: str
    current_representation: str
    candidate_representation: str
    hot_pct: str
    helper_ops_before: int
    helper_ops_erased: int
    materialization_ops_added: int
    net_helper_delta: int
    escape_barrier_count: int
    observer_barrier_count: int
    unknown_call_barrier_count: int
    storage_or_slot_proven: int
    implementation_risk: str
    risk_reason: str

    @property
    def positive_net(self) -> int:
        return 1 if self.net_helper_delta > 0 else 0


def build_candidates(args: argparse.Namespace) -> list[Candidate]:
    closeout = read_kv(args.micro_helper_closeout)
    typed = read_kv(args.typed_field_inventory)
    capsule_plan = read_kv(args.capsule_plan_inventory)
    capsule_caller = read_kv(args.capsule_caller_inventory)
    array = read_kv(args.array_slot_inventory)
    array_owner = read_kv(args.array_owner_refresh)

    require(
        closeout,
        "output_contract",
        "micro-helper-lane-closeout-and-representation-direct-lowering-selection-v0",
        args.micro_helper_closeout,
    )
    require(typed, "output_contract", "mir-typed-field-direct-op-net-inventory-v0", args.typed_field_inventory)
    require(capsule_plan, "output_contract", "capsule-value-result-plan-inventory-v0", args.capsule_plan_inventory)
    require(
        capsule_caller,
        "output_contract",
        "capsule-value-result-caller-region-inventory-v0",
        args.capsule_caller_inventory,
    )
    require(array, "output_contract", "mir-array-slot-residence-inventory-v0", args.array_slot_inventory)
    require(
        array_owner,
        "output_contract",
        "selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0",
        args.array_owner_refresh,
    )

    result_capsule_hot = (
        float(float_value(args.owner_refresh_values, "family_3_pct", args.owner_refresh_source))
        + float(float_value(args.owner_refresh_values, "family_4_pct", args.owner_refresh_source))
    )

    return [
        Candidate(
            family="typed_object_exact_slot_residence",
            current_representation="ExactSlotObject",
            candidate_representation="ResidentScalar",
            hot_pct=float_value(closeout, "row284_exact_slot_get_set_pct", args.micro_helper_closeout),
            helper_ops_before=int_value(typed, "projected_erased_exact_helper_call_count", args.typed_field_inventory),
            helper_ops_erased=int_value(typed, "projected_erased_exact_helper_call_count", args.typed_field_inventory),
            materialization_ops_added=int_value(typed, "projected_added_helper_call_count", args.typed_field_inventory),
            net_helper_delta=int_value(typed, "projected_net_helper_call_delta", args.typed_field_inventory),
            escape_barrier_count=0,
            observer_barrier_count=int_value(typed, "barrier_return_count", args.typed_field_inventory),
            unknown_call_barrier_count=int_value(typed, "barrier_unknown_call_count", args.typed_field_inventory),
            storage_or_slot_proven=1,
            implementation_risk="high",
            risk_reason="largest_positive_net_but_prior_selected_method_residence_and_direct_op_attempts_hit_representation_boundaries",
        ),
        Candidate(
            family="result_capsule_value_aggregate",
            current_representation="ExactSlotObject",
            candidate_representation="ValueAggregate",
            hot_pct=f"{result_capsule_hot:.2f}",
            helper_ops_before=int_value(capsule_plan, "record_success_field_op_count", args.capsule_plan_inventory),
            helper_ops_erased=int_value(capsule_plan, "value_aggregate_erased_helper_calls", args.capsule_plan_inventory),
            materialization_ops_added=int_value(
                capsule_plan,
                "value_aggregate_materialization_helper_calls",
                args.capsule_plan_inventory,
            ),
            net_helper_delta=int_value(capsule_caller, "caller_region_value_aggregate_net_delta", args.capsule_caller_inventory),
            escape_barrier_count=0,
            observer_barrier_count=int_value(
                capsule_caller,
                "public_method_return_boundary_count",
                args.capsule_caller_inventory,
            ),
            unknown_call_barrier_count=int_value(
                capsule_caller,
                "unknown_call_after_success_count",
                args.capsule_caller_inventory,
            ),
            storage_or_slot_proven=1,
            implementation_risk="medium",
            risk_reason="value_aggregate_contract_clean_but_current_record_success_region_materializes_at_public_return",
        ),
        Candidate(
            family="array_slot_native_direct",
            current_representation="ExactSlotObject",
            candidate_representation="NativeDirect",
            hot_pct=float_value(array_owner, "perf_array_total_pct", args.array_owner_refresh),
            helper_ops_before=int_value(array, "erased_get_set_helper_calls", args.array_slot_inventory),
            helper_ops_erased=int_value(array, "erased_get_set_helper_calls", args.array_slot_inventory),
            materialization_ops_added=(
                int_value(array, "added_guard_helper_calls", args.array_slot_inventory)
                + int_value(array, "added_writeback_helper_calls", args.array_slot_inventory)
            ),
            net_helper_delta=int_value(array, "net_helper_call_delta", args.array_slot_inventory),
            escape_barrier_count=int_value(array, "barrier_escape_count", args.array_slot_inventory),
            observer_barrier_count=0,
            unknown_call_barrier_count=int_value(array, "barrier_unknown_call_count", args.array_slot_inventory),
            storage_or_slot_proven=1,
            implementation_risk="low",
            risk_reason="small_positive_net_region_and_direct_op_pipeline_already_proved_as_selected_method_keeper",
        ),
    ]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--micro-helper-closeout",
        type=Path,
        default=ROOT
        / "docs/development/current/main/phases/phase-296x/296x-297-MICRO-HELPER-LANE-CLOSEOUT-AND-REPRESENTATION-DIRECT-LOWERING-SELECTION.md",
    )
    parser.add_argument(
        "--owner-refresh",
        type=Path,
        default=ROOT
        / "docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md",
    )
    parser.add_argument(
        "--typed-field-inventory",
        type=Path,
        default=ROOT / "docs/development/current/main/phases/phase-296x/296x-218-MIR-TYPED-FIELD-DIRECT-OP-NET-INVENTORY.md",
    )
    parser.add_argument(
        "--capsule-plan-inventory",
        type=Path,
        default=ROOT / "docs/development/current/main/phases/phase-296x/296x-279-CAPSULE-VALUE-RESULT-PLAN-INVENTORY.md",
    )
    parser.add_argument(
        "--capsule-caller-inventory",
        type=Path,
        default=ROOT / "docs/development/current/main/phases/phase-296x/296x-280-CAPSULE-VALUE-RESULT-CALLER-REGION-INVENTORY.md",
    )
    parser.add_argument(
        "--array-slot-inventory",
        type=Path,
        default=ROOT / "docs/development/current/main/phases/phase-296x/296x-208-MIR-ARRAY-SLOT-RESIDENCE-INVENTORY.md",
    )
    parser.add_argument(
        "--array-owner-refresh",
        type=Path,
        default=ROOT
        / "docs/development/current/main/phases/phase-296x/296x-212-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-MEASUREMENT.md",
    )
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    args.owner_refresh_source = args.owner_refresh
    args.owner_refresh_values = read_kv(args.owner_refresh)
    require(
        args.owner_refresh_values,
        "output_contract",
        "post-record-success-helper-fusion-owner-refresh-v0",
        args.owner_refresh,
    )

    candidates = build_candidates(args)
    positive_count = sum(candidate.positive_net for candidate in candidates)
    top_positive = max(
        (candidate for candidate in candidates if candidate.positive_net),
        key=lambda candidate: (candidate.net_helper_delta, float(candidate.hot_pct), candidate.family),
    )
    lowest_risk_positive = min(
        (candidate for candidate in candidates if candidate.positive_net),
        key=lambda candidate: (["low", "medium", "high"].index(candidate.implementation_risk), -candidate.net_helper_delta),
    )

    lines = [
        "output_contract=representation-candidate-inventory-v0",
        "input_contract=representation-direct-lowering-ssot-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"candidate_count={len(candidates)}",
        f"positive_net_candidate_count={positive_count}",
        f"top_positive_net_candidate={top_positive.family}",
        f"top_positive_net_delta={top_positive.net_helper_delta}",
        f"lowest_risk_positive_net_candidate={lowest_risk_positive.family}",
        f"lowest_risk_positive_net_delta={lowest_risk_positive.net_helper_delta}",
    ]
    for idx, candidate in enumerate(candidates):
        lines.extend(
            [
                f"candidate_{idx}_family={candidate.family}",
                f"candidate_{idx}_current_representation={candidate.current_representation}",
                f"candidate_{idx}_candidate_representation={candidate.candidate_representation}",
                f"candidate_{idx}_hot_pct={candidate.hot_pct}",
                f"candidate_{idx}_helper_ops_before={candidate.helper_ops_before}",
                f"candidate_{idx}_helper_ops_erased={candidate.helper_ops_erased}",
                f"candidate_{idx}_materialization_ops_added={candidate.materialization_ops_added}",
                f"candidate_{idx}_net_helper_delta={candidate.net_helper_delta}",
                f"candidate_{idx}_net_helper_delta_positive={candidate.positive_net}",
                f"candidate_{idx}_escape_barrier_count={candidate.escape_barrier_count}",
                f"candidate_{idx}_observer_barrier_count={candidate.observer_barrier_count}",
                f"candidate_{idx}_unknown_call_barrier_count={candidate.unknown_call_barrier_count}",
                f"candidate_{idx}_storage_or_slot_proven={candidate.storage_or_slot_proven}",
                f"candidate_{idx}_implementation_risk={candidate.implementation_risk}",
                f"candidate_{idx}_risk_reason={candidate.risk_reason}",
                f"candidate_{idx}_selected_as_first_pilot=0",
            ]
        )
    lines.extend(
        [
            "first_pilot_selection_required=1",
            "selected_next=first_representation_pilot_selection",
            "implementation_open=0",
            "optimization_open=0",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )

    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
