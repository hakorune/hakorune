#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_in_body_step_validation_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1915-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1915-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckInBodyStepValidationProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["selected_subcluster_id"] != "InBodyStepValidation":
    raise SystemExit("selected subcluster drift")
if state["source_count"] != 2:
    raise SystemExit("source count drift")

surfaces = fixture["source_surfaces"]
if [surface["symbol"] for surface in surfaces] != ["validate_in_body_step", "validate_in_body_step_v1"]:
    raise SystemExit(f"source surface drift: {surfaces}")
for surface in surfaces:
    if surface["validator_role"] != "strict_reject_tail_statement_validator":
        raise SystemExit("validator role drift")
    if surface["return_type"] != "Result<bool, Freeze>":
        raise SystemExit("return type drift")

descriptor = fixture["validator_descriptor"]
if descriptor["descriptor_id"] != "generic_loop_body_check_in_body_step_validation_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_strict_reject_tail_validation":
    raise SystemExit("source extraction drift")
if descriptor["entry_count"] != 2:
    raise SystemExit("entry count drift")
if descriptor["return_contract"] != "Result<bool, Freeze>":
    raise SystemExit("return contract drift")
if descriptor["reject_dispatch"] != "reject_or_false(strict, reason.as_freeze_message())":
    raise SystemExit("reject dispatch drift")

entries = {entry["symbol"]: entry for entry in descriptor["entries"]}
v0 = entries["validate_in_body_step"]
if v0["version"] != "v0":
    raise SystemExit("v0 version drift")
if v0["strict_continue_policy"] != "RejectAnyContinueInBodyBeforeTailScan":
    raise SystemExit("v0 continue policy drift")
if "InBodyStepWithContinue" not in v0["reject_reasons"]:
    raise SystemExit("v0 continue reject reason missing")
if v0["log_tags"] != ["generic_loop", "generic_loop_v0"]:
    raise SystemExit("v0 log tag drift")

v1 = entries["validate_in_body_step_v1"]
if v1["version"] != "v1":
    raise SystemExit("v1 version drift")
if v1["strict_continue_policy"] != "AllowContinueBeforeTailScan":
    raise SystemExit("v1 continue policy drift")
if "InBodyStepWithContinue" in v1["reject_reasons"]:
    raise SystemExit("v1 must not reject body continue before tail scan")
if v1["log_tags"] != ["generic_loop_v1"]:
    raise SystemExit("v1 log tag drift")

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
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTINUE-IF-STEP-VALIDATION-PROJECTION-POLICY-001":
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
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-v0
subcluster=InBodyStepValidation
source_count=2
policy=SourceExtractedStrictRejectValidationDescriptor
validator_descriptor_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTINUE-IF-STEP-VALIDATION-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
