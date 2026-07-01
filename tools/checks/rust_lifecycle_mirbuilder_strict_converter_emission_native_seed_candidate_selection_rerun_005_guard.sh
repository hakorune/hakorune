#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-005-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_005.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[strict-candidate-selection-rerun-005-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionRerun005V1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-005":
    die("fixture token mismatch")

pool = data.get("candidate_pool") or {}
expected = {
    "input_owner_edge_count": 3,
    "already_hako_adopted_count": 3,
    "bridge_eligible_remaining_count": 0,
    "bridge_blocked_remaining_count": 0,
    "selected_candidate_count": 0,
}
for key, value in expected.items():
    if pool.get(key) != value:
        die(f"candidate pool drift: {key}")

for row in data.get("candidate_rows") or []:
    if row.get("already_hako_adopted") is not True:
        die("all rows must be adopted")
    if row.get("selection_eligible_after_adoption") is not False:
        die("adopted rows must not remain selectable")
    if row.get("blocked_by_after_adoption") != ["AlreadyHakoAdopted"]:
        die("adopted row blocker mismatch")
    if row.get("next_card") is not None:
        die("adopted row next_card must be null")

decision = data.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    die("decision kind mismatch")
if decision.get("reason_token") != "NoBridgeEligibleCandidateAfterTypedObjectPlanAdoption":
    die("decision reason mismatch")
if decision.get("selected_next_card") != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "previous_rerun_consumed",
    "typed_object_plan_refresh_adoption_consumed",
]:
    if claims.get(key) != 1:
        die(f"{key} must be 1")
for key in [
    "manual_family_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
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

print("[strict-candidate-selection-rerun-005-guard] OK")
PY
