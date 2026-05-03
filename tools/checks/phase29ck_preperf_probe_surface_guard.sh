#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="phase29ck-preperf-probe-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ARCHIVED_ACTIVE_PATHS=(
  "tools/dev/phase29ck_backend_recipe_profile_probe.sh"
  "tools/dev/phase29ck_boundary_fallback_inventory_probe.sh"
)

for rel in "${ARCHIVED_ACTIVE_PATHS[@]}"; do
  if [[ -e "$ROOT_DIR/$rel" ]]; then
    guard_fail "$TAG" "archived phase29ck pre-perf diagnostics probe returned: $rel"
  fi
done

guard_require_files "$TAG" \
  "$ROOT_DIR/tools/dev/phase29ck_boundary_explicit_compat_probe.sh" \
  "$ROOT_DIR/tools/dev/phase29ck_boundary_historical_alias_probe.sh" \
  "$ROOT_DIR/tools/dev/phase29ck_stage1_mir_dialect_probe.sh"

echo "[$TAG] ok"
