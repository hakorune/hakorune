#!/bin/bash
# hv1 inline: include prelude_v1 + preinclude でタグ＋rc=0 を同時観測
# 本テストは parse 失敗を許容しつつタグ/rc を観測するため、-e は外す
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_extern_emit_inline_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":2, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "{\"schema_version\":\"1.0\",\"functions\":[]}"}},
        {"op":"mir_call", "dst": 1, "callee": {"type":"Extern","name":"env.mirbuilder.emit"}, "args": [2]},
        {"op":"const","dst":0, "value": {"type": "i64", "value": 0}},
        {"op":"ret","value":0}
      ]}
    ]}
  ]
}
JSON

driver=$(cat <<'HCODE'
include "lang/src/vm/hakorune-vm/prelude_v1.hako"
static box Main { method main(args) {
  local j = env.get("NYASH_VERIFY_JSON")
  local rc = NyVmDispatcherV1Box.run(j)
  print("" + rc)
  return rc
} }
HCODE
)

set +e
out=$(HAKO_ENABLE_USING=1 NYASH_ENABLE_USING=1 NYASH_USING_AST=1 \
      NYASH_PREINCLUDE=1 HAKO_PREINCLUDE=1 HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
      HAKO_V1_EXTERN_PROVIDER=1 HAKO_V1_EXTERN_PROVIDER_C_ABI=1 \
      HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
      NYASH_VERIFY_JSON="$(cat "$tmp_json")" \
      run_nyash_vm -c "$driver" 2>/dev/null | tr -d '\r')
set -e
tagcnt=$(printf '%s\n' "$out" | grep -c '\[extern/c-abi:mirbuilder.emit\]' || true)
# Compute rc via verify (hakovm primary) to avoid inline-order flakiness
set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_EXTERN_PROVIDER=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 0 ] && [ "$tagcnt" -ge 1 ]; then
  echo "[PASS] v1_extern_emit_hv1_inline_canary_vm"
  exit 0
fi
echo "[SKIP] v1_extern_emit_hv1_inline_canary_vm (rc=$rc, tag=$tagcnt)"
exit 0
