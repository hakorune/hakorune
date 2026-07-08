#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2110-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-RERUN-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownCollectionLenScalarI64ContractRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

closeouts = fixture.get("accepted_scoped_closeouts") or []
need(len(closeouts) == 3, "accepted closeout count drift")
need({row.get("contract_id") for row in closeouts} == {
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
    "CollectionLenScalarI64TypedDirectCloseoutContract",
}, "accepted closeout id drift")

materialized = fixture.get("materialized_contract") or {}
need(materialized.get("contract_id") == "CollectionLenScalarI64TypedDirectCloseoutContract", "materialized id drift")
need(materialized.get("surface_id") == "CollectionScalarI64Routes", "surface drift")
need(materialized.get("routes") == ["MapEntryCount", "ArraySlotLen", "StringLen", "AnyLength"], "route drift")
need(materialized.get("return_shape") == "ScalarI64", "return shape drift")
need(materialized.get("value_demand") == "ScalarI64", "value demand drift")
need(materialized.get("publication_policy") == "NoPublication", "publication drift")
need(materialized.get("effect_class") == "observe", "effect drift")

need(fixture.get("remaining_candidate_surfaces") == ["WriteScalarI64Routes"], "remaining candidate drift")
need(fixture.get("write_blocker") == "WriteResultPolicyRequiredBeforeDirectCloseout", "write blocker drift")

summary = fixture.get("summary") or {}
need(summary.get("collection_len_scalar_i64_contract_materialized") == 1, "summary materialized drift")
need(summary.get("accepted_scoped_closeout_count") == 3, "summary closeout count drift")
need(summary.get("remaining_candidate_surface_count") == 1, "summary remaining count drift")
need(summary.get("remaining_candidate_surface_id") == "WriteScalarI64Routes", "summary remaining id drift")
for key in [
    "write_result_policy_ready",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteResultPolicyBasis", "decision kind drift")
need(decision.get("reason_token") == "CollectionLenScopedCloseoutMaterializedWritePolicyRemains", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("collection_len_scalar_i64_contract_materialized") == 1, "missing materialized claim")
need(claims.get("accepted_scoped_closeout_count") == 3, "claim closeout count drift")
for key in [
    "write_result_policy_ready",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "hako_adoption",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
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
need(manifest_row.get("card", "").endswith("2110-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_rerun_guard.sh"), "manifest guard drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest drift")
need(state.get("current_blocker_token") == next_card, "CURRENT_STATE blocker drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun")
print("collection_len_scalar_i64_contract_materialized=1")
print("accepted_scoped_closeout_count=3")
print("remaining_candidate_surface_count=1")
print("remaining_candidate_surface_id=WriteScalarI64Routes")
print("scalar_known_transport_axis_closeout=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
