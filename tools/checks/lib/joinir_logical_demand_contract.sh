#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.

guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local route_id="$root_dir/src/mir/loop_recipe_contract/route_id.rs"
  local portable_recipe_dir="$root_dir/src/mir/loop_recipe_contract"
  local loop_structural_facts_dir="$root_dir/src/mir/loop_structural_facts"
  local direct_accum_recipe_producer="$portable_recipe_dir/direct_accum_producer.rs"
  # The compiler-owned DirectAccum profile is the sole disconnected issuer of
  # the portable producer. It is not a downstream production consumer; the
  # guard must distinguish this issuer from an accidental route caller.
  local direct_accum_issuer="$root_dir/src/mir/compiler/direct_accum_profile.rs"
  local direct_accum_capability="$root_dir/src/mir/compiler/direct_accum_capability.rs"
  local direct_accum_projection="$root_dir/src/mir/compiler/direct_accum_projection.rs"
  local loop_true_source_projection="$root_dir/src/mir/compiler/loop_true_break_continue_projection.rs"
  local loop_route_policy_dir="$root_dir/src/mir/loop_route_policy"
  local route_registry_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry"
  local generic_resolved_test_prefix="$route_registry_dir/generic_resolved_carrier_"
  local loop_phi_materializer="$root_dir/src/mir/builder/control_flow/plan/loop_phi_materializer.rs"
  local loop_phi_materializer_tests="$root_dir/src/mir/builder/control_flow/plan/loop_phi_materializer_tests.rs"
  local loop_accum_semantic_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_semantic_parity_tests.rs"
  local loop_accum_physical_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physical_parity_tests.rs"
  local loop_accum_physical_role_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physical_role_plan_tests.rs"
  local loop_accum_binding_ssa_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_binding_ssa_session_tests.rs"
  local loop_accum_emitter_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_binding_ssa_emitter_tests.rs"
  local loop_accum_candidate_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_binding_ssa_candidate_tests.rs"
  local loop_accum_digest_support="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physical_digest_test_support.rs"
  local loop_accum_semantic_digest_support="$root_dir/src/mir/builder/control_flow/plan/loop_accum_semantic_digest_test_support.rs"
  local loop_recipe_producer_tests="$root_dir/src/mir/builder/control_flow/plan/loop_recipe_producer_facade_tests.rs"
  local nested_predicate_producer="$root_dir/src/mir/compiler/nested_predicate_producer.rs"
  local nested_predicate_producer_tests="$root_dir/src/mir/compiler/nested_predicate_producer_tests.rs"
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
    "$loop_accum_binding_ssa_tests" "$loop_accum_emitter_tests" \
    "$loop_accum_candidate_tests" \
    "$loop_accum_digest_support" "$loop_accum_semantic_digest_support" \
    "$loop_physical_edge_path" "$direct_accum_issuer" "$direct_accum_capability" \
    "$direct_accum_projection" "$loop_true_source_projection"
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_physical_tests"; then
    guard_fail "$tag" "physical parity observer must remain cfg(test)-only"
  fi
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_physical_role_tests"; then
    guard_fail "$tag" "physical role-plan observer must remain cfg(test)-only"
  fi
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_binding_ssa_tests"; then
    guard_fail "$tag" "Binding-SSA session proof must remain cfg(test)-only"
  fi
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_emitter_tests"; then
    guard_fail "$tag" "Binding-SSA emitter proof must remain cfg(test)-only"
  fi
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_candidate_tests"; then
    guard_fail "$tag" "candidate observer proof must remain cfg(test)-only"
  fi
  for digest_support in "$loop_accum_digest_support" "$loop_accum_semantic_digest_support"; do
    if ! rg -q '^#!\[cfg\(test\)\]' "$digest_support"; then
      guard_fail "$tag" "physical parity digest support must remain cfg(test)-only: ${digest_support#"$root_dir/"}"
    fi
    if rg -n \
      '^(use|pub[[:space:]].*use)[[:space:]].*(MirBuilder|CorePlan|PlanLowerer|PhiTxn|BindingSsaBuilder|RouteAttemptOutcome|RouteFn|LoopPhiMaterializer|ASTNode|variable_map)' \
      "$digest_support" >/dev/null; then
      guard_fail "$tag" "immutable physical parity digest support acquired production or mutation authority: ${digest_support#"$root_dir/"}"
    fi
  done
  for binding_ssa_file in "$loop_accum_binding_ssa_tests" "$loop_accum_emitter_tests"; do
    if rg -n \
      'LoopPhiMaterializer|materialize_loop_phis|insert_phi_at_head|update_phi_instruction|CorePlan|PlanLowerer|RouteAttemptOutcome|Retry' \
      "$binding_ssa_file" >/dev/null; then
      guard_fail "$tag" "Binding-SSA proof bypassed canonical owner boundary: ${binding_ssa_file#"$root_dir/"}"
    fi
    if rg -n 'Option<' "$binding_ssa_file" | \
      rg -v 'Option<(PhiTxn|CanonicalCfgSessionV1|BindingSsaBuilderV1)' >/dev/null; then
      guard_fail "$tag" "Binding-SSA proof acquired retry/decline Option outside PHI transaction state: ${binding_ssa_file#"$root_dir/"}"
    fi
  done
  if rg -n \
    'prepare_external_commit|commit_raw_direct|route_loop|LoopPhiMaterializer|materialize_loop_phis' \
    "$loop_accum_candidate_tests" >/dev/null; then
    guard_fail "$tag" "candidate observer acquired publication or legacy production authority"
  fi
  local portable_recipe_files=()
  mapfile -t portable_recipe_files < <(find "$portable_recipe_dir" -type f -name '*.rs' | sort)
  guard_require_files "$tag" \
    "$portable_recipe_dir/README.md" \
    "$portable_recipe_dir/schema.rs" \
    "$portable_recipe_dir/verify.rs" \
    "$portable_recipe_dir/join_sig/mod.rs" \
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
    if [[ "$file" == "$portable_recipe_dir/tests.rs" || "$file" == *_tests.rs ]]; then
      continue
    fi
    portable_production_files+=("$file")
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
          -v binding_ssa_tests="$loop_accum_binding_ssa_tests" \
          -v producer_tests="$loop_recipe_producer_tests" \
          -v nested_producer="$nested_predicate_producer" \
          -v nested_producer_tests="$nested_predicate_producer_tests" \
          -v nested_topology="$root_dir/src/mir/compiler/nested_predicate_topology.rs" \
          -v nested_topology_tests="$root_dir/src/mir/compiler/nested_predicate_topology_tests.rs" \
          -v nested_physical_input="$root_dir/src/mir/compiler/nested_predicate_physical_input.rs" \
          -v nested_physical_input_tests="$root_dir/src/mir/compiler/nested_predicate_physical_input_tests.rs" \
          -v physicalizer="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physicalizer.rs" \
          -v edge_path="$loop_physical_edge_path" \
          'index($0, prefix) != 1 && $0 != materializer && $0 != materializer_tests && $0 != semantic_tests && $0 != physical_tests && $0 != physical_role_tests && $0 != binding_ssa_tests && $0 != producer_tests && $0 != nested_producer && $0 != nested_producer_tests && $0 != nested_topology && $0 != nested_topology_tests && $0 != nested_physical_input && $0 != nested_physical_input_tests && $0 != physicalizer && $0 != edge_path'
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
    if [[ "$file" == "$loop_structural_facts_dir/tests.rs" || "$file" == *_tests.rs ]]; then
      continue
    fi
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
      | awk -v prefix="$loop_structural_facts_dir/" -v producer="$direct_accum_recipe_producer" \
          -v projection="$loop_true_source_projection" \
          'index($0, prefix) != 1 && $0 != producer && $0 != projection' \
      | wc -l \
      | tr -d '[:space:]'
  )"
  if [[ "$structural_binding_callers" != "0" ]]; then
    guard_fail "$tag" "Loop source adapter acquired an unapproved production caller"
  fi
  local direct_accum_production_callers=()
  mapfile -t direct_accum_production_callers < <(
    { rg -l 'produce_direct_accum_recipe_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v producer="$direct_accum_recipe_producer" -v issuer="$direct_accum_issuer" \
          '$0 != producer && $0 != issuer && $0 !~ /_tests\.rs$/'
  )
  if (( ${#direct_accum_production_callers[@]} != 0 )); then
    guard_fail "$tag" "Direct Accum Recipe producer acquired a production caller"
  fi
  local direct_accum_issuer_calls
  direct_accum_issuer_calls="$(
    { rg -o 'produce_direct_accum_recipe_v1\(' "$direct_accum_issuer" || true; } \
      | wc -l \
      | tr -d '[:space:]'
  )"
  if [[ "$direct_accum_issuer_calls" != "2" ]]; then
    guard_fail "$tag" "Direct Accum issuer call count drift: count=$direct_accum_issuer_calls expected=2"
  fi
  local direct_accum_source_probe_callers=()
  mapfile -t direct_accum_source_probe_callers < <(
    { rg -l 'probe_direct_accum_source_unit_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v capability="$direct_accum_capability" '$0 != capability && $0 !~ /_tests\.rs$/'
  )
  if (( ${#direct_accum_source_probe_callers[@]} != 1 )) || \
     [[ "${direct_accum_source_probe_callers[0]}" != "$root_dir/src/mir/compiler/capability.rs" ]]; then
    guard_fail "$tag" "Direct Accum source-unit probe must have exactly one preflight caller"
  fi
  local direct_accum_physicalizer_production_callers=()
  mapfile -t direct_accum_physicalizer_production_callers < <(
    { rg -l 'physicalize_direct_accum_v1(_with_port)?\(' "$root_dir/src/mir" || true; } \
      | awk -v physicalizer="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physicalizer.rs" \
          '$0 != physicalizer && $0 !~ /_tests\.rs$/'
  )
  if (( ${#direct_accum_physicalizer_production_callers[@]} != 1 )) || \
     [[ "${direct_accum_physicalizer_production_callers[0]}" != "$root_dir/src/mir/builder/resolved_lowering/direct_accum_lowerer.rs" ]]; then
    guard_fail "$tag" "Direct Accum physicalizer caller drifted: expected resolved direct lowerer only"
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
          -v binding_ssa_tests="$loop_accum_binding_ssa_tests" \
          -v producer_tests="$loop_recipe_producer_tests" \
          -v nested_producer="$root_dir/src/mir/compiler/nested_predicate_producer.rs" \
          -v nested_producer_tests="$root_dir/src/mir/compiler/nested_predicate_producer_tests.rs" \
          -v generic_test_prefix="$generic_resolved_test_prefix" \
          'index($0, recipe_prefix) != 1 && index($0, structural_prefix) != 1 && !(index($0, generic_test_prefix) == 1 && $0 ~ /_tests\.rs$/) && $0 != materializer && $0 != materializer_tests && $0 != semantic_tests && $0 != physical_tests && $0 != physical_role_tests && $0 != binding_ssa_tests && $0 != producer_tests && $0 != nested_producer && $0 != nested_producer_tests'
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
          -v projection="$direct_accum_projection" \
          -v loop_true_projection="$loop_true_source_projection" \
          'index($0, structural_prefix) != 1 && index($0, resolved_prefix) != 1 && $0 != projection && $0 != loop_true_projection'
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
    "$loop_route_policy_dir/loop_true_break_continue.rs" \
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
      "$loop_route_policy_dir/adapter.rs"|"$loop_route_policy_dir/tests.rs"|"$loop_route_policy_dir"/*_tests.rs) ;;
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
      | awk -v prefix="$loop_route_policy_dir/" \
          'index($0, prefix) != 1 && $0 !~ /\/tests\.rs$/ && $0 !~ /_tests\.rs$/' \
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

# The portable If contract shares this lane guard with the Loop contract.  The
# D0-C seam permits exactly one named adapter outside the contract subtree;
# every other logical/physical-input constructor remains caller-zero.
guard_joinir_if_recipe_contract() {
  local root_dir="$1"
  local tag="$2"
  local contract_dir="$root_dir/src/mir/if_recipe_contract"
  local join_sig="$contract_dir/join_sig.rs"
  local physical_input="$contract_dir/physical_input.rs"
  local tests="$contract_dir/tests.rs"
  local adapter="$root_dir/src/mir/builder/resolved_lowering/if_recipe_adapter.rs"
  local physicalizer="$root_dir/src/mir/builder/resolved_lowering/trivial_ssa/if_recipe_physicalizer.rs"
  local lowerer="$root_dir/src/mir/builder/resolved_lowering/trivial_ssa/lowerer.rs"
  local materialization="$root_dir/src/mir/builder/resolved_lowering/trivial_ssa/lowerer/if_materialization.rs"
  local files=(
    "$contract_dir/README.md"
    "$contract_dir/mod.rs"
    "$contract_dir/error.rs"
    "$contract_dir/ids.rs"
    "$join_sig"
    "$contract_dir/normalize.rs"
    "$physical_input"
    "$contract_dir/schema.rs"
    "$contract_dir/source_binding.rs"
    "$contract_dir/verify.rs"
    "$tests"
    "$adapter"
    "$physicalizer"
    "$lowerer"
    "$materialization"
  )
  local file lines

  guard_require_files "$tag" "${files[@]}"
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "If recipe contract file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|ValueId|BasicBlockId|IfCfgSessionV1|RouteAttemptOutcomeV1|Retry|CanonicalSsaFunctionSessionV2|PhiTxn|BindingSsaBuilderV1' \
    "$adapter" >/dev/null; then
    guard_fail "$tag" "D0-C If adapter acquired AST, Builder, route, or physical SSA authority"
  fi

  # `schema.rs` legitimately uses Option for the explicit/implicit else
  # disposition.  The physical/logical proof files must not acquire it.
  if rg -n -w \
    'MirBuilder|CorePlan|ValueId|BasicBlockId|CanonicalCfgSession|PhiTxn|BindingSsaBuilderV1|ASTNode|RouteAttemptOutcomeV1|Retry|Option' \
    "$join_sig" "$physical_input" >/dev/null; then
    guard_fail "$tag" "If logical/physical-input proof acquired physical, AST, route, retry, or Option authority"
  fi
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|RouteAttemptOutcomeV1|Retry|Option|new_ssa|new_phi|lower_if_legacy_unselected|lower_if_materialization_core|ResolvedIfElsePortV1|else_port|Some\(true\)' \
    "$physicalizer" >/dev/null; then
    guard_fail "$tag" "If physicalizer acquired route, retry, raw AST, or a second physical owner"
  fi
  if ! rg -q 'lower_if_recipe_selected\(' "$physicalizer" || \
     rg -n 'lower_if_legacy_unselected|lower_if_materialization_core|lower_if_materialization\(' \
       "$physicalizer" >/dev/null; then
    guard_fail "$tag" "selected If physicalizer must use only the named selected helper"
  fi
  if ! rg -q 'fn lower_if_recipe_selected\(' "$materialization" || \
     ! rg -q 'fn lower_if_legacy_unselected\(' "$materialization" || \
     ! rg -q 'IfMaterializationTopologyV1::Selected' "$materialization"; then
    guard_fail "$tag" "If lowerer lost the selected/legacy shape-scoped helper split"
  fi
  if rg -n -U \
    '#\[derive\([^)]*Clone[^)]*\)\][[:space:]]*\n[[:space:]]*(pub\(crate\)[[:space:]]+)?struct[[:space:]]+(VerifiedIfJoinSigV1|VerifiedIfPhysicalInputV1)' \
    "$join_sig" "$physical_input" >/dev/null || \
     rg -n 'impl[[:space:]]+Clone[[:space:]]+for[[:space:]]+(VerifiedIfJoinSigV1|VerifiedIfPhysicalInputV1)' \
       "$join_sig" "$physical_input" >/dev/null; then
    guard_fail "$tag" "verified If logical/physical-input wrappers must remain non-Clone"
  fi
  if rg -n 'pub[[:space:]]+(artifact|join_sig)[[:space:]]*:' "$physical_input" >/dev/null; then
    guard_fail "$tag" "physical-input artifact/signature fields must remain private"
  fi
  if [[ "$(rg -c 'pub\(crate\) fn from_artifact\(' "$physical_input" || true)" != "1" ]]; then
    guard_fail "$tag" "physical-input must have exactly one consuming from_artifact issuer"
  fi
  if rg -n 'pub\(crate\) fn (new|from_parts|from_signature|from_join)' "$physical_input" >/dev/null; then
    guard_fail "$tag" "independent artifact/signature constructor appeared"
  fi

  local symbol_files=()
  mapfile -t symbol_files < <(
    { rg -l -w 'VerifiedIfJoinSigV1|VerifiedIfPhysicalInputV1|IfJoinSigElaboratorV1' \
        "$root_dir/src/mir" || true; }
  )
  for file in "${symbol_files[@]}"; do
    if [[ "$file" != "$contract_dir"/* && "$file" != "$adapter" && "$file" != "$physicalizer" ]]; then
      guard_fail "$tag" "If logical/physical-input symbol escaped caller-zero subtree: ${file#"$root_dir/"}"
    fi
  done

  local physical_callers=()
  mapfile -t physical_callers < <(
    { rg -l 'VerifiedIfPhysicalInputV1::from_artifact\(' "$root_dir/src/mir" || true; }
  )
  if (( ${#physical_callers[@]} != 2 )) || \
     ! printf '%s\n' "${physical_callers[@]}" | rg -q "^${tests}$" || \
     ! printf '%s\n' "${physical_callers[@]}" | rg -q "^${adapter}$"; then
    guard_fail "$tag" "physical-input issuer must have only contract tests plus the named D0-C adapter"
  fi
  local mapper_callers=()
  mapfile -t mapper_callers < <(
    { rg -l 'map_trivial_if_recipe_v1\(' "$root_dir/src/mir" || true; } |
      while IFS= read -r file; do
        [[ "$file" == "$root_dir/src/mir/resolved_value_profile/recipe_mapper.rs" ]] && continue
        [[ "$file" == *"/tests.rs" || "$file" == *"_tests.rs" ]] && continue
        printf '%s\n' "$file"
      done
  )
  if (( ${#mapper_callers[@]} != 1 )) || [[ "${mapper_callers[0]}" != "$adapter" ]]; then
    guard_fail "$tag" "If recipe mapper must have exactly one production caller: the named D0-C adapter"
  fi
  local physicalizer_callers=()
  mapfile -t physicalizer_callers < <(
    { rg -l 'physicalize_if_recipe_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v physicalizer="$physicalizer" '$0 != physicalizer && $0 !~ /_tests\.rs$/'
  )
  if (( ${#physicalizer_callers[@]} != 1 )) || [[ "${physicalizer_callers[0]}" != "$materialization" ]]; then
    guard_fail "$tag" "If physicalizer must have exactly one production caller: trivial SSA lowerer"
  fi
  local selected_helper_callers=()
  mapfile -t selected_helper_callers < <(
    { rg -l 'lower_if_recipe_selected\(' "$root_dir/src/mir" || true; } \
      | awk -v materialization="$materialization" '$0 != materialization && $0 !~ /_tests\.rs$/'
  )
  if (( ${#selected_helper_callers[@]} != 1 )) || [[ "${selected_helper_callers[0]}" != "$physicalizer" ]]; then
    guard_fail "$tag" "selected If helper must have exactly one production caller: physicalizer"
  fi
  if ! rg -q 'physical_input\.into_parts\(' "$physicalizer" || \
     ! rg -q 'Result<CanonicalIfPhysicalSuccessV1' "$physicalizer"; then
    guard_fail "$tag" "If physicalizer must consume the paired payload and return Result-only success"
  fi
  local physical_input_part_callers=()
  mapfile -t physical_input_part_callers < <(
    { rg -l 'physical_input\.into_parts\(' "$root_dir/src/mir" || true; } \
      | awk -v physicalizer="$physicalizer" '$0 != physicalizer && $0 !~ /_tests\.rs$/'
  )
  if (( ${#physical_input_part_callers[@]} != 0 )); then
    guard_fail "$tag" "If physical-input payload must be unpacked only by the named physicalizer"
  fi
  if rg -n 'claim_if\(|Pending\(_physical_input\)|drop\([^)]*physical_input' "$adapter" >/dev/null; then
    guard_fail "$tag" "If selected demand payload was silently dropped or claim-only"
  fi
  local elaborator_callers=()
  mapfile -t elaborator_callers < <(
    { rg -l 'IfJoinSigElaboratorV1::elaborate\(' "$root_dir/src/mir" || true; }
  )
  for file in "${elaborator_callers[@]}"; do
    if [[ "$file" != "$contract_dir"/* ]]; then
      guard_fail "$tag" "If JoinSig elaborator acquired an external caller: ${file#"$root_dir/"}"
    fi
  done
}
