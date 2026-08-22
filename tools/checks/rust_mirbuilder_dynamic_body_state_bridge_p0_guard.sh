#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-dynamic-body-state-bridge-p0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

BRIDGE="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/body_state_bridge.rs"
ASSEMBLY="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/assembly.rs"
EMITTER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs"
INNER_RETURN="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/inner_return_then.rs"
PROFILE_CLOSE="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/profile_close.rs"
IDENTITY="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/identity.rs"
OPERATION_CURSOR="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/operation_cursor.rs"
TESTS="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/tests.rs"
ADAPTER="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
STATE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state.rs"
OBSERVATION="$ROOT_DIR/src/mir/builder/normal_callable_semantic_observation.rs"
DYNAMIC_ORIGIN="$ROOT_DIR/src/mir/builder/normal_callable_dynamic_origin.rs"
DEMAND="$ROOT_DIR/src/mir/compiler/a_prime_i64_physical_capability/model.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-compare-connect0-d0-2026-08-22.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_dynamic_body_state_bridge_p0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_command "$TAG" awk
guard_require_files "$TAG" "$BRIDGE" "$ASSEMBLY" "$EMITTER" "$INNER_RETURN" \
  "$PROFILE_CLOSE" "$IDENTITY" "$OPERATION_CURSOR" "$TESTS" "$ADAPTER" "$STATE" "$OBSERVATION" \
  "$DYNAMIC_ORIGIN" "$DEMAND" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "mod body_state_bridge;" "$EMITTER" \
  "selected Dynamic emitter must retain one private body-state bridge"
guard_expect_fixed_in_file "$TAG" "assemble_unpublished_selected_dynamic_w6_from_parts" "$ADAPTER" \
  "selected Dynamic adapter must use the existing unpublished assembly seam"
guard_expect_fixed_in_file "$TAG" "Rc::clone(session.dynamic_source())" "$ADAPTER" \
  "the existing Dynamic source handle must cross the A-prime demand/session seam"
guard_expect_fixed_in_file "$TAG" "session.observe_body_state(&mut state, profile)?;" "$ADAPTER" \
  "the selected adapter must invoke the bridge exactly once"
guard_expect_fixed_in_file "$TAG" "state.finish()" "$ADAPTER" \
  "semantic state must finish after the bridge"
guard_expect_fixed_in_file "$TAG" "dynamic_source: Rc<" "$DEMAND" \
  "A-prime demand must carry the existing Dynamic source handle as a required field"
guard_expect_fixed_in_file "$TAG" ".publish(" "$INNER_RETURN" \
  "I11 must publish its existing physical read into the W6 ledger"
guard_expect_fixed_in_file "$TAG" "V14" "$INNER_RETURN" \
  "I11 must retain V14 without emitting a second instruction"
guard_expect_fixed_in_file "$TAG" "outer_return: CanonicalBindingReadReceiptV1" "$PROFILE_CLOSE" \
  "profile close must retain the existing After read receipt"
guard_expect_fixed_in_file "$TAG" "verify_single_predecessor_read_relation" "$PROFILE_CLOSE" \
  "canonical SSA must verify the sealed After-to-Header read relation"
guard_expect_fixed_in_file "$TAG" "verify_single_predecessor_phi" "$IDENTITY" \
  "canonical SSA must own the one-predecessor PHI relation check"
guard_expect_fixed_in_file "$TAG" "single_predecessor_phi_relation_accepts_distinct_target_value" "$IDENTITY" \
  "relation evidence must allow distinct target and predecessor ValueIds"
guard_expect_fixed_in_file "$TAG" "pub(super) fn check_closed" "$OPERATION_CURSOR" \
  "the close seam must check operation coverage before the bridge"
guard_expect_fixed_in_file "$TAG" "duplicate_body_bridge_rejects_and_discards_unpublished_effects" "$TESTS" \
  "focused evidence must exercise duplicate bridge rejection"
guard_expect_fixed_in_file "$TAG" "assert!(builder.function_state.current_function.is_none())" "$TESTS" \
  "duplicate bridge rejection must discard the unpublished function"
guard_expect_fixed_in_file "$TAG" "assert_eq!(headers.symbol_count(), 0)" "$TESTS" \
  "duplicate bridge rejection must not commit a module symbol"
guard_expect_fixed_in_file "$TAG" "finish_for_draft_seal" "$EMITTER" \
  "the bridge must precede canonical DraftSeal preparation"
guard_expect_fixed_in_file "$TAG" "SelectedDynamicBodyStateBridgeV1" "$CARD" \
  "the active card must retain the accepted bridge owner"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the body-state bridge guard"

for forbidden in \
  'RawInvocationChildPortV1' \
  'ASTNode' \
  'MirInstruction' \
  'emit_instruction' \
  'lower_' \
  'SourceBackedDynamicCallableIssuerV1'
do
  if rg -n -F -- "$forbidden" "$BRIDGE" >/dev/null 2>&1; then
    guard_fail "$TAG" "body-state bridge reaches a forbidden second authority/effect path: $forbidden"
  fi
done

if rg -n -F -- 'assemble_unpublished_selected_dynamic_w6(' "$ADAPTER" >/dev/null 2>&1; then
  guard_fail "$TAG" "selected Dynamic production adapter retains the pre-bridge assembly route"
fi

production_bridge_callers=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    *_tests.rs|*/tests.rs) continue ;;
  esac
  production_bridge_callers+=("$file")
done < <(rg -l --glob '*.rs' -F 'session.observe_body_state(&mut state, profile)?;' "$ROOT_DIR/src" || true)

if [[ "${#production_bridge_callers[@]}" -ne 1 || "${production_bridge_callers[0]:-}" != "$ADAPTER" ]]; then
  guard_fail "$TAG" "expected exactly one selected Dynamic body-state bridge caller; found ${production_bridge_callers[*]:-none}"
fi

line_for() {
  local file="$1"
  local pattern="$2"
  rg -n -m1 -F -- "$pattern" "$file" | awk -F: '{ print $1 }' || true
}

assert_order() {
  local file="$1"
  local before_pattern="$2"
  local after_pattern="$3"
  local before after
  before="$(line_for "$file" "$before_pattern")"
  after="$(line_for "$file" "$after_pattern")"
  if [[ -z "$before" || -z "$after" || "$before" -ge "$after" ]]; then
    guard_fail "$TAG" "required source order is missing in ${file#"$ROOT_DIR/"}: $before_pattern -> $after_pattern"
  fi
}

assert_order "$ADAPTER" \
  'session.observe_body_state(&mut state, profile)?;' \
  'state.finish()'
assert_order "$EMITTER" \
  'profile_close::emit' \
  'inspect(&mut self, &profile)'
assert_order "$EMITTER" \
  'inspect(&mut self, &profile)' \
  'finish_profile_close'
assert_order "$EMITTER" \
  'finish_profile_close' \
  '.finish_for_draft_seal('
assert_order "$PROFILE_CLOSE" \
  'seal_block(canonical, outer, after)?;' \
  'verify_single_predecessor_read_relation('

if rg -n -F -- 'outer_return.physical_value()' "$BRIDGE" >/dev/null 2>&1; then
  guard_fail "$TAG" "body bridge must not equate OuterReturn with Header-current or Backedge ValueIds"
fi

for file in "$BRIDGE" "$ASSEMBLY" "$EMITTER" "$INNER_RETURN" "$PROFILE_CLOSE" "$IDENTITY" \
  "$OPERATION_CURSOR" "$TESTS" "$ADAPTER" "$STATE" "$OBSERVATION" "$DYNAMIC_ORIGIN" "$DEMAND"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "body-state bridge source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
  if (( lines >= 760 )); then
    guard_fail "$TAG" "body-state bridge source reached the 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (one non-emitting bridge caller, retained W6 evidence, ordered DraftSeal seam, no second effect path)"
