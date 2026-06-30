#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-step-validation-closeout-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_step_validation_closeout.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-step-validation-closeout-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1919-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-step-validation-closeout-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1919-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckStepValidationCloseoutV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

closed = fixture["closed_subcluster"]
if closed["subcluster_id"] != "BodyCheckStepValidation":
    raise SystemExit("closed subcluster drift")
if closed["materialized_leaf_count"] != 4:
    raise SystemExit("materialized leaf count drift")
if closed["dispatch_resolution_selected"] is not True:
    raise SystemExit("dispatch resolution must be selected")

expected = {
    "tail_probe_descriptor": "generic_loop_body_check_tail_control_flow_probe_v1",
    "in_body_descriptor": "generic_loop_body_check_in_body_step_validation_v1",
    "continue_if_descriptor": "generic_loop_body_check_continue_if_step_validation_v1",
    "break_else_if_descriptor": "generic_loop_body_check_break_else_if_step_validation_v1",
    "step_kind_dispatch": "SourceExtractedStepKindDispatchResolution",
}
if fixture["consumed_descriptors"] != expected:
    raise SystemExit("consumed descriptors drift")

boundary = fixture["closeout_boundary"]
if boundary["docs_only_closeout"] != 0:
    raise SystemExit("docs-only closeout must be forbidden")
if boundary["machine_checkable_fixture"] != 1:
    raise SystemExit("machine-checkable fixture must be present")
if boundary["hako_projection_selected"] != 0:
    raise SystemExit("Hako projection must not be selected")
if boundary["next_owner_returned_to_priority_resolver"] != 1:
    raise SystemExit("closeout must return to priority resolver")

decision = fixture["decision"]
if decision["kind"] != "CloseSubclusterAndReturnToPriorityResolver":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key, expected_value in {
    "docs_only_closeout": 0,
    "machine_checkable_fixture": 1,
    "manual_family_selection": 0,
    "hako_projection_selected": 0,
    "hako_generation": 0,
    "hako_adopted_decision": 0,
    "native_seed_materialization": 0,
    "source_selfhost_claim": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
    "new_python_semantic_projector": 0,
    "runner_semantic_owner": 0,
}.items():
    if claims.get(key) != expected_value:
        raise SystemExit(f"claim drift: {key}")

provenance = fixture["provenance"]
if provenance["tool_role"] != "FactsAdapterGuardOrchestrator":
    raise SystemExit("tool role drift")
if provenance["semantic_projection_inference"] != 0:
    raise SystemExit("tool must not infer semantic projection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-step-validation-closeout-v0
closed_subcluster=BodyCheckStepValidation
materialized_leaf_count=4
dispatch_resolution_selected=1
docs_only_closeout=0
selected_next_card=MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
