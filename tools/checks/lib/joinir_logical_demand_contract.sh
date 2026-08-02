#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.

guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local demand_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/logical_demand"
  local route_id="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/route_id.rs"
  local simple_terminality="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/direct_simple_while_terminality.rs"
  local route_handlers="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs"
  local projection="$root_dir/src/mir/builder/control_flow/facts/stmt_view.rs"
  local live_facts="$root_dir/src/mir/builder/control_flow/plan/facts/live_loop_facts.rs"
  local simple_while="$root_dir/src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs"
  local accum_const="$root_dir/src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs"
  local files=(
    "$route_id"
    "$simple_terminality"
    "$demand_dir/mod.rs"
    "$demand_dir/source.rs"
    "$demand_dir/roles.rs"
    "$demand_dir/product.rs"
    "$demand_dir/producer.rs"
    "$projection"
    "$live_facts"
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
  for file in "${files[@]:1}"; do
    if rg -n -w \
      'MirBuilder|CorePlan|ValueId|BasicBlockId|MirInstruction|Phi|Frag|RouteFn|ComposeFn|try_execute_recipe_first_selection|select_recipe_first_routes|ENTRIES' \
      "$file" >/dev/null; then
      guard_fail "$tag" "logical loop demand/provenance acquired physical or selection authority: ${file#"$root_dir/"}"
    fi
  done
  if sed '/^#\[cfg(test)\]/,$d' "$demand_dir/producer.rs" | rg -n \
    'ASTNode::|flatten_scope_boxes|try_extract_|select_recipe_first_routes|ENTRIES' \
    >/dev/null; then
    guard_fail "$tag" "logical producer re-acquired AST, projection, extractor, or selector authority"
  fi
  local simple_route_body none_count
  simple_route_body="$(sed -n '/pub(crate) fn route_loop_simple_while(/,/pub(crate) fn route_loop_char_map(/p' "$route_handlers")"
  none_count="$(printf '%s\n' "$simple_route_body" | rg -c 'return Ok\(None\)' || true)"
  if [[ "$none_count" != "1" ]] || ! printf '%s\n' "$simple_route_body" | rg -q 'detect_nested_loop\(ctx\.body\)'; then
    guard_fail "$tag" "SimpleWhile terminality contract drifted from its single nested None pre-gate"
  fi
}
