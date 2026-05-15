#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-reuse-lifecycle-explicit-methods"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-403-REUSE-LIFECYCLE-001-EXPLICIT-REUSE-METHODS.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-404-MIMAP-022A-POST-LIFECYCLE-ROW-SELECTION.md"
SSOT="docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md"
LIFECYCLE_REF="docs/reference/language/lifecycle.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
RESULT_BOX="lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"
ATTACH_BOX="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako"
ARC_BOX="lang/src/hako_alloc/memory/arc_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_reuse_lifecycle_explicit_methods_guard.sh"

echo "[$TAG] running REUSE-LIFECYCLE-001 explicit reuse methods guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$NEXT_CARD" \
  "$SSOT" \
  "$LIFECYCLE_REF" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PAGE_BOX" \
  "$RESULT_BOX" \
  "$ATTACH_BOX" \
  "$ARC_BOX" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'REUSE-LIFECYCLE-001' "$CARD" "reuse lifecycle card must exist"
guard_expect_in_file "$TAG" 'Current allocator reuse inventory' "$SSOT" "lifecycle SSOT must include the allocator reuse inventory"
guard_expect_in_file "$TAG" 'reactivate\(\)' "$SSOT" "SSOT must list reactivate as explicit reuse"
guard_expect_in_file "$TAG" 'reuse\(\)' "$SSOT" "SSOT must list reuse as explicit reuse"
guard_expect_in_file "$TAG" 'reset\(\)' "$SSOT" "SSOT must list reset as explicit reuse"
guard_expect_in_file "$TAG" 'attachFreshPage' "$SSOT" "SSOT must list attachFreshPage as explicit attach surface"
guard_expect_in_file "$TAG" 'legacy non-constructor host facade exception' "$SSOT" "SSOT must classify the arc.birth exception"
guard_expect_in_file "$TAG" 'reset`, `reactivate`, `configure`, `clear`, or `attach`' "$LIFECYCLE_REF" "language lifecycle reference must keep explicit reuse wording"
guard_expect_in_file "$TAG" 'MIMAP-022A' "$NEXT_CARD" "next allocator row-selection card must exist"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

guard_expect_in_file "$TAG" 'reactivate\(\)' "$PAGE_BOX" "page model must keep explicit reactivate method"
guard_expect_in_file "$TAG" 'reuse\(\)' "$PAGE_BOX" "page model must keep explicit reuse method"
guard_expect_in_file "$TAG" 'reset\(\)' "$RESULT_BOX" "facade result capsules must keep explicit reset methods"
guard_expect_in_file "$TAG" 'attachFreshPage' "$ATTACH_BOX" "page-source adapter must keep explicit attach method"
guard_expect_in_file "$TAG" 'arc.birth\(ptr\)' "$ARC_BOX" "legacy arc host-facade exception must remain explicit"

matches="$(rg -n '\.birth[[:space:]]*\(' lang/src/hako_alloc -g '*.hako' || true)"
if [[ -n "$matches" ]]; then
  unexpected="$(printf '%s\n' "$matches" | grep -v 'lang/src/hako_alloc/memory/arc_box.hako:.*arc.birth(ptr)' || true)"
  if [[ -n "$unexpected" ]]; then
    echo "[$TAG] ERROR: hako_alloc gained direct receiver birth calls" >&2
    printf '%s\n' "$unexpected" >&2
    exit 1
  fi
  count="$(printf '%s\n' "$matches" | grep -c 'arc.birth(ptr)' || true)"
  if [[ "$count" != "1" ]]; then
    echo "[$TAG] ERROR: expected exactly one legacy arc.birth(ptr) host-facade exception, got $count" >&2
    printf '%s\n' "$matches" >&2
    exit 1
  fi
fi

echo "[$TAG] ok"
