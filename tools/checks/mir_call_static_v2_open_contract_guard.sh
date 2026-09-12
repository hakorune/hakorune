#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-static-v2-open-contract"
RUST="$ROOT_DIR/src/host_providers/llvm_codegen/static_invocation.rs"
CONTRACT="$ROOT_DIR/src/host_providers/llvm_codegen/capi_transport.rs"
C="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_static_v2_compile.inc"
OPTIONS="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_physical_options.inc"
HEADER="$ROOT_DIR/lang/c-abi/include/hako_llvmc_ffi.h"
TEST="$ROOT_DIR/lang/c-abi/tests/static_v2_session_test.py"

command -v rg >/dev/null
command -v wc >/dev/null

rg -q 'hako_llvmc_static_open_v2_with_options' "$RUST"
rg -q 'row_for_profile\(2\)' "$RUST"
rg -q 'OwnedPhysicalCompileContract::from_request' "$RUST"
if rg -n 'hako_llvmc_static_open_v2\\0|set_var|remove_var|std::env' "$RUST"; then
  echo "[$TAG] static Rust caller retains old dlsym or environment mutation" >&2
  exit 1
fi

rg -q 'hako_llvmc_static_open_v2_with_contract' "$C"
rg -q 'hako_llvmc_physical_options_copy\(&options, contract, error\)' "$C"
rg -q 'hako_llvmc_static_open_v2_with_options' "$HEADER"
rg -q 'HAKO_LLVMC_PHYSICAL_PROFILE_STATIC_V2' "$OPTIONS"
rg -q 'open_with_options' "$TEST"

for file in "$RUST" "$CONTRACT" "$C" "$OPTIONS" "$HEADER" "$TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[$TAG] source reached hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
