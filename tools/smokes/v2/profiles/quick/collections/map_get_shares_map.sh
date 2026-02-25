#!/usr/bin/env bash
set -euo pipefail

BIN="${NYASH_BIN:-./target/release/hakorune}"
if [ ! -x "$BIN" ]; then echo "nyash binary not found: $BIN" >&2; exit 2; fi

PROG=$(mktemp /tmp/map_get_shares_map.XXXXXX.hako)
cat >"$PROG" <<'HK'
static box Main {
  method main() {
    local m = new MapBox()
    m.set("m", new MapBox())
    local inner = m.get("m")
    inner.set("k", 42)
    // Reflect to original
    if m.get("m").has("k").toString() == "true" { return 0 } else { return 1 }
  }
}
HK

NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_PARSER_SEAM_TOLERANT=1 HAKO_PARSER_SEAM_TOLERANT=1 \
"$BIN" --backend vm "$PROG" >/dev/null 2>&1
rc=$?
rm -f "$PROG"
exit $rc
