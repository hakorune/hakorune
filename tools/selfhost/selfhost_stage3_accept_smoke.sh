#!/usr/bin/env bash
# Top-level proof facade.
# Canonical implementation lives under tools/selfhost/proof/.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/selfhost/proof/selfhost_stage3_accept_smoke.sh" "$@"
