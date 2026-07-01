#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-transport-policy-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_transport_policy_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2012-MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001"
next_card = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdTransportPolicyInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("unclassified_evidence_resolution", "").endswith("mirbuilder-carrier-type-transport-unclassified-evidence-resolution-v0.json"), "input drift")

summary = fixture.get("summary") or {}
need(summary.get("domain_object_id_input_count") == 116, "input count drift")
axis = summary.get("domain_subaxis_counts") or {}
expected = {
    "IdScalarDomainTransportAxis": 31,
    "PlanRecipeDomainTransportAxis": 48,
    "MirDomainTransportAxis": 19,
    "AstNodeDomainTransportAxis": 14,
    "ContextOrSpanDomainTransportAxis": 3,
    "OtherDomainObjectTransportAxis": 1,
}
for key, value in expected.items():
    need(axis.get(key) == value, f"subaxis count drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectIdScalarDomainTransportPolicy", "decision kind drift")
need(decision.get("reason_token") == "IdScalarDomainTransportClosestToExistingScalarTransport", "reason drift")
need(decision.get("selected_subaxis") == "IdScalarDomainTransportAxis", "selected subaxis drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("unclassified_evidence_resolution_consumed") == 1, "input consumed claim drift")
need(claims.get("domain_object_id_transport_inventory_ready") == 1, "ready claim drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "domain_object_count_as_proof",
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
    "domain_object_id_input_count = 116",
    "IdScalarDomainTransportAxis = 31",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-transport-policy-inventory")
print("domain_object_id_input_count=116")
print("IdScalarDomainTransportAxis=31")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
