#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-r7-pending-draft-seal"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

OWNER="$ROOT_DIR/src/mir/builder/resolved_lowering/draft_seal_owner.rs"
SESSION="$ROOT_DIR/src/mir/builder/calls/function_session.rs"
HELPER="$ROOT_DIR/src/mir/builder/resolved_lowering/mod.rs"
CALLER="$ROOT_DIR/src/mir/builder/normal_cataloged_box_method_lowering.rs"
TEST="$ROOT_DIR/src/mir/builder/resolved_lowering/completion_draft_seal_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-compatibility-retire-r7-pending-draft-seal-i0-2026-09-12.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_r7_pending_draft_seal_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$OWNER" "$SESSION" "$HELPER" "$CALLER" "$TEST" "$CARD"

guard_expect_fixed_in_file "$TAG" "DiscardedFunctionDraftSealErrorV1" "$OWNER" \
  "DraftSeal owner must issue an owned typed rejection after restoration"
guard_expect_fixed_in_file "$TAG" "into_discarded_error" "$OWNER" \
  "rejected DraftSeal must consume the owner before transport"
guard_expect_fixed_in_file "$TAG" "DraftSeal(DiscardedFunctionDraftSealErrorV1)" "$SESSION" \
  "session transport must retain the typed DraftSeal payload"
guard_expect_fixed_in_file "$TAG" "rejected.into_discarded_error()" "$HELPER" \
  "pending helper must use the consuming typed boundary"
guard_expect_fixed_in_file "$TAG" "map_err(ModuleLoweringPortChildErrorV1::Session)" "$CALLER" \
  "cataloged callable caller must preserve the typed session error"
guard_expect_fixed_in_file "$TAG" "callable_pending_draft_seal_rejection_keeps_typed_error_and_restores_parent" "$TEST" \
  "pending rejection test must observe typed payload and restoration"
guard_expect_fixed_in_file "$TAG" "Generic G0 pending" "$CARD" \
  "the I0 boundary must keep Generic G0 pending separate"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$ROOT_DIR/docs/tools/check-scripts-index.md" \
  "the reusable pending DraftSeal guard must be indexed"

if sed -n '191,210p' "$HELPER" | rg -n -F -- 'format!("{:?}", rejected.error())' >/dev/null 2>&1; then
  guard_fail "$TAG" "Debug-string bridge remains in pending DraftSeal helper"
fi

for file in "$OWNER" "$SESSION" "$HELPER" "$CALLER" "$TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "pending DraftSeal source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (owned typed rejection, restoration-preserving pending handoff, no Debug bridge)"
