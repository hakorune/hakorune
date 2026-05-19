#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"
source "$ROOT_DIR/tools/checks/lib/allocator_release_candidate_sections.sh"

if [ "$#" -eq 0 ]; then
  VALIDATION_LEVEL="L2"
else
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
fi
case "$VALIDATION_LEVEL" in
  L0|L1|L2) ;;
  L3|L4)
    echo "[$TAG] ERROR: MIMAP-280A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-proof/test.sh"
CARD_276A="docs/development/current/main/phases/phase-293x/293x-801-MIMAP-276A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-INVENTORY.md"
CARD_278A="docs/development/current/main/phases/phase-293x/293x-803-MIMAP-278A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-805-MIMAP-280A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
PROOF_MANIFEST_ROWS="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
LEDGER_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_box.hako"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh"

printf '[%s] checking MIMAP-280A segment arena backing modeled allocation-ledger release candidate\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_276A" \
  "$CARD_278A" \
  "$CARD" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$PROOF_MANIFEST_ROWS" \
  "$MODULE" \
  "$MEMORY_README" \
  "$LEDGER_OWNER" \
  "$CANDIDATE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

arena_backing_release_candidate_check_docs
arena_backing_release_candidate_check_forbidden

arena_backing_release_candidate_check_vm
arena_backing_release_candidate_check_mir

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
