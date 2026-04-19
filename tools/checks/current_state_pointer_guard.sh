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

guard_require_command "$TAG" rg
guard_require_command "$TAG" sed
guard_require_command "$TAG" awk
guard_require_files "$TAG" \
  "$STATE_DOC" \
  "$CURRENT_TASK_DOC" \
  "$NOW_DOC" \
  "$RESTART_DOC" \
  "$PHASE137X_README" \
  "$PHASE137X_TASKBOARD"

toml_scalar() {
  local key="$1"
  sed -n 's/^[[:space:]]*'"$key"'[[:space:]]*=[[:space:]]*"\(.*\)"[[:space:]]*$/\1/p' "$STATE_DOC" | head -n1
}

toml_array() {
  local key="$1"
  awk -v key="$key" '
    $0 ~ "^[[:space:]]*" key "[[:space:]]*=" { in_array=1; next }
    in_array && /\]/ { exit }
    in_array { print }
  ' "$STATE_DOC" | sed -n 's/^[[:space:]]*"\(.*\)"[[:space:]]*,\{0,1\}[[:space:]]*$/\1/p'
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
method_anchor="$(require_scalar method_anchor)"
taskboard="$(require_scalar taskboard)"
blocker_token="$(require_scalar current_blocker_token)"
pre_perf_gate="$(require_scalar pre_perf_gate)"
pre_perf_gate_status="$(require_scalar pre_perf_gate_status)"
optimization_return_lane="$(require_scalar optimization_return_lane)"

echo "[$TAG] checking current state mirrors"

for doc in "$CURRENT_TASK_DOC" "$NOW_DOC" "$RESTART_DOC" "$PHASE137X_README"; do
  expect_fixed "$active_lane" "$doc"
done

for doc in "$CURRENT_TASK_DOC" "$NOW_DOC" "$RESTART_DOC"; do
  expect_fixed "$method_anchor" "$doc"
  expect_fixed "$taskboard" "$doc"
  expect_fixed "$blocker_token" "$doc"
done

expect_fixed "$pre_perf_gate" "$PHASE137X_TASKBOARD"
expect_fixed "$pre_perf_gate_status" "$PHASE137X_TASKBOARD"
expect_fixed "$optimization_return_lane" "$PHASE137X_TASKBOARD"

while IFS= read -r pattern; do
  [[ -z "$pattern" ]] && continue
  if hits="$(rg -n -F "$pattern" "$CURRENT_TASK_DOC" "$ROOT_DIR/docs/development/current/main" \
    --glob '!CURRENT_STATE.toml' \
    --glob '!archive/**' \
    2>/dev/null)"; then
    printf '%s\n' "$hits" >&2
    guard_fail "$TAG" "stale current pointer pattern found: $pattern"
  fi
done < <(toml_array stale_pointer_patterns)

echo "[$TAG] ok"
