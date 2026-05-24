#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-stageb-param-type-annotation-alignment"
cd "$ROOT_DIR"

BIN="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  echo "[$TAG] ERROR: hakorune binary not found: $BIN" >&2
  echo "[$TAG] build target/release/hakorune before running this guard" >&2
  exit 2
fi

TMP_APP="$(mktemp "/tmp/${TAG}.app.XXXXXX.hako")"
RAW_OUT="/tmp/${TAG}.raw.$$"
ERR_OUT="/tmp/${TAG}.err.$$"
trap 'rm -f "$TMP_APP" "$RAW_OUT" "$ERR_OUT"' EXIT

cat >"$TMP_APP" <<'HAKO'
using lang.compiler.entry.func_scanner as FuncScannerBox
using lang.compiler.entry.stageb.stageb_json_builder_box as StageBJsonBuilderBox

static box Main {
  method main() {
    local body = "\n  method helper(x: usize, y) {\n    return x\n  }\n  method main() {\n    return 0\n  }\n"
    local defs = FuncScannerBox._scan_methods(body, "Main", 1, 1)
    local json = StageBJsonBuilderBox.build_defs_json(defs)
    print(json)
    return 0
  }
}
HAKO

if ! NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  "$BIN" --backend vm "$TMP_APP" >"$RAW_OUT" 2>"$ERR_OUT"; then
  echo "[$TAG] ERROR: FuncScanner/StageBJsonBuilder probe failed" >&2
  sed -n '1,120p' "$ERR_OUT" >&2 || true
  exit 1
fi

DEFS_FRAGMENT="$(rg -m 1 '^,"defs":' "$RAW_OUT" || true)"
if [ -z "$DEFS_FRAGMENT" ]; then
  echo "[$TAG] ERROR: defs fragment not found in probe output" >&2
  sed -n '1,120p' "$RAW_OUT" >&2 || true
  exit 1
fi

python3 - "$DEFS_FRAGMENT" <<'PY'
import json
import sys

fragment = sys.argv[1]
program = json.loads("{" + fragment[1:] + "}")
defs = program.get("defs", [])
if len(defs) != 1:
    raise SystemExit(f"expected one helper def, got {len(defs)}")
helper = defs[0]
if helper.get("name") != "helper":
    raise SystemExit(f"expected helper def, got {helper.get('name')!r}")
if helper.get("params") != ["me", "x", "y"]:
    raise SystemExit(f"unexpected params: {helper.get('params')!r}")
decls = helper.get("param_decls")
expected = [
    {"name": "me", "declared_type": None},
    {"name": "x", "declared_type": "usize"},
    {"name": "y", "declared_type": None},
]
if decls != expected:
    raise SystemExit(f"unexpected param_decls: {decls!r}")
PY

rg -F -q 'parse_param_decls_json' lang/src/compiler/entry/func_scanner_helpers.hako
rg -F -q 'param_decls' lang/src/compiler/entry/func_scanner.hako
rg -F -q 'param_decls_json' lang/src/compiler/entry/stageb/stageb_json_builder_box.hako

echo "[$TAG] ok"
