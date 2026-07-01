#!/usr/bin/env python3
"""Rerun directability after nominal ID scalar transport policy."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_SURVEY = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009"
NEXT_OWNER_REPAIR = "MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001"
POLICY = FIXTURES / "mirbuilder-id-scalar-domain-transport-policy-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    policy = read_json(POLICY)
    rows = policy.get("policy_rows") or []

    rerun_rows: list[dict[str, Any]] = []
    state_counts: Counter[str] = Counter()
    owner_counts: Counter[str] = Counter()
    type_counts: Counter[str] = Counter()
    for row in rows:
        confidence = row.get("owner_edge_confidence") or "None"
        state = (
            "DirectableWithNominalIdScalarTransport"
            if row.get("policy_state") == "IdScalarDomainTransportSelected"
            and row.get("nominal_transport")
            and confidence == "FixtureMapped"
            else "OwnerEdgeRepairRequired"
        )
        state_counts[state] += 1
        owner_counts[row.get("known_owner_edge") or "<none>"] += 1
        type_counts[row.get("canonical_id_type") or "<unsupported>"] += 1
        rerun_rows.append(
            {
                "source_id": row["source_id"],
                "canonical_id_type": row.get("canonical_id_type"),
                "nominal_transport": row.get("nominal_transport"),
                "known_owner_edge": row.get("known_owner_edge"),
                "owner_edge_confidence": confidence,
                "shape_signature": row.get("shape_signature"),
                "directability_state": state,
                "blocked_by": [] if state == "DirectableWithNominalIdScalarTransport" else ["OwnerEdgeConfidenceMissing"],
            }
        )

    directable_count = state_counts.get("DirectableWithNominalIdScalarTransport", 0)
    repair_count = state_counts.get("OwnerEdgeRepairRequired", 0)
    if directable_count:
        decision = {
            "kind": "SelectNativeOwnerSeedCapabilitySurveyRerun",
            "reason_token": "IdScalarTransportDirectableRowsAvailable",
            "selected_next_card": NEXT_SURVEY,
        }
    elif repair_count:
        decision = {
            "kind": "SelectOwnerEdgeRepair",
            "reason_token": "OnlyOwnerEdgeMissingIdScalarRowsRemain",
            "selected_next_card": NEXT_OWNER_REPAIR,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoIdScalarDirectabilityRows",
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDomainTransportDirectabilityRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "id_scalar_domain_transport_policy": rel(POLICY),
        },
        "provenance": {
            "id_scalar_domain_transport_policy_hash": sha256_file(POLICY),
        },
        "input_decision": policy.get("decision"),
        "rerun_rows": rerun_rows,
        "summary": {
            "input_id_scalar_row_count": len(rows),
            "directable_with_nominal_id_scalar_transport_count": directable_count,
            "owner_edge_repair_required_count": repair_count,
            "directability_state_counts": dict(sorted(state_counts.items())),
            "canonical_id_type_counts": dict(sorted(type_counts.items())),
            "owner_edge_counts": dict(sorted(owner_counts.items())),
        },
        "selection_rule": {
            "fixture_mapped_owner_edge_required_for_directable": True,
            "owner_edge_missing_rows_are_repair_backlog": True,
            "raw_i64_interchangeability": False,
            "object_layout_transport": False,
            "manual_owner_selection": False,
        },
        "decision": decision,
        "claims": {
            "id_scalar_domain_transport_policy_consumed": 1,
            "directability_rerun_ready": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_owner_selection": 0,
            "raw_i64_interchangeability": 0,
            "object_layout_transport": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-id-scalar-domain-transport-directability-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
