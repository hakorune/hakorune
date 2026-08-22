#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="loop-precutover-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

BUILDER_DIR="$ROOT_DIR/src/mir/builder"
RECURSIVE="$BUILDER_DIR/recursive_child_lowering.rs"
RAW_LOOP="$BUILDER_DIR/raw_loop_child_entry.rs"
RAW_LOOP_PORT="$BUILDER_DIR/raw_loop_child_port.rs"
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
guard_require_files "$TAG" "$RECURSIVE" "$RAW_LOOP" "$RAW_LOOP_PORT" "$LOAN_PORT" "$ROOT_LOWERING" \
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

# The ordinary compatibility AST/JoinIR terminal remains explicit. The
# selected Dynamic branch must not call it; its package adapter handoff is the
# sole selected-Dynamic production owner.
old_callers=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    "$RAW_LOOP"|*_tests.rs) continue ;;
  esac
  old_callers+=("$file")
done < <(rg -l --glob '*.rs' -F 'lower_with_existing_route_v1(' "$BUILDER_DIR" || true)
if [[ "${#old_callers[@]}" -ne 1 || "${old_callers[0]:-}" != "$RAW_LOOP_PORT" ]]; then
  guard_fail "$TAG" "ordinary compatibility physical edge drifted; expected exactly raw_loop_child_port.rs"
fi

# W6-E opens one selected-Dynamic production handoff in the package adapter.
# Lower-level issuer counts remain owned by the dedicated AOT activation guard.
handoff_count="$(rg -F -o -- 'assemble_unpublished_selected_dynamic_w6_from_parts(' "$LOAN_PORT" | wc -l | tr -d '[:space:]')"
if [[ "$handoff_count" -ne 1 ]]; then
  guard_fail "$TAG" "selected Dynamic package-adapter handoff must have one production caller: found $handoff_count"
fi
guard_expect_fixed_in_file "$TAG" \
  'dynamic-instance-route' "$LOAN_PORT" \
  "cataloged instance/Dynamic mismatch must fail before the ordinary route"

for file in "$RECURSIVE" "$RAW_LOOP" "$RAW_LOOP_PORT" "$LOAN_PORT" "$ROOT_LOWERING" "$DEMAND_ISSUER" "$A_PRIME_ISSUER" "$ROUTING"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "pre-cutover authority file reached 800-line boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (ordinary compatibility edge=1, selected Dynamic raw edge=0, adapter handoff=1)"
