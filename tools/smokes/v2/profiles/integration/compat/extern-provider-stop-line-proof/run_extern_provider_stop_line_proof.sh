#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="compat/extern-provider-stop-line-proof"

echo "[compat/extern-provider-stop-line-proof] extern-provider root-first proof"

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[compat/extern-provider-stop-line-proof] extern-provider root-first proof done."
