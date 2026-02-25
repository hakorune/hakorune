#!/bin/bash
# phase21_5_perf_guard_lib_contract_vm.sh
#
# Contract pin:
# - perf guard common helpers keep stable behavior.
# - retry helper retries both command-failure and parse-failure paths.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_guard_lib_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

COMMON_LIB="$NYASH_ROOT/tools/checks/lib/perf_guard_common.sh"
if [ ! -f "$COMMON_LIB" ]; then
  test_fail "$SMOKE_NAME: common lib not found: $COMMON_LIB"
  exit 2
fi
source "$COMMON_LIB"

PARSE_RETRY_COUNT_FILE="$(mktemp)"
FAIL_RETRY_COUNT_FILE="$(mktemp)"
BAD_PARSE_COUNT_FILE="$(mktemp)"
BASELINE_FILE="$(mktemp)"
trap 'rm -f "$PARSE_RETRY_COUNT_FILE" "$FAIL_RETRY_COUNT_FILE" "$BAD_PARSE_COUNT_FILE" "$BASELINE_FILE"' EXIT

echo "0" >"$PARSE_RETRY_COUNT_FILE"
echo "0" >"$FAIL_RETRY_COUNT_FILE"
echo "0" >"$BAD_PARSE_COUNT_FILE"

perf_guard_lib_counter_inc() {
  local counter_file="$1"
  local current
  current="$(cat "$counter_file")"
  current=$((current + 1))
  echo "$current" >"$counter_file"
  printf '%s\n' "$current"
}

perf_guard_lib_test_parse_positive_total() {
  local output="$1"
  local v
  v="$(perf_guard_json_get_int_or_zero "$output" '.total_ms')"
  [[ "$v" =~ ^[0-9]+$ ]] && [[ "$v" -gt 0 ]]
}

perf_guard_lib_test_cmd_parse_retry() {
  local count
  count="$(perf_guard_lib_counter_inc "$PARSE_RETRY_COUNT_FILE")"
  if [ "$count" -lt 2 ]; then
    echo '{"total_ms":0}'
  else
    echo '{"total_ms":123,"cases":{"sample":1}}'
  fi
}

perf_guard_lib_test_cmd_fail_retry() {
  local count
  count="$(perf_guard_lib_counter_inc "$FAIL_RETRY_COUNT_FILE")"
  if [ "$count" -lt 2 ]; then
    echo '[error] transient failure'
    return 1
  fi
  echo '{"total_ms":321}'
}

perf_guard_lib_test_cmd_always_bad_parse() {
  perf_guard_lib_counter_inc "$BAD_PARSE_COUNT_FILE" >/dev/null
  echo '{"total_ms":0}'
}

# json_get helpers
if [ "$(perf_guard_json_get_int_or_zero '{"v":7}' '.v')" != "7" ]; then
  test_fail "$SMOKE_NAME: json int extraction mismatch"
  exit 1
fi
if [ "$(perf_guard_json_get_int_or_zero '{"v":"oops"}' '.v')" != "0" ]; then
  test_fail "$SMOKE_NAME: json int fallback mismatch"
  exit 1
fi
if [ "$(perf_guard_json_get_str_or_default '{"name":"ok"}' '.name' 'n/a')" != "ok" ]; then
  test_fail "$SMOKE_NAME: json str extraction mismatch"
  exit 1
fi
if [ "$(perf_guard_json_get_str_or_default '{"name":""}' '.name' 'n/a')" != "n/a" ]; then
  test_fail "$SMOKE_NAME: json str default mismatch"
  exit 1
fi

cat >"$BASELINE_FILE" <<'JSON'
{"apps_vm_total_ms":456,"apps_entry_mode_hotspot_case":"mir_shape_guard","bad":"oops"}
JSON
if [ "$(perf_guard_baseline_get_int_or_zero "$BASELINE_FILE" '.apps_vm_total_ms')" != "456" ]; then
  test_fail "$SMOKE_NAME: baseline int extraction mismatch"
  exit 1
fi
if [ "$(perf_guard_baseline_get_int_or_zero "$BASELINE_FILE" '.bad')" != "0" ]; then
  test_fail "$SMOKE_NAME: baseline int fallback mismatch"
  exit 1
fi
if [ "$(perf_guard_baseline_get_str_or_default "$BASELINE_FILE" '.apps_entry_mode_hotspot_case' 'n/a')" != "mir_shape_guard" ]; then
  test_fail "$SMOKE_NAME: baseline str extraction mismatch"
  exit 1
fi
if [ "$(perf_guard_baseline_get_str_or_default "$BASELINE_FILE" '.missing' 'n/a')" != "n/a" ]; then
  test_fail "$SMOKE_NAME: baseline str default mismatch"
  exit 1
fi

# retry helper: parse failure -> retry -> success
echo "0" >"$PARSE_RETRY_COUNT_FILE"
OUT_PARSE_RETRY=""
if ! perf_guard_retry_capture "$SMOKE_NAME" "parse-retry" 3 OUT_PARSE_RETRY \
    perf_guard_lib_test_parse_positive_total perf_guard_lib_test_cmd_parse_retry; then
  test_fail "$SMOKE_NAME: parse-retry should succeed"
  exit 1
fi
if [ "$(cat "$PARSE_RETRY_COUNT_FILE")" -ne 2 ]; then
  test_fail "$SMOKE_NAME: parse-retry attempt count mismatch"
  exit 1
fi

# retry helper: command failure -> retry -> success
echo "0" >"$FAIL_RETRY_COUNT_FILE"
OUT_FAIL_RETRY=""
if ! perf_guard_retry_capture "$SMOKE_NAME" "command-retry" 3 OUT_FAIL_RETRY \
    perf_guard_lib_test_parse_positive_total perf_guard_lib_test_cmd_fail_retry; then
  test_fail "$SMOKE_NAME: command-retry should succeed"
  exit 1
fi
if [ "$(cat "$FAIL_RETRY_COUNT_FILE")" -ne 2 ]; then
  test_fail "$SMOKE_NAME: command-retry attempt count mismatch"
  exit 1
fi

# retry helper: parse failure exhausted -> fail
echo "0" >"$BAD_PARSE_COUNT_FILE"
OUT_BAD_PARSE=""
set +e
perf_guard_retry_capture "$SMOKE_NAME" "parse-exhaust" 2 OUT_BAD_PARSE \
  perf_guard_lib_test_parse_positive_total perf_guard_lib_test_cmd_always_bad_parse >/dev/null 2>&1
rc_bad_parse=$?
set -e
if [ "$rc_bad_parse" -eq 0 ]; then
  test_fail "$SMOKE_NAME: parse-exhaust should fail"
  exit 1
fi
if [ "$(cat "$BAD_PARSE_COUNT_FILE")" -ne 2 ]; then
  test_fail "$SMOKE_NAME: parse-exhaust attempt count mismatch"
  exit 1
fi

# retry helper: invalid parser fn -> fail (no command run)
set +e
perf_guard_retry_capture "$SMOKE_NAME" "invalid-parse-fn" 1 OUT_BAD_PARSE \
  perf_guard_lib_test_parse_missing perf_guard_lib_test_cmd_parse_retry >/dev/null 2>&1
rc_invalid_parse_fn=$?
set -e
if [ "$rc_invalid_parse_fn" -eq 0 ]; then
  test_fail "$SMOKE_NAME: invalid parse fn should fail"
  exit 1
fi

test_pass "$SMOKE_NAME"
