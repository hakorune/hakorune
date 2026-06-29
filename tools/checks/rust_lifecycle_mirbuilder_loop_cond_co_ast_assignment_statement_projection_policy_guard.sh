#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-cond-co-ast-assignment-statement-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-assignment-statement-projection-policy-v0.json"
INPUT_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-statement-lowering-surface-classification-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1866-MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001.md"
AST_SOURCE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs"
DELEGATE_SOURCE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/carrier_merge.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$INPUT_FIXTURE" "$REPORT" "$CARD" "$AST_SOURCE" "$DELEGATE_SOURCE"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-assignment-statement-projection-policy-v0.json").read_text())
input_fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-statement-lowering-surface-classification-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1866-MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001.md").read_text()
ast_source = Path("src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs").read_text()
delegate_source = Path("src/mir/builder/control_flow/plan/features/carrier_merge.rs").read_text()

token = "MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderLoopCondCoAstAssignmentStatementProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token or token not in card:
    raise SystemExit("token mismatch")
if input_fixture.get("decision", {}).get("selected_next_card") != token:
    raise SystemExit("input statement-shape classification does not point to this card")
if input_fixture.get("decision", {}).get("selected_shape_id") != "AssignmentStatementShape":
    raise SystemExit("input selected shape drift")
if fixture.get("selected_policy", {}).get("policy") != "DelegateToCarrierMergeAssignmentPolicy":
    raise SystemExit("selected policy drift")
if fixture.get("selected_policy", {}).get("projection_surface_selected") is not False:
    raise SystemExit("AST assignment arm must not be selected as projection surface")

for marker in ["ASTNode::Assignment", "lower_assignment_stmt", "effects_to_plans"]:
    if marker not in ast_source:
        raise SystemExit(f"AST assignment arm marker missing: {marker}")
if "pub(in crate::mir::builder) fn lower_assignment_stmt" not in delegate_source:
    raise SystemExit("delegated carrier-merge helper signature missing")
for marker in ["carrier_updates.insert", "builder.variable_ctx.variable_map.insert", "loop_body_lowering::lower_assignment_stmt"]:
    if marker not in delegate_source:
        raise SystemExit(f"delegated carrier-merge helper evidence missing: {marker}")

delegated = fixture.get("delegated_surface") or {}
report_item = next((item for item in report.get("items", []) if item.get("source_id") == delegated.get("source_id")), None)
if not report_item:
    raise SystemExit("delegated surface missing from source report")
if report_item.get("classification") != delegated.get("expected_report_classification"):
    raise SystemExit("delegated surface classification drift")
if report_item.get("reason_token") != delegated.get("expected_report_reason_token"):
    raise SystemExit("delegated surface reason drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectDelegatedProjectionPolicy":
    raise SystemExit("decision kind drift")
if decision.get("selected_next_card") != "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "projection_surface_selected",
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
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-loop-cond-co-ast-assignment-statement-projection-policy-v0
policy=DelegateToCarrierMergeAssignmentPolicy
decision=SelectDelegatedProjectionPolicy
projection_surface_selected=0
selected_next_card=MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
