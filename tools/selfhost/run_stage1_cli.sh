#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/selfhost/compat/run_stage1_cli.sh" "$@"
