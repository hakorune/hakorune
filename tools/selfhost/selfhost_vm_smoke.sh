#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/selfhost/proof/selfhost_vm_smoke.sh" "$@"
