#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[strict-candidate-selection-rerun-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionRerunV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-001":
    die("fixture token mismatch")

pool = data.get("candidate_pool") or {}
expected_pool = {
    "base_verified_hako_family_ir_count": 47,
    "base_bridge_eligible_count": 0,
    "result_carrier_policy_covered_count": 3,
    "bridge_eligible_after_policy_count": 0,
    "denied_boundary_vocabulary_blocked_count": 3,
    "unclassified_denied_boundary_count": 0,
}
for key, value in expected_pool.items():
    if pool.get(key) != value:
        die(f"candidate pool drift: {key}")

classes = data.get("denied_boundary_class_counts") or {}
if classes != {
    "ForbiddenNonClaimBoundary": 3,
    "NarrowRefreshScopeExclusion": 3,
    "ScopeExclusionBoundary": 3,
}:
    die(f"denied boundary class counts drift: {classes}")

rows = data.get("result_carrier_projection_rows") or []
if len(rows) != 3:
    die("result carrier projection rows must be 3")
for row in rows:
    if row.get("result_carrier_projection_policy_covered") is not True:
        die("result carrier projection policy must be covered")
    if row.get("bridge_state_after_policy") != "BridgeBlocked":
        die("row must remain bridge blocked")
    if row.get("blocked_by_after_policy") != ["DeniedBoundaryVocabularyRequiresNormalization"]:
        die("row blocked reason mismatch")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectDeniedBoundaryVocabularyNormalization":
    die("decision kind mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-STRICT-DENIED-BOUNDARY-VOCABULARY-NORMALIZATION-001":
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

if claims.get("result_carrier_verifier_projection_policy_consumed") != 1:
    die("1985 projection policy must be consumed")
if claims.get("strict_candidate_selection_rerun_ready") != 1:
    die("rerun ready claim missing")

print("[strict-candidate-selection-rerun-guard] OK")
PY
