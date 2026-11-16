#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="pinned-text-selected-preflight-smoke"
TEMP_DIR="$(mktemp -d /tmp/hako-pinned-text-preflight.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
"${CC:-cc}" \
  -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/verifier-negative" \
  "$ROOT_DIR/lang/c-abi/tests/pinned_text_selected_verifier_test.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c"
"$TEMP_DIR/verifier-negative"
"${CC:-cc}" \
  -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/final-module-closure" \
  "$ROOT_DIR/lang/c-abi/tests/pinned_text_final_module_closure_test.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c"
"$TEMP_DIR/final-module-closure"
echo "[$TAG] ok (independent C verifier and final-module closure)"
