#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-normalized-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_native_seed_candidate_selection_normalized_rerun.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[strict-candidate-selection-normalized-rerun-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionNormalizedRerunV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-NORMALIZED-RERUN-001":
    die("fixture token mismatch")

pool = data.get("candidate_pool") or {}
expected = {
    "normalized_row_count": 3,
    "bridge_eligible_after_normalization_count": 0,
    "forbidden_nonclaim_blocked_count": 3,
    "unclassified_denied_boundary_count": 0,
}
for key, value in expected.items():
    if pool.get(key) != value:
        die(f"candidate pool drift: {key}")

rows = data.get("normalized_candidate_rows") or []
if len(rows) != 3:
    die("normalized candidate rows must be 3")
for row in rows:
    if row.get("bridge_state_after_normalized_rerun") != "BridgeBlocked":
        die("row must remain bridge blocked")
    if row.get("blocked_by_after_normalized_rerun") != ["ForbiddenNonClaimBoundaryStillDenied"]:
        die("row blocked reason mismatch")
    if row.get("next_card") is not None:
        die("row next card must remain null")

decision = data.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    die("decision must keep stopped")
if decision.get("reason_token") != "NoBridgeEligibleCandidateAfterDeniedBoundaryNormalization":
    die("decision reason mismatch")
if decision.get("selected_next_card") != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "manual_family_selection",
    "manual_boundary_reclassification",
    "seed_eligibility_from_forbidden_nonclaim",
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
if claims.get("denied_boundary_vocabulary_normalization_consumed") != 1:
    die("1987 normalization must be consumed")

print("[strict-candidate-selection-normalized-rerun-guard] OK")
PY
