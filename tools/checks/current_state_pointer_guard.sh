#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="current-state-pointer-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STATE_DOC="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
CURRENT_TASK_DOC="$ROOT_DIR/CURRENT_TASK.md"
NOW_DOC="$ROOT_DIR/docs/development/current/main/10-Now.md"
RESTART_DOC="$ROOT_DIR/docs/development/current/main/05-Restart-Quick-Resume.md"
PHASE137X_README="$ROOT_DIR/docs/development/current/main/phases/phase-137x/README.md"
PHASE137X_TASKBOARD="$ROOT_DIR/docs/development/current/main/phases/phase-137x/137x-91-task-board.md"
STALE_PATTERNS_FILE="$ROOT_DIR/tools/checks/current_state_stale_pointer_patterns.txt"

guard_require_command "$TAG" rg
guard_require_command "$TAG" sed
guard_require_command "$TAG" awk
guard_require_files "$TAG" \
  "$STATE_DOC" \
  "$CURRENT_TASK_DOC" \
  "$NOW_DOC" \
  "$RESTART_DOC" \
  "$PHASE137X_README" \
  "$PHASE137X_TASKBOARD" \
  "$STALE_PATTERNS_FILE"

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

expect_fixed() {
  local needle="$1"
  local file="$2"
  if ! rg -Fq "$needle" "$file"; then
    guard_fail "$TAG" "$(realpath --relative-to="$ROOT_DIR" "$file") missing CURRENT_STATE token: $needle"
  fi
}

active_lane="$(require_scalar active_lane)"
active_phase="$(require_scalar active_phase)"
phase_status="$(require_scalar phase_status)"
method_anchor="$(require_scalar method_anchor)"
taskboard="$(require_scalar taskboard)"
blocker_token="$(require_scalar current_blocker_token)"
latest_card="$(require_scalar latest_card)"
latest_card_path="$(require_scalar latest_card_path)"
current_update_policy="$(require_scalar current_update_policy)"
pre_perf_gate="$(require_scalar pre_perf_gate)"
pre_perf_gate_status="$(require_scalar pre_perf_gate_status)"
optimization_return_lane="$(require_scalar optimization_return_lane)"

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
require_repo_file "$method_anchor" "method_anchor"
require_repo_file "$taskboard" "taskboard"
require_repo_file "$latest_card_path" "latest_card_path"
require_repo_file "$current_update_policy" "current_update_policy"

if [[ "$latest_card_path" != *"$latest_card"* ]]; then
  guard_fail "$TAG" "latest_card_path does not contain latest_card: $latest_card -> $latest_card_path"
fi

for doc in "$CURRENT_TASK_DOC" "$NOW_DOC" "$RESTART_DOC" "$PHASE137X_README"; do
  expect_fixed "$active_lane" "$doc"
done

for doc in "$CURRENT_TASK_DOC" "$NOW_DOC" "$RESTART_DOC"; do
  expect_fixed "docs/development/current/main/CURRENT_STATE.toml" "$doc"
  expect_fixed "$blocker_token" "$doc"
done

expect_fixed "$pre_perf_gate" "$PHASE137X_TASKBOARD"
expect_fixed "$pre_perf_gate_status" "$PHASE137X_TASKBOARD"
expect_fixed "$optimization_return_lane" "$PHASE137X_TASKBOARD"

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
