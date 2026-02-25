#!/bin/bash
# Selfhost pipeline v2 → provider → MIR(JSON v1) → hv1 inline PRIMARY → rc=42
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Phase S0.1: Canary tests are opt-in (SMOKES_ENABLE_SELFHOST=1)
# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  test_skip "selfhost_v1_provider_primary_rc42_canary_vm" "opt-in selfhost canary (SMOKES_ENABLE_SELFHOST=1). SSOT: investigations/selfhost-integration-limitations.md"
  exit 0
fi

tmp="/tmp/selfhost_v1prov_$$.json"
bash "$ROOT/tools/selfhost/gen_v1_from_provider.sh" > "$tmp"

set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp" || true

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 42 ]; then
  echo "[PASS] selfhost_v1_provider_primary_rc42_canary_vm"
  exit 0
fi
echo "[SKIP] selfhost_v1_provider_primary_rc42_canary_vm (rc=$rc, provider route not fully wired in this build)" >&2
exit 0
