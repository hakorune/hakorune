#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-break-else-if-step-validation-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_break_else_if_step_validation_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-break-else-if-step-validation-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1917-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-BREAK-ELSE-IF-STEP-VALIDATION-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-break-else-if-step-validation-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1917-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-BREAK-ELSE-IF-STEP-VALIDATION-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-BREAK-ELSE-IF-STEP-VALIDATION-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckBreakElseIfStepValidationProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["selected_subcluster_id"] != "BreakElseIfStepValidation":
    raise SystemExit("selected subcluster drift")
if state["source_count"] != 1:
    raise SystemExit("source count drift")

surfaces = fixture["source_surfaces"]
if [surface["symbol"] for surface in surfaces] != ["validate_break_else_if_step"]:
    raise SystemExit(f"source surface drift: {surfaces}")
surface = surfaces[0]
if surface["validator_role"] != "strict_reject_final_statement_validator":
    raise SystemExit("validator role drift")
if surface["return_type"] != "Result<bool, Freeze>":
    raise SystemExit("return type drift")

descriptor = fixture["validator_descriptor"]
if descriptor["descriptor_id"] != "generic_loop_body_check_break_else_if_step_validation_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_break_else_if_final_statement_validation":
    raise SystemExit("source extraction drift")
if descriptor["entry_count"] != 1:
    raise SystemExit("entry count drift")
if descriptor["return_contract"] != "Result<bool, Freeze>":
    raise SystemExit("return contract drift")
if descriptor["reject_dispatch"] != "reject_or_false(strict, reason.as_freeze_message())":
    raise SystemExit("reject dispatch drift")

entry = descriptor["entries"][0]
if entry["symbol"] != "validate_break_else_if_step":
    raise SystemExit("entry symbol drift")
if entry["required_position"] != "final_statement":
    raise SystemExit("required position drift")
if entry["accept_condition"] != "step_index + 1 == body.len()":
    raise SystemExit("accept condition drift")
if entry["reject_reason"] != "BreakElseStepMustBeFinalStmt":
    raise SystemExit("reject reason drift")
if entry["log_tags"] != ["generic_loop_v0"]:
    raise SystemExit("log tag drift")

policy = fixture["selected_policy"]
if policy["policy"] != "SourceExtractedStrictRejectValidationDescriptor":
    raise SystemExit("selected policy drift")
if policy["validator_descriptor_selected"] is not True:
    raise SystemExit("validator descriptor must be selected")
if policy["hako_projection_selected"] is not False:
    raise SystemExit("Hako projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectValidatorDescriptorPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("validator_descriptor_selected") != 1:
    raise SystemExit("validator descriptor selected claim must be 1")
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
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-break-else-if-step-validation-projection-policy-v0
subcluster=BreakElseIfStepValidation
source_count=1
policy=SourceExtractedStrictRejectValidationDescriptor
validator_descriptor_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
