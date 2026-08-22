#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-callslot-wire-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$ROOT_DIR/include/nyrt_dynamic_call_slot_v2.h" "$ROOT_DIR/src/abi/dynamic_call_slot_wire.rs" "$ROOT_DIR/src/llvm_py/builders/dynamic_v2_callslot_wire.py" "$ROOT_DIR/src/llvm_py/tests/test_dynamic_v2_callslot_wire.py"

WIRE_C="$ROOT_DIR/include/nyrt_dynamic_call_slot_v2.h"
WIRE_RS="$ROOT_DIR/src/abi/dynamic_call_slot_wire.rs"
WIRE_PY="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_callslot_wire.py"
WIRE_TEST="$ROOT_DIR/src/llvm_py/tests/test_dynamic_v2_callslot_wire.py"
OLD_WIRE="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/call_slot_wire.rs"

[[ ! -e "$OLD_WIRE" ]] || guard_fail "$TAG" "MIR-local wire copy still exists"
if rg -n -F -q "mod call_slot_wire" "$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs"; then
  guard_fail "$TAG" "MIR emitter still registers a wire schema module"
fi

guard_expect_fixed_in_file "$TAG" "DYNAMIC_V2_WIRE_REVISION_V2: u32 = 2" "$WIRE_RS" "Rust projection must use revision 2"
guard_expect_fixed_in_file "$TAG" "HAKO_DYNAMIC_V2_WIRE_REVISION_V2 UINT32_C(2)" "$WIRE_C" "C header must own revision 2"
guard_expect_fixed_in_file "$TAG" "WIRE_REVISION = 2" "$WIRE_PY" "Python projection must use revision 2"
guard_expect_fixed_in_file "$TAG" "immediate_i64_normal_has_no_lifecycle_disposition" "$WIRE_RS" "Rust ImmediateI64 validity test is missing"
guard_expect_fixed_in_file "$TAG" "test_immediate_i64_normal_has_no_lifecycle_disposition" "$WIRE_TEST" "Python ImmediateI64 validity test is missing"
guard_expect_fixed_in_file "$TAG" "_Static_assert(sizeof(HakoDynamicV2CallOutV1) == 48" "$WIRE_C" "C layout assertion is missing"
guard_expect_fixed_in_file "$TAG" "pub mod dynamic_call_slot_wire" "$ROOT_DIR/src/lib.rs" "shared Rust wire projection must be public to the kernel crate"
guard_expect_fixed_in_file "$TAG" "pub struct DynamicV2CallOutV1" "$WIRE_RS" "Rust wire layout must have one public owner"
if rg -n 'struct[[:space:]]+DynamicV2CallOutV1|repr\(C\).*DynamicV2CallOut' "$ROOT_DIR/crates/nyash_kernel/src"; then
  guard_fail "$TAG" "kernel must not define a duplicate DynamicV2CallOut wire"
fi

for pair in "Invalid = 0" "HostHandle = 1" "ImmediateI64 = 2" "Normal = 0" "Fault = 1" "Suspended = 2" "None = 0" "Forwarded = 1" "EndAuthorized = 2"; do
  guard_expect_fixed_in_file "$TAG" "$pair" "$WIRE_RS" "Rust enum drifted: $pair"
done
for pair in "TAG_INVALID = 0" "TAG_HOST_HANDLE = 1" "TAG_IMMEDIATE_I64 = 2" "STATUS_NORMAL = 0" "STATUS_FAULT = 1" "STATUS_SUSPENDED = 2" "DISPOSITION_NONE = 0" "DISPOSITION_FORWARDED = 1" "DISPOSITION_END_AUTHORIZED = 2"; do
  guard_expect_fixed_in_file "$TAG" "$pair" "$WIRE_PY" "Python projection drifted: $pair"
done
for pair in "HAKO_DYNAMIC_V2_TAG_INVALID UINT32_C(0)" "HAKO_DYNAMIC_V2_TAG_HOST_HANDLE UINT32_C(1)" "HAKO_DYNAMIC_V2_TAG_IMMEDIATE_I64 UINT32_C(2)" "HAKO_DYNAMIC_V2_STATUS_NORMAL UINT32_C(0)" "HAKO_DYNAMIC_V2_STATUS_FAULT UINT32_C(1)" "HAKO_DYNAMIC_V2_STATUS_SUSPENDED UINT32_C(2)" "HAKO_DYNAMIC_V2_DISPOSITION_NONE UINT32_C(0)" "HAKO_DYNAMIC_V2_DISPOSITION_FORWARDED UINT32_C(1)" "HAKO_DYNAMIC_V2_DISPOSITION_END_AUTHORIZED UINT32_C(2)"; do
  guard_expect_fixed_in_file "$TAG" "$pair" "$WIRE_C" "C layout owner drifted: $pair"
done

if rg -n -i "hako_dynamic_call_slot_v2|nyrt_host_call_slot|substring_hii|indexOf_hh|RuntimeDataBox|BoxCall" "$WIRE_C" "$WIRE_RS" "$WIRE_PY"; then
  guard_fail "$TAG" "wire projection contains a runtime/provider implementation caller"
fi
for file in "$WIRE_RS" "$WIRE_PY"; do
  lines="$(wc -l < "$file" | tr -d "[:space:]")"
  if (( lines >= 800 )); then guard_fail "$TAG" "wire projection reached hard 800-line boundary: $file has $lines"; fi
done
echo "[$TAG] ok"
