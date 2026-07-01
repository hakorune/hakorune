#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_domain_transport_directability_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2014-MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001.md"
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
current_state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001"
next_card = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarDomainTransportDirectabilityRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("id_scalar_domain_transport_policy", "").endswith("mirbuilder-id-scalar-domain-transport-policy-v0.json"), "input drift")

summary = fixture.get("summary") or {}
need(summary.get("input_id_scalar_row_count") == 31, "input count drift")
need(summary.get("directable_with_nominal_id_scalar_transport_count") == 19, "directable count drift")
need(summary.get("owner_edge_repair_required_count") == 12, "owner repair count drift")
states = summary.get("directability_state_counts") or {}
need(states.get("DirectableWithNominalIdScalarTransport") == 19, "directable state count drift")
need(states.get("OwnerEdgeRepairRequired") == 12, "repair state count drift")
owners = summary.get("owner_edge_counts") or {}
for key, value in {
    "<none>": 12,
    "mirbuilder::context_registry": 5,
    "mirbuilder::emission_ssa_phi": 6,
    "mirbuilder::join_i_r_plan": 7,
    "mirbuilder::join_i_r_route_verify": 1,
}.items():
    need(owners.get(key) == value, f"owner count drift: {key}")

rows = fixture.get("rerun_rows") or []
need(len(rows) == 31, "rerun row count drift")
for row in rows:
    row_state = row.get("directability_state")
    if row.get("owner_edge_confidence") == "FixtureMapped":
        need(row_state == "DirectableWithNominalIdScalarTransport", "fixture mapped row must be directable")
    else:
        need(row_state == "OwnerEdgeRepairRequired", "non fixture mapped row must require owner repair")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNativeOwnerSeedCapabilitySurveyRerun", "decision kind drift")
need(decision.get("reason_token") == "IdScalarTransportDirectableRowsAvailable", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("id_scalar_domain_transport_policy_consumed") == 1, "input consumed claim drift")
need(claims.get("directability_rerun_ready") == 1, "ready claim drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_owner_selection",
    "raw_i64_interchangeability",
    "object_layout_transport",
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

need(current_state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(current_state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    next_card,
    "directable_with_nominal_id_scalar_transport_count = 19",
    "owner_edge_repair_required_count = 12",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-domain-transport-directability-rerun")
print("directable_with_nominal_id_scalar_transport_count=19")
print("owner_edge_repair_required_count=12")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
