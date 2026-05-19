#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
exec bash tools/checks/run_row_guard.sh --only hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-closeout "$@"
