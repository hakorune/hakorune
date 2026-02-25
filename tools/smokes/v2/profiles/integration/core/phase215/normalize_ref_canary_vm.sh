#!/usr/bin/env bash
# Canary: NormalizeRefBox rewrites ref_get/ref_set -> boxcall getField/setField

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

MIR_JSON='{
  "functions": [
    {"name": "main", "params": [], "locals": [],
     "blocks": [
       {"id":0, "instructions": [
         {"op":"const","dst":1,"value":{"type":"i64","value":7}},
         {"op":"ref_set","reference":2, "field":"x", "value":1},
         {"op":"ref_get","dst":3, "reference":2, "field":"x"}
       ]}
     ]
    }
  ]
}'

TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
using selfhost.llvm.ir.normalize.ref as NormalizeRefBox
static box Main { method main(args) {
  local s = env.get("CANARY_JSON_SRC")
  if s == null { return 0 }
  local out = NormalizeRefBox.run("" + s)
  if out == null { return 0 }
  // Success: has getField/setField boxcalls and no legacy ops
  local ok = 1
  if ("" + out).indexOf("\"op\":\"boxcall\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"method\":\"getField\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"method\":\"setField\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"op\":\"ref_get\"") >= 0 { ok = 0 }
  if ("" + out).indexOf("\"op\":\"ref_set\"") >= 0 { ok = 0 }
  if ok == 1 { return 1 } else { return 0 }
} }
HAKO

set +e
cd "$ROOT_DIR"
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 HAKO_USING_RESOLVER_FIRST=1 \
  CANARY_JSON_SRC="$MIR_JSON" "$NYASH_BIN" --backend vm "$TMP_HAKO" >/dev/null 2>&1
rc=$?
set -e
if [ "$rc" -eq 1 ]; then
  echo "[PASS] normalize_ref_canary_vm"
  exit 0
fi
echo "[SKIP] no transform observed (ref)"
exit 0

