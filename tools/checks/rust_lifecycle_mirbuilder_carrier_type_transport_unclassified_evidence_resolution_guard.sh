#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-unclassified-evidence-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_unclassified_evidence_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2011-MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-001"
next_card = "MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportUnclassifiedEvidenceResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("carrier_type_transport_evidence_inventory_rerun_002", "").endswith("mirbuilder-carrier-type-transport-evidence-inventory-rerun-002-v0.json"), "input drift")

summary = fixture.get("summary") or {}
need(summary.get("unclassified_input_count") == 130, "input count drift")
need(summary.get("resolved_axis_count") == 6, "axis count drift")
axis = summary.get("axis_counts") or {}
expected = {
    "DomainObjectOrIdTransportAxis": 116,
    "ProductTupleTransportAxis": 9,
    "CollectionCarrierTransportAxis": 2,
    "IteratorOrBorrowTypeTransportAxis": 1,
    "ScalarKnownTransportAxis": 1,
    "OpaqueTypeTransportAxis": 1,
}
for key, value in expected.items():
    need(axis.get(key) == value, f"axis count drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectDomainObjectIdTransportPolicyInventory", "decision kind drift")
need(decision.get("reason_token") == "DomainObjectIdTransportAxisIsPureTypeTransport", "reason drift")
need(decision.get("selected_axis") == "DomainObjectOrIdTransportAxis", "selected axis drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("carrier_type_transport_evidence_inventory_rerun_002_consumed") == 1, "input consumed claim drift")
need(claims.get("unclassified_evidence_resolution_ready") == 1, "ready claim drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "return_type_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
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
    "unclassified_input_count = 130",
    "DomainObjectOrIdTransportAxis = 116",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-unclassified-evidence-resolution")
print("unclassified_input_count=130")
print("resolved_axis_count=6")
print("DomainObjectOrIdTransportAxis=116")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
