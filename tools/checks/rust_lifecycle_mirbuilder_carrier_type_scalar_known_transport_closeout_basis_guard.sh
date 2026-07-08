#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-transport-closeout-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_transport_closeout_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2100-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownTransportCloseoutBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

previous = fixture.get("previous_state") or {}
need(previous.get("accepted_component_evidence_source_count") == 1, "accepted source drift")
need(previous.get("root_component_requirement_count") == 1, "root count drift")
need(previous.get("selected_component_requirement") == "ScalarKnownCloseoutAuthority", "selected component drift")
need(previous.get("selected_next_card") == token, "previous selected next drift")
need(previous.get("scalar_known_root_authority_status") == "Proven", "root status drift")
need(previous.get("scalar_known_accepted_source_count") == 1, "scalar source count drift")

basis = fixture.get("closeout_basis") or {}
need(basis.get("target_axis") == "ScalarKnownTransportAxis", "target axis drift")
need(basis.get("target_requirement") == "ScalarKnownCloseoutAuthority", "target requirement drift")
need(basis.get("basis_only") is True, "basis-only drift")
need(basis.get("rerun_required_before_axis_closeout") is True, "rerun rule drift")
contracts = basis.get("accepted_contracts") or []
need(len(contracts) == 1, "accepted contract count drift")
contract = contracts[0]
need(contract.get("contract_id") == "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract", "contract drift")
need(contract.get("source_kind") == "TypedDirectCloseoutContract", "source kind drift")
need(contract.get("route_kind") == "MapLoadScalarI64", "route drift")
need(contract.get("return_shape") == "ScalarI64OrMissingZero", "return shape drift")
need(contract.get("value_demand") == "ScalarI64", "value demand drift")
need(contract.get("publication_policy") == "NoPublication", "publication policy drift")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "selection basis-only drift")
need(rule.get("axis_closeout_forbidden_at_basis") is True, "axis closeout rule drift")
need(rule.get("concrete_carrier_type_axis_selection") is False, "concrete axis drift")
need(rule.get("source_selfhost_claim") is False, "source selfhost rule drift")
need(rule.get("rerun_required_before_closeout") is True, "rerun closeout rule drift")

claims = fixture.get("claims") or {}
for key in [
    "scalar_known_transport_closeout_basis",
    "scalar_known_closeout_authority_root_consumed",
    "map_load_scalar_i64_typed_direct_closeout_contract_consumed",
    "basis_only",
    "rerun_required_before_axis_closeout",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "scalar_known_transport_axis_closeout",
    "concrete_carrier_type_axis_selection",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectScalarKnownTransportCloseoutRerun", "decision kind drift")
need(decision.get("reason_token") == "ScalarKnownTransportCloseoutBasisDefined", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") == "ScalarKnownCloseoutAuthority", "selected component drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2100-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-transport-closeout-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_transport_closeout_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-transport-closeout-basis")
print("scalar_known_transport_closeout_basis=1")
print("basis_only=1")
print("scalar_known_transport_axis_closeout=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
