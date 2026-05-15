#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-lifecycle-birth-new-only"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-400-LIFECYCLE-BIRTH-001-NEW-ONLY-BIRTH-POLICY.md")"
NEXT_CARD="$(guard_require_phase293x_card "$TAG" "293x-401-PARSER-BIRTH-001-DIRECT-BIRTH-NEGATIVE-FIXTURE.md")"
SSOT="docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md"
LIFECYCLE_REF="docs/reference/language/lifecycle.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_lifecycle_birth_new_only_guard.sh"

echo "[$TAG] running LIFECYCLE-BIRTH-001 new-only birth policy guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$NEXT_CARD" \
  "$SSOT" \
  "$LIFECYCLE_REF" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

for path in "$CARD" "$SSOT" "$LIFECYCLE_REF"; do
  guard_expect_in_file "$TAG" 'constructor hook' "$path" "$path must define birth as a constructor hook"
  guard_expect_in_file "$TAG" '[Dd]irect source' "$path" "$path must reject direct source birth calls"
  guard_expect_in_file "$TAG" 'new' "$path" "$path must point construction at new"
done

guard_expect_in_file "$TAG" 'LIFECYCLE-BIRTH-001' "$TASKBOARD" "taskboard must track the lifecycle row"
guard_expect_in_file "$TAG" 'PARSER-BIRTH-001' "$TASKBOARD" "taskboard must keep parser negative fixture follow-up visible"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list the lifecycle birth guard"
guard_expect_in_file "$TAG" 'legacy non-constructor host facade exception' "$CARD" "card must classify the arc.birth exception"
guard_expect_in_file "$TAG" 'PARSER-BIRTH-001' "$NEXT_CARD" "next parser fixture card must exist"

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
