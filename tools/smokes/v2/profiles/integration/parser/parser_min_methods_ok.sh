#!/usr/bin/env bash
set -euo pipefail

BIN="${NYASH_BIN:-./target/release/hakorune}"
if [ ! -x "$BIN" ]; then echo "nyash binary not found: $BIN" >&2; exit 2; fi

# Minimal static box with two methods; analyzer should see 2 methods via AST
PROG=$(mktemp /tmp/parser_min_methods_ok.XXXXXX.hako)
cat >"$PROG" <<'HK'
static box Main {
  method main() { return 0 }
  method helper() { return 0 }
}
HK

# Build a tiny program to call AnalysisBuilder directly (AST経路)
WRAP=$(mktemp /tmp/parser_min_methods_wrap.XXXXXX.hako)
cat >"$WRAP" <<'HK'
using tools.hako_parser.parser_core as HakoParserCoreBox
static box Main {
  method main(args) {
    local path = args.get(0)
    local text = args.get(1)
    local ast = HakoParserCoreBox.parse(text)
    if ast == null { return 1 }
    local boxes = ast.get("boxes")
    if boxes == null { return 1 }
    // count methods across boxes
    local total = 0
    local i = 0; while i < boxes.size() {
      local b = boxes.get(i); local ms = b.get("methods"); if ms != null { total = total + ms.size() }
      i = i + 1
    }
    if total >= 2 { return 0 } else { return 1 }
  }
}
HK

NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 \
"$BIN" --backend vm "$WRAP" -- "$PROG" "$(cat "$PROG")" >/dev/null 2>&1
rc=$?
rm -f "$PROG" "$WRAP"
exit $rc
