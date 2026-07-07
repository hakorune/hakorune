#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-nested-loop-decision-row-consultation"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-nested-loop-decision-row-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3252-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_authority_switch_readiness_consultation_guard.sh"
NESTED_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_nested_loop_reject_retire_rust_astnode_projector_candidate_guard.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$NESTED_GUARD" "$SNAPSHOT_IMPL" "$MATCHER_IMPL"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^selected_nested_loop_decision_next=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "readiness prerequisite did not select nested-loop decision"
fi

NESTED_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-nested" bash "$NESTED_GUARD")"
if ! grep -q '^parity_row=reject_nested_loop$' <<<"$NESTED_OUT"; then
  printf '%s\n' "$NESTED_OUT" >&2
  guard_fail "$TAG" "prior nested-loop reject evidence is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$PREV_OUT" "$NESTED_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, snapshot_path, matcher_path, prev_out, nested_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
snapshot_impl = Path(snapshot_path).read_text(encoding="utf-8")
matcher_impl = Path(matcher_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherNestedLoopDecisionRowConsultationV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

obs = fixture.get("observations") or {}
for key in [
    "programjson_can_observe_nested_loop",
    "canonical_loop_facts_snapshot_publishes_has_nested_loop",
    "nested_loop_should_not_be_accepted_floor",
]:
    if obs.get(key) != 1:
        raise SystemExit(f"missing observation: {key}")
if obs.get("current_matcher_boundary_rejects_nested_loop") != 0:
    raise SystemExit("current matcher boundary rejection must remain unclaimed")

if '"has_nested_loop" => me._loop_has_type(program_json, loop_body, "Loop")' not in snapshot_impl:
    raise SystemExit("snapshot does not publish has_nested_loop from loop-body scan")
if 'me._i(snapshot, "has_nested_loop")' in matcher_impl and 'nested_loop_present' not in matcher_impl:
    raise SystemExit("matcher boundary consumes has_nested_loop without stable nested_loop_present reject")

options = {row.get("option"): row for row in fixture.get("candidate_next_steps") or []}
if options.get("A_ACCEPT_NESTED_LOOP_MATCHER_ROW", {}).get("eligible") is not False:
    raise SystemExit("nested-loop accepted matcher row must be rejected")
selected = options.get("B_REJECT_BOUNDARY_IMPLEMENTATION") or {}
if selected.get("eligible") is not True or selected.get("selected_next_card") != next_card:
    raise SystemExit("reject-boundary implementation must be selected")
if options.get("C_SCAN_ONLY_DIAGNOSTIC", {}).get("eligible") is not False:
    raise SystemExit("scan-only diagnostic must not be selected")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectNestedLoopRejectBoundary":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad selected next card")

claims = fixture.get("claims") or {}
for key in ["nested_loop_decision_row_consultation", "selected_nested_loop_reject_boundary"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"nested_loop_decision_row_consultation", "selected_nested_loop_reject_boundary"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "B_REJECT_BOUNDARY_IMPLEMENTATION",
    "nested_loop_accepted_floor = 0",
    "programjson_runtime_route_authority = 0",
    "runtime_route_switch = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_card]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
for key in [
    "authority_switch_ready=0",
    "selected_nested_loop_decision_next=1",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
    "source_selfhost_claim=0",
]:
    if key not in prev_out:
        raise SystemExit(f"readiness prerequisite missing: {key}")
for key in [
    "parity_gate=green",
    "parity_row=reject_nested_loop",
    "rust_projector_runtime_dependency_removed=0",
    "source_selfhost_claim=0",
]:
    if key not in nested_out:
        raise SystemExit(f"nested-loop prerequisite missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-nested-loop-decision-row-consultation-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001
nested_loop_decision_row_consultation=1
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001
selected_nested_loop_reject_boundary=1
nested_loop_accepted_floor=0
nested_loop_reject_boundary_green=0
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
