#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "[compat-wrapper] tools/selfhost/run_all.sh -> tools/selfhost/run_compat_pure_pack.sh" >&2
exec bash "$ROOT/tools/selfhost/run_compat_pure_pack.sh" "$@"
