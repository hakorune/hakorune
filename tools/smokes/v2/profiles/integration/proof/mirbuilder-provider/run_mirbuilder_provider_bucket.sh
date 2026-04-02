#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="proof/mirbuilder-provider"

echo "[proof/mirbuilder-provider] bucket"

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[proof/mirbuilder-provider] bucket done."
