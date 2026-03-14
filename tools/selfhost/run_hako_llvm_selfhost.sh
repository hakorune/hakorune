#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
echo "[compat-wrapper] tools/selfhost/run_hako_llvm_selfhost.sh -> tools/selfhost/run_compat_pure_selfhost.sh" >&2
exec bash "$ROOT/tools/selfhost/run_compat_pure_selfhost.sh" "$@"
