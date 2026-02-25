#!/bin/bash
# extern: env.mirbuilder.emit — Hako provider stub（C‑ABIタグ＋空文字）; rc=0（ret 0）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_extern_emit_stub_$$.json"
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


# RC check via verify (hakovm primary)
set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_FLOW=1 HAKO_V1_EXTERN_PROVIDER=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

# Only rc check here（タグの観測は hv1 inline カナリーで担保）
if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_extern_emit_stub_canary_vm"
  exit 0
fi
echo "[FAIL] v1_extern_emit_stub_canary_vm (rc=$rc)" >&2; exit 1
