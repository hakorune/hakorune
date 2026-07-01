#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-result-carrier-verifier-contract-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_result_carrier_verifier_contract.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[result-carrier-verifier-contract-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderResultCarrierVerifierContractV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-RESULT-CARRIER-VERIFIER-CONTRACT-001":
    die("fixture token mismatch")

contract = data.get("contract") or {}
if contract.get("contract_id") != "ResultCarrierVerifierContractV1":
    die("contract id mismatch")
if contract.get("row_contract_count") != 3:
    die("contract row count must be 3")

summary = data.get("summary") or {}
if summary.get("result_carrier_contract_row_count") != 3:
    die("summary row count must be 3")
if summary.get("result_carrier_contract_ready") != 1:
    die("contract must be ready")

rows = data.get("contract_rows") or []
if len(rows) != 3:
    die("contract rows must be 3")
for row in rows:
    if row.get("contract_state") != "VerifierContractReady":
        die("row contract state must be ready")
    checks = row.get("required_checks") or {}
    if checks.get("result_transport_suffix_is_result_box") is not True:
        die("result transport suffix check failed")
    if checks.get("projection_contract_present") is not True:
        die("projection contract check failed")
    if checks.get("canonical_json_parity") != 1:
        die("canonical parity check failed")
    if checks.get("runtime_fallback") != 0:
        die("runtime fallback check failed")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectResultCarrierVerifierProjectionPolicy":
    die("decision kind mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
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

if claims.get("result_carrier_verifier_policy_consumed") != 1:
    die("1983 policy must be consumed")
if claims.get("result_carrier_verifier_contract_ready") != 1:
    die("contract ready claim missing")

print("[result-carrier-verifier-contract-guard] OK")
PY
