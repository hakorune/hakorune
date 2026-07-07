#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-guard-chain-cache-preflight-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3339-MIRBUILDER-GUARD-CHAIN-CACHE-PREFLIGHT-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-guard-chain-cache-preflight-v0.json"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
COMMON="$ROOT_DIR/tools/checks/lib/guard_common.sh"
RESULT_CACHE_GUARD="$ROOT_DIR/tools/checks/guard_result_cache_helper_guard.sh"
DIRTY_CACHE_GUARD="$ROOT_DIR/tools/checks/guard_result_cache_dirty_untracked_memo_guard.sh"
MIR_CACHE_GUARD="$ROOT_DIR/tools/checks/hako_mir_json_cache_wrapper_guard.sh"
EXE_CACHE_GUARD="$ROOT_DIR/tools/checks/hako_emit_exe_cache_wrapper_guard.sh"
LOCALSSA_DESIGN_STOP_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_localssa_finalize_compare_design_stop_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" "$COMMON" \
  "$RESULT_CACHE_GUARD" "$DIRTY_CACHE_GUARD" "$MIR_CACHE_GUARD" "$EXE_CACHE_GUARD" \
  "$LOCALSSA_DESIGN_STOP_GUARD"

RESULT_OUT="$(guard_cached_run "$TAG" bash "$RESULT_CACHE_GUARD")"
DIRTY_OUT="$(guard_cached_run "$TAG" bash "$DIRTY_CACHE_GUARD")"
MIR_OUT="$(guard_cached_run "$TAG" bash "$MIR_CACHE_GUARD")"
EXE_OUT="$(guard_cached_run "$TAG" bash "$EXE_CACHE_GUARD")"

if ! grep -q '^cached_command_executed_once=1$' <<<"$RESULT_OUT"; then
  printf '%s\n' "$RESULT_OUT" >&2
  guard_fail "$TAG" "guard result cache helper is not green"
fi
if ! grep -q '^dirty_cache_requires_allow_dirty=1$' <<<"$DIRTY_OUT"; then
  printf '%s\n' "$DIRTY_OUT" >&2
  guard_fail "$TAG" "dirty-cache opt-in contract is not green"
fi
if ! grep -q '^cache_status=miss_then_hit$' <<<"$MIR_OUT"; then
  printf '%s\n' "$MIR_OUT" >&2
  guard_fail "$TAG" "MIR JSON cache wrapper is not green"
fi
if ! grep -q '^cache_status=miss_then_hit$' <<<"$EXE_OUT"; then
  printf '%s\n' "$EXE_OUT" >&2
  guard_fail "$TAG" "EXE cache wrapper is not green"
fi

python3 - "$CARD" "$FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" "$COMMON" "$LOCALSSA_DESIGN_STOP_GUARD" <<'PY'
import json
import sys
from pathlib import Path

card = Path(sys.argv[1]).read_text(encoding="utf-8")
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
index = Path(sys.argv[5]).read_text(encoding="utf-8")
common = Path(sys.argv[6]).read_text(encoding="utf-8")
localssa_guard = Path(sys.argv[7]).read_text(encoding="utf-8")

def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-GUARD-CHAIN-CACHE-PREFLIGHT-001"
previous = "MIRBUILDER-GUARD-CHAIN-CACHE-THROUGHPUT-TASK-SELECTION-001"
next_card = "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001"

need(f"# 3339 - {token}" in card, "card token drift")
need(fixture.get("kind") == "MirBuilderGuardChainCachePreflightV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("prerequisite") == previous, "fixture prerequisite drift")
need((fixture.get("decision") or {}).get("selected_next_card") == next_card, "selected next drift")
need((fixture.get("resume_contract") or {}).get("resume_after") == next_card, "resume target drift")
need((fixture.get("resume_contract") or {}).get("localssa_design_stop_guard_runs_after_current_moves_to_resume_card") is True, "resume execution contract drift")
need("guard_cached_run()" in common, "guard_cached_run helper missing")
need("HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY" in common, "dirty cache env key missing")
need(f'latest_card = "{token}"' in state, "CURRENT_STATE latest card drift")
need(token in task_order, "task-order missing preflight token")
need(next_card in task_order, "task-order missing LocalSSA resume target")
need("tools/checks/rust_lifecycle_mirbuilder_guard_chain_cache_preflight_guard.sh" in index, "check index missing preflight guard")
need("CURRENT_STATE latest card must point to prerequisite or 3315" in localssa_guard, "LocalSSA design-stop guard must keep latest-card boundary")

claims = fixture.get("claims") or {}
for key in [
    "guard_chain_cache_preflight",
    "guard_cached_run_miss_then_hit",
    "dirty_cache_remains_opt_in",
    "hako_mir_json_cache_wrapper_green",
    "hako_emit_exe_cache_wrapper_green",
    "localssa_design_stop_resume_target_recorded",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "cache_implementation_changed",
    "dirty_cache_default_changed",
    "localssa_finalize_compare_execution",
    "mir_compare_emission",
    "mir_branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-guard-chain-cache-preflight-v0
token=MIRBUILDER-GUARD-CHAIN-CACHE-PREFLIGHT-001
guard_chain_cache_preflight=1
guard_cached_run_miss_then_hit=1
dirty_cache_remains_opt_in=1
hako_mir_json_cache_wrapper_green=1
hako_emit_exe_cache_wrapper_green=1
localssa_design_stop_resume_target_recorded=1
cache_implementation_changed=0
dirty_cache_default_changed=0
localssa_finalize_compare_execution=0
mir_compare_emission=0
mir_branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
summary=ok
REPORT
