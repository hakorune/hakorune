#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SHARED_READER="lang/c-abi/shims/hako_llvmc_ffi_map_lookup_fusion_metadata.inc"
GET_CONSUMER="lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc"
HAS_CONSUMER="lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc"

echo "[map-lookup-fusion-reader-boundary-guard] checking shared reader boundary"

for file in "$SHARED_READER" "$GET_CONSUMER" "$HAS_CONSUMER"; do
  if [ ! -f "$file" ]; then
    echo "[map-lookup-fusion-reader-boundary-guard] ERROR: missing file: $file" >&2
    exit 1
  fi
done

bad_direct_reads="$(
  rg -n 'yyjson_obj_get\([^)]*"map_lookup_fusion_routes"' lang/c-abi/shims -g '*.inc' \
    | grep -v "^${SHARED_READER}:" || true
)"
if [ -n "$bad_direct_reads" ]; then
  echo "[map-lookup-fusion-reader-boundary-guard] ERROR: direct map_lookup_fusion_routes reader outside shared seam" >&2
  printf '%s\n' "$bad_direct_reads" >&2
  exit 1
fi

for consumer in "$GET_CONSUMER" "$HAS_CONSUMER"; do
  if ! rg -q 'match_map_lookup_fusion_route_metadata' "$consumer"; then
    echo "[map-lookup-fusion-reader-boundary-guard] ERROR: consumer does not use shared reader: $consumer" >&2
    exit 1
  fi
done

echo "[map-lookup-fusion-reader-boundary-guard] ok"
