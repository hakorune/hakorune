#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_statement_value_construction_box_field_initialization_mutation_frame_contract.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json"
INPUT_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1927-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$INPUT_FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json").read_text())
input_fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1927-MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001.md").read_text()

token = "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001"
if fixture.get("kind") != "MirBuilderStatementValueConstructionBoxFieldInitializationMutationFrameContractV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token or token not in card:
    raise SystemExit("token mismatch")
if input_fixture.get("decision", {}).get("selected_next_card") != token:
    raise SystemExit("input projection policy does not point to this contract")

contract = fixture.get("mutation_frame_contract") or {}
if contract.get("delegated_mutation_owner") != "build_field_assignment_from_value":
    raise SystemExit("delegated mutation owner drift")
if contract.get("local_only_state") != ["seen initializer field set"]:
    raise SystemExit("local-only state drift")
if contract.get("read_only_inputs") != [
    "record constructor classifier",
    "MirBuilder.comp_ctx.user_defined_boxes",
]:
    raise SystemExit("read-only inputs drift")
if contract.get("mutation_order") != [
    "RejectRecordConstructorFieldInitializers",
    "CreateDestinationBox",
    "InitializeSeenFieldSet",
    "RejectDuplicateInitializerField",
    "ValidateUserDefinedBoxFieldMembership",
    "DelegateFieldAssignmentForInitializer",
    "ReturnDestinationValue",
]:
    raise SystemExit("mutation order drift")
if "object field assignments through build_field_assignment_from_value" not in contract.get("state_outputs", []):
    raise SystemExit("field assignment output missing")

for section in fixture.get("source_order_sections") or []:
    source = Path(section["source_path"]).read_text()
    last = -1
    for marker in section["markers"]:
        index = source.find(marker)
        if index < 0:
            raise SystemExit(f"source marker missing: {marker}")
        if index <= last:
            raise SystemExit(f"source marker out of order: {marker}")
        last = index

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectHakoShadowParity":
    raise SystemExit("decision kind drift")
if decision.get("selected_next_card") != "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-SHADOW-PARITY-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "hako_generation",
    "hako_shadow_projector_selected",
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
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0
mutation_frame_contract_ready=1
decision=SelectHakoShadowParity
selected_next_card=MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-SHADOW-PARITY-001
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
