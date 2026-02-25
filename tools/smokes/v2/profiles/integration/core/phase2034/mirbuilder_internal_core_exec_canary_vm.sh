#!/bin/bash
# MirBuilder internal → Gate‑C/Core exec canary — rc verification
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_emit_$$.hako"
tmp_json="/tmp/mirbuilder_emit_$$.json"

cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Program: if (1 < 2) return 10; else return 20;
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[{\"type\":\"If\",\"cond\":{\"type\":\"Compare\",\"op\":\"<\",\"lhs\":{\"type\":\"Int\",\"value\":1},\"rhs\":{\"type\":\"Int\",\"value\":2}},\"then\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":10}}],\"else\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":20}}]}]}";
  local arr = new ArrayBox(); arr.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", arr)
  if out == null { return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# 1) Emit MIR(JSON) to a temp file
set +e
out="$(NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1)"; rc=$?
set -e
mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{f=1;next}/\[MIR_END\]/{f=0}f')
if ! jq -e . >/dev/null 2>&1 <<<"$mir"; then
  echo "[FAIL] mirbuilder_internal_core_exec_canary_vm (no MIR JSON)" >&2
  rm -f "$tmp_hako" "$tmp_json" || true
  exit 1
fi

printf "%s" "$mir" > "$tmp_json"

# 2) Core‑Direct exec and rc check (expect rc=10)
set +e
HAKO_VERIFY_PRIMARY=core verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_hako" "$tmp_json" || true

if [ "$rc" -eq 10 ]; then
  echo "[PASS] mirbuilder_internal_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_internal_core_exec_canary_vm (rc=$rc, expect 10)" >&2; exit 1
