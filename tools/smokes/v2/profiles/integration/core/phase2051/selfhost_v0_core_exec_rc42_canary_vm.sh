#!/bin/bash
# Selfhost pipeline v2 → MIR(JSON v0) → Core exec → rc=42
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Phase S0.1: Canary tests are opt-in (SMOKES_ENABLE_SELFHOST=1)
# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  test_skip "selfhost_v0_core_exec_rc42_canary_vm" "opt-in selfhost canary (SMOKES_ENABLE_SELFHOST=1). SSOT: investigations/selfhost-integration-limitations.md"
  exit 0
fi

tmp="/tmp/selfhost_v0_$$.json"
bash "$ROOT/tools/selfhost/gen_v0_from_selfhost_pipeline_min.sh" > "$tmp"

set +e
"$NYASH_BIN" --json-file "$tmp" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp" || true

if [ "$rc" -eq 42 ]; then
  echo "[PASS] selfhost_v0_core_exec_rc42_canary_vm"
  exit 0
fi
echo "[FAIL] selfhost_v0_core_exec_rc42_canary_vm (rc=$rc, expect 42)" >&2
exit 1
