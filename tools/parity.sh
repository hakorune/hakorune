#!/usr/bin/env bash
set -euo pipefail

# Compatibility shim. The engineering home is tools/engineering/parity.sh.

ROOT=$(cd "$(dirname "$0")/.." && pwd)
exec "$ROOT/tools/engineering/parity.sh" "$@"
