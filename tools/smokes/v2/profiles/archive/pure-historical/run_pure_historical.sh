#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../.." && pwd)"
SUITE="pure-historical"

echo "[archive/pure-historical] archive-backed pure-lowering replay"

bash "$ROOT/tools/smokes/v2/run.sh" --profile archive --suite "$SUITE"

echo "[archive/pure-historical] done."
