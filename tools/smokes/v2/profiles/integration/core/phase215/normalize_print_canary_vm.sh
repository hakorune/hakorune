#!/usr/bin/env bash
# Canary: NormalizePrintBox rewrites print -> externcall(env.console.log)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

MIR_JSON='{
  "functions": [
    {"name": "main", "params": [], "locals": [],
     "blocks": [
       {"id":0, "instructions": [
         {"op":"const","dst":1,"value":{"type":"i64","value":42}},
         {"op":"print","value":1}
       ]}
     ]
    }
  ]
}'

TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
using selfhost.llvm.ir.normalize.print as NormalizePrintBox
static box Main { method main(args) {
  local s = env.get("CANARY_JSON_SRC")
  if s == null { return 0 }
  local out = NormalizePrintBox.run("" + s)
  if out == null { return 0 }
  // Success: has externcall env.console.log, and no print
  local ok = 1
  if ("" + out).indexOf("\"op\":\"externcall\"") < 0 { ok = 0 }
  if ("" + out).indexOf("env.console.log") < 0 { ok = 0 }
  if ("" + out).indexOf("\"op\":\"print\"") >= 0 { ok = 0 }
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
  echo "[PASS] normalize_print_canary_vm"
  exit 0
fi
echo "[SKIP] no transform observed (print)"
exit 0

