#!/usr/bin/env bash
# dual_emit_compare.sh — Dual‑emit MIR(JSON) (provider vs selfhost) and compare + bench
# Usage: tools/perf/dual_emit_compare.sh <input.hako> [rounds]
# Output: human summary + CSV snippets (provider/selfhost benches)

set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <input.hako> [rounds]" >&2
  exit 2
fi
IN="$1"; ROUNDS="${2:-3}"

ROOT="$(cd "$(dirname "$0")"/../.. && pwd)"
EMIT="$ROOT/tools/hakorune_emit_mir.sh"
BENCH="$ROOT/tools/perf/bench_hakorune_emit_mir.sh"
CMP="$ROOT/tools/perf/compare_mir_json.sh"

for f in "$EMIT" "$BENCH" "$CMP"; do
  [[ -x "$f" ]] || { echo "error: missing executable: $f" >&2; exit 2; }
done
[[ -f "$IN" ]] || { echo "error: input not found: $IN" >&2; exit 2; }

prov_csv=$(HAKO_SELFHOST_BUILDER_FIRST=0 "$BENCH" "$IN" "$ROUNDS" || true)
self_csv=$(HAKO_SELFHOST_BUILDER_FIRST=1 "$BENCH" "$IN" "$ROUNDS" || true)

calc_stats() {
  # stdin: CSV header then rows: round,ms,size,sha1
  awk -F, 'NR>1 && $2 ~ /^[0-9]+$/ { n++; s+=$2; arr[n]=$2 } END {
    if (n==0) { print "count=0 avg=NA p50=NA"; exit }
    asort(arr)
    p50 = (n%2==1)? arr[(n+1)/2] : (arr[n/2]+arr[n/2+1])/2
    printf("count=%d avg=%.0f p50=%.0f\n", n, (s/n), p50)
  }'
}

prov_stats=$(printf '%s\n' "$prov_csv" | calc_stats)
self_stats=$(printf '%s\n' "$self_csv" | calc_stats)

OUT_PROV="/tmp/dual_mir_provider_$$.json"
OUT_SELF="/tmp/dual_mir_selfhost_$$.json"
trap 'rm -f "$OUT_PROV" "$OUT_SELF" || true' EXIT

# Produce concrete MIR JSONs
HAKO_SELFHOST_BUILDER_FIRST=0 "$EMIT" "$IN" "$OUT_PROV" >/dev/null 2>&1 || true
HAKO_SELFHOST_BUILDER_FIRST=1 "$EMIT" "$IN" "$OUT_SELF" >/dev/null 2>&1 || true

echo "== Dual‑Emit Bench Summary =="
echo "input: $IN  rounds: $ROUNDS"
echo "provider: $prov_stats"
echo "selfhost: $self_stats"

if [[ -s "$OUT_PROV" && -s "$OUT_SELF" ]]; then
  echo "\n== Structural Compare (normalized) =="
  "$CMP" "$OUT_PROV" "$OUT_SELF" || true
else
  echo "\n[warn] one or both MIR outputs missing. Check bench CSV for ERROR rows." >&2
fi

echo "\n== Provider CSV =="
printf '%s\n' "$prov_csv" | sed -n '1,20p'
echo "\n== Selfhost CSV =="
printf '%s\n' "$self_csv" | sed -n '1,20p'

exit 0

