#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="phase2044-llvmlite-monitor-keep"

echo "[phase2044] llvmlite monitor-only keep"

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[phase2044] llvmlite monitor-only keep done."
