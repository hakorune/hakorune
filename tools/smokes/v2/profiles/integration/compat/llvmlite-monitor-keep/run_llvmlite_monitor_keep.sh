#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="compat/llvmlite-monitor-keep"

echo "[compat/llvmlite-monitor-keep] llvmlite monitor-only keep"

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[compat/llvmlite-monitor-keep] llvmlite monitor-only keep done."
