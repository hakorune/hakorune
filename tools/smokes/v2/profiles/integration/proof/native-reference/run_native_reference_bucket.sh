#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="proof/native-reference"

echo "[proof/native-reference] native backend reference canaries"

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[proof/native-reference] native backend reference canaries done."
