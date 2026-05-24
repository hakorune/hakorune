#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "[proof-app-test-entry] ERROR: expected proof app id" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
ID="$1"
shift

cd "$ROOT_DIR"
exec bash "$ROOT_DIR/tools/checks/run_proof_app.sh" --only "$ID" "$@"
