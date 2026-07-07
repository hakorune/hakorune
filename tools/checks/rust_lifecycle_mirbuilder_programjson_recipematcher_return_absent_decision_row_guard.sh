#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-return-absent-decision-row"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-return-absent-decision-row-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3242-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_break_continue_present_verified_recipe_support_gate.sh"
MATCHER="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/matcher/mod.rs"
ROUTER="$ROOT_DIR/src/mir/builder/control_flow/joinir/route_entry/router.rs"
SNAPSHOT="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$MATCHER" "$ROUTER" "$SNAPSHOT"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^if_break_if_continue_if_return_assignment_supported=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "break+continue prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$MATCHER" "$ROUTER" "$SNAPSHOT" "$PREV_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, matcher_path, router_path, snapshot_path, prev_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
matcher = Path(matcher_path).read_text(encoding="utf-8")
router = Path(router_path).read_text(encoding="utf-8")
snapshot = Path(snapshot_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherReturnAbsentDecisionRowV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

state = fixture.get("input_state") or {}
if state.get("rust_has_return_scope") != "loop body statements only":
    raise SystemExit("bad has_return scope")
if state.get("programjson_snapshot_currently_requires_final_return") is not True:
    raise SystemExit("snapshot final-return requirement drift")
if state.get("runtime_release_boundary_depends_on_break_continue_without_return") is not True:
    raise SystemExit("runtime release boundary state drift")

boundary = fixture.get("observed_boundary") or {}
if boundary.get("kind") != "ReturnAbsentDecisionRequired":
    raise SystemExit("bad boundary kind")
if boundary.get("unsafe_shortcut_rejected") is not True:
    raise SystemExit("unsafe shortcut must be rejected")

candidates = {row.get("id"): row for row in fixture.get("candidate_decisions") or []}
if candidates.get("A_ACCEPT_RETURN_ABSENT_ACCEPTED_FLOOR_NOW", {}).get("state") != "RejectedForNow":
    raise SystemExit("candidate A must be rejected for now")
if candidates.get("B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION", {}).get("state") != "RecommendedDefault":
    raise SystemExit("candidate B must be recommended")
if candidates.get("C_SCAN_ONLY_RETURN_ABSENT_DIAGNOSTIC", {}).get("state") != "ConsultationAlternative":
    raise SystemExit("candidate C must remain alternative")

decision = fixture.get("decision") or {}
if decision.get("kind") != "ConsultationRequired":
    raise SystemExit("decision must require consultation")
if decision.get("recommended_default") != "B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION":
    raise SystemExit("wrong recommended default")
if decision.get("selected_next_card") != "CONSULTATION_REQUIRED":
    raise SystemExit("selected next must be consultation")

claims = fixture.get("claims") or {}
if claims.get("design_stop") != 1:
    raise SystemExit("missing design stop claim")
for key, value in claims.items():
    if key == "design_stop":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    "let has_return = facts.exit_kinds_present.contains(&ExitKindFacts::Return)",
    "RecipeContractKind::LoopWithExit",
    "has_return,",
]:
    if needle not in matcher:
        raise SystemExit(f"matcher source missing: {needle}")
for needle in [
    "facts.exit_usage.has_break && facts.exit_usage.has_continue",
    "facts.exit_usage.has_return",
    "loop_cond.release_allowed()",
]:
    if needle not in router:
        raise SystemExit(f"router source missing: {needle}")
old_final_return_requirement = all(needle in snapshot for needle in [
    "if third < 0 { return me._err(\"missing_final_return\") }",
    "if me._token_eq(me._node_type(program_json, third), \"Return\") != 1 { return me._err(\"final_stmt_not_return\") }",
])
decoupled_final_return_boundary = all(needle in snapshot for needle in [
    "_final_top_level_return_present",
    "\"final_top_level_return_used_for_loop_body_has_return\" => 0",
])
if not old_final_return_requirement and not decoupled_final_return_boundary:
    raise SystemExit("snapshot final-return boundary marker missing")
if "local has_return = me._loop_has_type(program_json, loop_body, \"Return\")" not in snapshot:
    raise SystemExit("snapshot loop-body has_return source missing")

for needle in [
    token,
    "B. Defer return_absent to route-release consultation",
    "return_absent_green = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "CONSULTATION_REQUIRED", "B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001",
    "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
for key in [
    "if_break_if_continue_if_return_assignment_supported=1",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
]:
    if key not in prev_out:
        raise SystemExit(f"previous guard output missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-return-absent-decision-row-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001
design_stop=1
recommended_default=B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION
selected_next_card=CONSULTATION_REQUIRED
return_absent_green=0
return_absent_accepted_floor=0
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
