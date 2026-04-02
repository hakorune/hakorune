#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
exec bash "$SCRIPT_DIR/plugins/plugin_v2_smoke.sh" "$@"
