#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec python3 "$ROOT_DIR/tools/checks/lib/bare_static_recovery_proof.py" "$ROOT_DIR" "$@"
