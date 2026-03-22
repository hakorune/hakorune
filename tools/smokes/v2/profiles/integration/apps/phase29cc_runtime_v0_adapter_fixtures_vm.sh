#!/usr/bin/env bash
# phase29cc_runtime_v0_adapter_fixtures_vm.sh
# Contract lock (Step-3 adapter fixtures):
# - array_set_i64 / array_get_i64 semantics under adapter ON
# - strict mode freeze contract exists in handler source
# - string_len adapter route contract exists in source (registry + handler + core box)
# - map_size_i64 adapter route contract exists in source (registry + handler + core box)
# - runtime_data get/set/has/push route contract exists in source (handler + core box)
# - string fixture remains green under adapter ON (behavior smoke)
# - map size alias fixture remains green under adapter ON (behavior smoke)

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_runtime_v0_adapter_fixtures_vm"
STRING_FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg04_stringbox_pilot_min.hako"
HANDLER_FILE="$NYASH_ROOT/lang/src/vm/boxes/mir_call_v1_handler.hako"
REGISTRY_FILE="$NYASH_ROOT/lang/src/vm/boxes/abi_adapter_registry.hako"
ARRAY_CORE_FILE="$NYASH_ROOT/lang/src/runtime/collections/array_core_box.hako"
ARRAY_STATE_CORE_FILE="$NYASH_ROOT/lang/src/runtime/collections/array_state_core_box.hako"
STRING_CORE_FILE="$NYASH_ROOT/lang/src/runtime/collections/string_core_box.hako"
MAP_CORE_FILE="$NYASH_ROOT/lang/src/runtime/collections/map_core_box.hako"
RUNTIME_DATA_CORE_FILE="$NYASH_ROOT/lang/src/runtime/collections/runtime_data_core_box.hako"

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

check_collection_adapter_route_contract() {
  for f in "$HANDLER_FILE" "$REGISTRY_FILE" "$ARRAY_CORE_FILE" "$ARRAY_STATE_CORE_FILE" "$STRING_CORE_FILE" "$MAP_CORE_FILE" "$RUNTIME_DATA_CORE_FILE"; do
    if [ ! -f "$f" ]; then
      test_fail "$SMOKE_NAME: missing file ($f)"
      exit 1
    fi
  done

  if ! rg -F -q 'me._put("StringBox", "length", "nyash.string.len_h"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: StringBox.length adapter registry contract missing"
    exit 1
  fi
  if ! rg -F -q 'StringCoreBox.try_handle(seg, regs, mname)' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler string core orchestration contract missing"
    exit 1
  fi
  if ! rg -F -q 'ArrayCoreBox.try_handle(seg, regs, mname)' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler array orchestration contract missing"
    exit 1
  fi
  if ! rg -F -q 'me._put("ArrayBox", "push",   "nyash.array.slot_append_hh"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: ArrayBox.push adapter registry raw append contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.array.slot_len_h"' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core len extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.array.slot_append_hh"' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core push extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'try_handle(seg, regs, mname)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core orchestration helper contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.set_i64(recv_h, idx_i64, val_i64)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core set_i64 dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.get_i64(recv_h, idx_i64)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core get_i64 dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.len_i64(recv_h)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core len_i64 dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'record_push_state(regs, per_recv, rid, cur_len, value_state, arg0_id)' "$ARRAY_STATE_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array state core push-state helper contract missing"
    exit 1
  fi
  if ! rg -F -q 'record_set_state(regs, per_recv, rid, idx, cur_len, value_state, arg1_id)' "$ARRAY_STATE_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array state core set-state helper contract missing"
    exit 1
  fi
  if ! rg -F -q 'get_state_value(regs, per_recv, rid, idx)' "$ARRAY_STATE_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array state core get-state helper contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/array_core:set_i64]' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core set trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/array_core:get_i64]' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core get trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/array_core:len_i64]' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core len trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.string.len_h"' "$STRING_CORE_FILE"; then
    test_fail "$SMOKE_NAME: string core extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'try_handle(seg, regs, mname)' "$STRING_CORE_FILE"; then
    test_fail "$SMOKE_NAME: string core orchestration helper contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/string_core:len_i64]' "$STRING_CORE_FILE"; then
    test_fail "$SMOKE_NAME: string core trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'me._put("MapBox", "size",    "nyash.map.entry_count_h"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: MapBox.size adapter registry contract missing"
    exit 1
  fi
  if ! rg -F -q 'me._put("MapBox", "set",     "nyash.map.slot_store_hhh"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: MapBox.set adapter registry raw store contract missing"
    exit 1
  fi
  if ! rg -F -q 'me._put("MapBox", "get",     "nyash.map.slot_load_hh"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: MapBox.get adapter registry raw load contract missing"
    exit 1
  fi
  if ! rg -F -q 'me._put("MapBox", "has",     "nyash.map.probe_hh"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: MapBox.has adapter registry raw probe contract missing"
    exit 1
  fi
  if ! rg -F -q 'MapCoreBox.try_handle(seg, regs, mname)' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler map orchestration contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.map.entry_count_h"' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'try_handle(seg, regs, mname)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core orchestration helper contract missing"
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
  if ! rg -F -q 'me.size_i64(recv_h)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core size_i64 dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/map_core:size_i64]' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core size trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/map_core:set_state]' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core set trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'using lang.runtime.collections.runtime_data_core_box as RuntimeDataCoreBox' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler runtime_data core import contract missing"
    exit 1
  fi
  if ! rg -F -q 'RuntimeDataCoreBox.try_handle(seg, regs, mname)' "$HANDLER_FILE"; then
    test_fail "$SMOKE_NAME: handler runtime_data orchestration contract missing"
    exit 1
  fi
  if ! rg -F -q 'try_handle(seg, regs, mname)' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core orchestration helper contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.get_hh(recv_hrt, key_any)' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core get dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.set_hhh(recv_hrt, key_any, val_any)' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core set dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.has_hh(recv_hrt, key_any)' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core has dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.push_hh(recv_hrt, val_any)' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core push dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/runtime_data_core:get_hh]' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core get trace contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/runtime_data_core:set_hhh]' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core set trace contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/runtime_data_core:has_hh]' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core has trace contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/runtime_data_core:push_hh]' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core push trace contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.runtime_data.get_hh"' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core get extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.runtime_data.set_hhh"' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core set extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.runtime_data.has_hh"' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core has extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.runtime_data.push_hh"' "$RUNTIME_DATA_CORE_FILE"; then
    test_fail "$SMOKE_NAME: runtime_data core push extern route contract missing"
    exit 1
  fi
}

check_array_strict_contract() {
  if ! rg -F -q "[vm/adapter/freeze:array_set_i64]" "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: strict freeze tag contract missing in array core"
    exit 1
  fi
  if ! rg -F -q "HAKO_VM_ARRAY_CORE_STRICT" "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: strict env contract missing in array core"
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
check_collection_adapter_route_contract
check_array_strict_contract
run_string_behavior_smoke
run_map_behavior_smoke

test_pass "$SMOKE_NAME: PASS (array_get_i64/array_set_i64 + string_len/map_entry_count_i64 adapter route locked)"
