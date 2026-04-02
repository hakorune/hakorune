#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

# Quick profile guard: this aggregator is heavier than a single test.
# In quick profile (or when per-test timeout is tight), skip to keep the suite fast/green.
if [[ "${SMOKES_CURRENT_PROFILE:-}" = "quick" ]]; then
  echo "[SKIP] phase2100/run_all: skipped under quick profile (aggregator is heavy)" >&2
  exit 0
fi
to=${SMOKES_DEFAULT_TIMEOUT:-0}
case "$to" in ''|*[!0-9]*) to=0;; esac
if [ "$to" -gt 0 ] && [ "$to" -lt 60 ]; then
  echo "[SKIP] phase2100/run_all: SMOKES_DEFAULT_TIMEOUT=$to is too small for aggregator" >&2
  exit 0
fi

echo "[phase2100] Dispatching role buckets..."
bash "$ROOT/tools/smokes/v2/profiles/integration/core/phase2100/run_engineering_selfhost.sh"
bash "$ROOT/tools/smokes/v2/profiles/integration/core/phase2100/run_probe_llvmlite.sh"
bash "$ROOT/tools/smokes/v2/profiles/integration/core/phase2100/run_product_crate_exe.sh"
bash "$ROOT/tools/smokes/v2/profiles/integration/core/phase2100/run_experimental_native.sh"
bash "$ROOT/tools/smokes/v2/profiles/integration/core/phase2100/run_always_on_shared.sh"

echo "[phase2100] Done."
