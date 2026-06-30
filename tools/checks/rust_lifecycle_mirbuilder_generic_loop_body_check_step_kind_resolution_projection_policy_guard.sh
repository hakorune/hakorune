#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_step_kind_resolution_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1918-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1918-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckStepKindResolutionProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

variants = fixture["step_placement_variants"]
if variants["variants"] != ["Last", "InBody", "InContinueIf", "InBreakElseIf"]:
    raise SystemExit("StepPlacement variants drift")

dispatch = fixture["dispatch_resolution"]
for version in ["v0", "v1"]:
    table = dispatch[version]["dispatch_table"]
    if [row["placement"] for row in table] != ["InBody", "InContinueIf", "InBreakElseIf", "LastOrOther"]:
        raise SystemExit(f"{version} dispatch placement order drift")
    if table[1]["validator"] != "validate_continue_if_step":
        raise SystemExit(f"{version} continue-if validator drift")
    if table[2]["validator"] != "validate_break_else_if_step":
        raise SystemExit(f"{version} break-else-if validator drift")
    if table[3]["validator"] != "accept_without_step_validator":
        raise SystemExit(f"{version} default validator drift")

if dispatch["v0"]["dispatch_table"][0]["validator"] != "validate_in_body_step":
    raise SystemExit("v0 in-body validator drift")
if dispatch["v1"]["dispatch_table"][0]["validator"] != "validate_in_body_step_v1":
    raise SystemExit("v1 in-body validator drift")

consumed = fixture["consumed_validator_descriptors"]
expected_consumed = {
    "tail_probe_descriptor": "generic_loop_body_check_tail_control_flow_probe_v1",
    "in_body_descriptor": "generic_loop_body_check_in_body_step_validation_v1",
    "continue_if_descriptor": "generic_loop_body_check_continue_if_step_validation_v1",
    "break_else_if_descriptor": "generic_loop_body_check_break_else_if_step_validation_v1",
}
if consumed != expected_consumed:
    raise SystemExit(f"consumed descriptor drift: {consumed}")

policy = fixture["selected_policy"]
if policy["policy"] != "SourceExtractedStepKindDispatchResolution":
    raise SystemExit("selected policy drift")
if policy["dispatch_resolution_selected"] is not True:
    raise SystemExit("dispatch resolution must be selected")
if policy["hako_projection_selected"] is not False:
    raise SystemExit("Hako projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectDispatchResolutionPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("dispatch_resolution_selected") != 1:
    raise SystemExit("dispatch resolution selected claim must be 1")
for key in [
    "manual_family_selection",
    "hako_projection_selected",
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
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy-v0
policy=SourceExtractedStepKindDispatchResolution
dispatch_resolution_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
