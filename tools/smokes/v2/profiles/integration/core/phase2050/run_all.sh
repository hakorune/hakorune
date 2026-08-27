#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2050] Running exact owner pack..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --owner-profile integration --suite phase2050-owner-pack

echo "[phase2050] Done."
