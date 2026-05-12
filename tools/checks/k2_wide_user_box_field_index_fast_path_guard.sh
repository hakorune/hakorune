#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-user-box-field-index-fast-path"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-207-C201-USER-BOX-FIELD-INDEX-FAST-PATH.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
DECLS="src/runner/mir_json_emit/decls.rs"
DECL_TEST="src/runner/mir_json_emit/tests/decl_values.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_user_box_field_index_fast_path_guard.sh"

echo "[$TAG] checking C201 ordinary user-box field-index fast path"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$DECLS" \
  "$DECL_TEST" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C201 card must be complete"
guard_expect_in_file "$TAG" 'layout_id \+ field_index \+ storage' "$CARD" "C201 card must state the fast-path shape"
guard_expect_in_file "$TAG" 'C201 status:' "$PLAN" "mimalloc plan must record C201 status"
guard_expect_in_file "$TAG" '`C201` is complete as `293x-207`' "$RECORD_SSOT" "record SSOT must mark C201 complete"
guard_expect_in_file "$TAG" 'field_index_fast_path' "$DECLS" "MIR JSON decl emitter must expose fast-path metadata"
guard_expect_in_file "$TAG" 'layout_id' "$DECLS" "MIR JSON decl emitter must expose layout id"
guard_expect_in_file "$TAG" 'field_index' "$DECLS" "MIR JSON decl emitter must expose field index"
guard_expect_in_file "$TAG" 'storage' "$DECLS" "MIR JSON decl emitter must expose storage"
guard_expect_in_file "$TAG" 'field_index_fast_path' "$DECL_TEST" "unit test must assert fast-path metadata"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C201 guard"

if rg -n 'field_index_fast_path|layout_id|C201|record surface' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C201 metadata/proof matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q collect_sorted_user_box_decl_values_includes_typed_field_decls

echo "[$TAG] ok"
