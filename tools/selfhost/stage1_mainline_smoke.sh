#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/selfhost/mainline/stage1_mainline_smoke.sh" "$@"
