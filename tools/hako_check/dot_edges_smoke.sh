#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  echo "[DOT] hakorune not built: $BIN" >&2
  exit 2
fi

TMP_HAKO="/tmp/dot_edges_$$.hako"
cat >"$TMP_HAKO" <<'HK'
// Minimal two boxes with a call to form one edge
static box Helper {
  method echo(msg) { return 0 }
}
static box Main {
  method main() {
    Helper.echo("hi")
    return 0
  }
}
HK

export LD_LIBRARY_PATH="${ROOT}/target/release:${LD_LIBRARY_PATH:-}"
OUT=$(
  NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 \
  NYASH_DISABLE_PLUGINS=1 NYASH_BOX_FACTORY_POLICY=builtin_first \
  "$BIN" "$ROOT/tools/hako_check/cli.hako" -- --format dot --source-file "$TMP_HAKO" "$(sed 's/\r$//' "$TMP_HAKO")"
)
echo "$OUT" | sed -n '1,80p'
echo "$OUT" | grep -q '"Main.main/0" -> "Helper.echo/1";' && echo "[DOT] edge OK" || { echo "[DOT] edge MISSING" >&2; exit 1; }
rm -f "$TMP_HAKO"
