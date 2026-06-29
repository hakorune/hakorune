#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-carrier-merge-assignment-statement-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-projection-policy-v0.json"
INPUT_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-assignment-statement-projection-policy-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1867-MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001.md"
SOURCE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/carrier_merge.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$INPUT_FIXTURE" "$REPORT" "$CARD" "$SOURCE"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-projection-policy-v0.json").read_text())
input_fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-assignment-statement-projection-policy-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1867-MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001.md").read_text()
source = Path("src/mir/builder/control_flow/plan/features/carrier_merge.rs").read_text()

token = "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderCarrierMergeAssignmentStatementProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token or token not in card:
    raise SystemExit("token mismatch")
if input_fixture.get("decision", {}).get("selected_next_card") != token:
    raise SystemExit("delegating assignment policy does not point to this card")
if fixture.get("selected_policy", {}).get("policy") != "MutationFrameContractRequired":
    raise SystemExit("selected policy drift")
if fixture.get("selected_policy", {}).get("projection_surface_selected") is not False:
    raise SystemExit("projection surface must not be selected before mutation-frame contract")

input_state = fixture.get("input_state") or {}
report_item = next((item for item in report.get("items", []) if item.get("source_id") == input_state.get("source_id")), None)
if not report_item:
    raise SystemExit("source report item missing")
if report_item.get("classification") != input_state.get("source_report_classification"):
    raise SystemExit("source report classification drift")
if report_item.get("reason_token") != input_state.get("source_report_reason_token"):
    raise SystemExit("source report reason drift")

for marker in [
    "pub(in crate::mir::builder) fn lower_assignment_stmt",
    "current_bindings.iter()",
    "builder.variable_ctx.variable_map.insert",
    "loop_body_lowering::lower_assignment_stmt",
    "let Some((name, value_id)) = binding else",
    "carrier_updates.insert",
    "current_bindings.insert",
]:
    if marker not in source:
        raise SystemExit(f"mutation-frame evidence marker missing: {marker}")

evidence = fixture.get("mutation_frame_evidence") or {}
for key, value in evidence.items():
    if value is not True:
        raise SystemExit(f"mutation-frame evidence must be true: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectMutationFrameContract":
    raise SystemExit("decision kind drift")
if decision.get("selected_next_card") != "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "projection_surface_selected",
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
output_contract=rust-lifecycle-mirbuilder-carrier-merge-assignment-statement-projection-policy-v0
policy=MutationFrameContractRequired
decision=SelectMutationFrameContract
projection_surface_selected=0
selected_next_card=MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
