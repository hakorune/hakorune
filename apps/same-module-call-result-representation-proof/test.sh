#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec python3 "$ROOT_DIR/tools/checks/lib/same_module_call_result_representation_proof.py" "$ROOT_DIR" "$@"

