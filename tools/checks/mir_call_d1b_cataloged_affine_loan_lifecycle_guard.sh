#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-d1b-cataloged-affine-loan-lifecycle"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

# Historical phase arguments used to replay old lifecycle commands.  Their
# commit/card evidence now lives in the active Method manifest as tombstones.
# Keeping this stable registry entry argument-free prevents a landed phase from
# becoming a second current authority.
[[ $# -eq 0 ]] || fail "historical phase arguments are superseded; invoke without a phase"

exec python3 "$ROOT_DIR/tools/checks/lib/mir_call_d1b_active_surface_guard.py" "$ROOT_DIR"
