#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/checks/run_row_guard.sh" --only hako-alloc-source-release-ledger-lifecycle-key-migration-closeout
