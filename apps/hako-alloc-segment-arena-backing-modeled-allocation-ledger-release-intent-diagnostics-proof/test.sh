#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/checks/lib/proof_app_test_entry.sh" MIMAP-285A "$@"
