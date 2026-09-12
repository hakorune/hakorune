#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-r7-generic-g0-pending-draft-seal"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SESSION="$ROOT_DIR/src/mir/builder/calls/function_session.rs"
HELPER="$ROOT_DIR/src/mir/builder/resolved_lowering/mod.rs"
LOWERER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/generic_lowerer.rs"
CHILD="$ROOT_DIR/src/mir/builder/raw_root_physical/child_terminal.rs"
CALLABLE="$ROOT_DIR/src/mir/builder/raw_root_physical/callable_main_terminal.rs"
CALLER="$ROOT_DIR/src/mir/builder/normal_top_level_function_admission.rs"
TEST="$ROOT_DIR/src/mir/builder/calls/function_session/terminal.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-compatibility-retire-r7-generic-g0-pending-draft-seal-i0-2026-09-12.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_r7_generic_g0_pending_draft_seal_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" sed
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SESSION" "$HELPER" "$LOWERER" "$CHILD" "$CALLABLE" "$CALLER" "$TEST" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "GenericG0Admission(GenericG0PhysicalEmitterAdmissionRejectV1)" "$SESSION" \
  "Generic G0 admission must have a typed session transport variant"
guard_expect_fixed_in_file "$TAG" "Self::GenericG0Admission(_) => formatter.write_str(" "$SESSION" \
  "session display must keep Generic G0 admission as a stable transport tag"
guard_expect_fixed_in_file "$TAG" "map_err(CanonicalFunctionSessionErrorV1::GenericG0Admission)" "$HELPER" \
  "pending admission must not collapse the typed reject into Primary(String)"
guard_expect_fixed_in_file "$TAG" "CanonicalFunctionSessionErrorV1::DraftSeal(" "$LOWERER" \
  "Generic G0 DraftSeal rejection must use the typed session terminal"
guard_expect_fixed_in_file "$TAG" "rejected.into_discarded_error()" "$LOWERER" \
  "Generic G0 DraftSeal rejection must consume and restore the owner"
guard_expect_fixed_in_file "$TAG" "map_err(ModuleLoweringPortChildErrorV1::Session)" "$CALLER" \
  "top-level caller must preserve the typed session error"
guard_expect_fixed_in_file "$TAG" "GenericG0Admission(_)" "$CHILD" \
  "static-child abort mapping must classify typed Generic G0 admission"
guard_expect_fixed_in_file "$TAG" "GenericG0Admission(_)" "$CALLABLE" \
  "callable-main abort mapping must classify typed Generic G0 admission"
guard_expect_fixed_in_file "$TAG" "generic_g0_pending_session_admission_variant_is_typed" "$TEST" \
  "focused test must observe the typed session variant"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "the reusable Generic G0 pending guard must be indexed"
guard_expect_fixed_in_file "$TAG" "Generic G0 pending" "$CARD" \
  "the selected I0 card must name the bounded Generic G0 pending scope"

if sed -n '423,438p' "$HELPER" | rg -F 'format!("{error:?}")' >/dev/null 2>&1; then
  guard_fail "$TAG" "Generic G0 admission still crosses the pending boundary as a Debug string"
fi
if sed -n '67,92p' "$LOWERER" | rg -F 'generic-g0/draft-seal' >/dev/null 2>&1; then
  guard_fail "$TAG" "Generic G0 pending DraftSeal still formats a stage/detail string"
fi
if sed -n '67,92p' "$LOWERER" | rg -F 'Primary(detail)' >/dev/null 2>&1; then
  guard_fail "$TAG" "Generic G0 pending DraftSeal still uses Primary(String)"
fi

for file in "$SESSION" "$HELPER" "$LOWERER" "$CHILD" "$CALLABLE" "$CALLER" "$TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "selected source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (typed Generic G0 admission, owner-preserving DraftSeal, no pending Debug bridge)"
