#!/usr/bin/env bash
# phase29cc_runtime_v0_adapter_fixtures_vm.sh
# Contract lock (Step-3 adapter fixtures):
# - array_set_i64 / array_get_i64 semantics under adapter ON
# - raw array probe path exists below ArrayCoreBox get/set/len/push/reserve/grow
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
RAW_ARRAY_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/raw_array/raw_array_core_box.hako"
RAW_MAP_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/raw_map/raw_map_core_box.hako"
ATOMIC_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/atomic/atomic_core_box.hako"
TLS_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/tls/tls_core_box.hako"
GC_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/gc/gc_core_box.hako"
INITIALIZED_RANGE_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/verifier/initialized_range/initialized_range_core_box.hako"
OWNERSHIP_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/verifier/ownership/ownership_core_box.hako"
PTR_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/ptr/ptr_core_box.hako"
MEM_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/mem/mem_core_box.hako"
BUF_CORE_FILE="$NYASH_ROOT/lang/src/runtime/substrate/buf/buf_core_box.hako"
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
  for f in "$HANDLER_FILE" "$REGISTRY_FILE" "$ARRAY_CORE_FILE" "$ARRAY_STATE_CORE_FILE" "$RAW_ARRAY_CORE_FILE" "$RAW_MAP_CORE_FILE" "$ATOMIC_CORE_FILE" "$TLS_CORE_FILE" "$GC_CORE_FILE" "$INITIALIZED_RANGE_CORE_FILE" "$OWNERSHIP_CORE_FILE" "$PTR_CORE_FILE" "$MEM_CORE_FILE" "$BUF_CORE_FILE" "$STRING_CORE_FILE" "$MAP_CORE_FILE" "$RUNTIME_DATA_CORE_FILE"; do
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
  if ! rg -F -q 'me._put("ArrayBox", "get",    "nyash.array.slot_load_hi"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: ArrayBox.get adapter registry raw load contract missing"
    exit 1
  fi
  if ! rg -F -q 'me._put("ArrayBox", "set",    "nyash.array.set_hih"' "$REGISTRY_FILE"; then
    test_fail "$SMOKE_NAME: ArrayBox.set adapter registry fallback contract missing"
    exit 1
  fi
  if ! rg -F -q 'return RawArrayCoreBox.slot_len_i64(handle)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core raw-array len route contract missing"
    exit 1
  fi
  if ! rg -F -q 'return RawArrayCoreBox.slot_append_any(handle, value_any)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core raw-array append route contract missing"
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
  if ! rg -F -q 'return RawArrayCoreBox.slot_store_i64(handle, idx, value)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core raw-array store route contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.get_i64(recv_h, idx_i64)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core get_i64 dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'return RawArrayCoreBox.slot_load_i64(handle, idx)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core raw-array load route contract missing"
    exit 1
  fi
  if ! rg -F -q 'me.len_i64(recv_h)' "$ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: array core len_i64 dispatch contract missing"
    exit 1
  fi
  if ! rg -F -q 'PtrCoreBox.slot_store_i64(handle, idx, value)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array ptr store hop contract missing"
    exit 1
  fi
  if ! rg -F -q 'PtrCoreBox.slot_load_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array ptr load hop contract missing"
    exit 1
  fi
  if ! rg -F -q 'InitializedRangeCoreBox.ensure_initialized_index_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array initialized-range gate contract missing"
    exit 1
  fi
  if ! rg -F -q 'OwnershipCoreBox.ensure_handle_readable_i64(handle)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array ownership readable gate contract missing"
    exit 1
  fi
  if ! rg -F -q 'OwnershipCoreBox.ensure_handle_writable_i64(handle)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array ownership writable gate contract missing"
    exit 1
  fi
  if ! rg -F -q 'OwnershipCoreBox.ensure_any_readable_i64(value_any)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array ownership any-read gate contract missing"
    exit 1
  fi
  if ! rg -F -q 'PtrCoreBox.slot_len_i64(handle)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array ptr len hop contract missing"
    exit 1
  fi
  if ! rg -F -q 'PtrCoreBox.slot_append_any(handle, value_any)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array ptr append hop contract missing"
    exit 1
  fi
  if ! rg -F -q 'BufCoreBox.reserve_i64(handle, additional)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array buf reserve hop contract missing"
    exit 1
  fi
  if ! rg -F -q 'cap_i64(handle)' "$BUF_CORE_FILE"; then
    test_fail "$SMOKE_NAME: buf core cap contract missing"
    exit 1
  fi
  if ! rg -F -q 'BufCoreBox.grow_i64(handle, target_capacity)' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array buf grow hop contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_array:slot_store_i64]' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array store trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_array:slot_load_i64]' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array load trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_array:slot_len_i64]' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array len trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_array:slot_append_any]' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array append trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_array:slot_reserve_i64]' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array reserve trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_array:slot_grow_i64]' "$RAW_ARRAY_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw array grow trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'BufCoreBox.len_i64(handle)' "$INITIALIZED_RANGE_CORE_FILE"; then
    test_fail "$SMOKE_NAME: initialized-range buf len contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/initialized_range:ensure_initialized_index_i64]' "$INITIALIZED_RANGE_CORE_FILE"; then
    test_fail "$SMOKE_NAME: initialized-range trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.any.handle_live_h"(handle)' "$OWNERSHIP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: ownership core handle-live route contract missing"
    exit 1
  fi
  if ! rg -F -q 'fence_i64()' "$ATOMIC_CORE_FILE"; then
    test_fail "$SMOKE_NAME: atomic core fence contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "hako_barrier_touch_i64"(0)' "$ATOMIC_CORE_FILE"; then
    test_fail "$SMOKE_NAME: atomic core barrier-touch route contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/atomic:fence_i64]' "$ATOMIC_CORE_FILE"; then
    test_fail "$SMOKE_NAME: atomic core fence trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'last_error_text_h()' "$TLS_CORE_FILE"; then
    test_fail "$SMOKE_NAME: tls core last_error_text contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "hako_last_error"(0)' "$TLS_CORE_FILE"; then
    test_fail "$SMOKE_NAME: tls core hako_last_error route contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.box.from_i8_string"(raw)' "$TLS_CORE_FILE"; then
    test_fail "$SMOKE_NAME: tls core from_i8_string route contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/tls:last_error_text_h]' "$TLS_CORE_FILE"; then
    test_fail "$SMOKE_NAME: tls core last_error trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'write_barrier_i64(handle_or_ptr)' "$GC_CORE_FILE"; then
    test_fail "$SMOKE_NAME: gc core write_barrier contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.gc.barrier_write"(handle_or_ptr)' "$GC_CORE_FILE"; then
    test_fail "$SMOKE_NAME: gc core barrier_write route contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/gc:write_barrier_i64]' "$GC_CORE_FILE"; then
    test_fail "$SMOKE_NAME: gc core write_barrier trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/verifier:ownership_handle_readable]' "$OWNERSHIP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: ownership readable trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/verifier:ownership_handle_writable]' "$OWNERSHIP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: ownership writable trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/verifier:ownership_any_readable]' "$OWNERSHIP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: ownership any-read trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'alloc_i64(size)' "$MEM_CORE_FILE"; then
    test_fail "$SMOKE_NAME: mem core alloc contract missing"
    exit 1
  fi
  if ! rg -F -q 'realloc_i64(ptr, new_size)' "$MEM_CORE_FILE"; then
    test_fail "$SMOKE_NAME: mem core realloc contract missing"
    exit 1
  fi
  if ! rg -F -q 'free_i64(ptr)' "$MEM_CORE_FILE"; then
    test_fail "$SMOKE_NAME: mem core free contract missing"
    exit 1
  fi
  if ! rg -F -q 'len_i64(handle)' "$BUF_CORE_FILE"; then
    test_fail "$SMOKE_NAME: buf core len contract missing"
    exit 1
  fi
  if ! rg -F -q 'cap_i64(handle)' "$BUF_CORE_FILE"; then
    test_fail "$SMOKE_NAME: buf core cap contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.array.slot_cap_h"' "$BUF_CORE_FILE"; then
    test_fail "$SMOKE_NAME: buf core slot_cap backend contract missing"
    exit 1
  fi
  if ! rg -F -q 'reserve_i64(handle, additional)' "$BUF_CORE_FILE"; then
    test_fail "$SMOKE_NAME: buf core reserve contract missing"
    exit 1
  fi
  if ! rg -F -q 'grow_i64(handle, target_capacity)' "$BUF_CORE_FILE"; then
    test_fail "$SMOKE_NAME: buf core grow contract missing"
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
  if ! rg -F -q 'return RawMapCoreBox.entry_count_i64(handle)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core raw-map size route contract missing"
    exit 1
  fi
  if ! rg -F -q 'cap_i64(handle)' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map cap observer contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.map.cap_h"' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map cap backend contract missing"
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
  if ! rg -F -q 'return RawMapCoreBox.slot_store_any(handle, key_any, val_any)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core raw-map store route contract missing"
    exit 1
  fi
  if ! rg -F -q 'return RawMapCoreBox.slot_load_any(handle, key_any)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core raw-map load route contract missing"
    exit 1
  fi
  if ! rg -F -q 'return RawMapCoreBox.probe_any(handle, key_any)' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core raw-map probe route contract missing"
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
  if ! rg -F -q '[vm/adapter/map_core:slot_store_hhh]' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core raw store trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/map_core:slot_load_hh]' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core raw load trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/map_core:probe_hh]' "$MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: map core raw probe trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_map:cap_i64]' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map cap trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.map.slot_load_hh"' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map load extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.map.slot_store_hhh"' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map store extern route contract missing"
    exit 1
  fi
  if ! rg -F -q 'externcall "nyash.map.probe_hh"' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map probe extern route contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_map:slot_load_any]' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map load trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_map:slot_store_any]' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map store trace tag contract missing"
    exit 1
  fi
  if ! rg -F -q '[vm/adapter/raw_map:probe_any]' "$RAW_MAP_CORE_FILE"; then
    test_fail "$SMOKE_NAME: raw map probe trace tag contract missing"
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

test_pass "$SMOKE_NAME: PASS (array_get/set/len/push/reserve/grow + string_len/map_entry_count_i64 adapter route locked)"
