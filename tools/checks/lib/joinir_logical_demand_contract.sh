#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.

guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local route_id="$root_dir/src/mir/loop_recipe_contract/route_id.rs"
  local portable_recipe_dir="$root_dir/src/mir/loop_recipe_contract"
  local loop_structural_facts_dir="$root_dir/src/mir/loop_structural_facts"
  local loop_route_policy_dir="$root_dir/src/mir/loop_route_policy"
  local route_registry_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry"
  local loop_phi_materializer="$root_dir/src/mir/builder/control_flow/plan/loop_phi_materializer.rs"
  local loop_phi_materializer_tests="$root_dir/src/mir/builder/control_flow/plan/loop_phi_materializer_tests.rs"
  local loop_accum_semantic_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_semantic_parity_tests.rs"
  local loop_accum_physical_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physical_parity_tests.rs"
  local loop_accum_physical_role_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physical_role_plan_tests.rs"
  local loop_physical_edge_path="$root_dir/src/mir/builder/control_flow/plan/loop_physical_edge_path.rs"
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
  guard_require_files "$tag" \
    "$loop_phi_materializer" "$loop_phi_materializer_tests" \
    "$loop_accum_semantic_tests" "$loop_accum_physical_tests" \
    "$loop_accum_physical_role_tests" \
    "$loop_physical_edge_path"
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_physical_tests"; then
    guard_fail "$tag" "physical parity observer must remain cfg(test)-only"
  fi
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_physical_role_tests"; then
    guard_fail "$tag" "physical role-plan observer must remain cfg(test)-only"
  fi
  local portable_recipe_files=()
  mapfile -t portable_recipe_files < <(find "$portable_recipe_dir" -maxdepth 1 -name '*.rs' -type f | sort)
  guard_require_files "$tag" \
    "$portable_recipe_dir/README.md" \
    "$portable_recipe_dir/schema.rs" \
    "$portable_recipe_dir/verify.rs" \
    "$portable_recipe_dir/join_sig.rs" \
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
  local join_sig_external_files=()
  mapfile -t join_sig_external_files < <(
    { rg -l -w \
        'LoopJoinPortV1|LoopJoinEdgeRoleV1|LoopJoinPayloadV1|LoopJoinEdgeV1|LoopJoinLoopV1|LoopJoinSigV1|VerifiedLoopJoinSigV1|LoopJoinSigElaboratorV1|LoopJoinSigRejectReasonV1' \
        "$root_dir/src/mir" || true; } \
      | awk \
          -v prefix="$portable_recipe_dir/" \
          -v materializer="$loop_phi_materializer" \
          -v materializer_tests="$loop_phi_materializer_tests" \
          -v semantic_tests="$loop_accum_semantic_tests" \
          -v physical_tests="$loop_accum_physical_tests" \
          -v physical_role_tests="$loop_accum_physical_role_tests" \
          -v edge_path="$loop_physical_edge_path" \
          'index($0, prefix) != 1 && $0 != materializer && $0 != materializer_tests && $0 != semantic_tests && $0 != physical_tests && $0 != physical_role_tests && $0 != edge_path'
  )
  if (( ${#join_sig_external_files[@]} != 0 )); then
    guard_fail "$tag" "caller-zero logical JoinSig symbols escaped the contract subtree"
  fi
  if rg -n \
    'ssa::phi_input_materializer|for_pred|define_phi_final|materialize_all_phi_inputs|BindingSsaBuilderV1|compute_predecessors|update_cfg|RouteAttemptOutcome|RouteFn|CorePlan|CanonicalLoopFacts|ASTNode' \
    "$loop_phi_materializer" >/dev/null; then
    guard_fail "$tag" "Loop PHI materializer bypassed the JoinSig/PhiTxn boundary"
  fi
  local loop_phi_external_callers=()
  mapfile -t loop_phi_external_callers < <(
    { rg -l 'materialize_loop_phis\(' "$root_dir/src/mir" || true; } \
      | awk -v materializer="$loop_phi_materializer" \
          -v materializer_tests="$loop_phi_materializer_tests" \
          '$0 != materializer && $0 != materializer_tests'
  )
  if (( ${#loop_phi_external_callers[@]} != 0 )); then
    guard_fail "$tag" "caller-zero Loop PHI materializer acquired a production caller"
  fi
  local loop_structural_fact_files=()
  mapfile -t loop_structural_fact_files < <(
    find "$loop_structural_facts_dir" -maxdepth 1 -name '*.rs' -type f | sort
  )
  guard_require_files "$tag" \
    "$loop_structural_facts_dir/README.md" \
    "$loop_structural_facts_dir/mod.rs" \
    "$loop_structural_facts_dir/resolved_source_adapter.rs"
  if (( ${#loop_structural_fact_files[@]} == 0 )); then
    guard_fail "$tag" "Loop structural facts subtree has no Rust contract files"
  fi
  local loop_structural_production_files=()
  for file in "${loop_structural_fact_files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
    [[ "$file" == "$loop_structural_facts_dir/tests.rs" ]] || \
      loop_structural_production_files+=("$file")
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|ValueId|BasicBlockId|MirInstruction|Phi|Frag|LoopRouteContext|CanonicalLoopFacts|RouteAttemptOutcome|RouteFn|ComposeFn|LoopRecipeArtifactV1|LoopRouteId|LoopRecipeProvenanceV1|producer_route' \
    "${loop_structural_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "Loop structural source authority acquired artifact, route, retry, AST, or physical authority"
  fi
  local structural_binding_callers
  structural_binding_callers="$(
    { rg -l 'bind_resolved_loop_root_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v prefix="$loop_structural_facts_dir/" 'index($0, prefix) != 1' \
      | wc -l \
      | tr -d '[:space:]'
  )"
  if [[ "$structural_binding_callers" != "0" ]]; then
    guard_fail "$tag" "caller-zero Loop source adapter acquired a production caller"
  fi
  local external_portable_source_files=()
  mapfile -t external_portable_source_files < <(
    { rg -l -w \
        'LoopRecipeArtifactV1|LoopRecipeSourceBindingV1|LoopNodeSourceBindingV1|LoopRecipeSourceOwnerV1|LoopSourcePathV1|LoopSourcePathStepV1|LoopRecipeProvenanceV1' \
        "$root_dir/src/mir" || true; } \
      | awk \
          -v recipe_prefix="$portable_recipe_dir/" \
          -v structural_prefix="$loop_structural_facts_dir/" \
          -v materializer="$loop_phi_materializer" \
          -v materializer_tests="$loop_phi_materializer_tests" \
          -v semantic_tests="$loop_accum_semantic_tests" \
          -v physical_tests="$loop_accum_physical_tests" \
          -v physical_role_tests="$loop_accum_physical_role_tests" \
          'index($0, recipe_prefix) != 1 && index($0, structural_prefix) != 1 && $0 != materializer && $0 != materializer_tests && $0 != semantic_tests && $0 != physical_tests && $0 != physical_role_tests'
  )
  if (( ${#external_portable_source_files[@]} != 0 )); then
    guard_fail "$tag" "semantic or physical Loop consumer acquired source/provenance authority"
  fi
  local loop_phi_materializer_production
  loop_phi_materializer_production="$(sed '/^#\[cfg(test)\]/,$d' "$loop_phi_materializer")"
  if printf '%s\n' "$loop_phi_materializer_production" | rg -n \
    'LoopRecipeArtifactV1|LoopRecipeVerifierV1|ASTNode|LoopSourcePath|LoopRecipeProvenanceV1|LoopRouteId' \
    >/dev/null; then
    guard_fail "$tag" "Loop PHI materializer production path acquired source/provenance authority"
  fi
  local external_resolved_source_files=()
  mapfile -t external_resolved_source_files < <(
    { rg -l -w 'VerifiedResolvedLoopSourceV1' "$root_dir/src/mir" || true; } \
      | awk \
          -v structural_prefix="$loop_structural_facts_dir/" \
          -v resolved_prefix="$root_dir/src/mir/resolved_semantics/" \
          'index($0, structural_prefix) != 1 && index($0, resolved_prefix) != 1'
  )
  if (( ${#external_resolved_source_files[@]} != 0 )); then
    guard_fail "$tag" "sealed resolved Loop source capability escaped its adapter boundary"
  fi
  local loop_route_policy_files=()
  mapfile -t loop_route_policy_files < <(
    find "$loop_route_policy_dir" -maxdepth 1 -name '*.rs' -type f | sort
  )
  guard_require_files "$tag" \
    "$loop_route_policy_dir/README.md" \
    "$loop_route_policy_dir/mod.rs" \
    "$loop_route_policy_dir/schema.rs" \
    "$loop_route_policy_dir/evaluate.rs" \
    "$loop_route_policy_dir/policy.rs" \
    "$loop_route_policy_dir/policy_evidence.rs" \
    "$loop_route_policy_dir/adapter.rs" \
    "$loop_route_policy_dir/tests.rs"
  if (( ${#loop_route_policy_files[@]} == 0 )); then
    guard_fail "$tag" "frozen Loop route policy subtree has no Rust files"
  fi
  local loop_route_policy_production_files=()
  for file in "${loop_route_policy_files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
    case "$file" in
      "$loop_route_policy_dir/adapter.rs"|"$loop_route_policy_dir/tests.rs") ;;
      *) loop_route_policy_production_files+=("$file") ;;
    esac
  done
  if ! rg -q -U '#\[cfg\(test\)\][[:space:]]*\nmod adapter;' \
    "$loop_route_policy_dir/mod.rs"; then
    guard_fail "$tag" "Loop route migration adapter must remain cfg(test)-only"
  fi
  if rg -n -w \
    'ASTNode|MirBuilder|CanonicalLoopFacts|CorePlan|ValueId|BasicBlockId|MirInstruction|Frag|RouteFn|RouteAttemptOutcomeV1|Retry|LoopRecipeV1|VerifiedLoopRecipeV1|LoopPhysicalizerV1' \
    "${loop_route_policy_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "frozen Loop route policy acquired AST, recipe, retry, or physical authority"
  fi
  if rg -n \
    'builder::control_flow::joinir::route_entry::registry|select_recipe_first_routes|RecipeFirstRouteSelectionV1|\bENTRIES\b|pred_[a-z0-9_]+' \
    "${loop_route_policy_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "frozen Loop route policy imported live registry, selection, or predicate authority"
  fi
  if rg -n \
    'match[[:space:]]+[^\n]*(route_id|LoopRouteId)|LoopRouteId::[A-Za-z0-9_]+[[:space:]]*=>' \
    "${loop_route_policy_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "opaque Loop route provenance acquired dispatch authority"
  fi
  local pure_policy_external_callers=()
  mapfile -t pure_policy_external_callers < <(
    { rg -l 'evaluate_frozen_loop_route_schedule_v1\(' "$root_dir/src" || true; } \
      | awk -v prefix="$loop_route_policy_dir/" 'index($0, prefix) != 1'
  )
  if (( ${#pure_policy_external_callers[@]} != 0 )); then
    guard_fail "$tag" "pure Loop policy evaluator acquired a production caller"
  fi
  if rg -n -U \
    '#\[derive\([^]]*Clone[^]]*\)\][[:space:]]*\npub\(crate\) struct (FrozenLoopRouteScheduleV1|FrozenLoopRouteRowV1)' \
    "$loop_route_policy_dir/schema.rs" >/dev/null; then
    guard_fail "$tag" "frozen Loop route schedule or row became Clone"
  fi
  local freeze_facade_definitions freeze_facade_external_callers
  freeze_facade_definitions="$(
    rg -c 'freeze_loop_route_schedule_v1\(' \
      "$loop_route_policy_dir/mod.rs" \
      "$loop_route_policy_dir/schema.rs" \
      "$loop_route_policy_dir/evaluate.rs" || true
  )"
  freeze_facade_definitions="$(printf '%s\n' "$freeze_facade_definitions" | awk -F: '{sum += $NF} END {print sum + 0}')"
  if [[ "$freeze_facade_definitions" != "1" ]]; then
    guard_fail "$tag" "frozen Loop route facade must have exactly one production definition"
  fi
  freeze_facade_external_callers="$(
    { rg -l 'freeze_loop_route_schedule_v1\(' "$root_dir/src" || true; } \
      | awk -v prefix="$loop_route_policy_dir/" 'index($0, prefix) != 1' \
      | wc -l \
      | tr -d '[:space:]'
  )"
  if [[ "$freeze_facade_external_callers" != "0" ]]; then
    guard_fail "$tag" "caller-zero frozen Loop route facade acquired a production caller"
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
  retry_count="${retry_count:-0}"
  if [[ "$retry_count" != "0" ]] || \
     ! printf '%s\n' "$simple_route_body" | rg -q 'PreEffectDeclineReasonV1::NestedLoopShapeUnavailable' || \
     ! printf '%s\n' "$simple_route_body" | rg -q 'detect_nested_loop\(ctx\.body\)'; then
    guard_fail "$tag" "SimpleWhile pre-effect nested gate lost its typed decline"
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
  local selected_loop_adapter_calls
  selected_loop_adapter_calls="$({
    rg -o 'from_selected_loop_option' \
      "$route_handlers" "$handler_entry" || true
  } | wc -l | tr -d '[:space:]')"
  if [[ "$selected_loop_adapter_calls" != "9" ]]; then
    guard_fail "$tag" "non-Generic selected Loop boundary must seal through one typed Option adapter: count=$selected_loop_adapter_calls expected=9"
  fi
  if rg -n 'from_retry_option|from_post_effect_option|RouteAttemptOutcomeV1::Retry' \
    "$route_registry_dir" -g '*.rs' >/dev/null; then
    guard_fail "$tag" "ordinary Retry/ambiguous Option projection remains in Loop registry"
  fi
  if rg -n 'compose_facts\.expect' "$route_handlers" "$handler_entry" >/dev/null; then
    guard_fail "$tag" "selected Loop route still panics instead of issuing typed facts blocker"
  fi
  local generic_debt_files=()
  mapfile -t generic_debt_files < <(
    {
      # Test-only D3 observers may capture the migration receipt, but the
      # production constructor/owner must remain in the Generic handlers.
      rg -l 'PostEffectRetryDebtV1::Generic\(' "$route_registry_dir" \
        --glob '*.rs' --glob '!**/*_tests.rs' || true
    }
  )
  if (( ${#generic_debt_files[@]} != 1 )) || [[ "${generic_debt_files[0]}" != *"/handlers/generic.rs" ]]; then
    guard_fail "$tag" "Generic post-effect debt receipt must remain isolated to generic handlers"
  fi
  if rg -n 'GenericLegacy|PostEffectRetryDebtV1::LowerOption' "$route_registry_dir" >/dev/null; then
    guard_fail "$tag" "ambiguous Generic post-effect debt remains after receipt classification"
  fi
  local generic_receipt_calls generic_receipt_composers
  generic_receipt_calls="$(rg -n -U 'generic_debt\([[:space:]]*COMPOSER' "$route_registry_dir/handlers/generic.rs" | rg -c 'generic_debt' || true)"
  generic_receipt_composers="$(rg -c 'const COMPOSER: LegacyGenericComposerV1' "$route_registry_dir/handlers/generic.rs" | awk '{sum += $1} END {print sum + 0}')"
  if [[ "$generic_receipt_calls" != "8" ]] || [[ "$generic_receipt_composers" != "2" ]]; then
    guard_fail "$tag" "Generic V0/V1 receipt branches must remain symmetric: calls=$generic_receipt_calls composers=$generic_receipt_composers"
  fi
  if rg -n 'LegacyComposerResultReceiptV1|LegacyGenericResultKindV1|PostEffectRetryDebtV1' \
    "$loop_route_policy_dir" >/dev/null; then
    guard_fail "$tag" "pure route policy acquired migration receipt or post-effect debt"
  fi
  local receipt_owner_files=()
  mapfile -t receipt_owner_files < <(
    { rg -l 'enum LegacyGenericComposerV1|enum LegacyGenericResultKindV1|struct LegacyComposerResultReceiptV1' "$route_registry_dir" || true; }
  )
  if (( ${#receipt_owner_files[@]} != 1 )) || [[ "${receipt_owner_files[0]}" != *"/legacy_receipt.rs" ]]; then
    guard_fail "$tag" "Generic migration receipt owner drifted outside legacy_receipt.rs"
  fi
  if ! rg -q 'selected Loop route produced a non-Loop CorePlan root' \
    "$root_dir/src/mir/builder/control_flow/joinir/route_entry/router.rs"; then
    guard_fail "$tag" "shared Loop route boundary lost its non-Loop CorePlan blocker"
  fi
  if ! rg -q 'selected LoopBreakRecipe produced a non-Loop CorePlan root' "$route_handlers" || \
     ! rg -q 'selected AccumConstLoop produced a non-Loop CorePlan root' "$route_handlers"; then
    guard_fail "$tag" "strict Loop route boundaries lost their CorePlan root blockers"
  fi
}
