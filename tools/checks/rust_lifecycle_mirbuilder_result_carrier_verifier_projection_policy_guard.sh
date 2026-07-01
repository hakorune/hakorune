#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-result-carrier-verifier-projection-policy-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_result_carrier_verifier_projection_policy.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[result-carrier-verifier-projection-policy-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderResultCarrierVerifierProjectionPolicyV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001":
    die("fixture token mismatch")

policy = data.get("selected_policy") or {}
if policy.get("policy_id") != "VerifierBackedResultCarrierProjectionPolicyV1":
    die("policy id mismatch")
if policy.get("applies_to_contract") != "ResultCarrierVerifierContractV1":
    die("contract mismatch")
if policy.get("hako_projection_selected") is not False:
    die("hako projection must remain unselected")
if policy.get("candidate_rerun_required") is not True:
    die("candidate rerun must be required")

summary = data.get("summary") or {}
if summary.get("result_carrier_projection_policy_row_count") != 3:
    die("policy row count must be 3")
if summary.get("result_carrier_projection_policy_selected") != 1:
    die("projection policy must be selected")

rows = data.get("policy_rows") or []
if len(rows) != 3:
    die("policy rows must be 3")
for row in rows:
    if row.get("projection_policy_state") != "VerifierBackedResultCarrierProjectionSelected":
        die("row projection policy state mismatch")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectStrictConverterEmissionNativeSeedCandidateSelectionRerun":
    die("decision kind mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "hako_projection_selected",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "owner_name_as_transport_policy",
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
    if claims.get(key) != 0:
        die(f"claim must remain 0: {key}")

if claims.get("result_carrier_verifier_contract_consumed") != 1:
    die("1984 contract must be consumed")
if claims.get("result_carrier_projection_policy_selected") != 1:
    die("projection policy selected claim missing")

print("[result-carrier-verifier-projection-policy-guard] OK")
PY
