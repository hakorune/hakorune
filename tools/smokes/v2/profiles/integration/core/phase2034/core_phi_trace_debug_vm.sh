#!/bin/bash
# Debug helper: emit MIR(JSON) via env.mirbuilder.emit, ensure version=0 (patch via jq when missing),
# then run Core (--json-file) with PHI trace enabled to aid diagnosis.
# Default: SKIP unless SMOKES_ENABLE_DEBUG=1 (does not gate on rc; prints trace).

set -euo pipefail

if [ "${SMOKES_ENABLE_DEBUG:-0}" != "1" ]; then
  echo "[SKIP] core_phi_trace_debug_vm (enable with SMOKES_ENABLE_DEBUG=1)" >&2
  exit 0
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/mir_emit_phi_$$.hako"
tmp_json="/tmp/mir_emit_phi_$$.json"
tmp_json_v="/tmp/mir_emit_phi_v_$$.json"

cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Program: if (1 < 2) return 10; else return 20;
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[{\"type\":\"If\",\"cond\":{\"type\":\"Compare\",\"op\":\"<\",\"lhs\":{\"type\":\"Int\",\"value\":1},\"rhs\":{\"type\":\"Int\",\"value\":2}},\"then\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":10}}],\"else\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":20}}]}]}";
  local arr = new ArrayBox(); arr.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", arr)
  if out == null { return 1 }
  print("" + out)
  return 0
} }
HAKO

set +e
out="$(out="$(NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
json_only="$(echo "$out" | sed -n '/^{/,$p')"
echo "$json_only" | jq -e . > "$tmp_json"

# Ensure top-level version=0 exists
if jq -e 'has("version")' "$tmp_json" >/dev/null 2>&1; then
  cp "$tmp_json" "$tmp_json_v"
else
  jq '. + {"version":0}' "$tmp_json" > "$tmp_json_v"
fi

echo "[INFO] Running Core with PHI trace (see below)…" >&2
set +e
NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 NYASH_VM_TRACE_PHI=1 "$NYASH_BIN" --json-file "$tmp_json_v" 2>&1 | sed -n '1,220p'
code=$?
set -e
echo "[INFO] Core rc=$code" >&2
rm -f "$tmp_hako" "$tmp_json" "$tmp_json_v" || true
echo "[PASS] core_phi_trace_debug_vm (diagnostic run; see trace above)"
exit 0
