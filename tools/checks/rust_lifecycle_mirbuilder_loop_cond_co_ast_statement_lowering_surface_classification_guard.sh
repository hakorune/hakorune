#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-cond-co-ast-statement-lowering-surface-classification-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-statement-lowering-surface-classification-v0.json"
INPUT_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-statement-lowering-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1865-MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-SURFACE-CLASSIFICATION-001.md"
SOURCE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$INPUT_FIXTURE" "$CARD" "$SOURCE"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-statement-lowering-surface-classification-v0.json").read_text())
input_fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-ast-statement-lowering-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1865-MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-SURFACE-CLASSIFICATION-001.md").read_text()
source = Path("src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs").read_text()

token = "MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-SURFACE-CLASSIFICATION-001"
if fixture.get("kind") != "MirBuilderLoopCondCoAstStatementLoweringSurfaceClassificationV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token or token not in card:
    raise SystemExit("token mismatch")
if input_fixture.get("decision", {}).get("selected_next_card") != token:
    raise SystemExit("input projection policy does not point to this classification")
if fixture.get("input_state", {}).get("source_surface") != "lower_stmt_ast":
    raise SystemExit("source surface drift")
if "pub(super) fn lower_stmt_ast" not in source:
    raise SystemExit("lower_stmt_ast visibility marker missing")

shapes = fixture.get("shape_inventory") or []
shape_ids = [shape.get("shape_id") for shape in shapes]
if len(shapes) != 10 or len(set(shape_ids)) != len(shape_ids):
    raise SystemExit("shape inventory must contain 10 unique shapes")
if fixture.get("summary", {}).get("shape_count") != len(shapes):
    raise SystemExit("shape summary count drift")

for shape in shapes:
    for marker in shape.get("ast_markers") or []:
        if marker not in source:
            raise SystemExit(f"AST marker missing from source: {shape['shape_id']} {marker}")
    for marker in shape.get("evidence_markers") or []:
        if marker not in source:
            raise SystemExit(f"evidence marker missing from source: {shape['shape_id']} {marker}")
    if shape.get("classification") == "RejectStatementShape" and shape.get("eligible_for_projection_policy"):
        raise SystemExit(f"reject shape must not be projection-eligible: {shape['shape_id']}")
    if not shape.get("reason_token"):
        raise SystemExit(f"shape lacks reason token: {shape['shape_id']}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectShapeProjectionPolicy":
    raise SystemExit("decision kind drift")
if decision.get("selected_shape_id") != "AssignmentStatementShape":
    raise SystemExit("selected shape drift")
if decision.get("selected_next_card") != "MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "projection_policy_selected",
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
output_contract=rust-lifecycle-mirbuilder-loop-cond-co-ast-statement-lowering-surface-classification-v0
shape_count=10
decision=SelectShapeProjectionPolicy
selected_shape=AssignmentStatementShape
selected_next_card=MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
