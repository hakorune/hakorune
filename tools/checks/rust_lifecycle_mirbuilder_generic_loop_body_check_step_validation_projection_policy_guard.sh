#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-step-validation-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_step_validation_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-step-validation-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1913-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-step-validation-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1913-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckStepValidationProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["source_subcluster_id"] != "BodyCheckStepValidation":
    raise SystemExit("source subcluster drift")
if state["source_count"] != 5:
    raise SystemExit("source count drift")
if not state["source_module"].endswith("body_check/step_validation.rs"):
    raise SystemExit("source module drift")

expected_counts = {
    "BreakElseIfStepValidation": 1,
    "ContinueIfStepValidation": 1,
    "InBodyStepValidation": 2,
    "TailControlFlowProbe": 1,
}
if fixture["step_validation_subcluster_counts"] != expected_counts:
    raise SystemExit(f"subcluster count drift: {fixture['step_validation_subcluster_counts']}")
if len(fixture["source_surfaces"]) != 5:
    raise SystemExit("source surface count drift")
if len({surface["source_id"] for surface in fixture["source_surfaces"]}) != 5:
    raise SystemExit("source surfaces must be classified exactly once")

subclusters = {item["step_validation_subcluster_id"]: item for item in fixture["step_validation_subclusters"]}
if set(subclusters) != set(expected_counts):
    raise SystemExit(f"subcluster id drift: {sorted(subclusters)}")
if subclusters["TailControlFlowProbe"]["selection_eligible"] is not True:
    raise SystemExit("TailControlFlowProbe must be selected first")
for name, item in subclusters.items():
    if name != "TailControlFlowProbe" and item["selection_eligible"] is not False:
        raise SystemExit(f"only TailControlFlowProbe may be selection eligible: {name}")

policy = fixture["decomposition_policy"]
if policy["whole_step_validation_projection_selected"] is not False:
    raise SystemExit("whole step validation projection must not be selected")
if policy["module_role_decomposition"] is not True:
    raise SystemExit("module-role decomposition claim missing")
if policy["strict_reject_semantics_isolated"] is not True:
    raise SystemExit("strict/reject semantics must be isolated")
if policy["candidate_count_as_proof"] != 0:
    raise SystemExit("candidate count must not be proof")

decision = fixture["decision"]
if decision["kind"] != "SelectStepValidationSubcluster":
    raise SystemExit("decision kind drift")
if decision["selected_step_validation_subcluster_id"] != "TailControlFlowProbe":
    raise SystemExit("selected subcluster drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "whole_step_validation_projection",
    "projection_surface_selected",
    "candidate_count_as_proof",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")

provenance = fixture["provenance"]
if provenance["tool_role"] != "FactsAdapterGuardOrchestrator":
    raise SystemExit("tool role drift")
if provenance["semantic_projection_inference"] != 0:
    raise SystemExit("tool must not infer semantic projection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-step-validation-projection-policy-v0
source_subcluster=BodyCheckStepValidation
source_count=5
subcluster_count=4
selected_subcluster=TailControlFlowProbe
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001
whole_step_validation_projection=0
candidate_count_as_proof=0
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
