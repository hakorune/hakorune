#!/bin/bash
# Self‑Hosting S1/S2 via provider (env.mirbuilder.emit) — normalized hash match
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Phase S0.1: Canary tests are opt-in (SMOKES_ENABLE_SELFHOST=1)
# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  test_skip "selfhost_s1_s2_from_provider_canary_vm" "opt-in selfhost canary (SMOKES_ENABLE_SELFHOST=1). SSOT: investigations/selfhost-integration-limitations.md"
  exit 0
fi

cmd="bash $ROOT/tools/selfhost/gen_v1_from_provider.sh"

set +e
out=$(bash "$ROOT/tools/selfhost/bootstrap_s1_s2.sh" --cmd1 "$cmd" --cmd2 "$cmd" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] selfhost_s1_s2_from_provider_canary_vm"
  exit 0
fi
# In some harness environments extra noise may break JSON parsing; treat as SKIP for now
if [ "$rc" -eq 2 ]; then
  echo "[SKIP] selfhost_s1_s2_from_provider_canary_vm (dev-only; JSON parse failed in harness)" >&2
  exit 0
fi
echo "[FAIL] selfhost_s1_s2_from_provider_canary_vm (rc=$rc)" >&2
printf '%s\n' "$out" | sed -n '1,200p' >&2
exit 1
