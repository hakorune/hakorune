#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

LOCK_DOC="docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md"
CUTOVER_SSOT="docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md"
ABI_MATRIX="docs/reference/abi/ABI_BOUNDARY_MATRIX.md"
DEV_GATE="tools/checks/dev_gate.sh"
REGISTRY_FILE="lang/src/vm/boxes/abi_adapter_registry.hako"
HANDLER_FILE="lang/src/vm/boxes/mir_call_v1_handler.hako"
ARRAY_CORE_FILE="lang/src/runtime/collections/array_core_box.hako"
ARRAY_STATE_CORE_FILE="lang/src/runtime/collections/array_state_core_box.hako"
RAW_ARRAY_CORE_FILE="lang/src/runtime/substrate/raw_array/raw_array_core_box.hako"
RAW_MAP_CORE_FILE="lang/src/runtime/substrate/raw_map/raw_map_core_box.hako"
INITIALIZED_RANGE_CORE_FILE="lang/src/runtime/substrate/verifier/initialized_range/initialized_range_core_box.hako"
OWNERSHIP_CORE_FILE="lang/src/runtime/substrate/verifier/ownership/ownership_core_box.hako"
BUF_CORE_FILE="lang/src/runtime/substrate/buf/buf_core_box.hako"
PTR_CORE_FILE="lang/src/runtime/substrate/ptr/ptr_core_box.hako"
STRING_CORE_FILE="lang/src/runtime/collections/string_core_box.hako"
MAP_CORE_FILE="lang/src/runtime/collections/map_core_box.hako"

for file in \
  "$LOCK_DOC" \
  "$CUTOVER_SSOT" \
  "$ABI_MATRIX" \
  "$DEV_GATE" \
  "$REGISTRY_FILE" \
  "$HANDLER_FILE" \
  "$ARRAY_CORE_FILE" \
  "$ARRAY_STATE_CORE_FILE" \
  "$RAW_ARRAY_CORE_FILE" \
  "$RAW_MAP_CORE_FILE" \
  "$INITIALIZED_RANGE_CORE_FILE" \
  "$OWNERSHIP_CORE_FILE" \
  "$BUF_CORE_FILE" \
  "$PTR_CORE_FILE" \
  "$STRING_CORE_FILE" \
  "$MAP_CORE_FILE"; do
  if [ ! -f "$file" ]; then
    echo "[runtime-v0-abi-slice-guard] missing file: $file" >&2
    exit 1
  fi
done

for keyword in "string_len" "array_get_i64" "array_set_i64" "map_size_i64" "args borrowed / return owned"; do
  if ! rg -F -q "$keyword" "$LOCK_DOC" "$CUTOVER_SSOT"; then
    echo "[runtime-v0-abi-slice-guard] missing keyword: $keyword" >&2
    exit 1
  fi
done

if ! rg -q "Runtime V0 helper slice" "$ABI_MATRIX"; then
  echo "[runtime-v0-abi-slice-guard] ABI matrix missing V0 helper slice row" >&2
  exit 1
fi

if ! rg -q "phase29cc_runtime_v0_abi_slice_guard.sh" "$DEV_GATE"; then
  echo "[runtime-v0-abi-slice-guard] dev_gate missing runtime-v0-abi-slice guard wiring" >&2
  exit 1
fi

# Code-side adapter route contract (Step-3 anchor)
if ! rg -F -q 'me._put("StringBox", "length", "nyash.string.len_h"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing StringBox.length -> nyash.string.len_h" >&2
  exit 1
fi
if ! rg -F -q 'me._put("ArrayBox", "get",    "nyash.array.slot_load_hi"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing ArrayBox.get raw load mapping" >&2
  exit 1
fi
if ! rg -F -q 'me._put("ArrayBox", "set",    "nyash.array.set_hih"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing ArrayBox.set adapter mapping" >&2
  exit 1
fi
if ! rg -F -q 'me._put("ArrayBox", "push",   "nyash.array.slot_append_hh"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing ArrayBox.push raw append mapping" >&2
  exit 1
fi
if ! rg -F -q 'me._put("MapBox", "set",     "nyash.map.slot_store_hhh"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing MapBox.set raw store mapping" >&2
  exit 1
fi
if ! rg -F -q 'me._put("MapBox", "get",     "nyash.map.slot_load_hh"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing MapBox.get raw load mapping" >&2
  exit 1
fi
if ! rg -F -q 'me._put("MapBox", "has",     "nyash.map.probe_hh"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing MapBox.has raw probe mapping" >&2
  exit 1
fi
if ! rg -F -q 'StringCoreBox.try_handle(seg, regs, mname)' "$HANDLER_FILE"; then
  echo "[runtime-v0-abi-slice-guard] handler missing StringCoreBox orchestration route" >&2
  exit 1
fi
if ! rg -F -q 'MapCoreBox.try_handle(seg, regs, mname)' "$HANDLER_FILE"; then
  echo "[runtime-v0-abi-slice-guard] handler missing MapCoreBox orchestration route" >&2
  exit 1
fi
if ! rg -F -q 'ArrayCoreBox.try_handle(seg, regs, mname)' "$HANDLER_FILE"; then
  echo "[runtime-v0-abi-slice-guard] handler missing ArrayCoreBox orchestration route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.string.len_h"' "$STRING_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] string core missing nyash.string.len_h extern route" >&2
  exit 1
fi
if ! rg -F -q 'try_handle(seg, regs, mname)' "$STRING_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] string core missing try_handle contract" >&2
  exit 1
fi
if ! rg -F -q 'return RawArrayCoreBox.slot_load_i64(handle, idx)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing RawArrayCoreBox load route" >&2
  exit 1
fi
if ! rg -F -q 'return RawArrayCoreBox.slot_store_i64(handle, idx, value)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing RawArrayCoreBox store route" >&2
  exit 1
fi
if ! rg -F -q 'return RawArrayCoreBox.slot_len_i64(handle)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing RawArrayCoreBox len route" >&2
  exit 1
fi
if ! rg -F -q 'return RawArrayCoreBox.slot_append_any(handle, value_any)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing RawArrayCoreBox append route" >&2
  exit 1
fi
if ! rg -F -q 'PtrCoreBox.slot_load_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing ptr load route" >&2
  exit 1
fi
if ! rg -F -q 'InitializedRangeCoreBox.ensure_initialized_index_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing initialized-range gate" >&2
  exit 1
fi
if ! rg -F -q 'OwnershipCoreBox.ensure_handle_readable_i64(handle)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing ownership readable gate" >&2
  exit 1
fi
if ! rg -F -q 'OwnershipCoreBox.ensure_handle_writable_i64(handle)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing ownership writable gate" >&2
  exit 1
fi
if ! rg -F -q 'OwnershipCoreBox.ensure_any_readable_i64(value_any)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing ownership any-read gate" >&2
  exit 1
fi
if ! rg -F -q 'BufCoreBox.len_i64(handle)' "$INITIALIZED_RANGE_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] initialized-range missing buf len route" >&2
  exit 1
fi
if ! rg -F -q '[vm/adapter/initialized_range:ensure_initialized_index_i64]' "$INITIALIZED_RANGE_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] initialized-range missing trace tag" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.any.handle_live_h"(handle)' "$OWNERSHIP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ownership core missing nyash.any.handle_live_h route" >&2
  exit 1
fi
if ! rg -F -q '[vm/adapter/verifier:ownership_handle_readable]' "$OWNERSHIP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ownership core missing readable trace tag" >&2
  exit 1
fi
if ! rg -F -q '[vm/adapter/verifier:ownership_handle_writable]' "$OWNERSHIP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ownership core missing writable trace tag" >&2
  exit 1
fi
if ! rg -F -q '[vm/adapter/verifier:ownership_any_readable]' "$OWNERSHIP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ownership core missing any-read trace tag" >&2
  exit 1
fi
if ! rg -F -q 'PtrCoreBox.slot_store_i64(handle, idx, value)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing ptr store route" >&2
  exit 1
fi
if ! rg -F -q 'PtrCoreBox.slot_len_i64(handle)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing ptr len route" >&2
  exit 1
fi
if ! rg -F -q 'PtrCoreBox.slot_append_any(handle, value_any)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing ptr append route" >&2
  exit 1
fi
if ! rg -F -q 'BufCoreBox.reserve_i64(handle, additional)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing buf reserve route" >&2
  exit 1
fi
if ! rg -F -q 'cap_i64(handle)' "$BUF_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] buf core missing cap route" >&2
  exit 1
fi
if ! rg -F -q 'BufCoreBox.grow_i64(handle, target_capacity)' "$RAW_ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw array missing buf grow route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.slot_load_hi"' "$PTR_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ptr core missing nyash.array.slot_load_hi extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.slot_store_hii"' "$PTR_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ptr core missing nyash.array.slot_store_hii extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.slot_len_h"' "$PTR_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ptr core missing nyash.array.slot_len_h extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.slot_append_hh"' "$PTR_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ptr core missing nyash.array.slot_append_hh extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.slot_reserve_hi"' "$PTR_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ptr core missing nyash.array.slot_reserve_hi extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.slot_grow_hi"' "$PTR_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] ptr core missing nyash.array.slot_grow_hi extern route" >&2
  exit 1
fi
if ! rg -F -q 'record_push_state(regs, per_recv, rid, cur_len, value_state, arg0_id)' "$ARRAY_STATE_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array state core missing push-state helper contract" >&2
  exit 1
fi
if ! rg -F -q 'record_set_state(regs, per_recv, rid, idx, cur_len, value_state, arg1_id)' "$ARRAY_STATE_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array state core missing set-state helper contract" >&2
  exit 1
fi
if ! rg -F -q 'get_state_value(regs, per_recv, rid, idx)' "$ARRAY_STATE_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array state core missing get-state helper contract" >&2
  exit 1
fi
if ! rg -F -q 'try_handle(seg, regs, mname)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing try_handle contract" >&2
  exit 1
fi
if ! rg -F -q 'me.set_i64(recv_h, idx_i64, val_i64)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing set_i64 dispatch contract" >&2
  exit 1
fi
if ! rg -F -q 'me.get_i64(recv_h, idx_i64)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing get_i64 dispatch contract" >&2
  exit 1
fi
if ! rg -F -q 'me.push_hh(recv_h, val_any)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing push_hh dispatch contract" >&2
  exit 1
fi
if ! rg -F -q 'me.len_i64(recv_h)' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing len_i64 dispatch contract" >&2
  exit 1
fi
if ! rg -F -q 'using selfhost.runtime.substrate.raw_map.raw_map_core_box as RawMapCoreBox' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing raw_map substrate import" >&2
  exit 1
fi
if ! rg -F -q 'return RawMapCoreBox.entry_count_i64(handle)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing RawMapCoreBox entry_count route" >&2
  exit 1
fi
if ! rg -F -q 'entry_count_i64(handle)' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing entry_count contract" >&2
  exit 1
fi
if ! rg -F -q 'cap_i64(handle)' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing cap contract" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.map.entry_count_h"' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing nyash.map.entry_count_h extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.map.cap_h"' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing nyash.map.cap_h extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.map.slot_load_hh"' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing nyash.map.slot_load_hh extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.map.slot_store_hhh"' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing nyash.map.slot_store_hhh extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.map.probe_hh"' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing nyash.map.probe_hh extern route" >&2
  exit 1
fi
if ! rg -F -q 'OwnershipCoreBox.ensure_handle_readable_i64(handle)' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing ownership readable gate" >&2
  exit 1
fi
if ! rg -F -q 'OwnershipCoreBox.ensure_handle_writable_i64(handle)' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing ownership writable gate" >&2
  exit 1
fi
if ! rg -F -q 'OwnershipCoreBox.ensure_any_readable_i64(key_any)' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing ownership key any-read gate" >&2
  exit 1
fi
if ! rg -F -q 'OwnershipCoreBox.ensure_any_readable_i64(val_any)' "$RAW_MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] raw map core missing ownership val any-read gate" >&2
  exit 1
fi
if ! rg -F -q 'return RawMapCoreBox.slot_load_any(handle, key_any)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing raw map load route" >&2
  exit 1
fi
if ! rg -F -q 'return RawMapCoreBox.slot_store_any(handle, key_any, val_any)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing raw map store route" >&2
  exit 1
fi
if ! rg -F -q 'return RawMapCoreBox.probe_any(handle, key_any)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing raw map probe route" >&2
  exit 1
fi
if ! rg -F -q 'record_set_state(regs, per_recv, rid, key_str, cur_len, value_state, arg1_id)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing set-state helper contract" >&2
  exit 1
fi
if ! rg -F -q 'get_state_value(regs, per_recv, rid, key_str)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing get-state helper contract" >&2
  exit 1
fi
if ! rg -F -q 'has_state_value(regs, per_recv, rid, key_str)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing has-state helper contract" >&2
  exit 1
fi
if ! rg -F -q 'try_handle(seg, regs, mname)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing try_handle contract" >&2
  exit 1
fi
if ! rg -F -q 'me.size_i64(recv_h)' "$MAP_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] map core missing size_i64 dispatch contract" >&2
  exit 1
fi

echo "[runtime-v0-abi-slice-guard] ok"
