#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2105-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownStringSearchScalarI64TypedDirectCloseoutContractRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

closeouts = fixture.get("accepted_scoped_closeouts") or []
need(len(closeouts) == 2, "scoped closeout count drift")
ids = {row.get("contract_id") for row in closeouts}
need("MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract" in ids, "missing map load closeout")
need("StringSearchScalarI64TypedDirectCloseoutContract" in ids, "missing string search closeout")
string = [row for row in closeouts if row.get("contract_id") == "StringSearchScalarI64TypedDirectCloseoutContract"][0]
need(string.get("surface_id") == "StringScalarI64Routes", "string surface drift")
need(len(string.get("routes") or []) == 3, "string route count drift")
need(string.get("return_shape") == "ScalarI64", "return shape drift")
need(string.get("value_demand") == "ScalarI64", "value demand drift")
need(string.get("publication_policy") == "NoPublication", "publication drift")
need(string.get("core_method_lowering_tier") == "WarmDirectAbi", "tier drift")
need(string.get("effect_class") == "read", "effect drift")

remaining = set(fixture.get("remaining_uncovered_surface_ids") or [])
need(remaining == {"CollectionScalarI64Routes", "WriteScalarI64Routes"}, "remaining surface drift")

summary = fixture.get("summary") or {}
need(summary.get("string_search_scalar_i64_typed_direct_closeout_contract_materialized") == 1, "materialized drift")
need(summary.get("accepted_scoped_closeout_count") == 2, "summary scoped count drift")
need(summary.get("remaining_uncovered_scalar_known_surface_count") == 2, "summary remaining drift")
need(summary.get("scalar_known_transport_axis_closeout") == 0, "summary axis closeout drift")
need(summary.get("source_selfhost_claim") == 0, "summary source selfhost drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepScopedCloseout", "decision kind drift")
need(decision.get("reason_token") == "ScalarKnownTransportAxisStillHasUncoveredSurfaces", "reason drift")
need(decision.get("selected_next_card") == design_stop, "next card drift")
need(decision.get("consultation_required") is True, "consultation drift")

claims = fixture.get("claims") or {}
need(claims.get("string_search_scalar_i64_typed_direct_closeout_contract_materialized") == 1, "missing materialized claim")
need(claims.get("accepted_scoped_closeout_count") == 2, "claim scoped count drift")
for key in [
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
need(manifest_row.get("card", "").endswith("2105-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={design_stop}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun")
print("string_search_scalar_i64_typed_direct_closeout_contract_materialized=1")
print("accepted_scoped_closeout_count=2")
print("remaining_uncovered_scalar_known_surface_count=2")
print("selected_next_card=" + design_stop)
print("source_selfhost_claim=0")
print("summary=ok")
PY
