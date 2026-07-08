#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2104-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
STRING_SOURCE="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"
STRING_TEST="$ROOT/src/mir/generic_method_route_plan/tests/string_routes/search_routes.rs"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" "$STRING_SOURCE" "$STRING_TEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))
string_source = Path(sys.argv[6]).read_text(encoding="utf-8")
string_test = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownStringSearchScalarI64TypedDirectCloseoutContractBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("selected_surface_id") == "StringScalarI64Routes", "selected surface drift")
need(inputs.get("selected_contract_id") == "StringSearchScalarI64TypedDirectCloseoutContract", "selected contract drift")

contract = fixture.get("contract") or {}
need(contract.get("contract_id") == "StringSearchScalarI64TypedDirectCloseoutContract", "contract id drift")
need(contract.get("source_kind") == "TypedDirectCloseoutContract", "source kind drift")
need(contract.get("target_axis") == "ScalarKnownTransportAxis", "target axis drift")
need(contract.get("surface_id") == "StringScalarI64Routes", "surface drift")
need(contract.get("return_shape") == "ScalarI64", "return shape drift")
need(contract.get("value_demand") == "ScalarI64", "value demand drift")
need(contract.get("publication_policy") == "NoPublication", "publication drift")
need(contract.get("core_method_lowering_tier") == "WarmDirectAbi", "tier drift")
need(contract.get("effect_class") == "read", "effect drift")
need(contract.get("all_rows_join_contract") is True, "join drift")
need(contract.get("no_carrier_boundary_required_or_already_covered") is True, "carrier boundary drift")

routes = contract.get("routes") or []
need(len(routes) == 3, "route count drift")
route_names = {row.get("route_kind") for row in routes}
need(route_names == {"StringIndexOf", "StringLastIndexOf", "StringContains"}, "route set drift")

for expected in [
    "GenericMethodRouteKind::StringIndexOf",
    "GenericMethodRouteKind::StringLastIndexOf",
    "GenericMethodRouteKind::StringContains",
    "GenericMethodRouteProof::IndexOfSurfacePolicy",
    "GenericMethodRouteProof::LastIndexOfSurfacePolicy",
    "GenericMethodRouteProof::ContainsSurfacePolicy",
    "CoreMethodLoweringTier::WarmDirectAbi",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::ScalarI64",
    "GenericMethodPublicationPolicy::NoPublication",
]:
    need(expected in string_source or expected in string_test, f"missing evidence token: {expected}")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "basis-only drift")
need(rule.get("contract_materialization_requires_rerun") is True, "rerun rule drift")
need(rule.get("axis_closeout_forbidden_at_basis") is True, "axis closeout rule drift")
for key in [
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("typed_direct_closeout_contract_basis") == 1, "summary basis drift")
need(summary.get("string_search_route_count") == 3, "summary route count drift")
need(summary.get("direct_contract_materialized") == 0, "summary materialized drift")
need(summary.get("scalar_known_transport_axis_closeout") == 0, "summary axis closeout drift")
need(summary.get("source_selfhost_claim") == 0, "summary source selfhost drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectStringSearchScalarI64TypedDirectCloseoutContractRerun", "decision kind drift")
need(decision.get("reason_token") == "StringSearchScalarI64TypedDirectCloseoutContractBasisDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("string_search_scalar_i64_typed_direct_closeout_contract_basis") == 1, "missing positive claim")
need(claims.get("basis_only") == 1, "basis claim drift")
for key in [
    "direct_contract_materialized",
    "scalar_known_transport_axis_closeout",
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

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2104-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_basis_guard.sh"), "manifest guard drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest drift")
need(state.get("current_blocker_token") == next_card, "CURRENT_STATE blocker drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-basis")
print("typed_direct_closeout_contract_basis=1")
print("string_search_route_count=3")
print("direct_contract_materialized=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
