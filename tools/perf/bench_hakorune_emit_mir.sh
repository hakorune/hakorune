#!/usr/bin/env bash
# bench_hakorune_emit_mir.sh — Stage‑B → MIR(JSON) bench via Hakorune path
#
# Usage:
#   tools/perf/bench_hakorune_emit_mir.sh <input.hako> [rounds]
#
# Env toggles (forwarded as-is):
#   HAKO_USING_RESOLVER_FIRST=1        # resolver-first
#   HAKO_SELFHOST_BUILDER_FIRST=1      # try selfhost builder first
#   HAKO_MIR_BUILDER_BOX=hako.mir.builder|min  # builder box selector
#   HAKO_SELFHOST_TRACE=1              # extra trace (stderr)
#
# Output: CSV (round,ms,size_bytes,sha1)

set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <input.hako> [rounds]" >&2
  exit 2
fi
IN="$1"; ROUNDS="${2:-5}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
EMIT_ROUTE_HELPER="$ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
if [[ ! -x "$EMIT_ROUTE_HELPER" ]]; then echo "error: $EMIT_ROUTE_HELPER not found/executable" >&2; exit 2; fi
if [[ ! -f "$IN" ]]; then echo "error: input not found: $IN" >&2; exit 2; fi

sha1() {
  if command -v sha1sum >/dev/null 2>&1; then sha1sum | awk '{print $1}';
  elif command -v shasum >/dev/null 2>&1; then shasum -a 1 | awk '{print $1}';
  else openssl sha1 | awk '{print $2}'; fi
}

echo "round,ms,size,sha1"
for ((i=1; i<=ROUNDS; i++)); do
  OUT="/tmp/hako_mir_bench_$$.json"
  rm -f "$OUT" || true
  start=$(date +%s%3N)
  # Forward env toggles implicitly
  if ! "$EMIT_ROUTE_HELPER" --route hako-helper --timeout-secs "${PERF_EMIT_TIMEOUT_SECS:-120}" --out "$OUT" --input "$IN" >/dev/null 2>&1; then
    echo "$i,ERROR,0,NA"; continue
  fi
  end=$(date +%s%3N)
  ms=$((end - start))
  size=$(stat -c '%s' "$OUT" 2>/dev/null || stat -f '%z' "$OUT")
  norm=$(jq -cS . "$OUT" 2>/dev/null || cat "$OUT")
  digest=$(printf '%s' "$norm" | sha1)
  echo "$i,$ms,$size,$digest"
  rm -f "$OUT" || true
done

exit 0
