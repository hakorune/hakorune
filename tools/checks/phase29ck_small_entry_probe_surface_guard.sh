#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="phase29ck-small-entry-probe-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ARCHIVED_ACTIVE_PATHS=(
  "tools/dev/phase29ck_small_entry_startup_probe.sh"
  "tools/dev/phase29ck_small_entry_gc_sections_experiment.sh"
)

for rel in "${ARCHIVED_ACTIVE_PATHS[@]}"; do
  if [[ -e "$ROOT_DIR/$rel" ]]; then
    guard_fail "$TAG" "archived phase29ck small-entry diagnostics probe returned: $rel"
  fi
done

guard_require_files "$TAG" \
  "$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh"

echo "[$TAG] ok"
