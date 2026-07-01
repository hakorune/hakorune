#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-domain-owner-edge-repair-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_domain_owner_edge_repair.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2018-MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001"
next_card = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarDomainOwnerEdgeRepairV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("seed_readiness_resolution", "").endswith("mirbuilder-id-scalar-domain-seed-readiness-resolution-v0.json"), "readiness input drift")
need(inputs.get("directability_rerun", "").endswith("mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"), "directability input drift")
need(inputs.get("other_owner_edge_confidence_repair", "").endswith("mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json"), "repair authority drift")

policy = fixture.get("repair_policy") or {}
need(policy.get("policy_id") == "IdScalarOwnerEdgeRepairFromExistingOtherOwnerEdgeRepairV1", "repair policy drift")
need(policy.get("selection_authority") == "exact_source_id_match_to_other_owner_edge_confidence_repair", "selection authority drift")
need(policy.get("semantic_projection_inference") == 0, "semantic projection inference must stay zero")
need(policy.get("manual_owner_selection") == 0, "manual owner selection must stay zero")

summary = fixture.get("summary") or {}
need(summary.get("input_repair_required_count") == 12, "input repair count drift")
need(summary.get("repaired_row_count") == 12, "repaired row count drift")
need(summary.get("unrepaired_row_count") == 0, "unrepaired row count drift")
need(summary.get("distinct_repaired_owner_edge_count") == 6, "distinct owner count drift")

rows = fixture.get("repaired_rows") or []
need(len(rows) == 12, "repaired rows length drift")
for row in rows:
    need(row.get("repair_state") == "Repaired", "row must be repaired")
    need(row.get("repaired_owner_edge_id"), "row missing repaired owner edge")
    need(row.get("repaired_owner_edge_confidence") in {"ExactSymbol", "FixtureMapped", "FileScoped"}, "bad repaired confidence")
    need(row.get("repair_reason_token") == "FileScopedOwnerEdgeDerivedFromSourcePath", "bad repair reason")
need(not fixture.get("unrepaired_rows"), "unrepaired rows must be empty")

expected_edges = {
    "hakorune_mir_builder::builder_init": 1,
    "hakorune_mir_builder::builder_value_kind": 1,
    "hakorune_mir_builder::joinir_id_remapper": 1,
    "hakorune_mir_builder::utils::id_alloc": 3,
    "hakorune_mir_builder::utils::local_ssa": 5,
    "hakorune_mir_region::function_slot_registry": 1,
}
actual_edges = {
    item["owner_edge_id"]: item["count"]
    for item in summary.get("repaired_owner_edge_counts") or []
}
need(actual_edges == expected_edges, "repaired owner edge counts drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectSeedReadinessResolutionRerun", "decision kind drift")
need(decision.get("reason_token") == "IdScalarOwnerEdgeRepairComplete", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "seed_readiness_resolution_consumed",
    "directability_rerun_consumed",
    "other_owner_edge_confidence_repair_consumed",
    "all_repair_required_rows_have_repair_attempt",
    "all_repair_required_rows_repaired",
]:
    need(claims.get(key) == 1, f"input/repair claim drift: {key}")
for key in [
    "manual_owner_selection",
    "family_name_based_policy",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "raw_i64_interchangeability",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    next_card,
    "input_repair_required_count = 12",
    "repaired_row_count = 12",
    "unrepaired_row_count = 0",
    "IdScalarOwnerEdgeRepairComplete",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-domain-owner-edge-repair")
print("input_repair_required_count=12")
print("repaired_row_count=12")
print("unrepaired_row_count=0")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
