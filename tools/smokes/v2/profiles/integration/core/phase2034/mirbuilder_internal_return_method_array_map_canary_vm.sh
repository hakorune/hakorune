#!/bin/bash
# Return(Method recv Var, method=size/get/set/push) → mir_call structure check
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_return_method_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Local a = new ArrayBox(); return a.size();  (shape only, not executed here)
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[" +
    "{\"type\":\"Local\",\"name\":\"a\",\"expr\":{\"type\":\"New\",\"class\":\"ArrayBox\",\"args\":[]}}," +
    "{\"type\":\"Return\",\"expr\":{\"type\":\"Method\",\"recv\":{\"type\":\"Var\",\"name\":\"a\"},\"method\":\"size\",\"args\":[]}}]}";
  // Use delegate provider directly for MIR generation
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

set +e
out="$(HAKO_MIR_BUILDER_INTERNAL=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag')
# Accept either mir_call (registry fast path) or boxcall (internal path) with method=size
if [ -n "$mir" ]; then
  if echo "$mir" | grep -q '"op":"mir_call"' && echo "$mir" | grep -q '"method":"size"'; then
    echo "[PASS] mirbuilder_internal_return_method_array_map_canary_vm"; exit 0
  fi
  if echo "$mir" | grep -q '"op":"boxcall"' && echo "$mir" | grep -q '"method":"size"'; then
    echo "[PASS] mirbuilder_internal_return_method_array_map_canary_vm"; exit 0
  fi
fi
echo "[FAIL] mirbuilder_internal_return_method_array_map_canary_vm" >&2; exit 1
