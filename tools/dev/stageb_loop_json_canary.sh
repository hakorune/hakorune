#!/usr/bin/env bash
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT"
source "$ROOT/tools/selfhost/lib/stageb_program_json_capture.sh"

TMP_SRC=$(mktemp --suffix .hako)
RAW_OUT=$(mktemp)
JSON_OUT=$(mktemp --suffix .program.json)
cleanup() {
  rm -f "$TMP_SRC" "$RAW_OUT" "$JSON_OUT" 2>/dev/null || true
}
trap cleanup EXIT

cat >"$TMP_SRC" <<'HAKO'
static box Main {
  method main(){
    local n = 10
    local i = 0
    loop(i < n) {
      i = i + 1
    }
    return i
  }
}
HAKO

NYASH_BIN=${NYASH_BIN:-"$ROOT/target/release/hakorune"}
[[ -x "$NYASH_BIN" ]] || NYASH_BIN="$ROOT/target/release/nyash"

# Emit Program(JSON v0) via Stage‑B
NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
  "$NYASH_BIN" --backend vm lang/src/compiler/entry/compiler_stageb.hako -- --source "$(cat "$TMP_SRC")" >"$RAW_OUT" 2>/dev/null
stageb_program_json_extract_from_stdin < "$RAW_OUT" > "$JSON_OUT"

python3 - "$JSON_OUT" <<'PY'
import json
import pathlib
import sys

j = json.loads(pathlib.Path(sys.argv[1]).read_text())
assert j.get('kind')=='Program', 'not Program(JSON)'
loops=[x for x in j.get('body',[]) if isinstance(x,dict) and x.get('type')=='Loop']
assert loops, 'no Loop node found'
loop=loops[0]
cond=loop.get('cond',{})
assert cond.get('type')=='Compare' and cond.get('op')=='<', 'cond not Compare <'
lhs=cond.get('lhs',{})
rhs=cond.get('rhs',{})
assert lhs.get('type')=='Var' and lhs.get('name')=='i', 'lhs not Var i'
assert (rhs.get('type') in ('Var','Int')), 'rhs not Var/Int'
body=loop.get('body')
assert isinstance(body,list) and len(body)>0, 'empty loop body'
# expect Local i with Binary +
local_i=[b for b in body if b.get('type')=='Local' and b.get('name')=='i']
assert local_i, 'no Local i in body'
expr=local_i[0].get('expr',{})
assert expr.get('type')=='Binary' and expr.get('op')=='+', 'Local i not Binary +'
print('[PASS] stageb_loop_json_canary')
PY
