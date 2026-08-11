#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="loop-precutover-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

BUILDER_DIR="$ROOT_DIR/src/mir/builder"
RECURSIVE="$BUILDER_DIR/recursive_child_lowering.rs"
RAW_LOOP="$BUILDER_DIR/raw_loop_child_entry.rs"
LOAN_PORT="$BUILDER_DIR/normal_callable_semantic_loan_port.rs"
ROOT_LOWERING="$BUILDER_DIR/program_root_lowering.rs"
DEMAND_ISSUER="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/issuer.rs"
DEMAND_ROOT="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe"
A_PRIME_ISSUER="$ROOT_DIR/src/mir/compiler/a_prime_i64_physical_capability/issuer.rs"
ROUTING="$BUILDER_DIR/control_flow/joinir/routing.rs"
EMITTER_DIR="$BUILDER_DIR/resolved_lowering/selected_dynamic_physical_emitter"
EMITTER_ABI="$BUILDER_DIR/resolved_lowering/selected_dynamic_physical_abi.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$RECURSIVE" "$RAW_LOOP" "$LOAN_PORT" "$ROOT_LOWERING" \
  "$DEMAND_ISSUER" "$A_PRIME_ISSUER" "$ROUTING" "$EMITTER_DIR/mod.rs" \
  "$EMITTER_DIR/tests.rs"

guard_expect_fixed_in_file "$TAG" \
  "NormalCallableSemanticPackageMode::Installed" "$ROOT_LOWERING" \
  "installed package must remain the selected callable ingress"
guard_expect_fixed_in_file "$TAG" \
  "with_selected_lowering_input" "$LOAN_PORT" \
  "selected lowering must borrow the package-owned semantic input"
guard_expect_fixed_in_file "$TAG" \
  "input.semantic()" "$LOAN_PORT" \
  "the package semantic variant must be consumed at the lowering boundary"
guard_expect_fixed_in_file "$TAG" \
  "lower_loop_or_freeze_v1" "$RAW_LOOP" \
  "the pre-cutover legacy physical terminal must remain explicit"
guard_expect_fixed_in_file "$TAG" \
  "try_cf_loop_joinir" "$ROUTING" \
  "the current legacy physical route must remain identifiable"
guard_expect_fixed_in_file "$TAG" \
  "issue_dynamic_full_loop_operation_physical_demand_v2" "$DEMAND_ISSUER" \
  "the new Dynamic demand must have one named issuer"
guard_expect_fixed_in_file "$TAG" \
  "issue_selected_a_prime_i64_physical_demand" "$A_PRIME_ISSUER" \
  "the selected A-prime demand must have one named issuer"

# Before cutover the old AST/JoinIR terminal is intentionally retained. There
# must be exactly one non-test caller, and it is the located-child handoff. A
# second caller would create a competing production physical authority.
old_callers=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    "$RAW_LOOP"|*_tests.rs) continue ;;
  esac
  old_callers+=("$file")
done < <(rg -l --glob '*.rs' -F 'lower_with_existing_route_v1(' "$BUILDER_DIR" || true)
if [[ "${#old_callers[@]}" -ne 1 || "${old_callers[0]:-}" != "$RECURSIVE" ]]; then
  guard_fail "$TAG" "legacy selected physical edge drifted; expected exactly recursive_child_lowering.rs"
fi

# The new V2 demand is still Builder-free and test/canary-only until the named
# physical-session/cutover rows. Its production caller count must remain zero.
new_callers=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  [[ "$file" == "$DEMAND_ISSUER" || "$file" == */tests.rs || "$file" == *_tests.rs ]] && continue
  production_file="$(mktemp "${TMPDIR:-/tmp}/loop-precutover-demand.XXXXXX")"
  sed '/^#\[cfg(test)\]/,$d' "$file" >"$production_file"
  if rg -F -q -- 'issue_dynamic_full_loop_operation_physical_demand_v2(' "$production_file"; then
    new_callers+=("$file")
  fi
  rm -f "$production_file"
done < <(rg -l --glob '*.rs' -F 'issue_dynamic_full_loop_operation_physical_demand_v2(' "$DEMAND_ROOT" || true)
if [[ "${#new_callers[@]}" -ne 0 ]]; then
  guard_fail "$TAG" "new Dynamic physical demand gained a pre-cutover production caller: ${new_callers[*]}"
fi

# The selected A-prime demand is the next named physical-session input, not a
# second pre-cutover route. Keep it test/canary-only until VM/LLVM capability,
# site-keyed Completion, and the fresh session are all closed.
a_prime_callers=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    "$A_PRIME_ISSUER"|*/tests.rs|*_tests.rs) continue ;;
  esac
  a_prime_callers+=("$file")
done < <(rg -l --glob '*.rs' -F 'issue_selected_a_prime_i64_physical_demand(' "$ROOT_DIR/src/mir" || true)
if [[ "${#a_prime_callers[@]}" -ne 0 ]]; then
  guard_fail "$TAG" "selected A-prime demand gained a pre-cutover production caller: ${a_prime_callers[*]}"
fi

# The I8 emitter is a selected-fixture canary only.  Keep the plan-consuming
# session entry and its leaf out of production until the I7/End gate closes.
for pattern in \
  'DynamicV2PhysicalEmissionSessionV1::begin(' \
  'DynamicV2PhysicalEmissionSessionV1::emit_i8_const(' \
  'issue_selected_dynamic_v2_emission_plan('; do
  emitter_callers=()
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    case "$file" in
      */tests.rs|*_tests.rs|"$DEMAND_ISSUER"|"$A_PRIME_ISSUER"|"$EMITTER_ABI") continue ;;
    esac
    emitter_callers+=("$file")
  done < <(rg -l --glob '*.rs' -F "$pattern" "$ROOT_DIR/src/mir" || true)
  if [[ "${#emitter_callers[@]}" -ne 0 ]]; then
    guard_fail "$TAG" "pre-cutover emitter gained a production caller for ${pattern}: ${emitter_callers[*]}"
  fi
done

for file in "$RECURSIVE" "$RAW_LOOP" "$LOAN_PORT" "$ROOT_LOWERING" "$DEMAND_ISSUER" "$A_PRIME_ISSUER" "$ROUTING"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "pre-cutover authority file reached 800-line boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (legacy production edge=1, V2/A-prime/emitter production callers=0)"
