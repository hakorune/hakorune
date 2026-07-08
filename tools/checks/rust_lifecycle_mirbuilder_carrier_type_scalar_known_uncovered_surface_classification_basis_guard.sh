#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2102-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-BASIS-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownUncoveredSurfaceClassificationBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("previous_reason_token") == "ScalarKnownTransportAxisHasUncoveredScalarSurfaces", "previous reason drift")

previous = fixture.get("previous_state") or {}
need(previous.get("uncovered_scalar_known_surface_count") == 3, "previous uncovered count drift")
need(previous.get("scalar_known_transport_axis_closeout") == 0, "previous axis closeout drift")
need(previous.get("scoped_map_load_scalar_i64_closeout") == 1, "previous scoped closeout drift")
need(previous.get("selected_next_card") == design_stop, "previous next drift")

required_dimensions = {
    "surface_id",
    "route_kind_set",
    "method_surface",
    "return_shape",
    "value_demand",
    "publication_policy",
    "proof_or_policy_source",
    "core_method_op",
    "core_method_lowering_tier",
    "effect_class",
    "receiver_key_value_result_origin_evidence",
    "test_anchor",
}
need(set(fixture.get("classification_dimensions") or []) == required_dimensions, "classification dimensions drift")

surface_classes = fixture.get("surface_classes") or []
need(len(surface_classes) == 3, "surface class count drift")
by_surface = {row.get("surface_id"): row for row in surface_classes}
need(set(by_surface) == {
    "StringScalarI64Routes",
    "CollectionScalarI64Routes",
    "WriteScalarI64Routes",
}, "surface set drift")

string = by_surface["StringScalarI64Routes"]
need(string.get("candidate_contract_id") == "StringSearchScalarI64TypedDirectCloseoutContract", "string contract drift")
need(string.get("effect_class") == "read", "string effect drift")
need(string.get("core_method_lowering_tier") == "WarmDirectAbi", "string tier drift")
need(set(string.get("route_kind_set") or []) == {"StringIndexOf", "StringLastIndexOf", "StringContains"}, "string route drift")
need(string.get("post_classification_priority_hint") == "lowest_risk_candidate", "string priority drift")

collection = by_surface["CollectionScalarI64Routes"]
need(collection.get("candidate_contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "collection contract drift")
need(collection.get("effect_class") == "observe", "collection effect drift")
need("MapEntryCount" in (collection.get("route_kind_set") or []), "collection len route drift")

write = by_surface["WriteScalarI64Routes"]
need(write.get("candidate_contract_id") == "WriteResultScalarI64ClassificationOnly", "write contract drift")
need(write.get("effect_class") == "mutate", "write effect drift")
need(write.get("publication_policy") == "MixedNoPublicationAndNone", "write publication drift")
need(write.get("post_classification_priority_hint") == "do_not_select_before_write_result_policy", "write priority drift")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "basis-only rule drift")
need(rule.get("direct_surface_selection_allowed") is False, "direct surface selection drift")
need(rule.get("classification_rerun_required_before_contract_selection") is True, "rerun rule drift")
for key in [
    "route_membership_alone_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("classification_basis") == 1, "summary basis drift")
need(summary.get("classified_surface_count") == 3, "summary class count drift")
need(summary.get("direct_contract_selection") == 0, "summary direct selection drift")
need(summary.get("scalar_known_transport_axis_closeout") == 0, "summary axis closeout drift")
need(summary.get("source_selfhost_claim") == 0, "summary source selfhost drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectScalarKnownUncoveredSurfaceClassificationRerun", "decision kind drift")
need(decision.get("reason_token") == "ScalarKnownUncoveredSurfaceClassificationBasisDefined", "reason drift")
need(decision.get("selected_surface_id") is None, "surface must not be selected")
need(decision.get("selected_contract_id") is None, "contract must not be selected")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "scalar_known_uncovered_surface_classification_basis",
    "classification_dimensions_defined",
    "basis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_contract_selection",
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
need(manifest_row.get("card", "").endswith("2102-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis")
print("classification_basis=1")
print("classified_surface_count=3")
print("direct_contract_selection=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
