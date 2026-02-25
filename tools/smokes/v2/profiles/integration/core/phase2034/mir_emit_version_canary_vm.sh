#!/bin/bash
# Canary: Program(JSON v0) → env.mirbuilder.emit → MIR(JSON v0)
# Check: top-level "version" exists. This currently FAILs to surface the bug.
# Default: SKIP unless explicitly enabled (avoid breaking quick profile).

set -euo pipefail

if [ "${SMOKES_ENABLE_DEBUG:-0}" != "1" ]; then
  echo "[SKIP] mir_emit_version_canary_vm (enable with SMOKES_ENABLE_DEBUG=1)" >&2
  exit 0
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/mir_emit_ver_$$.hako"
tmp_json="/tmp/mir_emit_ver_$$.json"

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
if ! echo "$json_only" | jq -e . > "$tmp_json" 2>/dev/null; then
  echo "[FAIL] mir_emit_version_canary_vm (no MIR JSON)" >&2
  rm -f "$tmp_hako" "$tmp_json" || true
  exit 1
fi

if ! grep -q '"version"' "$tmp_json"; then
  echo "[FAIL] mir_emit_version_canary_vm (missing top-level \"version\")" >&2
  rm -f "$tmp_hako" "$tmp_json" || true
  exit 1
fi

echo "[PASS] mir_emit_version_canary_vm"
rm -f "$tmp_hako" "$tmp_json" || true
exit 0
