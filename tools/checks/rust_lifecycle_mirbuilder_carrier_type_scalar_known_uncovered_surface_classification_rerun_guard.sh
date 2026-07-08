#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2103-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-RERUN-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownUncoveredSurfaceClassificationRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

rows = fixture.get("classification_rows") or []
need(len(rows) == 3, "classification row count drift")
by_surface = {row.get("surface_id"): row for row in rows}
string = by_surface.get("StringScalarI64Routes") or {}
need(string.get("classification_eligible") is True, "string eligibility drift")
need(string.get("candidate_contract_id") == "StringSearchScalarI64TypedDirectCloseoutContract", "string contract drift")
need(string.get("selection_reason") == "LowestRiskReadOnlyWarmDirectScalarI64NoPublication", "selection reason drift")
need(set(string.get("route_kind_set") or []) == {"StringIndexOf", "StringLastIndexOf", "StringContains"}, "string route set drift")

collection = by_surface.get("CollectionScalarI64Routes") or {}
need(collection.get("classification_eligible") is False, "collection must not be eligible")
need("MixedWithAlreadyClosedMapLoadScalarI64" in (collection.get("blocked_by") or []), "collection block drift")

write = by_surface.get("WriteScalarI64Routes") or {}
need(write.get("classification_eligible") is False, "write must not be eligible")
need("WriteResultPolicyRequiredBeforeDirectCloseout" in (write.get("blocked_by") or []), "write block drift")

for expected in [
    "GenericMethodRouteKind::StringIndexOf",
    "GenericMethodRouteKind::StringLastIndexOf",
    "GenericMethodRouteKind::StringContains",
    "GenericMethodRouteProof::IndexOfSurfacePolicy",
    "GenericMethodRouteProof::LastIndexOfSurfacePolicy",
    "GenericMethodRouteProof::ContainsSurfacePolicy",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::ScalarI64",
    "GenericMethodPublicationPolicy::NoPublication",
    "CoreMethodLoweringTier::WarmDirectAbi",
]:
    need(expected in string_source or expected in string_test, f"missing string evidence token: {expected}")

summary = fixture.get("summary") or {}
need(summary.get("classified_surface_count") == 3, "summary class count drift")
need(summary.get("selection_eligible_surface_count") == 1, "summary eligible count drift")
need(summary.get("selected_surface_count") == 1, "summary selected count drift")
need(summary.get("direct_contract_materialized") == 0, "summary direct materialization drift")
need(summary.get("scalar_known_transport_axis_closeout") == 0, "summary axis closeout drift")
need(summary.get("source_selfhost_claim") == 0, "summary source selfhost drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectStringSearchScalarI64TypedDirectCloseoutContractBasis", "decision kind drift")
need(decision.get("reason_token") == "ExactlyOneScalarKnownUncoveredSurfaceClassified", "reason drift")
need(decision.get("selected_surface_id") == "StringScalarI64Routes", "selected surface drift")
need(decision.get("selected_contract_id") == "StringSearchScalarI64TypedDirectCloseoutContract", "selected contract drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("scalar_known_uncovered_surface_classification_rerun") == 1, "missing rerun claim")
need(claims.get("string_search_scalar_i64_contract_selected") == 1, "missing string selection claim")
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
need(manifest_row.get("card", "").endswith("2103-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_rerun_guard.sh"), "manifest guard drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest drift")
need(state.get("current_blocker_token") == next_card, "CURRENT_STATE blocker drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun")
print("selection_eligible_surface_count=1")
print("selected_surface_id=StringScalarI64Routes")
print("selected_contract_id=StringSearchScalarI64TypedDirectCloseoutContract")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
