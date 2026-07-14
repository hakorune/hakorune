#!/usr/bin/env bash

# Private SSA-RC0 helper. The public authority guard remains a bounded facade.

guard_ownership_transition_require_count() {
  local tag="$1"
  local file="$2"
  local literal="$3"
  local expected="$4"
  local actual
  actual="$(
    (rg -F -o -- "$literal" "$file" || true) | wc -l | tr -d '[:space:]'
  )"
  if [[ "$actual" != "$expected" ]]; then
    guard_fail "$tag" "D′ SSA-RC0 anchor drifted: file=$file literal=$literal expected=$expected actual=$actual"
  fi
}

guard_resolved_ownership_transition_planner_contract() {
  local tag="$1"
  local root="$2"
  local planner="$root/src/mir/builder/resolved_lowering/ownership"
  local lower="$root/src/mir/builder/resolved_lowering"
  local helper="${BASH_SOURCE[0]}"
  local files=(
    "$planner/README.md"
    "$planner/mod.rs"
    "$planner/value.rs"
    "$planner/assignment.rs"
    "$planner/scope_exit.rs"
    "$planner/error.rs"
    "$planner/tests.rs"
    "$helper"
  )

  guard_require_files "$tag" "${files[@]}"
  guard_ownership_transition_require_count "$tag" "$lower/mod.rs" "mod ownership;" 1

  local production=(
    "$planner/mod.rs"
    "$planner/value.rs"
    "$planner/assignment.rs"
    "$planner/scope_exit.rs"
    "$planner/error.rs"
  )
  local forbidden
  for forbidden in MirBuilder MirInstruction BasicBlockId BTreeMap HashMap \
    next_value_id emit_instruction; do
    if rg -n --fixed-strings "$forbidden" "${production[@]}" >/dev/null; then
      guard_fail "$tag" "D′ SSA-RC0 pure planner imported materialization authority: $forbidden"
    fi
  done

  local symbol
  for symbol in plan_assignment plan_declaration plan_scope_close \
    plan_function_exit plan_unpublished_draft_discard; do
    if find "$lower" -maxdepth 1 -type f -name '*.rs' -print0 | \
      xargs -0 rg -n --fixed-strings "$symbol" >/dev/null 2>&1; then
      guard_fail "$tag" "D′ SSA-RC0 planner gained a production caller: $symbol"
    fi
  done

  guard_ownership_transition_require_count "$tag" "$planner/value.rs" \
    "enum LoweredValueOwnershipV1" 1
  guard_ownership_transition_require_count "$tag" "$planner/value.rs" \
    "enum NextBindingValuePlanV1" 1
  guard_ownership_transition_require_count "$tag" "$planner/assignment.rs" \
    "enum AssignmentOwnershipPlanV1" 1
  guard_ownership_transition_require_count "$tag" "$planner/scope_exit.rs" \
    "struct ScopeCloseOwnershipPlanV1" 1
  guard_ownership_transition_require_count "$tag" "$planner/scope_exit.rs" \
    "struct FunctionExitOwnershipPlanV1" 1

  local file lines
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "D′ SSA-RC0 source/check reached the 800-line stop boundary: $file ($lines)"
    fi
  done
  echo "[$tag] SSA-RC0 ownership-transition planner: pure=1 production-callers=0"
}
