#!/usr/bin/env bash
set -euo pipefail

BIN="${NYASH_BIN:-./target/release/hakorune}"
if [ ! -x "$BIN" ]; then echo "nyash binary not found: $BIN" >&2; exit 2; fi

PROG=$(mktemp /tmp/string_size_alias.XXXXXX.hako)
cat >"$PROG" <<'HK'
static box Main {
  method main() {
    local s = "hello"
    if s.length().toString() != "5" { return 1 }
    if s.size().toString() != "5" { return 2 }
    return 0
  }
}
HK

NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_PARSER_SEAM_TOLERANT=1 HAKO_PARSER_SEAM_TOLERANT=1 \
"$BIN" --backend vm "$PROG" >/dev/null 2>&1
rc=$?
rm -f "$PROG"
exit $rc
