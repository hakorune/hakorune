#!/bin/bash
set -euo pipefail

ROOT_DIR="${NYASH_ROOT:-$(cd "$(dirname "$0")/../../../../../.." && pwd)}"
exec bash "$ROOT_DIR/tools/smokes/v2/profiles/integration/vm_hako_caps/mapbox/mapbox_get_ported_vm.sh" "$@"
