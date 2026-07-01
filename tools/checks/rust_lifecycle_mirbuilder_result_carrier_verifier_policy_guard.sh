#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-result-carrier-verifier-policy-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_result_carrier_verifier_policy.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[result-carrier-verifier-policy-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderResultCarrierVerifierPolicyV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-RESULT-CARRIER-VERIFIER-POLICY-001":
    die("fixture token mismatch")

summary = data.get("summary") or {}
if summary.get("selected_policy_lane") != "ResultCarrierVerifierPolicyCandidate":
    die("selected policy lane mismatch")
if summary.get("result_carrier_candidate_count") != 3:
    die("result carrier candidate count must be 3")
if summary.get("result_carrier_policy_ready") != 1:
    die("result carrier policy must be ready")

policy = data.get("selected_policy") or {}
if policy.get("policy_id") != "ResultCarrierVerifierPolicyV1":
    die("policy id mismatch")
if policy.get("requires_projection_contract") is not True:
    die("projection contract must be required")
if policy.get("requires_canonical_json_parity") is not True:
    die("canonical json parity must be required")
if policy.get("requires_runtime_fallback_zero") is not True:
    die("runtime fallback zero must be required")
if policy.get("hako_generation") is not False:
    die("policy card must not generate Hako")

rows = data.get("policy_rows") or []
if len(rows) != 3:
    die("policy rows must be 3")
for row in rows:
    result_transport = row.get("result_transport") or ""
    if not result_transport.endswith("ResultBox"):
        die(f"bad result transport: {result_transport}")
    if not row.get("projection_contract"):
        die("row missing projection contract")
    if row.get("canonical_json_parity") != 1:
        die("row canonical json parity must be 1")
    if row.get("runtime_fallback") != 0:
        die("row runtime fallback must be 0")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectResultCarrierVerifierContract":
    die("decision kind mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-RESULT-CARRIER-VERIFIER-CONTRACT-001":
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

if claims.get("policy_lane_priority_resolution_consumed") != 1:
    die("1982 priority resolution must be consumed")
if claims.get("result_carrier_verifier_policy_defined") != 1:
    die("policy defined claim missing")

print("[result-carrier-verifier-policy-guard] OK")
PY
