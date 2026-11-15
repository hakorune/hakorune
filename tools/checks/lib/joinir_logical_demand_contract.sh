#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.
guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local route_id="$root_dir/src/mir/loop_recipe_contract/route_id.rs"
  local portable_recipe_dir="$root_dir/src/mir/loop_recipe_contract"
  local producer_id="$portable_recipe_dir/producer_id.rs"
  local loop_structural_facts_dir="$root_dir/src/mir/loop_structural_facts"
  local direct_accum_recipe_producer="$portable_recipe_dir/direct_accum_producer.rs"
  local variable_accum_recipe_producer="$portable_recipe_dir/variable_accum_recurrence_producer.rs";
  # The compiler-owned DirectAccum profile is the sole disconnected issuer of
  # the portable producer. It is not a downstream production consumer; the
  # guard must distinguish this issuer from an accidental route caller.
  local direct_accum_issuer="$root_dir/src/mir/compiler/direct_accum_profile.rs"
  local direct_accum_capability="$root_dir/src/mir/compiler/direct_accum_capability.rs"
  local direct_accum_projection="$root_dir/src/mir/compiler/direct_accum_projection.rs"
  local direct_accum_observation_adapter="$root_dir/src/mir/compiler/direct_accum_observation.rs"
  local nested_observation_source="$root_dir/src/mir/loop_structural_facts/nested_predicate_observation.rs"
  local nested_observation_adapter="$root_dir/src/mir/compiler/nested_predicate_observation.rs"
  local nested_observation_policy="$root_dir/src/mir/loop_route_policy/nested_predicate_observation.rs"
  local nested_observation_tests="$root_dir/src/mir/loop_route_policy/nested_predicate_observation_tests.rs"
  local family_selector_tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"
  local loop_true_source_projection="$root_dir/src/mir/compiler/loop_true_break_continue_projection.rs"
  local loop_true_observation_adapter="$root_dir/src/mir/compiler/loop_true_break_continue_observation.rs"
  local loop_cond_source_projection="$root_dir/src/mir/compiler/loop_cond_break_continue_projection.rs"
  local loop_cond_observation_adapter="$root_dir/src/mir/compiler/loop_cond_break_continue_observation.rs"
  local generic_g0_observation_adapter="$root_dir/src/mir/compiler/generic_g0_observation.rs"
  local loop_route_policy_dir="$root_dir/src/mir/loop_route_policy"
  local route_registry_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry"
  local generic_resolved_test_prefix="$route_registry_dir/generic_resolved_carrier_"
  local nested_predicate_producer="$root_dir/src/mir/compiler/nested_predicate_producer.rs"
  local nested_predicate_producer_tests="$root_dir/src/mir/compiler/nested_predicate_producer_tests.rs";
  # The accepted Dynamic physical-input co-seal is the only named consumer
  # outside the portable contract subtree. Its two fixture files are test
  # evidence only; do not widen this to the whole Dynamic compiler subtree.
  local dynamic_physical_input="$root_dir/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/physical_input.rs"
  local dynamic_semantic_tests="$root_dir/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/tests.rs"
  local dynamic_recipe_tests="$root_dir/src/mir/compiler/dynamic_full_body_recipe/tests.rs"
  local dynamic_recipe_mod="$root_dir/src/mir/compiler/dynamic_full_body_recipe/mod.rs"
  local projection="$root_dir/src/mir/builder/control_flow/facts/stmt_view.rs"
  local simple_while="$root_dir/src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs"
  local accum_const="$root_dir/src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs"
  local loop_break_facts="$root_dir/src/mir/builder/control_flow/plan/loop_break/facts/types.rs"
  local if_phi_join="$root_dir/src/mir/builder/control_flow/facts/if_phi_join_facts.rs"
  local files=(
    "$route_id"
    "$projection"
    "$simple_while"
    "$accum_const"
    "$loop_break_facts"
    "$if_phi_join"
  )
  local file lines

  guard_require_files "$tag" "${files[@]}"
  guard_require_files "$tag" \
    "$direct_accum_issuer" "$direct_accum_capability" "$variable_accum_recipe_producer" \
    "$direct_accum_projection" "$direct_accum_observation_adapter" \
    "$loop_true_source_projection" "$loop_true_observation_adapter" \
    "$loop_cond_source_projection" "$loop_cond_observation_adapter" "$nested_observation_source" \
    "$nested_observation_adapter" "$nested_observation_policy" "$nested_observation_tests" \
    "$generic_g0_observation_adapter"
  local portable_recipe_files=()
  mapfile -t portable_recipe_files < <(find "$portable_recipe_dir" -type f -name '*.rs' | sort)
  guard_require_files "$tag" \
    "$portable_recipe_dir/README.md" \
    "$portable_recipe_dir/schema.rs" \
    "$producer_id" \
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
    "${portable_production_files[@]}" | rg -v ':[0-9]+:[[:space:]]*//' >/dev/null; then
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
  if rg -n -w 'LoopRouteId|producer_route' \
    "$portable_recipe_dir/schema.rs" \
    "$portable_recipe_dir/direct_accum_producer.rs" \
    "$portable_recipe_dir/loop_true_break_continue_producer.rs" \
    "$nested_predicate_producer" >/dev/null; then
    guard_fail "$tag" "portable schema/producer imported legacy LoopRouteId provenance"
  fi
  if ! rg -n -F 'producer_id: LoopRecipeProducerIdV1' "$portable_recipe_dir/schema.rs" >/dev/null || \
     ! rg -n -F 'LoopRecipeProducerIdV1::DirectAccumV1' "$portable_recipe_dir/direct_accum_producer.rs" >/dev/null || \
     ! rg -n -F 'LoopRecipeProducerIdV1::LoopTrueBreakContinueV1' "$portable_recipe_dir/loop_true_break_continue_producer.rs" >/dev/null || \
     ! rg -n -F 'LoopRecipeProducerIdV1::NestedPredicateV1' "$nested_predicate_producer" >/dev/null; then
    guard_fail "$tag" "portable producer ID issuer anchors drifted"
  fi
  local legacy_wire_refs
  legacy_wire_refs="$({ rg -n -w 'producer_route' \
    "$portable_recipe_dir/fixtures" --glob '*.json' || true; } | wc -l | tr -d '[:space:]')"
  if [[ "$legacy_wire_refs" != "0" ]]; then
    guard_fail "$tag" "portable fixtures still carry legacy producer_route wire"
  fi
  local join_sig_external_files=()
  mapfile -t join_sig_external_files < <(
    { rg -l -w \
        'LoopJoinPortV1|LoopJoinEdgeRoleV1|LoopJoinPayloadV1|LoopJoinEdgeV1|LoopJoinLoopV1|LoopJoinSigV1|VerifiedLoopJoinSigV1|LoopJoinSigElaboratorV1|LoopJoinSigRejectReasonV1' \
        "$root_dir/src/mir" || true; } \
      | awk \
          -v prefix="$portable_recipe_dir/" \
          -v nested_producer="$nested_predicate_producer" \
          -v nested_producer_tests="$nested_predicate_producer_tests" \
          -v nested_topology="$root_dir/src/mir/compiler/nested_predicate_topology.rs" \
          -v nested_topology_tests="$root_dir/src/mir/compiler/nested_predicate_topology_tests.rs" \
          -v nested_physical_input="$root_dir/src/mir/compiler/nested_predicate_physical_input.rs" \
          -v nested_physical_input_tests="$root_dir/src/mir/compiler/nested_predicate_physical_input_tests.rs" \
          -v dynamic_physical_input="$dynamic_physical_input" \
          -v dynamic_semantic_tests="$dynamic_semantic_tests" \
          -v dynamic_recipe_tests="$dynamic_recipe_tests" \
          -v main0_continue_coseal="$root_dir/src/mir/compiler/main0_continue_recipe_coseal.rs" \
          -v main0_derived_coseal="$root_dir/src/mir/compiler/main0_derived_predicate_recipe_coseal.rs" \
          -v node_admission="$root_dir/src/mir/compiler/loop_node_physical_admission.rs" \
          -v physicalizer="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physicalizer.rs" \
          -v physicalizer_session="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physicalizer/session.rs" \
          'index($0, prefix) != 1 && $0 != nested_producer && $0 != nested_producer_tests && $0 != nested_topology && $0 != nested_topology_tests && $0 != nested_physical_input && $0 != nested_physical_input_tests && $0 != dynamic_physical_input && $0 != dynamic_semantic_tests && $0 != dynamic_recipe_tests && $0 != main0_continue_coseal && $0 != main0_derived_coseal && $0 != node_admission && $0 != physicalizer && $0 != physicalizer_session'
  )
  if (( ${#join_sig_external_files[@]} != 0 )); then
    guard_fail "$tag" "caller-zero logical JoinSig symbols escaped the contract subtree"
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
  for nested_file in "$nested_observation_source" "$nested_observation_adapter" \
    "$nested_observation_policy" "$nested_observation_tests"; do
    lines="$(wc -l < "$nested_file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "Nested S1 file exceeds boundary: $nested_file"
  done
  if ! rg -q 'the only compiler-side translation point' "$nested_observation_adapter"; then
    guard_fail "$tag" "Nested S1 adapter lost its sole-compiler-translation-point contract"
  fi
  [[ "$(rg -o -F '#[test]' "$nested_observation_tests" | wc -l | tr -d '[:space:]')" == "7" ]] ||
    guard_fail "$tag" "Nested S1 focused test count drift"
  if rg -n -F 'ASTNode' "$nested_observation_source" "$nested_observation_policy" >/dev/null ||
     rg -n -F 'MirBuilder' "$nested_observation_source" "$nested_observation_policy" >/dev/null ||
     rg -n -F 'loop_recipe_contract' "$nested_observation_source" "$nested_observation_policy" >/dev/null; then
    guard_fail "$tag" "Nested S1 logical observer acquired source/physical authority"
  fi
  if rg -l -F 'issue_nested_predicate_family_observation_v1(' "$root_dir/src/mir" |
    awk -v p="$nested_observation_policy" -v t="$nested_observation_tests" \
      -v at="$root_dir/src/mir/loop_route_policy/family_admission_tests.rs" \
      -v st="$family_selector_tests" \
      -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      -v w="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
      '$0 != p && $0 != t && $0 != at && $0 != st && $0 != m && $0 != w && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested S1 policy observer acquired a production caller"
  fi
  if rg -l -F 'VerifiedResolvedLoopSourceV1' "$root_dir/src/mir" |
    awk -v a="$direct_accum_observation_adapter" -v n="$nested_observation_adapter" \
      -v p="$direct_accum_projection" -v l="$loop_true_source_projection" \
      -v t="$loop_true_observation_adapter" -v c="$loop_cond_source_projection" \
      -v d="$loop_cond_observation_adapter" -v g="$generic_g0_observation_adapter" \
      -v s="$root_dir/src/mir/loop_structural_facts/" \
      -v r="$root_dir/src/mir/resolved_semantics/" \
      -v b="$root_dir/src/mir/builder/normal_callable_loop_source_facts/" \
      -v t2="$root_dir/src/mir/loop_recipe_contract/s6c_scan_with_init_tests.rs" \
      -v x2="$root_dir/src/mir/compiler/loop_break_composite_source_projection.rs" \
      -v x3="$root_dir/src/mir/compiler/loop_break_source_projection.rs" \
      -v x4="$root_dir/src/mir/compiler/loop_family_window_probe_tests.rs" \
      -v x5="$root_dir/src/mir/compiler/main0_continue_source_map.rs" \
      -v x6="$root_dir/src/mir/compiler/main0_derived_predicate_source_map.rs" \
      '$0 != a && $0 != n && $0 != p && $0 != l && $0 != t && $0 != c && $0 != d && $0 != g && $0 != t2 && $0 != x2 && $0 != x3 && $0 != x4 && $0 != x5 && $0 != x6 && index($0,s) != 1 && index($0,r) != 1 && index($0,b) != 1 { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "sealed resolved Loop source capability escaped its adapter boundary"
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
      | awk -v prefix="$loop_structural_facts_dir/" -v producer="$direct_accum_recipe_producer" -v variable_producer="$variable_accum_recipe_producer" \
          -v projection="$loop_true_source_projection" \
          -v dynamic_recipe_mod="$dynamic_recipe_mod" \
          -v wire_tests_prefix="$root_dir/src/mir/loop_recipe_contract/wire_parity_tests" \
          -v wire_route_tests="$root_dir/src/mir/loop_recipe_contract/wire_route_parity_tests.rs" \
          -v loopcond_issue="$root_dir/src/mir/compiler/loop_cond_break_continue_typed_map_issue.rs" \
          -v main0_continue_coseal="$root_dir/src/mir/compiler/main0_continue_recipe_coseal.rs" \
          -v main0_derived_coseal="$root_dir/src/mir/compiler/main0_derived_predicate_recipe_coseal.rs" \
          'index($0, prefix) != 1 && index($0, wire_tests_prefix) != 1 && $0 != producer && $0 != variable_producer && $0 != projection && $0 != dynamic_recipe_mod && $0 != wire_route_tests && $0 != loopcond_issue && $0 != main0_continue_coseal && $0 != main0_derived_coseal' \
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
          -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
          '$0 != producer && $0 != issuer && $0 != spine && $0 !~ /_tests\.rs$/'
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
  if [[ "$direct_accum_issuer_calls" != "1" ]]; then
    guard_fail "$tag" "Direct Accum issuer call count drift: count=$direct_accum_issuer_calls expected=1"
  fi
  local direct_accum_source_probe_callers=()
  mapfile -t direct_accum_source_probe_callers < <(
    { rg -l 'probe_direct_accum_source_unit_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v capability="$direct_accum_capability" '$0 != capability && $0 !~ /_tests\.rs$/'
  )
  if (( ${#direct_accum_source_probe_callers[@]} != 2 )) || \
     ! printf '%s\n' "${direct_accum_source_probe_callers[@]}" | rg -q -x "$root_dir/src/mir/compiler/capability.rs" || \
     ! printf '%s\n' "${direct_accum_source_probe_callers[@]}" | rg -q -x "$root_dir/src/mir/compiler/generic_g0_capability.rs"; then
    guard_fail "$tag" "Direct Accum source-unit probe caller set drifted"
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
          -v nested_producer="$root_dir/src/mir/compiler/nested_predicate_producer.rs" \
          -v nested_producer_tests="$root_dir/src/mir/compiler/nested_predicate_producer_tests.rs" \
          -v main0_continue_coseal="$root_dir/src/mir/compiler/main0_continue_recipe_coseal.rs" \
          -v main0_derived_coseal="$root_dir/src/mir/compiler/main0_derived_predicate_recipe_coseal.rs" \
          -v dynamic_recipe_mod="$dynamic_recipe_mod" \
          -v dynamic_physical_input="$dynamic_physical_input" \
          -v generic_test_prefix="$generic_resolved_test_prefix" \
          'index($0, recipe_prefix) != 1 && index($0, structural_prefix) != 1 && !(index($0, generic_test_prefix) == 1 && $0 ~ /_tests\.rs$/) && $0 != nested_producer && $0 != nested_producer_tests && $0 != main0_continue_coseal && $0 != main0_derived_coseal && $0 != dynamic_recipe_mod && $0 != dynamic_physical_input'
  )
  if (( ${#external_portable_source_files[@]} != 0 )); then
    guard_fail "$tag" "semantic or physical Loop consumer acquired source/provenance authority"
  fi
  local external_resolved_source_files=()
  mapfile -t external_resolved_source_files < <(
    { rg -l -w 'VerifiedResolvedLoopSourceV1' "$root_dir/src/mir" || true; } \
      | awk \
          -v structural_prefix="$loop_structural_facts_dir/" \
          -v resolved_prefix="$root_dir/src/mir/resolved_semantics/" \
          -v projection="$direct_accum_projection" \
          -v observation_adapter="$direct_accum_observation_adapter" \
          -v nested_observation_adapter="$nested_observation_adapter" \
          -v loop_true_projection="$loop_true_source_projection" \
          -v loop_true_observation_adapter="$loop_true_observation_adapter" \
          -v loop_cond_projection="$loop_cond_source_projection" \
          -v loop_cond_observation_adapter="$loop_cond_observation_adapter" \
          -v generic_g0_observation_adapter="$generic_g0_observation_adapter" \
          -v dynamic_recipe_mod="$dynamic_recipe_mod" \
          -v dynamic_physical_input="$dynamic_physical_input" \
          -v callable_facts_prefix="$root_dir/src/mir/builder/normal_callable_loop_source_facts/" \
          -v s6c_tests="$root_dir/src/mir/loop_recipe_contract/s6c_scan_with_init_tests.rs" \
          -v break_composite="$root_dir/src/mir/compiler/loop_break_composite_source_projection.rs" \
          -v break_source="$root_dir/src/mir/compiler/loop_break_source_projection.rs" \
          -v window_probe_tests="$root_dir/src/mir/compiler/loop_family_window_probe_tests.rs" \
          -v main0_continue_map="$root_dir/src/mir/compiler/main0_continue_source_map.rs" \
          -v main0_derived_map="$root_dir/src/mir/compiler/main0_derived_predicate_source_map.rs" \
          'index($0, structural_prefix) != 1 && index($0, resolved_prefix) != 1 && index($0, callable_facts_prefix) != 1 && $0 != projection && $0 != observation_adapter && $0 != nested_observation_adapter && $0 != loop_true_projection && $0 != loop_true_observation_adapter && $0 != loop_cond_projection && $0 != loop_cond_observation_adapter && $0 != generic_g0_observation_adapter && $0 != dynamic_recipe_mod && $0 != dynamic_physical_input && $0 != s6c_tests && $0 != break_composite && $0 != break_source && $0 != window_probe_tests && $0 != main0_continue_map && $0 != main0_derived_map'
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
    "$loop_route_policy_dir/loop_true_break_continue.rs" \
    "$loop_route_policy_dir/policy.rs" \
    "$loop_route_policy_dir/policy_evidence.rs"
  if (( ${#loop_route_policy_files[@]} == 0 )); then
    guard_fail "$tag" "Loop route policy subtree has no Rust files"
  fi
  local loop_route_policy_production_files=()
  for file in "${loop_route_policy_files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
    case "$file" in
      "$loop_route_policy_dir"/*_tests.rs) ;;
      *) loop_route_policy_production_files+=("$file") ;;
    esac
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CanonicalLoopFacts|CorePlan|ValueId|BasicBlockId|MirInstruction|Frag|RouteFn|RouteAttemptOutcomeV1|Retry|LoopRecipeV1|VerifiedLoopRecipeV1|LoopPhysicalizerV1' \
    "${loop_route_policy_production_files[@]}" | rg -v ':[0-9]+:[[:space:]]*//' >/dev/null; then
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
  # M12-R2C part 2 retired the synthetic 19-row schedule and the winner
  # ceremony. These names must stay absent from src/ so the deleted authority
  # cannot be reintroduced.
  local retired_schedule_hits
  retired_schedule_hits="$(
    { rg -l -g '*.rs' 'evaluate_frozen_loop_route_schedule_v1|freeze_loop_route_schedule_v1|FrozenLoopRouteScheduleV1|FrozenLoopRouteRowV1|VerifiedLoopPolicyWinnerV1' \
      "$root_dir/src" || true; } | wc -l | tr -d '[:space:]'
  )"
  if [[ "$retired_schedule_hits" != "0" ]]; then
    guard_fail "$tag" "retired frozen Loop schedule/winner authority reappeared in src/"
  fi
  if rg -n -U \
    '#\[derive\([^]]*Clone[^]]*\)\][[:space:]]*\npub\(crate\) struct (FrozenLoopRouteScheduleV1|FrozenLoopRouteRowV1)' \
    "$loop_route_policy_dir/schema.rs" >/dev/null; then
    guard_fail "$tag" "frozen Loop route schedule or row became Clone"
  fi
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  local logical_files=(
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
}
