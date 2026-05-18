#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
exec bash tools/checks/run_proof_app.sh --only MIMAP-200A
