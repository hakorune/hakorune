#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-denied-boundary-vocabulary-normalization-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_denied_boundary_vocabulary_normalization.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[strict-denied-boundary-vocabulary-normalization-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderStrictDeniedBoundaryVocabularyNormalizationV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-STRICT-DENIED-BOUNDARY-VOCABULARY-NORMALIZATION-001":
    die("fixture token mismatch")

policy = data.get("normalization_policy") or {}
if policy.get("policy_id") != "StrictDeniedBoundaryVocabularyNormalizationV1":
    die("policy id mismatch")
for key in [
    "forbidden_nonclaim_never_proves_seed_eligibility",
    "scope_exclusion_not_transport_gap",
    "narrow_refresh_scope_exclusion_not_transport_gap",
    "unknown_boundary_requires_design_consultation",
]:
    if policy.get(key) is not True:
        die(f"policy flag must be true: {key}")

input_summary = data.get("input_summary") or {}
if input_summary.get("result_carrier_policy_covered_count") != 3:
    die("result carrier policy covered count drift")
if input_summary.get("denied_boundary_vocabulary_blocked_count") != 3:
    die("denied boundary blocked count drift")
if input_summary.get("input_unclassified_denied_boundary_count") != 0:
    die("input unclassified boundary count drift")

summary = data.get("summary") or {}
if summary.get("normalized_row_count") != 3:
    die("normalized row count must be 3")
if summary.get("unclassified_denied_boundary_count") != 0:
    die("unclassified denied boundary count must be 0")
if summary.get("seed_eligibility_selected_count") != 0:
    die("seed eligibility must not be selected")

classes = data.get("normalized_class_summary") or {}
expected_classes = {
    "ForbiddenNonClaimBoundary": 12,
    "NarrowRefreshScopeExclusion": 6,
    "ScopeExclusionBoundary": 16,
}
if classes != expected_classes:
    die(f"normalized class summary drift: {classes}")

rows = data.get("normalized_boundary_rows") or []
if len(rows) != 3:
    die("normalized boundary rows must be 3")
for row in rows:
    if row.get("result_carrier_projection_policy_covered") is not True:
        die("result carrier projection policy must be covered")
    if row.get("bridge_state_after_normalization") != "BridgeBlocked":
        die("row must remain bridge blocked after normalization")
    if row.get("blocked_by_after_normalization") != ["StrictCandidateSelectionNormalizedRerunRequired"]:
        die("normalized row blocked reason mismatch")
    for boundary in row.get("normalized_boundaries") or []:
        if boundary.get("seed_eligibility_evidence") is not False:
            die("boundary must not be seed eligibility evidence")
        if boundary.get("class") == "ForbiddenNonClaimBoundary" and boundary.get("transport_gap") is not False:
            die("forbidden non-claim must not be a transport gap")
        if boundary.get("class") in {"ScopeExclusionBoundary", "NarrowRefreshScopeExclusion"}:
            if boundary.get("transport_gap") is not False:
                die("scope exclusions must not be transport gaps")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectStrictConverterEmissionNativeSeedCandidateSelectionNormalizedRerun":
    die("decision kind mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-NORMALIZED-RERUN-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "manual_boundary_reclassification",
    "seed_eligibility_selected",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
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

if claims.get("strict_candidate_selection_rerun_consumed") != 1:
    die("1986 rerun must be consumed")
if claims.get("denied_boundary_vocabulary_normalized") != 1:
    die("vocabulary normalized claim missing")

print("[strict-denied-boundary-vocabulary-normalization-guard] OK")
PY
