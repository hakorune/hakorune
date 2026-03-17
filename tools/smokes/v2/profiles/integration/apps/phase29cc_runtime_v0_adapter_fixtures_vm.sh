#!/usr/bin/env bash
# phase29cc_runtime_v0_adapter_fixtures_vm.sh
# Contract lock (Step-3 adapter fixtures):
# - array_set_i64 / array_get_i64 semantics under adapter ON
# - strict mode freeze contract exists in handler source
# - string_len adapter route contract exists in source (registry + handler + core box)
# - map_size_i64 adapter route contract exists in source (registry + handler + core box)
# - string fixture remains green under adapter ON (behavior smoke)
# - map size alias fixture remains green under adapter ON (behavior smoke)

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_runtime_v0_adapter_fixtures_vm"
STRING_FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg04_stringbox_pilot_min.hako"
HANDLER_FILE="$NYASH_ROOT/lang/src/vm/boxes/mir_call_v1_handler.hako"
REGISTRY_FILE="$NYASH_ROOT/lang/src/vm/boxes/abi_adapter_registry.hako"
STRING_CORE_FILE="$NYASH_ROOT/lang/src/runtime/collections/string_core_box.hako"
MAP_CORE_FILE="$NYASH_ROOT/lang/src/runtime/collections/map_core_box.hako"

JSON_ARRAY_OK='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_type":"ArrayBox"},"args":[],"effects":["alloc"],"flags":{}}},{"op":"const","dst":2,"value":{"type":"i64","value":0}},{"op":"const","dst":3,"value":{"type":"i64","value":42}},{"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"set","receiver":1},"args":[2,3],"effects":[],"flags":{}}},{"op":"mir_call","dst":5,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"get","receiver":1},"args":[2],"effects":[],"flags":{}}},{"op":"ret","value":5}]}]}]}'
JSON_ARRAY_SET_FAIL='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_type":"ArrayBox"},"args":[],"effects":["alloc"],"flags":{}}},{"op":"const","dst":2,"value":{"type":"i64","value":-1}},{"op":"const","dst":3,"value":{"type":"i64","value":7}},{"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"set","receiver":1},"args":[2,3],"effects":[],"flags":{}}},{"op":"ret","value":4}]}]}]}'

run_array_semantics_checks() {
  set +e
  local out_array_ok
  out_array_ok=$(env \
    HAKO_ABI_ADAPTER=1 \
    HAKO_VERIFY_PRIMARY=hakovm \
    NYASH_VERIFY_JSON="$JSON_ARRAY_OK" \
    "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1)
  local rc_array_ok=$?
  set -e
  if [ "$rc_array_ok" -ne 42 ]; then
    echo "$out_array_ok" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: array set/get semantic rc mismatch (got=$rc_array_ok expect=42)"
    exit 1
  fi

  set +e
  local out_array_set_fail
  out_array_set_fail=$(env \
    HAKO_ABI_ADAPTER=1 \
    HAKO_VERIFY_PRIMARY=hakovm \
    NYASH_VERIFY_JSON="$JSON_ARRAY_SET_FAIL" \
    "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1)
  local rc_array_set_fail=$?
  set -e
  if [ "$rc_array_set_fail" -ne 0 ]; then
    echo "$out_array_set_fail" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: array set fail-case rc mismatch (got=$rc_array_set_fail expect=0)"
    exit 1
  fi
}

check_string_adapter_route_contract() {
  for f in "$HANDLER_FILE" "$REGISTRY_FILE" "$STRING_CORE_FILE" "$MAP_CORE_FILE"; do
    if [ ! -f "$f" ]; then
      test_fail "$SMOKE_NAME: missing file ($f)"
      exit 1
    fi
  done

  if ! rg -F -q 'me._put("StringBox", "length", "nyash.string.len_h"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: StringBox.length adapter registry contract missing"
    exit 1
  fi
  if ! rg -F -q 'StringCoreBox.len_i64(' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler string core route contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/string_core:len_i64]' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler string adapter trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.string.len_h"' "$STRING_CORE_FILE"; then
    test_fail "$SMOKE_NAME: string core extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'me._put("MapBox", "size",    "nyash.map.size_h"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: MapBox.size adapter registry contract missing"
    exit 1
  fi
  if ! rg -F -q 'MapCoreBox.size_i64(' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler map core route contract missing"
    exit 1
  fi
  if ! rg -F -q 'MapCoreBox.record_set_state(' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler map set-state contract missing"
    exit 1
  fi
  if ! rg -F -q 'MapCoreBox.get_state_value(' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler map get-state contract missing"
    exit 1
  fi
  if ! rg -F -q 'MapCoreBox.has_state_value(' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler map has-state contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/map_core:size_i64]' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler map adapter trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.map.size_h"' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'record_set_state(regs, per_recv, rid, key_str, cur_len, value_state, arg1_id)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core set-state helper contract missing"
    exit 1
  fi
  if ! rg -F -q 'get_state_value(regs, per_recv, rid, key_str)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core get-state helper contract missing"
    exit 1
  fi
  if ! rg -F -q 'has_state_value(regs, per_recv, rid, key_str)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core has-state helper contract missing"
    exit 1
  fi
}

check_array_strict_contract() {
  if ! rg -F -q "[vm/adapter/freeze:array_set_i64]" "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: strict freeze tag contract missing in handler"
    exit 1
  fi
  if ! rg -F -q "HAKO_VM_ARRAY_CORE_STRICT" "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: strict env contract missing in handler"
    exit 1
  fi
}

run_string_behavior_smoke() {
  if [ ! -f "$STRING_FIXTURE" ]; then
    test_fail "$SMOKE_NAME: string fixture missing ($STRING_FIXTURE)"
    exit 1
  fi

  export HAKO_ABI_ADAPTER=1
  set +e
  local out_string
  out_string=$("$NYASH_BIN" --backend vm "$STRING_FIXTURE" 2>&1)
  local rc_string=$?
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
}

run_map_behavior_smoke() {
  set +e
  local out_map
  out_map=$(env \
    HAKO_ABI_ADAPTER=1 \
    HAKO_VM_MIRCALL_SIZESTATE=1 \
    HAKO_VERIFY_PRIMARY=hakovm \
    NYASH_VERIFY_JSON='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_type":"MapBox"},"args":[],"effects":["alloc"],"flags":{}}},{"op":"const","dst":2,"value":{"type":"i64","value":7}},{"op":"const","dst":3,"value":{"type":"i64","value":42}},{"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"set","receiver":1},"args":[2,3],"effects":[],"flags":{}}},{"op":"mir_call","dst":5,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"size","receiver":1},"args":[],"effects":[],"flags":{}}},{"op":"ret","value":5}]}}]}' \
    "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1)
  local rc_map=$?
  set -e
  if [ "$rc_map" -ne 1 ]; then
    echo "$out_map" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: map size smoke rc mismatch (got=$rc_map expect=1)"
    exit 1
  fi
}

run_array_semantics_checks
check_string_adapter_route_contract
check_array_strict_contract
run_string_behavior_smoke
run_map_behavior_smoke

test_pass "$SMOKE_NAME: PASS (array_get_i64/array_set_i64 + string_len/map_size_i64 adapter route locked)"
