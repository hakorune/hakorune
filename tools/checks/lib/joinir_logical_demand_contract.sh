#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.

guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local route_id="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/route_id.rs"
  local simple_terminality="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/direct_simple_while_terminality.rs"
  local live_ordered_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/live_ordered_terminality"
  local live_ordered_parent="$live_ordered_dir/mod.rs"
  local live_ordered_transaction="$live_ordered_dir/transaction.rs"
  local live_ordered_product="$live_ordered_dir/logical_product.rs"
  local route_handlers="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs"
  local projection="$root_dir/src/mir/builder/control_flow/facts/stmt_view.rs"
  local simple_while="$root_dir/src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs"
  local accum_const="$root_dir/src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs"
  local files=(
    "$route_id"
    "$simple_terminality"
    "$live_ordered_parent"
    "$live_ordered_transaction"
    "$live_ordered_product"
    "$projection"
    "$simple_while"
    "$accum_const"
  )
  local file lines

  guard_require_files "$tag" "${files[@]}"
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  local logical_files=(
    "$live_ordered_product"
    "$projection"
    "$simple_while"
    "$accum_const"
  )
  for file in "${logical_files[@]}"; do
    if rg -n -w \
      'MirBuilder|CorePlan|ValueId|BasicBlockId|MirInstruction|Phi|Frag|RouteFn|ComposeFn|try_execute_recipe_first_selection|select_recipe_first_routes|ENTRIES' \
      "$file" >/dev/null; then
      guard_fail "$tag" "logical loop demand/provenance acquired physical or selection authority: ${file#"$root_dir/"}"
    fi
  done
  local selection_calls bridge_files
  selection_calls="$(rg -c 'select_recipe_first_routes\(Some\(&canonical\)\)' "$live_ordered_transaction" || true)"
  if [[ "$selection_calls" != "1" ]]; then
    guard_fail "$tag" "live ordered transaction must select the canonical raw schedule exactly once"
  fi
  local transaction_production
  transaction_production="$(sed '/^#\[cfg(test)\]/,$d' "$live_ordered_transaction")"
  if printf '%s\n' "$transaction_production" | rg -n \
    'diagnostic_effective|matched_routes|ASTNode::|try_extract_|LoopSourceView|logical_demand' \
    >/dev/null; then
    guard_fail "$tag" "live ordered transaction re-acquired diagnostic, AST, source-view, or legacy-demand authority"
  fi
  if sed '/^#\[cfg(test)\]/,$d' "$live_ordered_product" | rg -n \
    'ASTNode::|LoopFacts|select_recipe_first_routes|logical_demand|MirBuilder|CorePlan|ValueId|BasicBlockId|RouteFn|ComposeFn' \
    >/dev/null; then
    guard_fail "$tag" "logical product issuer acquired source, selection, or physical authority"
  fi
  bridge_files="$(rg -l '\bbind_live_loop_facts_v1\b' "$root_dir/src/mir/builder/control_flow" | wc -l | tr -d '[:space:]')"
  if [[ "$bridge_files" != "2" ]]; then
    guard_fail "$tag" "live facts binding bridge must remain registry-defined and facts-builder-called only"
  fi
  local simple_route_body none_count
  simple_route_body="$(sed -n '/pub(crate) fn route_loop_simple_while(/,/pub(crate) fn route_loop_char_map(/p' "$route_handlers")"
  none_count="$(printf '%s\n' "$simple_route_body" | rg -c 'return Ok\(None\)' || true)"
  if [[ "$none_count" != "1" ]] || ! printf '%s\n' "$simple_route_body" | rg -q 'detect_nested_loop\(ctx\.body\)'; then
    guard_fail "$tag" "SimpleWhile terminality contract drifted from its single nested None pre-gate"
  fi
}
