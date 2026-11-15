#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-inspect-selected-dynamic-provenance-guard"
TEMP_DIR="$(mktemp -d /tmp/hako-inspect-selected-dynamic.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

for source in \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_checked_callout_lowering.inc" \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc" \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc"; do
  if [[ "$(wc -l <"$source")" -ge 760 ]]; then
    echo "[$TAG] ERROR: source reached the 760-line split boundary: $source" >&2
    exit 1
  fi
done

cc -I"$ROOT/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/driver" \
  "$ROOT/lang/c-abi/tests/selected_dynamic_lowered_llvm_provenance_driver.c" \
  "$ROOT/lang/c-abi/shims/hako_aot.c" \
  "$ROOT/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl

PYTHONPATH="$ROOT/tools/hako_check" python3 -m unittest \
  tools.hako_check.tests.test_inspect_origin_footprint \
  tools.hako_check.tests.test_inspect_origin_footprint_c_reference \
  tools.hako_check.tests.test_inspect_provenance_dispositions \
  tools.hako_check.tests.test_inspect_provenance_model \
  tools.hako_check.tests.test_inspect_selected_dynamic_provenance

# The private emitter producer retired; current library/driver checks remain above.
echo "[$TAG] ok"
