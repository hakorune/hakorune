#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-statement-value-construction-box-field-initialization-hako-shadow-parity-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PROJECTOR="$ROOT_DIR/lang/src/compiler/lib/box_field_initialization_projector.hako"
SUPPORT="$ROOT_DIR/lang/src/compiler/lib/projector_support.hako"
README="$ROOT_DIR/lang/src/compiler/lib/README.md"
TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_statement_value_construction_box_field_initialization_hako_shadow_parity.py"
CONTRACT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json"
SHADOW_RESULT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-hako-shadow-result-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1928-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-SHADOW-PARITY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ROOT_DIR/tools/bin/hako" "$PROJECTOR" "$SUPPORT" "$README" "$TOOL" "$CONTRACT" "$SHADOW_RESULT" "$CARD"

bash "$ROOT_DIR/tools/bin/hako" --backend mir --verify "$PROJECTOR"
python3 "$TOOL" --check

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

projector = Path("lang/src/compiler/lib/box_field_initialization_projector.hako").read_text()
support = Path("lang/src/compiler/lib/projector_support.hako").read_text()
readme = Path("lang/src/compiler/lib/README.md").read_text()
contract = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json").read_text())
shadow_result = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-hako-shadow-result-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1928-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-SHADOW-PARITY-001.md").read_text()

token = "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-SHADOW-PARITY-001"
if token not in card:
    raise SystemExit("card token missing")

required_projector_text = [
    "BoxFieldInitializationHakoProjector",
    "project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    "project_shadow_json(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    'CompilerProjectionValueBox.object_set(shadow_record, "kind", CompilerProjectionValueBox.create_string("BoxFieldInitializationHakoShadowProjectorV1"))',
    'CompilerProjectionValueBox.object_set(shadow_record, "family_id", CompilerProjectionValueBox.create_string("hakorune_mir_builder::statement_value_construction_box_field_initialization"))',
    'CompilerProjectionValueBox.object_set(shadow_record, "stage_id", CompilerProjectionValueBox.create_string("box_field_initialization"))',
    'CompilerProjectorSupportBox.ok(shadow_record)',
]
for needle in required_projector_text:
    if needle not in projector:
        raise SystemExit(f"missing box field initialization projector text: {needle}")

for needle in ["CanonicalJsonWriterBox.canonicalize(shadow_record)", "ok(shadow_record)"]:
    if needle not in support:
        raise SystemExit(f"missing projector support text: {needle}")

if "box_field_initialization_projector.hako" not in readme:
    raise SystemExit("README must keep the box field initialization projector landing zone visible")

if contract.get("kind") != "MirBuilderStatementValueConstructionBoxFieldInitializationMutationFrameContractV1":
    raise SystemExit("contract kind drift")
frame = contract.get("mutation_frame_contract") or {}
if frame.get("delegated_mutation_owner") != "build_field_assignment_from_value":
    raise SystemExit("contract delegated mutation owner drift")
if frame.get("mutation_order") != [
    "RejectRecordConstructorFieldInitializers",
    "CreateDestinationBox",
    "InitializeSeenFieldSet",
    "RejectDuplicateInitializerField",
    "ValidateUserDefinedBoxFieldMembership",
    "DelegateFieldAssignmentForInitializer",
    "ReturnDestinationValue",
]:
    raise SystemExit("contract mutation order drift")

if shadow_result.get("kind") != "MirBuilderStatementValueConstructionBoxFieldInitializationHakoShadowResultV1":
    raise SystemExit("shadow result kind drift")
result = shadow_result.get("result") or {}
if result.get("err") != 0 or result.get("err_line") != "":
    raise SystemExit("shadow result must be green")
shadow_record = result.get("shadow_record") or {}
if shadow_record.get("kind") != "BoxFieldInitializationShadowCandidateV1":
    raise SystemExit("shadow record kind drift")
if shadow_record.get("family_id") != "hakorune_mir_builder::statement_value_construction_box_field_initialization":
    raise SystemExit("shadow record family drift")
if shadow_record.get("stage_id") != "box_field_initialization":
    raise SystemExit("shadow record stage drift")
if shadow_record.get("source_authority") != contract.get("input_state", {}).get("source_surfaces"):
    raise SystemExit("shadow source authority drift")
if shadow_record.get("mutation_frame_contract") != frame:
    raise SystemExit("shadow mutation-frame contract drift")
if result.get("shadow_json") != json.dumps(shadow_record, indent=2, sort_keys=True) + "\n":
    raise SystemExit("shadow canonical JSON drift")

stage = shadow_result.get("stage_state") or {}
expected_stage = {
    "family_id": "hakorune_mir_builder::statement_value_construction_box_field_initialization",
    "stage_id": "box_field_initialization",
    "hako_shadow": "BoxFieldInitializationHakoProjector",
    "promotion_token": "BoxFieldInitializationHakoShadowPromotionTokenV1",
    "retirement_token": "BoxFieldInitializationHakoShadowRetirementTokenV1",
}
for key, value in expected_stage.items():
    if stage.get(key) != value:
        raise SystemExit(f"stage-state drift: {key}")

non_claims = shadow_result.get("non_claims") or {}
for key in [
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
]:
    if non_claims.get(key) != 0:
        raise SystemExit(f"shadow result non-claim must remain 0: {key}")

print("output_contract=rust-lifecycle-mirbuilder-statement-value-construction-box-field-initialization-hako-shadow-parity-v0")
print("box_field_initialization_hako_shadow_parity=green")
print("projector=BoxFieldInitializationHakoProjector")
print("canonical_json_parity=green")
print("hako_generation=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
