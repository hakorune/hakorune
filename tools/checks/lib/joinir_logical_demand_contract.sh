#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.

guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local demand_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/logical_demand"
  local route_id="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/route_id.rs"
  local projection="$root_dir/src/mir/builder/control_flow/facts/stmt_view.rs"
  local simple_while="$root_dir/src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs"
  local files=(
    "$route_id"
    "$demand_dir/mod.rs"
    "$demand_dir/source.rs"
    "$demand_dir/roles.rs"
    "$demand_dir/product.rs"
    "$demand_dir/producer.rs"
    "$projection"
    "$simple_while"
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
}
