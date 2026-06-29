#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-carrier-merge-assignment-statement-hako-shadow-parity-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PROJECTOR="$ROOT_DIR/lang/src/compiler/lib/carrier_merge_assignment_projector.hako"
SUPPORT="$ROOT_DIR/lang/src/compiler/lib/projector_support.hako"
README="$ROOT_DIR/lang/src/compiler/lib/README.md"
CONTRACT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0.json"
SHADOW_RESULT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-hako-shadow-result-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1869-MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ROOT_DIR/tools/bin/hako" "$PROJECTOR" "$SUPPORT" "$README" "$CONTRACT" "$SHADOW_RESULT" "$CARD"

bash "$ROOT_DIR/tools/bin/hako" --backend mir --verify "$PROJECTOR"

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

projector = Path("lang/src/compiler/lib/carrier_merge_assignment_projector.hako").read_text()
support = Path("lang/src/compiler/lib/projector_support.hako").read_text()
readme = Path("lang/src/compiler/lib/README.md").read_text()
contract = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0.json").read_text())
shadow_result = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-hako-shadow-result-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1869-MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001.md").read_text()

token = "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001"
if token not in card:
    raise SystemExit("card token missing")

required_projector_text = [
    "CarrierMergeAssignmentHakoProjector",
    "project_shadow_record(contract, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    "project_shadow_json(contract, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    'CompilerProjectionValueBox.object_set(shadow_record, "kind", CompilerProjectionValueBox.create_string("CarrierMergeAssignmentHakoShadowProjectorV1"))',
    'CompilerProjectionValueBox.object_set(shadow_record, "family_id", CompilerProjectionValueBox.create_string("hakorune_mir_builder::carrier_merge_assignment"))',
    'CompilerProjectionValueBox.object_set(shadow_record, "stage_id", CompilerProjectionValueBox.create_string("carrier_merge_assignment"))',
    'CompilerProjectorSupportBox.ok(shadow_record)',
]
for needle in required_projector_text:
    if needle not in projector:
        raise SystemExit(f"missing carrier-merge assignment projector text: {needle}")

for needle in ["CanonicalJsonWriterBox.canonicalize(shadow_record)", "ok(shadow_record)"]:
    if needle not in support:
        raise SystemExit(f"missing projector support text: {needle}")

if "carrier_merge_assignment_projector.hako" not in readme:
    raise SystemExit("README must keep the carrier-merge assignment projector landing zone visible")

if contract.get("kind") != "MirBuilderCarrierMergeAssignmentStatementMutationFrameContractV1":
    raise SystemExit("contract kind drift")
frame = contract.get("mutation_frame_contract") or {}
if frame.get("read_only_inputs") != ["carrier_phis"]:
    raise SystemExit("contract read-only inputs drift")

if shadow_result.get("kind") != "MirBuilderCarrierMergeAssignmentHakoShadowResultV1":
    raise SystemExit("shadow result kind drift")
result = shadow_result.get("result") or {}
if result.get("err") != 0 or result.get("err_line") != "":
    raise SystemExit("shadow result must be green")
shadow_record = result.get("shadow_record") or {}
if shadow_record.get("kind") != "CarrierMergeAssignmentShadowCandidateV1":
    raise SystemExit("shadow record kind drift")
if shadow_record.get("family_id") != "hakorune_mir_builder::carrier_merge_assignment":
    raise SystemExit("shadow record family drift")
if shadow_record.get("stage_id") != "carrier_merge_assignment":
    raise SystemExit("shadow record stage drift")
if shadow_record.get("source_authority") != contract.get("input_state", {}).get("source_id"):
    raise SystemExit("shadow source authority drift")
if shadow_record.get("mutation_frame_contract") != frame:
    raise SystemExit("shadow mutation-frame contract drift")
if result.get("shadow_json") != json.dumps(shadow_record, indent=2, sort_keys=True) + "\n":
    raise SystemExit("shadow canonical JSON drift")

stage = shadow_result.get("stage_state") or {}
expected_stage = {
    "family_id": "hakorune_mir_builder::carrier_merge_assignment",
    "stage_id": "carrier_merge_assignment",
    "hako_shadow": "CarrierMergeAssignmentHakoProjector",
    "promotion_token": "CarrierMergeAssignmentHakoShadowPromotionTokenV1",
    "retirement_token": "CarrierMergeAssignmentHakoShadowRetirementTokenV1",
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

print("output_contract=rust-lifecycle-mirbuilder-carrier-merge-assignment-hako-shadow-parity-v0")
print("carrier_merge_assignment_hako_shadow_parity=green")
print("projector=CarrierMergeAssignmentHakoProjector")
print("canonical_json_parity=green")
print("hako_generation=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
