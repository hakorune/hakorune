#!/usr/bin/env bash
# emit_boxcall_length_canary_vm.sh — Ensure --emit-mir-json contains boxcall length

# Note: test_runner.sh handles shell options; set -euo pipefail conflicts with run_test

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

test_emit_boxcall_length_json() {
  require_env || return 1
  local SRC
  SRC=$(mktemp --suffix .hako)
  cat >"$SRC" <<'HKR'
static box Main {
  s
  n

  main() {
    me.s = new StringBox("nyash")
    me.n = me.s.length()
    return me.n
  }
}
HKR
  local OUT_JSON
  OUT_JSON=$(mktemp --suffix .json)
  # Force v0 call shape; ensure we emit mir json from runner
  # Use --no-optimize to prevent call inlining
  NYASH_MIR_UNIFIED_CALL=0 NYASH_DISABLE_PLUGINS=1 HAKO_ALLOW_NYASH=1 "$NYASH_BIN" --no-optimize --emit-mir-json "$OUT_JSON" --backend mir "$SRC" >/dev/null 2>&1
  # Assert tokens (account for pretty-printed JSON with spaces)
  cat "$OUT_JSON" | assert_has_tokens '"op": "boxcall"' '"method": "length"'
}

run_test "emit_boxcall_length_json" test_emit_boxcall_length_json
