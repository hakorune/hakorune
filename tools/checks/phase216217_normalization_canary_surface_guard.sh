#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="phase216217-normalization-canary-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ARCHIVED_ACTIVE_PATHS=(
  "tools/dev/phase216_chain_canary.sh"
  "tools/dev/phase216_chain_canary_binop.sh"
  "tools/dev/phase216_chain_canary_binop_precedence_block.sh"
  "tools/dev/phase216_chain_canary_call.sh"
  "tools/dev/phase216_chain_canary_loop_undefined_block.sh"
  "tools/dev/phase216_chain_canary_return.sh"
  "tools/dev/phase216_direct_loop_progression_canary.sh"
  "tools/dev/phase217_method_norm_canary.sh"
  "tools/dev/phase217_methodize_canary.sh"
  "tools/dev/phase217_methodize_json_canary.sh"
  "tools/dev/phase217_methodize_json_strict.sh"
)

for rel in "${ARCHIVED_ACTIVE_PATHS[@]}"; do
  if [[ -e "$ROOT_DIR/$rel" ]]; then
    guard_fail "$TAG" "archived phase216/217 normalization canary returned: $rel"
  fi
done

guard_require_files "$TAG" \
  "$ROOT_DIR/tools/dev/phase2160_mirbuilder_module_load_probe.sh"

echo "[$TAG] ok"
