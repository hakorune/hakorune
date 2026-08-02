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
  local m1_test="$root_dir/src/mir/compiler/loop_candidate_abort_p0.rs"

  guard_require_files "$tag" "$manifest" "$routing" "$router" "$raw_child" \
    "$recursive_child" "$normal" "$raw_compile" "$raw_open" "$raw_recipe" \
    "$canonical" "$canonical_dispatch" "$canonical_input" "$m1_test"

  local header=$'ingress_kind\tpublic_ingress\tcandidate_owner\tloop_reachability\tpublication_owner\tambient_write_policy'
  [[ "$(head -n1 "$manifest")" == "$header" ]] || \
    guard_fail "$tag" "Loop candidate scope manifest header drift"
  if ! awk -F '\t' '
    NR == 1 { next }
    NF != 6 { exit 1 }
    $1 !~ /^(normal|repl|raw-public|raw-reference|vm-hako-reference|canonical-resolved|canonical-core-script|canonical-core-main)$/ { exit 1 }
    $3 !~ /(SessionV1|TransactionV1)$/ { exit 1 }
    $4 !~ /^(reachable|typed-unreachable)$/ { exit 1 }
    $6 !~ /^identity-monotonic\+diagnostic-scratch/ { exit 1 }
    ($1 ~ /^(normal|repl|vm-hako-reference)$/ && $4 != "reachable") { exit 1 }
    ($1 ~ /^(raw-public|raw-reference|canonical-)/ && $4 != "typed-unreachable") { exit 1 }
    END { if (NR != 9) exit 1 }
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
}
