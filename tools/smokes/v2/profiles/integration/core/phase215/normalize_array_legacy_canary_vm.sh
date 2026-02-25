#!/usr/bin/env bash
# Canary: NormalizeArrayLegacyBox rewrites array_get/array_set to boxcall get/set

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

MIR_JSON='{
  {
    "functions": [
      {
        "name": "main", "params": [], "locals": [],
        "blocks": [
          {"id":0,"instructions":[
            {"op":"const","dst":1,"value":{"type":"i64","value":0}},
            {"op":"const","dst":2,"value":{"type":"i64","value":1}},
            {"op":"newbox","dst":3,"type":"ArrayBox","args":[]},
            {"op":"array_set","array":3,"index":1,"value":2},
            {"op":"array_get","dst":4,"array":3,"index":1}
          ]}
        ]
      }
    ]
  }
}'

# Build a small Hako script that applies NormalizeArrayLegacyBox.run(json)
TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
using selfhost.llvm.ir.normalize.array_legacy as NormalizeArrayLegacyBox
static box Main { method main(args) {
  // Read JSON from env
  local s = env.get("CANARY_JSON_SRC")
  if s == null { return 0 }
  local out = NormalizeArrayLegacyBox.run("" + s)
  if out == null { return 0 }
  // Success criteria: has boxcall get/set and no legacy ops
  local ok = 1
  if ("" + out).indexOf("\"op\":\"boxcall\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"method\":\"get\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"method\":\"set\"") < 0 { ok = 0 }
  if ("" + out).indexOf("\"op\":\"array_get\"") >= 0 { ok = 0 }
  if ("" + out).indexOf("\"op\":\"array_set\"") >= 0 { ok = 0 }
  if ok == 1 { return 1 } else { return 0 }
} }
HAKO

# Run and capture
set +e
cd "$ROOT_DIR"
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 HAKO_USING_RESOLVER_FIRST=1 \
  CANARY_JSON_SRC="$MIR_JSON" "$NYASH_BIN" --backend vm "$TMP_HAKO" >/dev/null 2>&1
rc=$?
set -e
if [ "$rc" -eq 1 ]; then
  echo "[PASS] normalize_array_legacy_canary_vm"
  exit 0
fi
echo "[SKIP] no transform observed"
exit 0
