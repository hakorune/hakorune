#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="current-state-pointer-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
MAX_ACTIVE_DOC_LINES=1000
MAX_TASK_ORDER_LINE_CHARS=500
MAX_LANDED_TAIL_ROWS=12

STATE_DOC="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
CURRENT_TASK_DOC="$ROOT_DIR/CURRENT_TASK.md"
NOW_DOC="$ROOT_DIR/docs/development/current/main/10-Now.md"
RESTART_DOC="$ROOT_DIR/docs/development/current/main/05-Restart-Quick-Resume.md"
POLICY_DOC="$ROOT_DIR/docs/development/current/main/design/current-docs-update-policy-ssot.md"
PHASE137X_README="$ROOT_DIR/docs/development/current/main/phases/phase-137x/README.md"
PHASE137X_TASKBOARD="$ROOT_DIR/docs/development/current/main/phases/phase-137x/137x-91-task-board.md"
STALE_PATTERNS_FILE="$ROOT_DIR/tools/checks/current_state_stale_pointer_patterns.txt"
DESIGN_STOP_CONTRACT_FILE="$ROOT_DIR/tools/checks/current_state_design_stop_contract.txt"

guard_require_command "$TAG" rg
guard_require_command "$TAG" sed
guard_require_command "$TAG" awk
guard_require_command "$TAG" wc
guard_require_files "$TAG" \
  "$STATE_DOC" \
  "$CURRENT_TASK_DOC" \
  "$NOW_DOC" \
  "$RESTART_DOC" \
  "$POLICY_DOC" \
  "$PHASE137X_README" \
  "$PHASE137X_TASKBOARD" \
  "$STALE_PATTERNS_FILE" \
  "$DESIGN_STOP_CONTRACT_FILE"

toml_scalar() {
  local key="$1"
  sed -n 's/^[[:space:]]*'"$key"'[[:space:]]*=[[:space:]]*"\(.*\)"[[:space:]]*$/\1/p' "$STATE_DOC" | head -n1
}

require_scalar() {
  local key="$1"
  local value
  value="$(toml_scalar "$key")"
  if [[ -z "$value" ]]; then
    guard_fail "$TAG" "CURRENT_STATE.toml missing scalar: $key"
  fi
  printf '%s' "$value"
}

count_landed_tail_rows() {
  awk '
    BEGIN {
      in_tail = 0
      found = 0
      count = 0
    }
    /^[[:space:]]*landed_tail[[:space:]]*=[[:space:]]*\[/ {
      in_tail = 1
      next
    }
    in_tail && /^[[:space:]]*\]/ {
      found = 1
      print count
      exit
    }
    in_tail && /^[[:space:]]*"/ {
      count += 1
    }
    END {
      if (!found) {
        exit 2
      }
    }
  ' "$STATE_DOC"
}

active_lane="$(require_scalar active_lane)"
active_phase="$(require_scalar active_phase)"
phase_status="$(require_scalar phase_status)"
work_mode="$(require_scalar work_mode)"
current_execution_row="$(require_scalar current_execution_row)"
current_execution_design="$(require_scalar current_execution_design)"
next_design_card="$(require_scalar next_design_card)"
next_execution_card="$(require_scalar next_execution_card)"
mirbuilder_north_star="$(require_scalar mirbuilder_north_star)"
method_anchor="$(require_scalar method_anchor)"
taskboard="$(require_scalar taskboard)"
latest_workstream_card="$(toml_scalar latest_workstream_card)"
blocker_token="$(require_scalar current_blocker_token)"
latest_card="$(require_scalar latest_card)"
latest_card_path="$(require_scalar latest_card_path)"
current_update_policy="$(require_scalar current_update_policy)"
pre_perf_gate="$(require_scalar pre_perf_gate)"
pre_perf_gate_status="$(require_scalar pre_perf_gate_status)"
optimization_return_lane="$(require_scalar optimization_return_lane)"

case "$work_mode" in
  fast|design_stop|closeout) ;;
  *) guard_fail "$TAG" "CURRENT_STATE.toml work_mode must be fast, design_stop, or closeout: $work_mode" ;;
esac

if [[ "$work_mode" == "design_stop" ]] && [[ -z "$(toml_scalar current_design_stop)" ]]; then
  guard_fail "$TAG" "design_stop work_mode requires current_design_stop"
fi

require_repo_file() {
  local rel="$1"
  local label="$2"
  if [[ "$rel" = /* ]]; then
    guard_fail "$TAG" "$label must be repo-relative: $rel"
  fi
  if [[ ! -f "$ROOT_DIR/$rel" ]]; then
    guard_fail "$TAG" "$label points to missing file: $rel"
  fi
}

echo "[$TAG] checking compact current state"

require_repo_file "$active_phase" "active_phase"
require_repo_file "$phase_status" "phase_status"
require_repo_file "$mirbuilder_north_star" "mirbuilder_north_star"
require_repo_file "$method_anchor" "method_anchor"
require_repo_file "$taskboard" "taskboard"
if [[ -n "$latest_workstream_card" ]]; then
  require_repo_file "$latest_workstream_card" "latest_workstream_card"
  active_doc_lines="$(wc -l < "$ROOT_DIR/$latest_workstream_card" | tr -d '[:space:]')"
  if (( active_doc_lines > MAX_ACTIVE_DOC_LINES )); then
    guard_fail "$TAG" "latest_workstream_card exceeds ${MAX_ACTIVE_DOC_LINES} lines: $latest_workstream_card has $active_doc_lines"
  fi
  active_doc_max_line_chars="$(awk '{ if (length($0) > max) max = length($0) } END { print max + 0 }' "$ROOT_DIR/$latest_workstream_card")"
  if (( active_doc_max_line_chars > MAX_TASK_ORDER_LINE_CHARS )); then
    guard_fail "$TAG" "latest_workstream_card exceeds ${MAX_TASK_ORDER_LINE_CHARS} characters per line: $latest_workstream_card has $active_doc_max_line_chars"
  fi
fi
require_repo_file "$latest_card_path" "latest_card_path"
require_repo_file "$current_execution_design" "current_execution_design"
require_repo_file "$current_update_policy" "current_update_policy"

if ! landed_tail_rows="$(count_landed_tail_rows)"; then
  guard_fail "$TAG" "CURRENT_STATE.toml missing landed_tail array"
fi
if (( landed_tail_rows > MAX_LANDED_TAIL_ROWS )); then
  guard_fail "$TAG" "CURRENT_STATE.toml landed_tail exceeds ${MAX_LANDED_TAIL_ROWS} rows: $landed_tail_rows"
fi

if [[ "$latest_card_path" != *"$latest_card"* ]]; then
  guard_fail "$TAG" "latest_card_path does not contain latest_card: $latest_card -> $latest_card_path"
fi

if ! rg -F -q -- "$current_execution_row" "$ROOT_DIR/$latest_card_path"; then
  guard_fail "$TAG" "current_execution_row is absent from latest_card_path: $current_execution_row -> $latest_card_path"
fi
case "$work_mode" in
  design_stop)
    if [[ "$next_design_card" != "$current_execution_row" ]]; then
      guard_fail "$TAG" "design_stop next_design_card must equal current_execution_row: $next_design_card != $current_execution_row"
    fi
    if [[ "$next_execution_card" != none* ]]; then
      guard_fail "$TAG" "design_stop next_execution_card must be none until the design stop is accepted: $next_execution_card"
    fi
    ;;
  fast)
    if [[ "$next_execution_card" != "$current_execution_row" ]]; then
      guard_fail "$TAG" "fast next_execution_card must equal current_execution_row: $next_execution_card != $current_execution_row"
    fi
    ;;
esac

for doc in "$CURRENT_TASK_DOC" "$NOW_DOC" "$RESTART_DOC"; do
  guard_expect_fixed_in_file "$TAG" "docs/development/current/main/CURRENT_STATE.toml" "$doc" "$(realpath --relative-to="$ROOT_DIR" "$doc") missing CURRENT_STATE token: docs/development/current/main/CURRENT_STATE.toml"
  guard_expect_fixed_in_file "$TAG" "active_lane" "$doc" "$(realpath --relative-to="$ROOT_DIR" "$doc") missing CURRENT_STATE token: active_lane"
  guard_expect_fixed_in_file "$TAG" "current_blocker_token" "$doc" "$(realpath --relative-to="$ROOT_DIR" "$doc") missing CURRENT_STATE token: current_blocker_token"
done

guard_require_design_stop_pause_contract "$TAG" "$ROOT_DIR" "$work_mode" "$DESIGN_STOP_CONTRACT_FILE"

guard_expect_fixed_in_file "$TAG" "docs/development/current/main/CURRENT_STATE.toml" "$PHASE137X_README" "$(realpath --relative-to="$ROOT_DIR" "$PHASE137X_README") missing CURRENT_STATE token: docs/development/current/main/CURRENT_STATE.toml"
guard_expect_fixed_in_file "$TAG" "active_lane" "$PHASE137X_README" "$(realpath --relative-to="$ROOT_DIR" "$PHASE137X_README") missing CURRENT_STATE token: active_lane"
guard_expect_fixed_in_file "$TAG" "current_blocker_token" "$PHASE137X_README" "$(realpath --relative-to="$ROOT_DIR" "$PHASE137X_README") missing CURRENT_STATE token: current_blocker_token"

guard_expect_fixed_in_file "$TAG" "$pre_perf_gate" "$PHASE137X_TASKBOARD" "$(realpath --relative-to="$ROOT_DIR" "$PHASE137X_TASKBOARD") missing CURRENT_STATE token: $pre_perf_gate"
guard_expect_fixed_in_file "$TAG" "$pre_perf_gate_status" "$PHASE137X_TASKBOARD" "$(realpath --relative-to="$ROOT_DIR" "$PHASE137X_TASKBOARD") missing CURRENT_STATE token: $pre_perf_gate_status"
guard_expect_fixed_in_file "$TAG" "$optimization_return_lane" "$PHASE137X_TASKBOARD" "$(realpath --relative-to="$ROOT_DIR" "$PHASE137X_TASKBOARD") missing CURRENT_STATE token: $optimization_return_lane"

while IFS= read -r pattern; do
  [[ -z "$pattern" ]] && continue
  [[ "$pattern" = \#* ]] && continue
  if hits="$(rg -n -F "$pattern" "$CURRENT_TASK_DOC" "$ROOT_DIR/docs/development/current/main" \
    --glob '!CURRENT_STATE.toml' \
    --glob '!archive/**' \
    2>/dev/null)"; then
    printf '%s\n' "$hits" >&2
    guard_fail "$TAG" "stale current pointer pattern found: $pattern"
  fi
done < "$STALE_PATTERNS_FILE"

echo "[$TAG] ok"
