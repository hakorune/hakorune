#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-domain-transport-policy-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_domain_transport_policy.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2013-MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001.md"
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


token = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001"
next_card = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderIdScalarDomainTransportPolicyV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("domain_object_id_transport_policy_inventory", "").endswith("mirbuilder-domain-object-id-transport-policy-inventory-v0.json"), "input drift")

policy = fixture.get("selected_policy") or {}
need(policy.get("policy_id") == "NominalIdScalarDomainTransportV1", "policy id drift")
need(policy.get("physical_lane") == "i64", "physical lane drift")
need(policy.get("semantic_transport_is_nominal") is True, "nominal transport drift")
need(policy.get("raw_i64_interchangeability") is False, "raw i64 must not be interchangeable")
need(policy.get("object_layout_transport") is False, "object layout must not be selected")
need(policy.get("hako_generation") is False, "policy card must not emit Hako")

expected_transports = {
    "ValueId": "ValueIdAsI64",
    "BasicBlockId": "BasicBlockIdAsI64",
    "BindingId": "BindingIdAsI64",
    "BodyId": "BodyIdAsI64",
    "SlotId": "SlotIdAsI64",
    "TypedValueId": "TypedValueIdAsI64",
}
need(fixture.get("nominal_transports") == dict(sorted(expected_transports.items())), "nominal transport table drift")

summary = fixture.get("summary") or {}
need(summary.get("id_scalar_input_count") == 31, "input count drift")
need(summary.get("id_scalar_domain_transport_policy_ready") == 1, "policy ready drift")
need(summary.get("unsupported_id_scalar_type_count") == 0, "unsupported id scalar drift")
expected_counts = {
    "ValueId": 17,
    "BasicBlockId": 9,
    "BindingId": 2,
    "BodyId": 1,
    "SlotId": 1,
    "TypedValueId": 1,
}
for key, value in expected_counts.items():
    need((summary.get("canonical_id_type_counts") or {}).get(key) == value, f"id type count drift: {key}")
need((summary.get("owner_edge_confidence_counts") or {}).get("FixtureMapped") == 19, "fixture mapped count drift")
need((summary.get("owner_edge_confidence_counts") or {}).get("None") == 12, "none owner confidence count drift")

rows = fixture.get("policy_rows") or []
need(len(rows) == 31, "policy row count drift")
for row in rows:
    need(row.get("policy_state") == "IdScalarDomainTransportSelected", "row policy state drift")
    need(row.get("nominal_transport") == expected_transports.get(row.get("canonical_id_type")), "row nominal transport drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectIdScalarDomainTransportDirectabilityRerun", "decision kind drift")
need(decision.get("reason_token") == "NominalIdScalarTransportPolicyDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("domain_object_id_transport_policy_inventory_consumed") == 1, "input consumed claim drift")
need(claims.get("id_scalar_domain_transport_policy_defined") == 1, "policy defined claim drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "domain_object_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "raw_i64_interchangeability",
    "object_layout_transport",
    "generator_object_transport",
    "invalid_sentinel_semantics",
    "reserved_id_policy",
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
    "id_scalar_input_count = 31",
    "ValueId = 17",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-domain-transport-policy")
print("id_scalar_input_count=31")
print("policy_id=NominalIdScalarDomainTransportV1")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
