#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
# Backward-compat shim; canonical orchestrator now lives in tools/archive/legacy-selfhost/compat-codegen/.
exec "$ROOT/tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_pack.sh" "$@"
