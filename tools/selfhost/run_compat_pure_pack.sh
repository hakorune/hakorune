#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
# Backward-compat shim; canonical orchestrator lives in tools/compat/legacy-codegen/.
exec "$ROOT/tools/compat/legacy-codegen/run_compat_pure_pack.sh" "$@"
