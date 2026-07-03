#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-stageb-field-type-annotation-alignment"
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
using lang.compiler.build.build_program_fragment_box as BuildProgramFragmentBox

static box Main {
  method main() {
    local src = "box Main {\n  count: usize = 0usize\n  static method main() {\n    return 0\n  }\n}\n"
    local base = "{\"version\":0,\"kind\":\"Program\",\"body\":[],\"static_data_plans\":[]}"
    local json = BuildProgramFragmentBox.enrich(base, src)
    print(json)
    return 0
  }
}
HAKO

if ! NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  "$BIN" --backend vm "$TMP_APP" >"$RAW_OUT" 2>"$ERR_OUT"; then
  echo "[$TAG] ERROR: mode-B compatibility user_box_decls scanner probe failed" >&2
  sed -n '1,120p' "$ERR_OUT" >&2 || true
  exit 1
fi

PROGRAM_JSON="$(rg -m 1 '^\{"version":0,"kind":"Program"' "$RAW_OUT" || true)"
if [ -z "$PROGRAM_JSON" ]; then
  echo "[$TAG] ERROR: enriched Program(JSON v0) not found in probe output" >&2
  sed -n '1,120p' "$RAW_OUT" >&2 || true
  exit 1
fi

python3 - "$PROGRAM_JSON" <<'PY'
import json
import sys

program = json.loads(sys.argv[1])
decls = program.get("user_box_decls", [])
if len(decls) != 1:
    raise SystemExit(f"expected one user box decl, got {len(decls)}")
decl = decls[0]
if decl.get("name") != "Main":
    raise SystemExit(f"expected Main decl, got {decl.get('name')!r}")
if decl.get("fields") != ["count"]:
    raise SystemExit(f"unexpected fields: {decl.get('fields')!r}")
field_decls = decl.get("field_decls")
expected = [{"name": "count", "declared_type": "usize", "is_weak": False}]
if field_decls != expected:
    raise SystemExit(f"unexpected field_decls: {field_decls!r}")
PY

rg -F -q 'StageBUserBoxDeclScannerBox' lang/src/compiler/build/build_program_fragment_box.hako
rg -F -q 'user_box_decls' lang/src/compiler/entry/stageb/stageb_user_box_decl_scanner_box.hako
rg -F -q 'field_decls' lang/src/compiler/entry/stageb/stageb_user_box_decl_scanner_box.hako

echo "[$TAG] ok"
