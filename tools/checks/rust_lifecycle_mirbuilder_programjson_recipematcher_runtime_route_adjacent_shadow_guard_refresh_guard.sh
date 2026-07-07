#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-runtime-route-adjacent-shadow-guard-refresh-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-runtime-route-adjacent-shadow-guard-refresh-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3322-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-REFRESH-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_runtime_route_adjacent_shadow_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$PREV_GUARD"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GUARD")"
for required in \
  '^runtime_route_adjacent_shadow_guard=1$' \
  '^runtime_authority=rust_astnode$' \
  '^programjson_runtime_route_authority=0$' \
  '^runtime_route_switch=0$' \
  '^route_selection=0$' \
  '^source_selfhost_claim=0$'
do
  if ! grep -q "$required" <<<"$PREV_OUT"; then
    printf '%s\n' "$PREV_OUT" >&2
    guard_fail "$TAG" "runtime-adjacent shadow guard prerequisite missing $required"
  fi
done

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-REFRESH-001"
next_card = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonRecipeMatcherRuntimeRouteAdjacentShadowGuardRefreshV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001", "bad prerequisite")
need(fixture.get("decision", {}).get("selected_next_card") == next_card, "bad selected next")

scope = fixture.get("refresh_scope") or {}
need(scope.get("runtime_authority") == "Rust ASTNode route", "runtime authority drift")
need(scope.get("programjson_route") == "shadow_only", "ProgramJSON route drift")
need(scope.get("boundary") == "after try_build_outcome(ctx), before registry candidate selection", "boundary drift")

claims = fixture.get("claims") or {}
for key in [
    "runtime_route_adjacent_shadow_guard_refresh",
    "runtime_route_adjacent_shadow_guard_green",
    "runtime_authority_remains_rust_astnode",
    "programjson_shadow_checked_by_lifecycle_gate",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "programjson_runtime_route_authority",
    "runtime_route_switch",
    "recipe_matcher_input_authority",
    "route_selection",
    "mir_lowering",
    "mir_mutation",
    "id_allocation",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "Rust remains runtime authority.",
    "ProgramJSON remains shadow-only evidence.",
    "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
]:
    need(needle in card, f"card missing {needle}")
need(f'latest_card = "{token}"' in current_state, "CURRENT_STATE latest card drift")
need(f"next_documented_task =\n  {next_card}" in task_order, "task-order next task drift")
need("SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001 -> SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007" in task_order, "task-order next chain drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-runtime-route-adjacent-shadow-guard-refresh-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-REFRESH-001
runtime_route_adjacent_shadow_guard_refresh=1
runtime_route_adjacent_shadow_guard_green=1
runtime_authority=rust_astnode
programjson_shadow_checked_by_lifecycle_gate=1
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
summary=ok
REPORT
