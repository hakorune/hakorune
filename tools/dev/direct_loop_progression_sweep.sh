#!/usr/bin/env bash
# direct_loop_progression_sweep.sh
# Sweep direct --emit-mir-json -> --mir-json-file route for loop-heavy fixtures.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

usage() {
  cat <<'USAGE'
Usage:
  tools/dev/direct_loop_progression_sweep.sh [--profile <name>] [--allow-emit-fail] [fixture ...]

Profiles:
  default         fixed 8-fixture daily sweep (strict)
  phase29x-green  phase29bq/cb green subset (strict)
  phase29x-probe  phase29bq/ca/cb loop fixtures auto-discovery (monitor mode; emit fail allowed by default)

Examples:
  tools/dev/direct_loop_progression_sweep.sh
  tools/dev/direct_loop_progression_sweep.sh --profile phase29x-green
  tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe
  tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --strict-emit-fail
USAGE
}

BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
PROFILE="${DIRECT_SWEEP_PROFILE:-default}"
ALLOW_EMIT_FAIL=0
ALLOW_EMIT_FAIL_SET=0

declare -a FIXTURES=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile)
      PROFILE="${2:-}"
      shift 2
      ;;
    --allow-emit-fail)
      ALLOW_EMIT_FAIL=1
      ALLOW_EMIT_FAIL_SET=1
      shift
      ;;
    --strict-emit-fail)
      ALLOW_EMIT_FAIL=0
      ALLOW_EMIT_FAIL_SET=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    --)
      shift
      while [[ $# -gt 0 ]]; do
        FIXTURES+=("$1")
        shift
      done
      ;;
    -* )
      echo "[FAIL] direct_loop_progression_sweep: unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
    *)
      FIXTURES+=("$1")
      shift
      ;;
  esac
done

if [[ ! -x "$BIN" ]]; then
  echo "[FAIL] direct_loop_progression_sweep: binary missing/executable: $BIN" >&2
  exit 1
fi

discover_phase29x_fixtures() {
  rg --files apps/tests \
    | rg 'phase29(bq|ca|cb).*' \
    | rg '(loop|generic_loop|loop_cond|loop_true)' \
    | sort
}

load_profile_fixtures() {
  local profile="$1"
  case "$profile" in
    default)
      cat <<'LIST'
apps/tests/phase216_mainline_loop_undefined_value_blocker_min.hako
apps/tests/phase216_mainline_loop_count_param_nonsym_min.hako
apps/tests/loop_min_while.hako
apps/tests/joinir_min_loop.hako
apps/tests/phase217_if_sum_multi_min.hako
apps/tests/phase270_p0_loop_min_const.hako
apps/tests/phase29cb_generic_loop_in_body_step_min.hako
apps/tests/phase29y_loop_if_assignment_carry_min.hako
LIST
      ;;
    phase29x-green)
      cat <<'LIST'
apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_blockexpr_min.hako
apps/tests/phase29bq_selfhost_blocker_module_roots_loop_min.hako
apps/tests/phase29bq_generic_loop_v1_recipe_then_only_empty_join_min.hako
apps/tests/phase29bq_selfhost_blocker_phi_collect_outer_loop_min.hako
apps/tests/phase29bq_selfhost_blocker_decode_escapes_loop_min.hako
apps/tests/phase29bq_generic_loop_v1_nested_min.hako
apps/tests/phase29bq_loop_cond_multi_exit_min.hako
apps/tests/phase29bq_generic_loop_v1_recipe_loop_if_loop_pure_min.hako
apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_return_local_min.hako
apps/tests/phase29cb_generic_loop_in_body_step_min.hako
LIST
      ;;
    phase29x-probe)
      discover_phase29x_fixtures
      ;;
    *)
      echo "[FAIL] direct_loop_progression_sweep: unknown profile: $profile" >&2
      exit 2
      ;;
  esac
}

if [[ "${#FIXTURES[@]}" -eq 0 ]]; then
  if [[ "$PROFILE" == "phase29x-probe" && "$ALLOW_EMIT_FAIL_SET" -eq 0 ]]; then
    ALLOW_EMIT_FAIL=1
  fi
  mapfile -t FIXTURES < <(load_profile_fixtures "$PROFILE")
fi

if [[ "${#FIXTURES[@]}" -eq 0 ]]; then
  echo "[FAIL] direct_loop_progression_sweep: no fixtures selected" >&2
  exit 1
fi

extract_signal_line() {
  local file="$1"
  local line=""

  line="$(rg -m1 '(\[freeze:contract\]|\[emit-mir/direct-verify\]|\[joinir/reject_detail\]|vm step budget exceeded|Invalid value|\[ERROR\]|MIR JSON parse error|unsupported instruction|missing receiver|No plugin provider|panic|error:)' "$file" || true)"
  if [[ -z "$line" ]]; then
    line="$(rg -m1 -v '^(\[ConsoleBox\] Plugin initialized|Net plugin:|\s*)$' "$file" || true)"
  fi
  if [[ -z "$line" ]]; then
    line="<none>"
  fi
  printf '%s' "$line"
}

classify_emit_failure() {
  local file="$1"
  if rg -q '\[emit-mir/direct-verify\]' "$file"; then
    echo "emit:direct-verify"
    return 0
  fi
  if rg -q '\[joinir/reject_detail\]' "$file"; then
    echo "emit:joinir-reject"
    return 0
  fi
  if rg -q '\[freeze:contract\]' "$file"; then
    echo "emit:freeze-contract"
    return 0
  fi
  echo "emit:other"
}

extract_freeze_contract_detail() {
  local file="$1"
  local line=""
  line="$(rg -m1 '\[(plan/)?freeze:contract\][^\n]*' "$file" || true)"
  if [[ -z "$line" ]]; then
    line="$(rg -m1 'Detail:\s+\[(plan/)?freeze:contract\][^\n]*' "$file" || true)"
  fi
  if [[ -z "$line" ]]; then
    line="<freeze-contract:unknown>"
  fi
  line="$(printf '%s' "$line" | sed -E 's/^Detail:\s*//')"
  printf '%s' "$line"
}

classify_run_result() {
  local rc="$1"
  local file="$2"
  if [[ "$rc" -eq 0 ]]; then
    echo "run:ok"
    return 0
  fi
  if rg -q 'vm step budget exceeded|Invalid value: \[vm\] use of undefined value|undefined value ValueId' "$file"; then
    echo "run:loop-progression-blocker"
    return 0
  fi
  if rg -q '\[vm/error\]' "$file"; then
    echo "run:vm-error"
    return 0
  fi
  if [[ -s "$file" ]]; then
    echo "run:nonzero"
    return 0
  fi
  echo "run:nonzero-empty"
}

declare -A CLASS_COUNTS=()
declare -A FREEZE_CONTRACT_COUNTS=()

bump_class_count() {
  local key="$1"
  CLASS_COUNTS["$key"]=$(( ${CLASS_COUNTS["$key"]:-0} + 1 ))
}

bump_freeze_contract_count() {
  local key="$1"
  FREEZE_CONTRACT_COUNTS["$key"]=$(( ${FREEZE_CONTRACT_COUNTS["$key"]:-0} + 1 ))
}

echo "fixture|emit_rc|run_rc|class|signal"

total_count=0
emit_fail_count=0
unexpected_emit_fail_count=0
route_blocker_count=0
run_nonzero_count=0

for fixture in "${FIXTURES[@]}"; do
  total_count=$((total_count + 1))
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
  class=""
  signal=""

  if [[ "$emit_rc" -eq 0 ]]; then
    set +e
    "$BIN" --mir-json-file "$tmp_json" >/dev/null 2>"$tmp_run_err"
    run_rc=$?
    set -e

    class="$(classify_run_result "$run_rc" "$tmp_run_err")"
    signal="$(extract_signal_line "$tmp_run_err")"
    bump_class_count "$class"

    if [[ "$run_rc" -ne 0 ]]; then
      run_nonzero_count=$((run_nonzero_count + 1))
    fi
    if [[ "$class" == "run:loop-progression-blocker" ]]; then
      route_blocker_count=$((route_blocker_count + 1))
    fi
  else
    emit_fail_count=$((emit_fail_count + 1))
    class="$(classify_emit_failure "$tmp_emit_err")"
    signal="$(extract_signal_line "$tmp_emit_err")"
    bump_class_count "$class"
    if [[ "$class" == "emit:freeze-contract" ]]; then
      freeze_detail="$(extract_freeze_contract_detail "$tmp_emit_err")"
      bump_freeze_contract_count "$freeze_detail"
    fi
    if [[ "$ALLOW_EMIT_FAIL" -eq 0 ]]; then
      unexpected_emit_fail_count=$((unexpected_emit_fail_count + 1))
    fi
  fi

  printf "%s|%s|%s|%s|%s\n" "$fixture" "$emit_rc" "$run_rc" "$class" "$signal"

  rm -f "$tmp_json" "$tmp_emit_err" "$tmp_run_err"
done

echo "summary: profile=$PROFILE emit_fail_count=$emit_fail_count unexpected_emit_fail_count=$unexpected_emit_fail_count run_nonzero_count=$run_nonzero_count route_blocker_count=$route_blocker_count total=$total_count"

echo "class_counts:"
for key in "${!CLASS_COUNTS[@]}"; do
  printf "  %s=%s\n" "$key" "${CLASS_COUNTS[$key]}"
done | sort

if [[ ${#FREEZE_CONTRACT_COUNTS[@]} -gt 0 ]]; then
  echo "freeze_contract_details:"
  for key in "${!FREEZE_CONTRACT_COUNTS[@]}"; do
    printf "%s\t%s\n" "${FREEZE_CONTRACT_COUNTS[$key]}" "$key"
  done | sort -nr | while IFS=$'\t' read -r count detail; do
    printf "  %s %s\n" "$count" "$detail"
  done
fi

if [[ "$unexpected_emit_fail_count" -ne 0 ]]; then
  echo "[FAIL] direct_loop_progression_sweep: unexpected emit failures detected" >&2
  exit 1
fi

if [[ "$route_blocker_count" -ne 0 ]]; then
  echo "[FAIL] direct_loop_progression_sweep: loop progression blockers detected" >&2
  exit 1
fi

echo "[PASS] direct_loop_progression_sweep"
exit 0
