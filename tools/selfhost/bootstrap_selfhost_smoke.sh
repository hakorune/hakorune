#!/usr/bin/env bash
# Top-level proof facade.
# Canonical implementation lives under tools/selfhost/proof/.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/selfhost/proof/bootstrap_selfhost_smoke.sh" "$@"
