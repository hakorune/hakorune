#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="proof/hako-primary-no-fallback"

echo "[proof/hako-primary-no-fallback] bucket"

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[proof/hako-primary-no-fallback] bucket done."
