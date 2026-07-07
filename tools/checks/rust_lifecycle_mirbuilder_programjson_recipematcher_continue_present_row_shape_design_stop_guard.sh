#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-continue-present-row-shape-design-stop"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-continue-present-row-shape-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3238-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-ROW-SHAPE-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
ACCEPTED_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_accepted_floor_matrix_guard.sh"
RUST_FACTS="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/feature_facts.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$ACCEPTED_GUARD" "$RUST_FACTS"

ACCEPTED_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-accepted" bash "$ACCEPTED_GUARD")"
if ! grep -q '^accepted_floor_matrix=1$' <<<"$ACCEPTED_OUT"; then
  printf '%s\n' "$ACCEPTED_OUT" >&2
  guard_fail "$TAG" "accepted floor matrix prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$RUST_FACTS" "$ACCEPTED_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, rust_facts_path, accepted_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
rust_facts = Path(rust_facts_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-ROW-SHAPE-DESIGN-STOP-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherContinuePresentRowShapeDesignStopV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

state = fixture.get("input_state") or {}
if state.get("rust_has_return_scope") != "loop body statements only":
    raise SystemExit("rust has_return scope drift")

boundary = fixture.get("observed_boundary") or {}
if boundary.get("kind") != "ContinuePresentRowShapeDecisionRequired":
    raise SystemExit("bad boundary kind")
if boundary.get("unsafe_shortcut_rejected") is not True:
    raise SystemExit("unsafe shortcut must be rejected")

candidates = {row.get("id"): row for row in fixture.get("candidate_shapes") or []}
if candidates.get("A_CONTINUE_PLUS_IN_BODY_RETURN_PLUS_ASSIGNMENT", {}).get("state") != "RecommendedDefault":
    raise SystemExit("candidate A must be recommended")
if candidates.get("B_RETURN_PLUS_CONTINUE_PLUS_ASSIGNMENT", {}).get("state") != "ConsultationAlternative":
    raise SystemExit("candidate B must remain consultation alternative")
if candidates.get("C_SCAN_ONLY_CONTINUE_ROW", {}).get("state") != "RejectedForAcceptedFloor":
    raise SystemExit("candidate C must be rejected for accepted floor")

decision = fixture.get("decision") or {}
if decision.get("kind") != "ConsultationRequired":
    raise SystemExit("decision must require consultation")
if decision.get("recommended_default") != "A_CONTINUE_PLUS_IN_BODY_RETURN_PLUS_ASSIGNMENT":
    raise SystemExit("wrong recommended default")
if decision.get("selected_next_card") != "CONSULTATION_REQUIRED":
    raise SystemExit("selected next must be consultation")

claims = fixture.get("claims") or {}
if claims.get("design_stop") != 1:
    raise SystemExit("missing design_stop claim")
for key, value in claims.items():
    if key == "design_stop":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    "ASTNode::Return { .. } => usage.has_return = true",
    "for nested in then_body",
    "for nested in else_body",
    "ASTNode::Loop { .. } | ASTNode::LoopRange { .. } => {}",
]:
    if needle not in rust_facts:
        raise SystemExit(f"Rust fact semantics missing: {needle}")

for needle in [
    token,
    "A. Continue plus in-body Return plus Assignment",
    "unsafe shortcut",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "CONSULTATION_REQUIRED", "A_CONTINUE_PLUS_IN_BODY_RETURN_PLUS_ASSIGNMENT"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-LOOP-BODY-IFCONTINUE-IFRETURN-ASSIGNMENT-BOXCOUNT-ACCEPTED-FLOOR-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-PRESENT-VERIFIED-RECIPE-SUPPORT-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
for key in [
    "accepted_floor_matrix=1",
    "continue_present_status=blocked_verified_recipe_missing",
    "selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001",
]:
    if key not in accepted_out:
        raise SystemExit(f"accepted matrix prerequisite missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-continue-present-row-shape-design-stop-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-ROW-SHAPE-DESIGN-STOP-001
design_stop=1
recommended_default=A_CONTINUE_PLUS_IN_BODY_RETURN_PLUS_ASSIGNMENT
selected_next_card=CONSULTATION_REQUIRED
continue_present_green=0
break_present_green=0
break_and_continue_present_green=0
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
