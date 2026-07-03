#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-stageb-numeric-literal-suffix-alignment"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/smokes/v2/lib/stageb_helpers.sh"

BIN="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  echo "[$TAG] ERROR: hakorune binary not found: $BIN" >&2
  echo "[$TAG] build target/release/hakorune before running this guard" >&2
  exit 2
fi

APP_SRC='box Main { static method main() { return 0usize } }'
export NYASH_ROOT="$ROOT_DIR"
export NYASH_BIN="$BIN"
if ! JSON_PATH="$(stageb_compile_to_json_with_args "$APP_SRC")"; then
  echo "[$TAG] ERROR: mode-B compatibility parser route failed" >&2
  exit 1
fi
trap 'rm -f "$JSON_PATH"' EXIT
PROGRAM_JSON="$(cat "$JSON_PATH")"

python3 - "$PROGRAM_JSON" <<'PY'
import json
import sys

program = json.loads(sys.argv[1])
body = program.get("body", [])
if len(body) != 1:
    raise SystemExit(f"expected one statement after suffix scan, got {len(body)}")
stmt = body[0]
if stmt.get("type") != "Return":
    raise SystemExit(f"expected Return statement, got {stmt.get('type')!r}")
expr = stmt.get("expr", {})
if expr.get("type") != "Int":
    raise SystemExit(f"expected Int expr, got {expr.get('type')!r}")
if expr.get("value") != 0:
    raise SystemExit(f"expected value 0, got {expr.get('value')!r}")
if expr.get("declared_type") != "usize":
    raise SystemExit(f"expected declared_type usize, got {expr.get('declared_type')!r}")
for node in body:
    if node.get("type") == "Expr" and node.get("expr", {}).get("name") == "usize":
        raise SystemExit("suffix leaked as a trailing Var(usize) expression")
PY

rg -F -q 'is_alnum_or_underscore' lang/src/compiler/parser/scan/parser_common_utils_box.hako
rg -F -q 'declared_type' lang/src/compiler/parser/scan/parser_number_scan_box.hako
rg -F -q 'declared_type' lang/src/compiler/stage1/json_program_box.hako

echo "[$TAG] ok"
