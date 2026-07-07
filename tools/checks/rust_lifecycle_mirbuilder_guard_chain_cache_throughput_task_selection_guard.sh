#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-guard-chain-cache-throughput-task-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3338-MIRBUILDER-GUARD-CHAIN-CACHE-THROUGHPUT-TASK-SELECTION-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-guard-chain-cache-throughput-task-selection-v0.json"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
COMMON="$ROOT_DIR/tools/checks/lib/guard_common.sh"
RESULT_CACHE_GUARD="$ROOT_DIR/tools/checks/guard_result_cache_helper_guard.sh"
MIR_CACHE_GUARD="$ROOT_DIR/tools/checks/hako_mir_json_cache_wrapper_guard.sh"
EXE_CACHE_GUARD="$ROOT_DIR/tools/checks/hako_emit_exe_cache_wrapper_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" \
  "$COMMON" "$RESULT_CACHE_GUARD" "$MIR_CACHE_GUARD" "$EXE_CACHE_GUARD"

RESULT_OUT="$(guard_cached_run "$TAG" bash "$RESULT_CACHE_GUARD")"
if ! grep -q '^cached_command_executed_once=1$' <<<"$RESULT_OUT"; then
  printf '%s\n' "$RESULT_OUT" >&2
  guard_fail "$TAG" "guard result cache helper prerequisite is not green"
fi

python3 - "$CARD" "$FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" "$COMMON" <<'PY'
import json
import sys
from pathlib import Path

card = Path(sys.argv[1]).read_text(encoding="utf-8")
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
index = Path(sys.argv[5]).read_text(encoding="utf-8")
common = Path(sys.argv[6]).read_text(encoding="utf-8")

def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-GUARD-CHAIN-CACHE-THROUGHPUT-TASK-SELECTION-001"
next_card = "MIRBUILDER-GUARD-CHAIN-CACHE-PREFLIGHT-001"
resume = "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001"

need(f"# 3338 - {token}" in card, "card token drift")
need(fixture.get("kind") == "MirBuilderGuardChainCacheThroughputTaskSelectionV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need((fixture.get("decision") or {}).get("selected_next_card") == next_card, "selected next drift")
need((fixture.get("decision") or {}).get("resume_after") == resume, "resume card drift")
need("guard_cached_run()" in common, "guard_cached_run helper missing")
need("HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY" in common, "dirty cache env key missing")

claims = fixture.get("claims") or {}
for key in [
    "guard_chain_cache_task_selected",
    "existing_guard_result_cache_available",
    "existing_hako_mir_json_cache_wrapper_available",
    "existing_hako_emit_exe_cache_wrapper_available",
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

need(f'latest_card = "{token}"' in state, "CURRENT_STATE latest card drift")
need(token in task_order, "task-order missing task token")
need(next_card in task_order, "task-order missing selected next")
need(resume in task_order, "task-order missing resume card")
need("tools/checks/rust_lifecycle_mirbuilder_guard_chain_cache_throughput_task_selection_guard.sh" in index, "check index missing guard")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-guard-chain-cache-throughput-task-selection-v0
token=MIRBUILDER-GUARD-CHAIN-CACHE-THROUGHPUT-TASK-SELECTION-001
decision=SelectGuardChainCachePreflight
guard_chain_cache_task_selected=1
existing_guard_result_cache_available=1
existing_hako_mir_json_cache_wrapper_available=1
existing_hako_emit_exe_cache_wrapper_available=1
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
selected_next_card=MIRBUILDER-GUARD-CHAIN-CACHE-PREFLIGHT-001
resume_after=MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
summary=ok
REPORT
