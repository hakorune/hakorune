#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_002.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[strict-candidate-selection-rerun-002-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionRerun002V1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-002":
    die("fixture token mismatch")

pool = data.get("candidate_pool") or {}
expected = {
    "input_owner_edge_count": 3,
    "bridge_eligible_after_bridge_policy_v2_count": 3,
    "bridge_blocked_after_bridge_policy_v2_count": 0,
    "selected_candidate_count": 1,
}
for key, value in expected.items():
    if pool.get(key) != value:
        die(f"candidate pool drift: {key}")

rows = data.get("candidate_rows") or []
if len(rows) != 3:
    die("candidate rows must be 3")
for row in rows:
    if row.get("bridge_state_after_bridge_policy_v2") != "BridgeEligible":
        die("row must be bridge eligible")
    if row.get("blocked_by_after_bridge_policy_v2") != []:
        die("row blocked_by must be empty")
    if not row.get("next_card", "").endswith("-HAKO-NATIVE-SOURCE-SEED-001"):
        die("row next card must be native source seed")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectNativeSeedCandidate":
    die("decision kind mismatch")
if decision.get("reason_token") != "BridgePolicyV2StrictEmissionCandidateSelected":
    die("decision reason mismatch")
if decision.get("selected_owner_edge_id") != "hakorune_mir_builder::direct_state_plan_refresh":
    die("selected owner must follow stable lexical priority")
if decision.get("selected_next_card") != "MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "bridge_policy_v2_consumed",
    "forbidden_nonclaim_boundary_scope_resolution_consumed",
]:
    if claims.get(key) != 1:
        die(f"{key} must be 1")
for key in [
    "manual_family_selection",
    "seed_eligibility_from_forbidden_nonclaim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        die(f"claim must remain 0: {key}")

print("[strict-candidate-selection-rerun-002-guard] OK")
PY
