runtime_v0_abi_slice_fail() {
  local message="$1"
  echo "[${RUNTIME_V0_ABI_SLICE_TAG:-runtime-v0-abi-slice-guard}] $message" >&2
  exit 1
}

runtime_v0_abi_slice_require_contains() {
  local needle="$1"
  local file="$2"
  local message="$3"
  if ! rg -F -q "$needle" "$file"; then
    runtime_v0_abi_slice_fail "$message"
  fi
}

runtime_v0_abi_slice_require_regex() {
  local needle="$1"
  local file="$2"
  local message="$3"
  if ! rg -q "$needle" "$file"; then
    runtime_v0_abi_slice_fail "$message"
  fi
}

runtime_v0_abi_slice_require_absent() {
  local needle="$1"
  local file="$2"
  local message="$3"
  if rg -F -q "$needle" "$file"; then
    runtime_v0_abi_slice_fail "$message"
  fi
}

runtime_v0_abi_slice_require_count_at_least() {
  local needle="$1"
  local file="$2"
  local minimum="$3"
  local message="$4"
  local count
  count="$(rg -F "$needle" "$file" | wc -l | tr -d ' ')"
  if [[ "${count:-0}" -lt "$minimum" ]]; then
    runtime_v0_abi_slice_fail "$message"
  fi
}

runtime_v0_abi_slice_check_collections_hot() {
  runtime_v0_abi_slice_require_contains 'func = "nyash.array.slot_store_hih"' "$COLLECTIONS_HOT_FILE" "collections hot missing nyash.array.slot_store_hih hot route"
  runtime_v0_abi_slice_require_absent 'func = "nyash.array.set_h"' "$COLLECTIONS_HOT_FILE" "collections hot still references nyash.array.set_h"
}

runtime_v0_abi_slice_check_lock_docs_and_manifest() {
  for keyword in "string_len" "array_get_i64" "array_set_i64" "map_size_i64" "args borrowed / return owned"; do
    runtime_v0_abi_slice_require_contains "$keyword" "$LOCK_DOC" "missing keyword: $keyword"
    runtime_v0_abi_slice_require_contains "$keyword" "$CUTOVER_SSOT" "missing keyword: $keyword"
  done
  runtime_v0_abi_slice_require_contains "Runtime V0 helper slice" "$ABI_MATRIX" "ABI matrix missing V0 helper slice row"
  runtime_v0_abi_slice_require_contains "phase29cc_runtime_v0_abi_slice_guard.sh" "$DEV_GATE" "dev_gate missing runtime-v0-abi-slice guard wiring"
  runtime_v0_abi_slice_require_contains "abi_adapter_registry_defaults" "$REGISTRY_FILE" "registry missing generated defaults import"
  runtime_v0_abi_slice_require_contains "AbiAdapterRegistryDefaultsBox.populate(tab)" "$REGISTRY_FILE" "registry missing generated defaults populate call"
  runtime_v0_abi_slice_require_contains "static box AbiAdapterRegistryDefaultsBox" "$GENERATED_DEFAULTS_FILE" "generated defaults box name mismatch"
  runtime_v0_abi_slice_require_contains "HAKO_ABI_ADAPTER_DEV" "$REGISTRY_FILE" "registry missing HAKO_ABI_ADAPTER_DEV dev canary"
}

runtime_v0_abi_slice_check_adapter_defaults() {
  runtime_v0_abi_slice_require_regex 'MapBox",\s*"get".*nyash\.map\.slot_load_hh.*integer' "$GENERATED_DEFAULTS_FILE" "generated defaults missing MapBox.get -> slot_load_hh (unbox integer)"
  runtime_v0_abi_slice_require_regex 'MapBox",\s*"set".*nyash\.map\.slot_store_hhh' "$GENERATED_DEFAULTS_FILE" "generated defaults missing MapBox.set -> slot_store_hhh"
  runtime_v0_abi_slice_require_regex 'MapBox",\s*"has".*nyash\.map\.probe_hh' "$GENERATED_DEFAULTS_FILE" "generated defaults missing MapBox.has -> probe_hh"
  runtime_v0_abi_slice_require_regex 'MapBox",\s*"size".*nyash\.map\.entry_count_i64' "$GENERATED_DEFAULTS_FILE" "generated defaults missing MapBox.size -> entry_count_i64"
  runtime_v0_abi_slice_require_regex 'ArrayBox",\s*"get".*nyash\.array\.slot_load_hi' "$GENERATED_DEFAULTS_FILE" "generated defaults missing ArrayBox.get -> slot_load_hi"
  runtime_v0_abi_slice_require_regex 'ArrayBox",\s*"set".*nyash\.array\.slot_store_hih' "$GENERATED_DEFAULTS_FILE" "generated defaults missing ArrayBox.set -> slot_store_hih"
  runtime_v0_abi_slice_require_regex 'ArrayBox",\s*"push".*nyash\.array\.slot_append_hh' "$GENERATED_DEFAULTS_FILE" "generated defaults missing ArrayBox.push -> slot_append_hh"
  runtime_v0_abi_slice_require_regex 'StringBox",\s*"length".*nyash\.string\.len_h' "$GENERATED_DEFAULTS_FILE" "generated defaults missing StringBox.length -> nyash.string.len_h"
  runtime_v0_abi_slice_require_contains 'StringCoreBox.try_handle(seg, regs, kinds, handle_regs, mname)' "$HANDLER_FILE" "handler missing StringCoreBox orchestration route"
  runtime_v0_abi_slice_require_contains 'MapCoreBox.try_handle(seg, regs, handle_regs, mname)' "$HANDLER_FILE" "handler missing MapCoreBox orchestration route"
  runtime_v0_abi_slice_require_contains 'ArrayCoreBox.try_handle(seg, regs, kinds, handle_regs, mname)' "$HANDLER_FILE" "handler missing ArrayCoreBox orchestration route"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.string.len_h"' "$STRING_CORE_FILE" "string core missing nyash.string.len_h extern route"
  runtime_v0_abi_slice_require_contains 'try_handle(seg, regs, kinds, handle_regs, mname)' "$STRING_CORE_FILE" "string core missing try_handle contract"
}

runtime_v0_abi_slice_check_raw_array() {
  runtime_v0_abi_slice_require_contains 'return RawArrayCoreBox.slot_load_i64(handle, idx)' "$ARRAY_CORE_FILE" "array core missing RawArrayCoreBox load route"
  runtime_v0_abi_slice_require_contains 'return RawArrayCoreBox.slot_store_i64(handle, idx, value)' "$ARRAY_CORE_FILE" "array core missing RawArrayCoreBox store route"
  runtime_v0_abi_slice_require_contains 'return RawArrayCoreBox.slot_store_string_handle(handle, idx, value_h)' "$ARRAY_CORE_FILE" "array core missing RawArrayCoreBox string-store route"
  runtime_v0_abi_slice_require_contains 'return RawArrayCoreBox.slot_len_i64(handle)' "$ARRAY_CORE_FILE" "array core missing RawArrayCoreBox len route"
  runtime_v0_abi_slice_require_contains 'return RawArrayCoreBox.slot_append_any(handle, value_any)' "$ARRAY_CORE_FILE" "array core missing RawArrayCoreBox append route"
  runtime_v0_abi_slice_require_contains 'PtrCoreBox.slot_load_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE" "raw array missing ptr load route"
  runtime_v0_abi_slice_require_contains 'slot_load_usize(handle, idx: usize)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize load alias"
  runtime_v0_abi_slice_require_contains 'slot_store_usize(handle, idx: usize, value)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize store alias"
  runtime_v0_abi_slice_require_contains 'slot_len_usize(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize len alias"
  runtime_v0_abi_slice_require_contains 'slot_cap_usize(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize cap alias"
  runtime_v0_abi_slice_require_contains 'slot_slice_any_usize(handle, start: usize, end: usize)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize slice alias"
  runtime_v0_abi_slice_require_contains 'slot_reserve_usize(handle, additional: usize)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize reserve alias"
  runtime_v0_abi_slice_require_contains 'slot_grow_usize(handle, target_capacity: usize)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize grow alias"
  runtime_v0_abi_slice_require_contains 'PtrCoreBox.slot_store_string_handle(handle, idx, value_h)' "$RAW_ARRAY_CORE_FILE" "raw array missing ptr string-store route"
  runtime_v0_abi_slice_require_contains 'BoundsCoreBox.ensure_index_usize(handle, idx)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize index bounds gate"
  runtime_v0_abi_slice_require_contains 'InitializedRangeCoreBox.ensure_initialized_index_usize(handle, idx)' "$RAW_ARRAY_CORE_FILE" "raw array missing usize initialized-range gate"
  runtime_v0_abi_slice_require_contains 'InitializedRangeCoreBox.ensure_initialized_index_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE" "raw array missing initialized-range gate"
  runtime_v0_abi_slice_require_count_at_least 'BoundsCoreBox.ensure_index_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE" 4 "raw array missing bounds gates for load/store/string-store/remove"
  runtime_v0_abi_slice_require_count_at_least 'InitializedRangeCoreBox.ensure_initialized_index_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE" 2 "raw array missing initialized-range gates for load/remove"
  runtime_v0_abi_slice_require_contains 'BoundsCoreBox.ensure_insert_index_i64(handle, idx)' "$RAW_ARRAY_CORE_FILE" "raw array missing insert-index bounds gate"
  runtime_v0_abi_slice_require_contains '[vm/adapter/verifier:bounds_insert_i64]' "$BOUNDS_CORE_FILE" "bounds core missing insert trace tag"
  runtime_v0_abi_slice_require_contains 'OwnershipCoreBox.ensure_handle_readable_i64(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing ownership readable gate"
  runtime_v0_abi_slice_require_contains 'OwnershipCoreBox.ensure_handle_writable_i64(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing ownership writable gate"
  runtime_v0_abi_slice_require_contains 'OwnershipCoreBox.ensure_any_readable_i64(value_any)' "$RAW_ARRAY_CORE_FILE" "raw array missing ownership any-read gate"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.len_i64(handle)' "$INITIALIZED_RANGE_CORE_FILE" "initialized-range missing buf len route"
  runtime_v0_abi_slice_require_contains '[vm/adapter/initialized_range:ensure_initialized_index_i64]' "$INITIALIZED_RANGE_CORE_FILE" "initialized-range missing trace tag"
  runtime_v0_abi_slice_require_contains 'PtrCoreBox.slot_store_i64(handle, idx, value)' "$RAW_ARRAY_CORE_FILE" "raw array missing ptr store route"
  runtime_v0_abi_slice_require_contains 'PtrCoreBox.slot_len_i64(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing ptr len route"
  runtime_v0_abi_slice_require_contains 'slot_cap_i64(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing cap observer route"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.cap_i64(handle)' "$RAW_ARRAY_CORE_FILE" "raw array cap must route through buf substrate"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.len_usize(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing buf usize len route"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.cap_usize(handle)' "$RAW_ARRAY_CORE_FILE" "raw array missing buf usize cap route"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.reserve_usize(handle, additional)' "$RAW_ARRAY_CORE_FILE" "raw array missing buf usize reserve route"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.grow_usize(handle, target_capacity)' "$RAW_ARRAY_CORE_FILE" "raw array missing buf usize grow route"
  runtime_v0_abi_slice_require_contains '[vm/adapter/raw_array:slot_cap_i64]' "$RAW_ARRAY_CORE_FILE" "raw array missing cap trace tag"
  runtime_v0_abi_slice_require_contains '[vm/adapter/raw_array:slot_cap_usize]' "$RAW_ARRAY_CORE_FILE" "raw array missing usize cap trace tag"
  runtime_v0_abi_slice_require_contains 'PtrCoreBox.slot_append_any(handle, value_any)' "$RAW_ARRAY_CORE_FILE" "raw array missing ptr append route"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.reserve_i64(handle, additional)' "$RAW_ARRAY_CORE_FILE" "raw array missing buf reserve route"
  runtime_v0_abi_slice_require_contains 'cap_i64(handle)' "$BUF_CORE_FILE" "buf core missing cap route"
  runtime_v0_abi_slice_require_contains 'using selfhost.runtime.substrate.value_repr.current_lane_box as CurrentLaneBox' "$BUF_CORE_FILE" "buf core missing current-lane helper import"
  runtime_v0_abi_slice_require_contains 'len_usize(handle)' "$BUF_CORE_FILE" "buf core missing usize len facade"
  runtime_v0_abi_slice_require_contains 'cap_usize(handle)' "$BUF_CORE_FILE" "buf core missing usize cap facade"
  runtime_v0_abi_slice_require_contains 'reserve_usize(handle, additional: usize)' "$BUF_CORE_FILE" "buf core missing usize reserve facade"
  runtime_v0_abi_slice_require_contains 'grow_usize(handle, target_capacity: usize)' "$BUF_CORE_FILE" "buf core missing usize grow facade"
  runtime_v0_abi_slice_require_contains 'PtrCoreBox.slot_cap_i64(handle)' "$BUF_CORE_FILE" "buf core cap route must go through ptr substrate"
  runtime_v0_abi_slice_require_absent 'externcall "nyash.array.slot_cap_h"' "$BUF_CORE_FILE" "buf core must not own direct slot_cap extern route"
  runtime_v0_abi_slice_require_contains 'BufCoreBox.grow_i64(handle, target_capacity)' "$RAW_ARRAY_CORE_FILE" "raw array missing buf grow route"
  runtime_v0_abi_slice_require_contains 'record_push_state(regs, per_recv, rid, cur_len, value_state, arg0_id)' "$ARRAY_STATE_CORE_FILE" "array state core missing push-state helper contract"
  runtime_v0_abi_slice_require_contains 'record_set_state(regs, per_recv, rid, idx, cur_len, value_state, arg1_id)' "$ARRAY_STATE_CORE_FILE" "array state core missing set-state helper contract"
  runtime_v0_abi_slice_require_contains 'get_state_value(regs, per_recv, rid, idx)' "$ARRAY_STATE_CORE_FILE" "array state core missing get-state helper contract"
  runtime_v0_abi_slice_require_contains 'try_handle(seg, regs, kinds, handle_regs, mname)' "$ARRAY_CORE_FILE" "array core missing try_handle contract"
  runtime_v0_abi_slice_require_contains 'me.set_i64(recv_h, idx_i64, val_i64)' "$ARRAY_CORE_FILE" "array core missing set_i64 dispatch contract"
  runtime_v0_abi_slice_require_contains 'me.get_i64(recv_h, idx_i64)' "$ARRAY_CORE_FILE" "array core missing get_i64 dispatch contract"
  runtime_v0_abi_slice_require_contains 'me.push_hh(recv_h, append_any)' "$ARRAY_CORE_FILE" "array core missing push_hh dispatch contract"
  runtime_v0_abi_slice_require_contains 'me.len_i64(recv_h)' "$ARRAY_CORE_FILE" "array core missing len_i64 dispatch contract"
}

runtime_v0_abi_slice_check_raw_map() {
  runtime_v0_abi_slice_require_contains 'using selfhost.runtime.substrate.raw_map.raw_map_core_box as RawMapCoreBox' "$MAP_CORE_FILE" "map core missing raw_map substrate import"
  runtime_v0_abi_slice_require_contains 'return RawMapCoreBox.entry_count_i64(handle)' "$MAP_CORE_FILE" "map core missing RawMapCoreBox entry_count route"
  runtime_v0_abi_slice_require_contains 'entry_count_i64(handle)' "$RAW_MAP_CORE_FILE" "raw map core missing entry_count contract"
  runtime_v0_abi_slice_require_contains 'cap_i64(handle)' "$RAW_MAP_CORE_FILE" "raw map core missing cap contract"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.map.entry_count_i64"' "$RAW_MAP_CORE_FILE" "raw map core missing nyash.map.entry_count_i64 extern route"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.map.cap_h"' "$RAW_MAP_CORE_FILE" "raw map core missing nyash.map.cap_h extern route"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.map.slot_load_hh"' "$RAW_MAP_CORE_FILE" "raw map core missing nyash.map.slot_load_hh extern route"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.map.slot_store_hhh"' "$RAW_MAP_CORE_FILE" "raw map core missing nyash.map.slot_store_hhh extern route"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.map.probe_hh"' "$RAW_MAP_CORE_FILE" "raw map core missing nyash.map.probe_hh extern route"
  runtime_v0_abi_slice_require_contains 'OwnershipCoreBox.ensure_handle_readable_i64(handle)' "$RAW_MAP_CORE_FILE" "raw map core missing ownership readable gate"
  runtime_v0_abi_slice_require_contains 'OwnershipCoreBox.ensure_handle_writable_i64(handle)' "$RAW_MAP_CORE_FILE" "raw map core missing ownership writable gate"
  runtime_v0_abi_slice_require_contains 'OwnershipCoreBox.ensure_any_readable_i64(key_any)' "$RAW_MAP_CORE_FILE" "raw map core missing ownership key any-read gate"
  runtime_v0_abi_slice_require_contains 'OwnershipCoreBox.ensure_any_readable_i64(val_any)' "$RAW_MAP_CORE_FILE" "raw map core missing ownership val any-read gate"
  runtime_v0_abi_slice_require_contains 'return RawMapCoreBox.slot_load_any(handle, key_any)' "$MAP_CORE_FILE" "map core missing raw map load route"
  runtime_v0_abi_slice_require_contains 'return RawMapCoreBox.slot_store_any(handle, key_any, val_any)' "$MAP_CORE_FILE" "map core missing raw map store route"
  runtime_v0_abi_slice_require_contains 'return RawMapCoreBox.probe_any(handle, key_any)' "$MAP_CORE_FILE" "map core missing raw map probe route"
  runtime_v0_abi_slice_require_contains 'record_set_state(regs, per_recv, rid, key_str, cur_len, value_state, arg1_id)' "$MAP_CORE_FILE" "map core missing set-state helper contract"
  runtime_v0_abi_slice_require_contains 'get_state_value(regs, per_recv, rid, key_str)' "$MAP_CORE_FILE" "map core missing get-state helper contract"
  runtime_v0_abi_slice_require_contains 'has_state_value(regs, per_recv, rid, key_str)' "$MAP_CORE_FILE" "map core missing has-state helper contract"
  runtime_v0_abi_slice_require_contains 'try_handle(seg, regs, handle_regs, mname)' "$MAP_CORE_FILE" "map core missing try_handle contract"
  runtime_v0_abi_slice_require_contains 'me.size_i64(recv_h)' "$MAP_CORE_FILE" "map core missing size_i64 dispatch contract"
}

runtime_v0_abi_slice_check_substrates() {
  runtime_v0_abi_slice_require_contains 'fence_i64()' "$ATOMIC_CORE_FILE" "atomic core missing fence contract"
  runtime_v0_abi_slice_require_contains 'externcall "hako_barrier_touch_i64"(0)' "$ATOMIC_CORE_FILE" "atomic core missing hako_barrier_touch_i64 route"
  runtime_v0_abi_slice_require_contains '[vm/adapter/atomic:fence_i64]' "$ATOMIC_CORE_FILE" "atomic core missing fence trace tag"
  runtime_v0_abi_slice_require_contains 'order_seq_cst_i64()' "$ATOMIC_CORE_FILE" "atomic core missing memory-order vocabulary"
  runtime_v0_abi_slice_require_contains 'fence_order_i64(order)' "$ATOMIC_CORE_FILE" "atomic core missing ordered fence contract"
  runtime_v0_abi_slice_require_contains 'externcall "hako_barrier_touch_i64"(order)' "$ATOMIC_CORE_FILE" "atomic core missing ordered fence barrier route"
  runtime_v0_abi_slice_require_contains 'last_error_text_h()' "$TLS_CORE_FILE" "tls core missing last_error_text contract"
  runtime_v0_abi_slice_require_contains 'last_error_is_ok_i64()' "$TLS_CORE_FILE" "tls core missing last_error_is_ok contract"
  runtime_v0_abi_slice_require_contains 'last_error_code_i64()' "$TLS_CORE_FILE" "tls core missing last_error_code contract"
  runtime_v0_abi_slice_require_contains 'externcall "hako_last_error"(0)' "$TLS_CORE_FILE" "tls core missing hako_last_error route"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.box.from_i8_string"(raw)' "$TLS_CORE_FILE" "tls core missing nyash.box.from_i8_string route"
  runtime_v0_abi_slice_require_contains '[vm/adapter/tls:last_error_text_h]' "$TLS_CORE_FILE" "tls core missing last_error trace tag"
  runtime_v0_abi_slice_require_contains 'write_barrier_i64(handle_or_ptr)' "$GC_CORE_FILE" "gc core missing write_barrier contract"
  runtime_v0_abi_slice_require_contains 'externcall "nyash.gc.barrier_write"(handle_or_ptr)' "$GC_CORE_FILE" "gc core missing nyash.gc.barrier_write route"
  runtime_v0_abi_slice_require_contains '[vm/adapter/gc:write_barrier_i64]' "$GC_CORE_FILE" "gc core missing write_barrier trace tag"
  runtime_v0_abi_slice_require_contains 'externcall "hako_osvm_reserve_bytes_i64"(len_bytes)' "$OSVM_CORE_FILE" "osvm core missing hako_osvm_reserve_bytes_i64 route"
  runtime_v0_abi_slice_require_contains 'externcall "hako_osvm_commit_bytes_i64"(base, len_bytes)' "$OSVM_CORE_FILE" "osvm core missing hako_osvm_commit_bytes_i64 route"
  runtime_v0_abi_slice_require_contains 'externcall "hako_osvm_decommit_bytes_i64"(base, len_bytes)' "$OSVM_CORE_FILE" "osvm core missing hako_osvm_decommit_bytes_i64 route"
  runtime_v0_abi_slice_require_contains 'externcall "hako_osvm_unreserve_bytes_i64"(base, len_bytes)' "$OSVM_CORE_FILE" "osvm core missing hako_osvm_unreserve_bytes_i64 route"
  runtime_v0_abi_slice_require_contains '[vm/adapter/osvm:reserve_bytes_i64]' "$OSVM_CORE_FILE" "osvm core missing reserve_bytes trace tag"
  runtime_v0_abi_slice_require_contains '[vm/adapter/osvm:commit_bytes_i64]' "$OSVM_CORE_FILE" "osvm core missing commit_bytes trace tag"
  runtime_v0_abi_slice_require_contains '[vm/adapter/osvm:decommit_bytes_i64]' "$OSVM_CORE_FILE" "osvm core missing decommit_bytes trace tag"
  runtime_v0_abi_slice_require_contains '[vm/adapter/osvm:unreserve_bytes_i64]' "$OSVM_CORE_FILE" "osvm core missing unreserve_bytes trace tag"
  runtime_v0_abi_slice_require_contains '[vm/adapter/osvm:page_size_i64]' "$OSVM_CORE_FILE" "osvm core missing page_size trace tag"
  runtime_v0_abi_slice_require_contains 'clz_i64(value)' "$INTRIN_CORE_FILE" "intrin core missing clz_i64 contract"
  runtime_v0_abi_slice_require_contains 'ctz_i64(value)' "$INTRIN_CORE_FILE" "intrin core missing ctz_i64 contract"
  runtime_v0_abi_slice_require_contains 'popcnt_i64(value)' "$INTRIN_CORE_FILE" "intrin core missing popcnt_i64 contract"
  runtime_v0_abi_slice_require_contains 'externcall "hako_intrin_clz_i64"(value)' "$INTRIN_CORE_FILE" "intrin core missing hako_intrin_clz_i64 route"
  runtime_v0_abi_slice_require_contains 'externcall "hako_intrin_ctz_i64"(value)' "$INTRIN_CORE_FILE" "intrin core missing hako_intrin_ctz_i64 route"
  runtime_v0_abi_slice_require_contains 'externcall "hako_intrin_popcnt_i64"(value)' "$INTRIN_CORE_FILE" "intrin core missing hako_intrin_popcnt_i64 route"
  runtime_v0_abi_slice_require_contains '[vm/adapter/intrin:clz_i64]' "$INTRIN_CORE_FILE" "intrin core missing clz trace tag"
  runtime_v0_abi_slice_require_contains '[vm/adapter/intrin:ctz_i64]' "$INTRIN_CORE_FILE" "intrin core missing ctz trace tag"
  runtime_v0_abi_slice_require_contains '[vm/adapter/intrin:popcnt_i64]' "$INTRIN_CORE_FILE" "intrin core missing popcnt trace tag"
}
