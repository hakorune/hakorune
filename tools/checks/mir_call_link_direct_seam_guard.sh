#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-link-direct-seam"
ROUTE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_route.inc"
AOT="$ROOT_DIR/lang/c-abi/shims/hako_aot_shared_impl.inc"
INTERNAL="$ROOT_DIR/lang/c-abi/shims/hako_aot_internal.h"

command -v rg >/dev/null
command -v wc >/dev/null

test "$(rg -c 'hako_aot_link_obj_for_llvmc\(' "$ROUTE")" = 2
test "$(rg -c '^HAKO_AOT_INTERNAL_LINK int hako_aot_link_obj_for_llvmc\(' "$AOT")" = 1
rg -q 'HAKO_AOT_LINK_INVOCATION_COMPAT_V1' "$INTERNAL"
rg -q 'HAKO_AOT_LINK_INVOCATION_EXPLICIT_V2' "$INTERNAL"
if rg -n 'hako_llvmc_set_env_value\("HAKO_AOT_USE_FFI"|hako_aot_link_obj\(obj_in|hako_aot_link_obj_v2\(' "$ROUTE"; then
  echo "[$TAG] FFI link route retains process-global mutation or public re-entry" >&2
  exit 1
fi

for file in "$ROUTE" "$AOT" "$INTERNAL"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[$TAG] source reached hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
