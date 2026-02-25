#!/usr/bin/env bash
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT"

TMP_SRC=$(mktemp --suffix .hako)
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
OUT=$(NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
      NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
      NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
      "$NYASH_BIN" --backend vm lang/src/compiler/entry/compiler_stageb.hako -- --source "$(cat "$TMP_SRC")" 2>/dev/null | awk '/^{/,/^}$/')

python3 - "$TMP_SRC" << 'PY' <<EOF
import json,sys
src_path=sys.stdin.readline().strip()  # not used, reserved
s=sys.stdin.read()
j=json.loads(s)
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
EOF

rm -f "$TMP_SRC"
exit 0
