#!/bin/bash
# phase29cm_collections_hot_raw_route_contract_vm.sh
#
# Contract pin (B1f):
# - collections_hot should rewrite the current daily Array/Map hot boxcalls to raw seams.
# - This pin observes AotPrepBox.run_json directly because crate/ny-llvmc exe route does not
#   expose stable IR dumps on demand.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cm_collections_hot_raw_route_contract_vm"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"

tmp_src="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.hako")"
tmp_mir="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.mir.json")"
tmp_check="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.check.hako")"
tmp_log="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"

cleanup() {
  rm -f "$tmp_src" "$tmp_mir" "$tmp_check" "$tmp_log" >/dev/null 2>&1 || true
}
trap cleanup EXIT

cat >"$tmp_src" <<'HCODE'
static box Main {
  method main(args) {
    local a = new ArrayBox()
    a.push(11)
    local v = a.get(0)
    local m = new MapBox()
    m.set("k", v)
    if m.has("k") {
      local out = m.get("k")
      return out
    }
    return 0
  }
}
HCODE

set +e
"$EMIT_ROUTE" --route hako-helper --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$tmp_mir" --input "$tmp_src" >"$tmp_log" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 60 "$tmp_log" || true
  test_fail "$SMOKE_NAME: MIR emit failed rc=$emit_rc"
  exit 1
fi

cat >"$tmp_check" <<'HCODE'
using selfhost.llvm.ir.aot_prep as AotPrepBox

static box Main {
  method main(args) {
    local src = env.get("CANARY_JSON_SRC")
    if src == null { return 0 }

    local out = AotPrepBox.run_json("" + src)
    if out == null { return 0 }

    local text = "" + out
    local ok = 1

    if text.indexOf("\"func\":\"nyash.array.slot_load_hi\"") < 0 { ok = 0 }
    if text.indexOf("\"func\":\"nyash.array.slot_append_hh\"") < 0 { ok = 0 }
    if text.indexOf("\"func\":\"nyash.map.slot_load_hh\"") < 0 { ok = 0 }
    if text.indexOf("\"func\":\"nyash.map.slot_store_hhh\"") < 0 { ok = 0 }
    if text.indexOf("\"func\":\"nyash.map.probe_hh\"") < 0 { ok = 0 }

    if text.indexOf("\"method\":\"push\"") >= 0 { ok = 0 }
    if text.indexOf("\"method\":\"get\"") >= 0 { ok = 0 }
    if text.indexOf("\"method\":\"set\"") >= 0 { ok = 0 }
    if text.indexOf("\"method\":\"has\"") >= 0 { ok = 0 }
    if text.indexOf("\"op\":\"boxcall\"") >= 0 { ok = 0 }

    if ok == 1 { return 1 }
    return 0
  }
}
HCODE

CANARY_JSON_SRC="$(cat "$tmp_mir")"

set +e
cd "$NYASH_ROOT"
NYASH_ENABLE_USING=1 \
HAKO_ENABLE_USING=1 \
HAKO_USING_RESOLVER_FIRST=1 \
NYASH_AOT_COLLECTIONS_HOT=1 \
NYASH_AOT_MAP_KEY_MODE=hh \
CANARY_JSON_SRC="$CANARY_JSON_SRC" \
"$NYASH_BIN" --backend vm "$tmp_check" >>"$tmp_log" 2>&1
check_rc=$?
set -e

if [ "$check_rc" -ne 1 ]; then
  tail -n 80 "$tmp_log" || true
  test_fail "$SMOKE_NAME: collections_hot raw seam rewrite not observed"
  exit 1
fi

test_pass "$SMOKE_NAME"
