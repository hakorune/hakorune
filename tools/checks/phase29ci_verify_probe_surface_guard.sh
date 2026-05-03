#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="phase29ci-verify-probe-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

if [[ -e "$ROOT_DIR/tools/dev/phase29ci_verify_primary_core_route_probe.sh" ]]; then
  guard_fail "$TAG" "archived phase29ci W17 verify probe returned to active tools/dev"
fi

guard_require_files "$TAG" \
  "$ROOT_DIR/tools/archive/legacy-selfhost/engineering/phase29ci_verify_primary_core_route_probe.sh"

echo "[$TAG] ok"
