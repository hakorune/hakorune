#!/usr/bin/env bash

guard_joinir_loop_compile_candidate_scope() {
  local root_dir="$1"
  local tag="$2"
  local manifest="$root_dir/tools/checks/manifests/joinir_loop_compile_candidate_scope_v1.tsv"
  local routing="$root_dir/src/mir/builder/control_flow/joinir/routing.rs"
  local router="$root_dir/src/mir/builder/control_flow/joinir/route_entry/router.rs"
  local raw_child="$root_dir/src/mir/builder/raw_loop_child_entry.rs"
  local recursive_child="$root_dir/src/mir/builder/recursive_child_lowering.rs"
  local normal="$root_dir/src/mir/compiler/normal_default_pipeline.rs"
  local raw_compile="$root_dir/src/mir/compiler/raw_published_compile.rs"
  local raw_open="$root_dir/src/mir/compiler/raw_root_eligibility.rs"
  local raw_recipe="$root_dir/src/mir/compiler/raw_root_source_facts/recipe_projection.rs"
  local canonical="$root_dir/src/mir/compiler/source_bound_package.rs"
  local canonical_dispatch="$root_dir/src/mir/compiler/canonical_core_dispatch.rs"
  local canonical_input="$root_dir/src/mir/compiler/lowering_input.rs"
  local capability="$root_dir/src/mir/compiler/capability.rs"
  local first_family_plan="$root_dir/src/mir/compiler/capability/first_family_plan.rs"
  local source_bound_plan="$root_dir/src/mir/compiler/source_bound_plan.rs"
  local m1_test="$root_dir/src/mir/compiler/loop_candidate_abort_p0.rs"
  local direct_accum_cutover="$root_dir/src/mir/compiler/resolved_direct_accum_cutover.rs"
  local hardening_test="$root_dir/src/mir/compiler/resolved_direct_accum_hardening_p0.rs"
  local external_commit="$root_dir/src/mir/compiler/external_commit.rs"
  local loop_region="$root_dir/src/mir/resolved_semantics/loop_region.rs"
  local source_adapter="$root_dir/src/mir/loop_structural_facts/resolved_source_adapter.rs"

  guard_require_files "$tag" "$manifest" "$routing" "$router" "$raw_child" \
    "$recursive_child" "$normal" "$raw_compile" "$raw_open" "$raw_recipe" \
    "$canonical" "$canonical_dispatch" "$canonical_input" "$m1_test" \
    "$direct_accum_cutover" "$hardening_test" "$external_commit" \
    "$capability" "$first_family_plan" "$source_bound_plan" "$loop_region" \
    "$source_adapter"

  local header=$'ingress_kind\tpublic_ingress\tcandidate_owner\tloop_reachability\tpublication_owner\tambient_write_policy'
  [[ "$(head -n1 "$manifest")" == "$header" ]] || \
    guard_fail "$tag" "Loop candidate scope manifest header drift"
  if ! awk -F '\t' '
    NR == 1 { next }
    NF != 6 { exit 1 }
    $1 !~ /^(normal|repl|raw-public|raw-reference|vm-hako-reference|canonical-resolved|canonical-resolved-direct-accum|canonical-core-script|canonical-core-main)$/ { exit 1 }
    $3 !~ /(SessionV1|TransactionV1)$/ { exit 1 }
    $4 !~ /^(reachable|typed-unreachable)$/ { exit 1 }
    $6 !~ /^identity-monotonic\+diagnostic-scratch/ { exit 1 }
    ($1 ~ /^(normal|repl|vm-hako-reference)$/ && $4 != "reachable") { exit 1 }
    ($1 ~ /^(raw-public|raw-reference|canonical-resolved|canonical-core-)/ && $1 != "canonical-resolved-direct-accum" && $4 != "typed-unreachable") { exit 1 }
    ($1 == "canonical-resolved-direct-accum" && $4 != "reachable") { exit 1 }
    END { if (NR != 10) exit 1 }
  ' "$manifest"; then
    guard_fail "$tag" "Loop candidate scope manifest row contract failed"
  fi

  local route_refs
  route_refs="$(rg -n -F 'route_loop(' "$root_dir/src" --glob '*.rs' || true)"
  if [[ "$(printf '%s\n' "$route_refs" | sed '/tests\//d' | sed '/_tests\.rs:/d' | sed '/::tests::/d' | sed '/pub(crate) fn route_loop/d' | sed '/^$/d' | wc -l | tr -d '[:space:]')" != "1" ]]; then
    guard_fail "$tag" "production route_loop caller count drift"
  fi
  if ! rg -n -F 'route_loop(self, &ctx)?' "$routing" >/dev/null || \
     ! rg -n -F 'pub(crate) fn route_loop(' "$router" >/dev/null; then
    guard_fail "$tag" "route_loop definition/caller anchors drift"
  fi

  local joinir_refs
  joinir_refs="$(rg -n -F 'try_cf_loop_joinir(' "$root_dir/src" --glob '*.rs' || true)"
  if [[ "$(printf '%s\n' "$joinir_refs" | sed '/tests\//d' | sed '/_tests\.rs:/d' | sed '/^$/d' | wc -l | tr -d '[:space:]')" != "2" ]]; then
    guard_fail "$tag" "try_cf_loop_joinir definition/caller count drift"
  fi
  if ! rg -n -F 'lower_loop_or_freeze_v1(' "$routing" "$raw_child" "$recursive_child" >/dev/null; then
    guard_fail "$tag" "shared Loop terminal owner anchors drift"
  fi

  if ! rg -n -F 'route_loop(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /route_entry\/router\.rs$/ && $1 !~ /joinir\/routing\.rs$/ && $1 !~ /_tests?\.rs$/ { found = 1 } END { exit found }'; then
    guard_fail "$tag" "route_loop caller escaped the sole routing owner"
  fi
  if ! rg -n -F 'try_cf_loop_joinir(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /joinir\/routing\.rs$/ && $1 !~ /_tests?\.rs$/ { found = 1 } END { exit found }'; then
    guard_fail "$tag" "try_cf_loop_joinir caller escaped the routing owner"
  fi

  for required in \
    "$normal|ModuleBuilderInvocationSessionV1::open_for_token|1" \
    "$normal|prepare_external_commit|1" \
    "$normal|prepared.commit(&mut compiler.builder)|1" \
    "$raw_compile|open_physical(&self.builder)|1" \
    "$raw_compile|prepare_external_commit|1" \
    "$raw_open|ModuleBuilderInvocationSessionV1::open_for_token|1" \
    "$canonical|ModuleBuilderInvocationSessionV1::open_for_token|1" \
    "$direct_accum_cutover|pub(super) fn compile_direct_accum_source_bound(|1" \
    "$canonical|lower_resolved_direct_accum_function_draft|1" \
    "$canonical_dispatch|prepare_normal_main_module_transaction|1"
  do
    local file="${required%%|*}"
    local rest="${required#*|}"
    local pattern="${rest%|*}"
    local expected="${rest##*|}"
    local count="$(rg -o -F "$pattern" "$file" | wc -l | tr -d '[:space:]')"
    [[ "$count" == "$expected" ]] || guard_fail "$tag" "candidate scope anchor drift: $pattern count=$count expected=$expected"
  done

  for forbidden in \
    'compiler.builder.try_cf_loop_joinir' \
    'compiler.builder.route_loop' \
    'self.builder.try_cf_loop_joinir' \
    'self.builder.route_loop'
  do
    if rg -n -F "$forbidden" "$root_dir/src" --glob '*.rs' >/dev/null; then
      guard_fail "$tag" "direct live Builder Loop edge returned: $forbidden"
    fi
  done

  if rg -n -F 'CanonicalFirstFamilyPlanV1::DirectAccum' "$root_dir/src" --glob '*.rs' >/dev/null || \
     rg -n -F 'ExactCanonicalPreflightPlanV1::DirectAccum' "$root_dir/src" --glob '*.rs' >/dev/null; then
    guard_fail "$tag" "DirectAccum escaped the canonical Loop family envelope"
  fi
  for required in \
    'CanonicalLoopFamilyPlanV1' \
    'Loop(CanonicalLoopFamilyPlanV1::DirectAccum' \
    'CanonicalFirstFamilyPlanV1::Loop' \
    'ExactCanonicalPreflightPlanV1::Loop'
  do
    rg -n -F "$required" "$capability" "$first_family_plan" "$source_bound_plan" "$canonical" >/dev/null || \
      guard_fail "$tag" "canonical Loop family envelope anchor missing: $required"
  done

  for required in \
    'VerifiedResolvedLoopSourceForestV1' \
    'resolved_loop_source_forest' \
    'SkippedIntermediateLoop' \
    'UnsupportedAncestry'
  do
    rg -n -F "$required" "$loop_region" >/dev/null || \
      guard_fail "$tag" "Nested source-forest anchor missing: $required"
  done
  if rg -n -F 'resolved_loop_source_forest(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /resolved_semantics\/loop_region\.rs$/ && $1 !~ /_tests?\.rs$/ && $1 !~ /\/tests\.rs$/ { found = 1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested source forest escaped its caller-zero resolver boundary"
  fi
  for forbidden in 'ASTNode' 'MirBuilder' 'LoopRouteId' 'ValueId' 'BasicBlockId' 'Retry'
  do
    if rg -n -F "$forbidden" "$loop_region" >/dev/null; then
      guard_fail "$tag" "Nested source forest imported physical/route authority: $forbidden"
    fi
  done

  for required in \
    'VerifiedLoopSourceForestBindingV1' \
    'bind_resolved_loop_source_forest_v1(' \
    'into_source_binding(' \
    'LoopSourceForestBindingRejectV1'
  do
    rg -n -F "$required" "$source_adapter" >/dev/null || \
      guard_fail "$tag" "Nested source-binding adapter anchor missing: $required"
  done
  if rg -n -F 'bind_resolved_loop_source_forest_v1(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /loop_structural_facts\/resolved_source_adapter\.rs$/ && $1 !~ /_tests?\.rs$/ && $1 !~ /\/tests\.rs$/ { found = 1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested source-binding adapter escaped its caller-zero boundary"
  fi
  for forbidden in 'ASTNode' 'MirBuilder' 'LoopRouteId' 'ValueId' 'BasicBlockId' 'Retry'
  do
    if rg -n -F "$forbidden" "$source_adapter" >/dev/null; then
      guard_fail "$tag" "Nested source-binding adapter imported physical/route authority: $forbidden"
    fi
  done

  if ! rg -n -F 'RawLocatedScalarStmtV1::Loop { .. }' "$raw_recipe" >/dev/null || \
     ! rg -n -F 'RawUnsupportedBodyStatementKindV1::Loop' "$raw_recipe" >/dev/null || \
     ! rg -n -F 'unsupported_shape_fails_before_builder_effects_without_legacy_retry' "$canonical_input" >/dev/null || \
     ! rg -n -F 'ASTNode::Loop {' "$canonical_input" >/dev/null; then
    guard_fail "$tag" "Raw/canonical Loop pre-effect rejection anchors drift"
  fi

  for required in \
    'loop_effect_then_later_failure_discards_candidate_and_reuses_live_compiler' \
    'reset_loop_physical_effect_probe' \
    'take_loop_physical_effect_probe' \
    'loop_candidate_test_fingerprint' \
    'Undefined variable: missing' \
    'loop-failure.hako' \
    'loop-reused.hako'
  do
    rg -n -F "$required" "$m1_test" >/dev/null || \
      guard_fail "$tag" "M1 candidate-abort proof anchor missing: $required"
  done

  for required in \
    'production_failure_after_prepare_discards_candidate_and_reuses_compiler' \
    'successful_direct_accum_public_result_uses_final_barrier_contract' \
    'compile_direct_accum_source_bound_with_prepared_failure_for_test'
  do
    rg -n -F "$required" "$hardening_test" "$direct_accum_cutover" >/dev/null || \
      guard_fail "$tag" "resolved DirectAccum hardening proof anchor missing: $required"
  done

  rg -n -F 'project_canonical_verification_result' "$external_commit" >/dev/null || \
    guard_fail "$tag" "canonical final-barrier projection anchor missing"
  if rg -n -F 'pre_transform.map_err' "$external_commit" >/dev/null; then
    guard_fail "$tag" "canonical publication still projects pre_transform directly"
  fi
}
