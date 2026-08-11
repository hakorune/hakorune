#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="loop-physical-transfer-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LAYOUT="$ROOT_DIR/src/mir/loop_recipe_contract/physical_layout.rs"
TRANSFER="$ROOT_DIR/src/mir/loop_recipe_contract/physical_transfer.rs"
VIEW="$ROOT_DIR/src/mir/loop_recipe_contract/join_sig/transfer_view_v1.rs"
ALLOCATOR="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/segment_allocator.rs"
AFTER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/recursive_after.rs"
LEDGER="$ROOT_DIR/src/mir/loop_recipe_contract/operation_physical_demand_ledger.rs"
V1_DEMAND="$ROOT_DIR/src/mir/loop_recipe_contract/operation_physical_demand.rs"
V1_DISPATCH="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_dispatcher.rs"
V1_SEGMENT_DISPATCH="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/segment_dispatcher.rs"
V2_DEMAND="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/model.rs"
PHYSICAL_INPUT="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/physical_input.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$LAYOUT" "$TRANSFER" "$VIEW" "$ALLOCATOR" "$AFTER" \
  "$LEDGER" "$V1_DEMAND" "$V1_DISPATCH" "$V1_SEGMENT_DISPATCH" "$V2_DEMAND"
guard_require_files "$TAG" "$PHYSICAL_INPUT"

guard_expect_fixed_in_file "$TAG" \
  "logical_transfer_view()" "$LAYOUT" \
  "physical layout must consume the JoinSig-owned transfer view"
guard_expect_fixed_in_file "$TAG" \
  "segment.role()" "$ALLOCATOR" \
  "segment allocation must consume the placement-owned segment role"
guard_expect_fixed_in_file "$TAG" \
  "bind_predicate" "$LAYOUT" \
  "predicate transfer binding must have one physical owner"
guard_expect_fixed_in_file "$TAG" \
  "bind_backedge" "$LAYOUT" \
  "backedge transfer binding must have one physical owner"
guard_expect_fixed_in_file "$TAG" \
  "bind_nested_loop" "$LAYOUT" \
  "nested-loop entry binding must have one physical owner"
guard_expect_fixed_in_file "$TAG" \
  "operation_physical_demand_ledger" "$V1_DEMAND" \
  "V1 prepared demand must retain one complete source/effect ledger"
guard_expect_fixed_in_file "$TAG" \
  "let ledger = program.ledger()" "$V1_DISPATCH" \
  "V1 physical dispatcher must borrow the complete ledger"
guard_expect_fixed_in_file "$TAG" \
  "let ledger = program.ledger()" "$V1_SEGMENT_DISPATCH" \
  "segment dispatcher must borrow the complete ledger"
guard_expect_fixed_in_file "$TAG" \
  "ReadyCallableLoopProfileCloseV1" "$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/tail_completion.rs" \
  "Callable profile-close must live in the callable adapter"
guard_expect_fixed_in_file "$TAG" \
  "verify_arm" "$PHYSICAL_INPUT" \
  "Dynamic physical input must validate both exact branch arms"
guard_expect_fixed_in_file "$TAG" \
  "expected_exit_kind" "$PHYSICAL_INPUT" \
  "Dynamic physical input must validate the logical exit role/target and exact exit item"
guard_expect_fixed_in_file "$TAG" \
  "mod tail_completion;" "$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/mod.rs" \
  "callable tail adapter module must remain wired"
guard_expect_fixed_in_file "$TAG" \
  "#[cfg(test)]" "$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/mod.rs" \
  "callable tail adapter must remain test-only"

for forbidden in \
  'ReadyCallableLoopProfileCloseV1' \
  'profile_counts' \
  '(7, 4, 2, 1)' \
  'condition_key' \
  'into_profile_close' \
  'ExactTrivialReturnAbiV1' \
  'VerifiedCallableTailV1'
do
  if rg -n -F -- "$forbidden" "$AFTER" >/dev/null 2>&1; then
    guard_fail "$TAG" "common recursive After retains Callable profile authority: $forbidden"
  fi
done

for file in "$LAYOUT" "$ALLOCATOR" "$AFTER"; do
  if rg -n -F "LoopConditionV1" "$file" >/dev/null 2>&1; then
    guard_fail "$TAG" "physical consumer still reads Recipe condition authority: ${file#"$ROOT_DIR/"}"
  fi
done

for forbidden in \
  'LoopPhysicalTransferV1::Predicate {' \
  'LoopPhysicalTransferV1::Jump {' \
  'LoopPhysicalTransferV1::OpenNestedLoop {'
do
  production_layout="$(mktemp "${TMPDIR:-/tmp}/loop-physical-layout.XXXXXX")"
  sed '/^#\[cfg(test)\]/,$d' "$LAYOUT" >"$production_layout"
  if rg -n -F -- "$forbidden" "$production_layout" >/dev/null 2>&1; then
    rm -f "$production_layout"
    guard_fail "$TAG" "physical layout still constructs transfer directly: $forbidden"
  fi
  rm -f "$production_layout"
done

for forbidden in \
  'as_recipe()' \
  'segment_role' \
  'LoopConditionV1'
do
  if rg -n -F -- "$forbidden" "$ALLOCATOR" >/dev/null 2>&1; then
    guard_fail "$TAG" "segment allocator still reconstructs placement role: $forbidden"
  fi
done

for file in "$V1_DISPATCH" "$V1_SEGMENT_DISPATCH"; do
  for forbidden in \
    'program.read_binding_rows()' \
    'program.derived_carrier_seed_rows()' \
    'program.write_binding_rows()' \
    'recipe.items.iter().find' \
    'effect_relations().iter().find' \
    'evidence().iter().find'
  do
    if rg -n -F -- "$forbidden" "$file" >/dev/null 2>&1; then
      guard_fail "$TAG" "V1 physical consumer re-scans source/effect projection: ${file#"$ROOT_DIR/"}: $forbidden"
    fi
  done
done

PHYSICALIZER_DIR="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer"
legacy_topology_callers="$(rg -l --glob '*.rs' 'physicalize_topology_v1\(' "$PHYSICALIZER_DIR" | grep -v '/topology.rs' || true)"
if [[ -n "$legacy_topology_callers" && "$legacy_topology_callers" != "$PHYSICALIZER_DIR/tests.rs" ]]; then
  guard_fail "$TAG" "legacy fixed-topology entry gained a non-test caller: $legacy_topology_callers"
fi
if rg -n --glob '*.rs' -F 'physicalize_topology_for_operation_demand_v1(' "$PHYSICALIZER_DIR" \
  | grep -v '/topology.rs' >/dev/null 2>&1; then
  guard_fail "$TAG" "operation-demand fixed-topology entry gained a caller"
fi
for pattern in \
  'prepare_loop_segment_operation_dispatch_v1(' \
  'allocate_for_layout('
do
  callers="$(rg -l --glob '*.rs' -F -- "$pattern" "$PHYSICALIZER_DIR" | grep -v '/segment_dispatcher.rs' | grep -v '/segment_allocator.rs' || true)"
  while IFS= read -r caller; do
    [[ -z "$caller" ]] && continue
    case "$caller" in
      "$PHYSICALIZER_DIR/callable_production_canary_tests.rs"|\
      "$PHYSICALIZER_DIR/generic_production_canary_tests.rs") ;;
      *) guard_fail "$TAG" "segment physical route gained an unexpected caller: $caller" ;;
    esac
  done <<< "$callers"
done
segment_target_callers="$(rg -l --glob '*.rs' -F 'VerifiedLoopOperationTargetBlockV1::issue_for_segment' "$PHYSICALIZER_DIR" | grep -v '/segment_dispatcher.rs' || true)"
if [[ -n "$segment_target_callers" ]]; then
  guard_fail "$TAG" "segment target issuer gained an unexpected caller: $segment_target_callers"
fi

if rg -n -F -- '.zip(' "$V2_DEMAND" >/dev/null 2>&1; then
  guard_fail "$TAG" "V2 physical demand pairs independent arrays by storage order"
fi

for file in "$LAYOUT" "$TRANSFER" "$VIEW" "$ALLOCATOR" "$AFTER" "$LEDGER" "$V1_DEMAND" \
  "$V1_DISPATCH" "$V1_SEGMENT_DISPATCH" "$V2_DEMAND" "$PHYSICAL_INPUT"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "800-line boundary exceeded: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok"
