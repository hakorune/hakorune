#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.

guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local route_id="$root_dir/src/mir/loop_recipe_contract/route_id.rs"
  local portable_recipe_dir="$root_dir/src/mir/loop_recipe_contract"
  local simple_terminality="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/direct_simple_while_terminality.rs"
  local accum_terminality="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/direct_accum_const_loop_terminality.rs"
  local if_phi_terminality="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/direct_if_phi_join_terminality.rs"
  local loop_break_terminality="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/direct_loop_break_terminality.rs"
  local live_ordered_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/live_ordered_terminality"
  local live_ordered_parent="$live_ordered_dir/mod.rs"
  local live_ordered_transaction="$live_ordered_dir/transaction.rs"
  local all_route_preflight="$live_ordered_dir/all_route_preflight.rs"
  local live_ordered_product="$live_ordered_dir/logical_product.rs"
  local loop_preflight="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/loop_preflight.rs"
  local handler_entry="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/handlers.rs"
  local route_handlers="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs"
  local projection="$root_dir/src/mir/builder/control_flow/facts/stmt_view.rs"
  local simple_while="$root_dir/src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs"
  local accum_const="$root_dir/src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs"
  local loop_break_facts="$root_dir/src/mir/builder/control_flow/plan/loop_break/facts/types.rs"
  local if_phi_join="$root_dir/src/mir/builder/control_flow/facts/if_phi_join_facts.rs"
  local files=(
    "$route_id"
    "$simple_terminality"
    "$accum_terminality"
    "$if_phi_terminality"
    "$loop_break_terminality"
    "$live_ordered_parent"
    "$live_ordered_transaction"
    "$all_route_preflight"
    "$live_ordered_product"
    "$loop_preflight"
    "$projection"
    "$simple_while"
    "$accum_const"
    "$loop_break_facts"
    "$if_phi_join"
  )
  local file lines

  guard_require_files "$tag" "${files[@]}"
  local portable_recipe_files=()
  mapfile -t portable_recipe_files < <(find "$portable_recipe_dir" -maxdepth 1 -name '*.rs' -type f | sort)
  guard_require_files "$tag" \
    "$portable_recipe_dir/README.md" \
    "$portable_recipe_dir/schema.rs" \
    "$portable_recipe_dir/verify.rs" \
    "$portable_recipe_dir/normalize.rs"
  if (( ${#portable_recipe_files[@]} == 0 )); then
    guard_fail "$tag" "portable Loop recipe subtree has no Rust contract files"
  fi
  for file in "${portable_recipe_files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  local portable_production_files=()
  for file in "${portable_recipe_files[@]}"; do
    [[ "$file" == "$portable_recipe_dir/tests.rs" ]] || portable_production_files+=("$file")
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|ValueId|BasicBlockId|MirInstruction|Phi|Frag|RouteAttemptOutcome|RouteFn|ComposeFn' \
    "${portable_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "portable Loop recipe acquired source, physical, or retry authority"
  fi
  if rg -n \
    'mutation_family|LoopMutationFamily|LoopRecipeFamily|legacy_family|opaque.*(emit|command)' \
    "${portable_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "portable Loop recipe acquired legacy family or opaque emission authority"
  fi
  if rg -n -w 'LoopRouteId|producer_route' \
    "$portable_recipe_dir/verify.rs" "$portable_recipe_dir/normalize.rs" >/dev/null; then
    guard_fail "$tag" "portable semantic verifier/normalizer acquired route provenance authority"
  fi
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  local logical_files=(
    "$live_ordered_product"
    "$loop_preflight"
    "$projection"
    "$simple_while"
    "$accum_const"
    "$loop_break_facts"
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
  local all_route_selection_calls all_route_production
  all_route_production="$(sed '/^#\[cfg(test)\]/,$d' "$all_route_preflight")"
  all_route_selection_calls="$(printf '%s\n' "$all_route_production" | rg -c 'select_recipe_first_routes\(Some\(&canonical\)\)' || true)"
  all_route_selection_calls="${all_route_selection_calls:-0}"
  if [[ "$all_route_selection_calls" != "0" ]]; then
    guard_fail "$tag" "all-route preflight must consume the router-selected raw schedule without reselecting"
  fi
  if printf '%s\n' "$all_route_production" | rg -n \
    'diagnostic_effective|matched_routes|ASTNode::|try_extract_|LoopSourceView|logical_product|qualify_live_loop_facts' \
    >/dev/null; then
    guard_fail "$tag" "all-route preflight acquired diagnostic, AST, or direct-product authority"
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
  local simple_route_body retry_count
  simple_route_body="$(sed -n '/pub(crate) fn route_loop_simple_while(/,/pub(crate) fn route_loop_char_map(/p' "$route_handlers")"
  retry_count="$(printf '%s\n' "$simple_route_body" | rg -c 'return Ok\(RouteAttemptOutcomeV1::Retry\)' || true)"
  if [[ "$retry_count" != "1" ]] || ! printf '%s\n' "$simple_route_body" | rg -q 'detect_nested_loop\(ctx\.body\)'; then
    guard_fail "$tag" "SimpleWhile terminality contract drifted from its single nested Retry pre-gate"
  fi
  local accum_route_body accum_retry_count
  accum_route_body="$(sed -n '/pub(crate) fn route_accum_const_loop(/,/pub(crate) fn route_nested_loop_minimal(/p' "$route_handlers")"
  accum_retry_count="$(printf '%s\n' "$accum_route_body" | rg -c 'return Ok\(RouteAttemptOutcomeV1::Retry\)' || true)"
  accum_retry_count="${accum_retry_count:-0}"
  if [[ "$accum_retry_count" != "0" ]]; then
    guard_fail "$tag" "AccumConstLoop terminality contract acquired a Retry path"
  fi
  local loop_break_route_body loop_break_retry_count
  loop_break_route_body="$(sed -n '/pub(crate) fn route_loop_break_recipe(/,/pub(crate) fn route_if_phi_join(/p' "$route_handlers")"
  loop_break_retry_count="$(printf '%s\n' "$loop_break_route_body" | rg -c 'return Ok\(RouteAttemptOutcomeV1::Retry\)' || true)"
  loop_break_retry_count="${loop_break_retry_count:-0}"
  if [[ "$loop_break_retry_count" != "0" ]]; then
    guard_fail "$tag" "LoopBreakRecipe terminality contract acquired a Retry path"
  fi
  local if_phi_route_body if_phi_retry_count
  if_phi_route_body="$(sed -n '/pub(crate) fn route_if_phi_join(/,/pub(crate) fn route_loop_continue_only(/p' "$route_handlers")"
  if_phi_retry_count="$(printf '%s\n' "$if_phi_route_body" | rg -c 'return Ok\(RouteAttemptOutcomeV1::Retry\)' || true)"
  if_phi_retry_count="${if_phi_retry_count:-0}"
  if [[ "$if_phi_retry_count" != "0" ]]; then
    guard_fail "$tag" "IfPhiJoin terminality contract acquired a Retry path"
  fi
  if rg -n '\b(env\.(planner_required|strict_or_dev|has_body_local)|outcome\.recipe_contract)' \
    "$route_handlers" "$handler_entry" >/dev/null; then
    guard_fail "$tag" "route decline authority bypassed the execution witness"
  fi
  if ! rg -q 'compose_facts: Option<&CanonicalLoopFacts>' "$route_handlers"; then
    guard_fail "$tag" "route compose facts must remain an explicit non-witness input"
  fi
  if rg -n '\.facts\(\)' "$route_handlers" "$handler_entry" >/dev/null; then
    guard_fail "$tag" "route decline authority acquired witness facts"
  fi
  local shared_decline_issuers
  shared_decline_issuers="$(rg -c 'issue_shared_absent_contract_decline\(' "$handler_entry" || true)"
  if [[ "$shared_decline_issuers" != "1" ]]; then
    guard_fail "$tag" "route_standard must remain the sole shared-decline issuer"
  fi
}
