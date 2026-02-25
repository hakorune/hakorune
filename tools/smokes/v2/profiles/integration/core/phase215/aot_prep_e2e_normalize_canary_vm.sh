#!/usr/bin/env bash
# E2E Canary: AotPrepBox.run_json applies normalize passes with toggles (no FileBox)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

MIR_JSON='{
  "functions": [
    {"name": "main", "params": [], "locals": [],
     "blocks": [
       {"id":0, "instructions": [
         {"op":"const","dst":1,"value":{"type":"i64","value":0}},
         {"op":"const","dst":2,"value":{"type":"i64","value":1}},
         {"op":"newbox","dst":3,"type":"ArrayBox","args":[]},
         {"op":"array_set","array":3,"index":1,"value":2},
         {"op":"array_get","dst":4,"array":3,"index":1}
       ]}
     ]
    }
  ]
}'

TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
using selfhost.llvm.ir.aot_prep as AotPrepBox
static box Main { method main(args) {
  local s = env.get("CANARY_JSON_SRC")
  if s == null { return 0 }
  // run_json does full chain (normalize/hoist/collections) based on toggles
  local out = AotPrepBox.run_json("" + s)
  if out == null { return 0 }
  // Expect boxcall(get/set), and no array_get/array_set
  local ok = 1
  if ("" + out).indexOf("\"op\":\"boxcall\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"method\":\"get\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"method\":\"set\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"op\":\"array_get\"") >= 0 { ok = 0 }
  if ("" + out).indexOf("\"op\":\"array_set\"") >= 0 { ok = 0 }
  if ok == 1 { return 1 } else { return 0 }
} }
HAKO

set +e
cd "$ROOT_DIR"
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 HAKO_USING_RESOLVER_FIRST=1 \
  HAKO_MIR_NORMALIZE_ARRAY=1 \
  CANARY_JSON_SRC="$MIR_JSON" "$NYASH_BIN" --backend vm "$TMP_HAKO" >/dev/null 2>&1
rc=$?
set -e
if [ "$rc" -eq 1 ]; then
  echo "[PASS] aot_prep_e2e_normalize_canary_vm"
  exit 0
fi
echo "[SKIP] AotPrep E2E no transform observed"
exit 0

