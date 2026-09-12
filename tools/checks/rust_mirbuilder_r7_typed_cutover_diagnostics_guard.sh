#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-r7-typed-cutover-diagnostics"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

INPUT="$ROOT_DIR/src/mir/compiler/lowering_input.rs"
DIRECT="$ROOT_DIR/src/mir/compiler/resolved_direct_accum_cutover.rs"
NESTED="$ROOT_DIR/src/mir/compiler/resolved_nested_predicate_cutover.rs"
DIRECT_TEST="$ROOT_DIR/src/mir/compiler/resolved_direct_accum_hardening_p0.rs"
NESTED_TEST="$ROOT_DIR/src/mir/compiler/nested_predicate_profile_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-compatibility-retire-r7-typed-diagnostics-i0-2026-09-12.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_r7_typed_cutover_diagnostics_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$INPUT" "$DIRECT" "$NESTED" "$DIRECT_TEST" "$NESTED_TEST" "$CARD"

guard_expect_fixed_in_file "$TAG" "CanonicalResolvedCutoverStageErrorV1" "$INPUT" \
  "resolved cutover stages must retain typed owner errors"
guard_expect_fixed_in_file "$TAG" "CanonicalResolvedCutoverFailureV1" "$INPUT" \
  "resolved cutovers must carry one explicit family envelope"
guard_expect_fixed_in_file "$TAG" "ResolvedCutover(CanonicalResolvedCutoverFailureV1)" "$INPUT" \
  "the existing canonical lowering boundary must expose the typed transport"
guard_expect_fixed_in_file "$TAG" "CanonicalResolvedCutoverFailureV1::DirectAccum" "$DIRECT" \
  "DirectAccum bridge must fix its family without string reconstruction"
guard_expect_fixed_in_file "$TAG" "CanonicalResolvedCutoverFailureV1::NestedPredicate" "$NESTED" \
  "Nested Predicate bridge must fix its family without string reconstruction"
guard_expect_fixed_in_file "$TAG" "ExternalCommitPreparationErrorV1::EvidenceMismatch" "$DIRECT_TEST" \
  "DirectAccum late-discard evidence must match a typed stage and payload"
guard_expect_fixed_in_file "$TAG" "ExternalCommitPreparationErrorV1::EvidenceMismatch" "$NESTED_TEST" \
  "Nested late-discard evidence must match a typed stage and payload"
guard_expect_fixed_in_file "$TAG" "pending draft-seal behavior is unchanged" "$CARD" \
  "the I0 card must keep pending draft-seal outside this slice"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$ROOT_DIR/docs/tools/check-scripts-index.md" \
  "the reusable typed-diagnostics guard must be indexed"

for file in "$DIRECT" "$NESTED"; do
  if rg -n -F -- 'format!("{error:?}")' "$file" >/dev/null 2>&1; then
    guard_fail "$TAG" "Debug-string bridge remains in ${file#"$ROOT_DIR/"}"
  fi
  if rg -n -F -- 'bridge_error("' "$file" >/dev/null 2>&1; then
    guard_fail "$TAG" "string stage bridge remains in ${file#"$ROOT_DIR/"}"
  fi
done

for file in "$INPUT" "$DIRECT" "$NESTED" "$DIRECT_TEST" "$NESTED_TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "typed cutover source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (typed family/stage transport, no Debug bridge, late-discard evidence)"
