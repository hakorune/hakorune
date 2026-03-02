#!/usr/bin/env bash
# direct_loop_progression_sweep.sh
# Sweep direct --emit-mir-json -> --mir-json-file route for loop-heavy fixtures.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"

if [[ ! -x "$BIN" ]]; then
  echo "[FAIL] direct_loop_progression_sweep: binary missing/executable: $BIN" >&2
  exit 1
fi

declare -a DEFAULT_FIXTURES=(
  "apps/tests/phase216_mainline_loop_undefined_value_blocker_min.hako"
  "apps/tests/phase216_mainline_loop_count_param_nonsym_min.hako"
  "apps/tests/loop_min_while.hako"
  "apps/tests/joinir_min_loop.hako"
  "apps/tests/phase217_if_sum_multi_min.hako"
  "apps/tests/phase270_p0_loop_min_const.hako"
  "apps/tests/phase29cb_generic_loop_in_body_step_min.hako"
  "apps/tests/phase29y_loop_if_assignment_carry_min.hako"
)

declare -a FIXTURES=()
if [[ "$#" -gt 0 ]]; then
  FIXTURES=("$@")
else
  FIXTURES=("${DEFAULT_FIXTURES[@]}")
fi

echo "fixture|emit_rc|run_rc|err_head"

emit_fail_count=0
route_blocker_count=0

for fixture in "${FIXTURES[@]}"; do
  if [[ ! -f "$fixture" ]]; then
    echo "[FAIL] direct_loop_progression_sweep: missing fixture: $fixture" >&2
    exit 1
  fi

  tmp_json="$(mktemp --suffix .json)"
  tmp_emit_err="$(mktemp --suffix .emit.err)"
  tmp_run_err="$(mktemp --suffix .run.err)"

  set +e
  timeout "${EMIT_TIMEOUT_SECS}s" "$BIN" --emit-mir-json "$tmp_json" "$fixture" >/dev/null 2>"$tmp_emit_err"
  emit_rc=$?
  set -e

  run_rc="-"
  err_head=""
  if [[ "$emit_rc" -eq 0 ]]; then
    set +e
    "$BIN" --mir-json-file "$tmp_json" >/dev/null 2>"$tmp_run_err"
    run_rc=$?
    set -e
    err_head="$(head -n 1 "$tmp_run_err" || true)"
    if rg -q "vm step budget exceeded|Invalid value: \\[rust-vm\\] use of undefined value|undefined value ValueId" "$tmp_run_err"; then
      route_blocker_count=$((route_blocker_count + 1))
    fi
  else
    emit_fail_count=$((emit_fail_count + 1))
    err_head="$(head -n 1 "$tmp_emit_err" || true)"
  fi

  if [[ -z "$err_head" ]]; then
    err_head="<none>"
  fi
  printf "%s|%s|%s|%s\n" "$fixture" "$emit_rc" "$run_rc" "$err_head"

  rm -f "$tmp_json" "$tmp_emit_err" "$tmp_run_err"
done

echo "summary: emit_fail_count=$emit_fail_count route_blocker_count=$route_blocker_count total=${#FIXTURES[@]}"

if [[ "$emit_fail_count" -ne 0 ]]; then
  echo "[FAIL] direct_loop_progression_sweep: emit failures detected" >&2
  exit 1
fi

if [[ "$route_blocker_count" -ne 0 ]]; then
  echo "[FAIL] direct_loop_progression_sweep: loop progression blockers detected" >&2
  exit 1
fi

echo "[PASS] direct_loop_progression_sweep"
exit 0
