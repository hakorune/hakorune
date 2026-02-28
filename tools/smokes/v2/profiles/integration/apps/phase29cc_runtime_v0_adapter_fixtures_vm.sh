#!/usr/bin/env bash
# phase29cc_runtime_v0_adapter_fixtures_vm.sh
# Contract lock (Step-3 adapter fixtures):
# - array_set_i64 / array_get_i64 semantics under adapter ON
# - strict mode freeze contract exists in handler source
# - string_len fixture remains green with adapter ON

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_runtime_v0_adapter_fixtures_vm"
STRING_FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg04_stringbox_pilot_min.hako"
HANDLER_FILE="$NYASH_ROOT/lang/src/vm/boxes/mir_call_v1_handler.hako"

JSON_ARRAY_OK='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_type":"ArrayBox"},"args":[],"effects":["alloc"],"flags":{}}},{"op":"const","dst":2,"value":{"type":"i64","value":0}},{"op":"const","dst":3,"value":{"type":"i64","value":42}},{"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"set","receiver":1},"args":[2,3],"effects":[],"flags":{}}},{"op":"mir_call","dst":5,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"get","receiver":1},"args":[2],"effects":[],"flags":{}}},{"op":"ret","value":5}]}]}]}'
JSON_ARRAY_SET_FAIL='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_type":"ArrayBox"},"args":[],"effects":["alloc"],"flags":{}}},{"op":"const","dst":2,"value":{"type":"i64","value":-1}},{"op":"const","dst":3,"value":{"type":"i64","value":7}},{"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"set","receiver":1},"args":[2,3],"effects":[],"flags":{}}},{"op":"ret","value":4}]}]}]}'

set +e
out_array_ok=$(env \
  HAKO_ABI_ADAPTER=1 \
  HAKO_VERIFY_PRIMARY=hakovm \
  NYASH_VERIFY_JSON="$JSON_ARRAY_OK" \
  "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1)
rc_array_ok=$?
set -e

if [ "$rc_array_ok" -ne 42 ]; then
  echo "$out_array_ok" | tail -n 120 >&2 || true
  test_fail "$SMOKE_NAME: array set/get semantic rc mismatch (got=$rc_array_ok expect=42)"
  exit 1
fi

set +e
out_array_set_fail=$(env \
  HAKO_ABI_ADAPTER=1 \
  HAKO_VERIFY_PRIMARY=hakovm \
  NYASH_VERIFY_JSON="$JSON_ARRAY_SET_FAIL" \
  "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1)
rc_array_set_fail=$?
set -e

if [ "$rc_array_set_fail" -ne 0 ]; then
  echo "$out_array_set_fail" | tail -n 120 >&2 || true
  test_fail "$SMOKE_NAME: array set fail-case rc mismatch (got=$rc_array_set_fail expect=0)"
  exit 1
fi

if [ ! -f "$HANDLER_FILE" ]; then
  test_fail "$SMOKE_NAME: handler file missing ($HANDLER_FILE)"
  exit 1
fi

if ! rg -F -q "[vm/adapter/freeze:array_set_i64]" "$HANDLER_FILE"; then
  test_fail "$SMOKE_NAME: strict freeze tag contract missing in handler"
  exit 1
fi

if ! rg -F -q "HAKO_VM_ARRAY_CORE_STRICT" "$HANDLER_FILE"; then
  test_fail "$SMOKE_NAME: strict env contract missing in handler"
  exit 1
fi

if [ ! -f "$STRING_FIXTURE" ]; then
  test_fail "$SMOKE_NAME: string fixture missing ($STRING_FIXTURE)"
  exit 1
fi

export HAKO_ABI_ADAPTER=1
set +e
out_string=$("$NYASH_BIN" --backend vm "$STRING_FIXTURE" 2>&1)
rc_string=$?
set -e
unset HAKO_ABI_ADAPTER

if [ "$rc_string" -ne 0 ]; then
  echo "$out_string" | tail -n 120 >&2 || true
  test_fail "$SMOKE_NAME: string_len fixture failed rc=$rc_string"
  exit 1
fi

if ! echo "$out_string" | rg -q '^string_len='; then
  echo "$out_string" | tail -n 120 >&2 || true
  test_fail "$SMOKE_NAME: expected string_len output line"
  exit 1
fi

if ! echo "$out_string" | rg -q '^string_len2='; then
  echo "$out_string" | tail -n 120 >&2 || true
  test_fail "$SMOKE_NAME: expected string_len2 output line"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (array_get_i64/array_set_i64/string_len adapter fixtures locked)"
