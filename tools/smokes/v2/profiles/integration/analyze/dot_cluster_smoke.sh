#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[SMOKE] build missing: $BIN" >&2
  echo "Run: cargo build --release" >&2
  exit 2
fi

# Use an existing test pair to generate DOT and check cluster emission
OUT="/tmp/hc_dot_$$.dot"
set +e
NYASH_JSON_ONLY=1 "$ROOT/tools/hako_check.sh" --format dot "$ROOT/tools/hako_check/tests/HC011_dead_methods" > "$OUT" 2>/dev/null
rc=$?
set -e

if ! grep -q 'subgraph "cluster_' "$OUT"; then
  echo "[SMOKE/FAIL] dot_cluster_smoke: no clusters found" >&2
  sed -n '1,120p' "$OUT" >&2 || true
  exit 1
fi
echo "[SMOKE/OK] dot_cluster_smoke"
rm -f "$OUT"
