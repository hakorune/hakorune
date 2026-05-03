#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="legacy-dev-utility-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ARCHIVED_ACTIVE_PATHS=(
  "tools/dev/enable_mirbuilder_dev_env.sh"
  "tools/dev/enable_phase216_env.sh"
  "tools/dev/rename_nyash_to_hako.sh"
)

for rel in "${ARCHIVED_ACTIVE_PATHS[@]}"; do
  if [[ -e "$ROOT_DIR/$rel" ]]; then
    guard_fail "$TAG" "archived legacy dev utility returned: $rel"
  fi
done

guard_require_files "$TAG" \
  "$ROOT_DIR/tools/archive/legacy-selfhost/engineering/enable_mirbuilder_dev_env.sh" \
  "$ROOT_DIR/tools/archive/legacy-selfhost/engineering/enable_phase216_env.sh" \
  "$ROOT_DIR/tools/archive/legacy-selfhost/engineering/rename_nyash_to_hako.sh"

echo "[$TAG] ok"
