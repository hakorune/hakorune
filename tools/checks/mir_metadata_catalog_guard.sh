#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-metadata-catalog"
cd "$ROOT_DIR"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/mir_metadata_catalog_sections.sh"

DOC="$ROOT_DIR/docs/reference/mir/metadata-facts-ssot.md"
ROOT_EMIT="$ROOT_DIR/src/runner/mir_json_emit/root.rs"
FUNCTION_TYPES="$ROOT_DIR/src/mir/function/types.rs"
SEMANTIC_REFRESH="$ROOT_DIR/src/mir/semantic_refresh.rs"
RUNE_CONTRACTS="$ROOT_DIR/src/mir/verification/rune_contracts.rs"
INLINE_REQUIRED="$ROOT_DIR/src/mir/verification/inline_required.rs"
STRING_KERNEL_VERIFIER="$ROOT_DIR/src/mir/verification/string_kernel.rs"
EXACT_NUMERIC_CONTRACTS="$ROOT_DIR/src/mir/exact_numeric_field_contracts.rs"
EXACT_NUMERIC_BACKEND="$ROOT_DIR/src/mir/exact_numeric_backend_capability.rs"
EXACT_SEED_BACKEND="$ROOT_DIR/src/mir/exact_seed_backend_route.rs"
ARRAY_RECORD_BOUNDARY="$ROOT_DIR/src/mir/array_record_materialization_boundary.rs"
ARRAY_RECORD_PACKED_PILOT="$ROOT_DIR/src/mir/array_record_packed_autouse_pilot.rs"
SOURCE_PACKED_AUTOUSE="$ROOT_DIR/src/mir/source_packed_array_autouse_pilot.rs"
SOURCE_PACKED_DIRECT="$ROOT_DIR/src/mir/source_packed_array_direct_read_consumption.rs"
ARRAY_RECORD_BACKEND="$ROOT_DIR/src/mir/array_record_backend_capability.rs"
LLVM_COMMON_SHIM="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_common.inc"
LLVM_SUM_LOCAL_SHIM="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_sum_local_seed_metadata_helpers.inc"
LLVM_STRING_CANDIDATE_SHIM="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_string_candidate_plan_readers.inc"
LLVM_PURE_COMPILE_SHIM="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc"
PACKED_BACKEND_GUARD="$ROOT_DIR/tools/checks/k2_wide_packed_record_backend_failfast_guard.sh"
SOURCE_PACKED_AUTOUSE_GUARD="$ROOT_DIR/tools/checks/k2_wide_source_packed_array_autouse_pilot_guard.sh"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" realpath
guard_require_files "$TAG" \
  "$DOC" "$ROOT_EMIT" "$FUNCTION_TYPES" \
  "$SEMANTIC_REFRESH" "$RUNE_CONTRACTS" "$INLINE_REQUIRED" \
  "$STRING_KERNEL_VERIFIER" "$EXACT_NUMERIC_CONTRACTS" \
  "$EXACT_NUMERIC_BACKEND" "$EXACT_SEED_BACKEND" "$ARRAY_RECORD_BOUNDARY" \
  "$ARRAY_RECORD_PACKED_PILOT" "$SOURCE_PACKED_AUTOUSE" \
  "$SOURCE_PACKED_DIRECT" "$ARRAY_RECORD_BACKEND" "$LLVM_COMMON_SHIM" \
  "$LLVM_SUM_LOCAL_SHIM" "$LLVM_STRING_CANDIDATE_SHIM" \
  "$LLVM_PURE_COMPILE_SHIM" "$PACKED_BACKEND_GUARD" \
  "$SOURCE_PACKED_AUTOUSE_GUARD" "$INDEX"

mir_metadata_catalog_check_all

echo "[$TAG] ok module_keys=14 seed_keys=11"
