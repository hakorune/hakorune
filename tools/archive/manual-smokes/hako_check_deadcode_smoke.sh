#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
exec "$ROOT/tools/hako_check/deadcode_smoke.sh" "$@"
