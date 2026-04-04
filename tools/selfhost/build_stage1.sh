#!/usr/bin/env bash
# Top-level mainline facade.
# Canonical implementation lives under tools/selfhost/mainline/.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/selfhost/mainline/build_stage1.sh" "$@"
