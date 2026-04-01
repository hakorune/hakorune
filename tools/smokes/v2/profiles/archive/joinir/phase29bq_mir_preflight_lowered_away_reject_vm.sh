#!/bin/bash
# phase29bq_mir_preflight_lowered_away_reject_vm.sh
# historical compat wrapper.
# Canonical gate moved to:
#   phase29bq_mir_preflight_unsupported_reject_vm.sh

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
exec "$ROOT_DIR/smokes/v2/profiles/integration/joinir/phase29bq_mir_preflight_unsupported_reject_vm.sh"
