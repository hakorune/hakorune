#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
oracle = json.loads((base / "carrier-info-merge-from-oracle-vectors-v0.json").read_text())
result = json.loads((base / "carrier-info-merge-from-emitter-verifier-result-v0.json").read_text())
surface = (base / "carrier-info-merge-from-emitter-surface-v0.hako").read_text()

assert oracle["schema_version"] == 0
assert oracle["kind"] == "RustOracleVectors"
assert oracle["subject"] == result["subject"]
assert result["result"] == "VerifiedPlan"

claims = result["claims"]
assert claims["emission_allowed"] is True
assert claims["emission_scope"] == "CarrierInfo::merge_from only"
assert claims["backend_behavior_changed"] is False
assert claims["full_variable_context_parity"] is False
assert claims["mirbuilder_wide_lifecycle"] is False

promotion = oracle["promotion_scope"]
assert promotion["phi_join_id_claim"] is False
assert promotion["full_variable_context_claim"] is False
assert promotion["mirbuilder_wide_claim"] is False

vector_ids = {vector["id"] for vector in oracle["vectors"]}
assert vector_ids == {
    "append_missing_carrier_and_sort",
    "duplicate_carrier_is_not_added",
}

required_surface_tokens = [
    "subject: CarrierInfo::merge_from",
    "plan_kind: OwnedCarrierInfoMerge",
    "Verified boundary: receiver is owned and mutable.",
    "Verified boundary: other is read-only and is not mutated.",
    "Verified plan: append missing carriers by name, then sort.",
    "Verified plan: clone trim_helper only as existing route metadata.",
    "Verified plan: append missing promoted_body_locals as owned strings.",
]
for token in required_surface_tokens:
    assert token in surface, token

denied_surface_tokens = {
    "join_id_assignment": "Denied boundary: no join_id producer is emitted here.",
    "trim_helper_lifecycle_owner": "Denied boundary: no trim_helper lifecycle owner is claimed here.",
    "promoted_body_locals_lifecycle_owner": (
        "Verified plan: append missing promoted_body_locals as owned strings."
    ),
    "general_resolver": "Denied boundary: no general converter rewrite is claimed here.",
}

for denied in oracle["denied_vectors"]:
    assert denied in denied_surface_tokens, denied
    assert denied_surface_tokens[denied] in surface, denied

for forbidden in [
    "phi_join_id_claim: 1",
    "full_variable_context_parity: 1",
    "mirbuilder_wide_lifecycle: 1",
    "backend_behavior_changed: 1",
    "general resolver selection owner",
]:
    assert forbidden not in surface, forbidden
PY

cat <<'REPORT'
output_contract=rust-lifecycle-emitter-oracle-parity-v0
selected_family_parity_checked=1
surface_matches_oracle_contract=1
subject=CarrierInfo::merge_from
plan_kind=OwnedCarrierInfoMerge
crate_wide_lifecycle_parity=0
mirbuilder_wide_lifecycle=0
backend_behavior_changed=0
rustc_integration_started=0
summary=ok
REPORT
