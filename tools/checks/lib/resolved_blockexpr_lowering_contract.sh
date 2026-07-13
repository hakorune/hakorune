# Static and executable B0-L3a contract kept out of the near-limit authority guard.

guard_resolved_blockexpr_lowering_contract() {
  local tag="$1"
  local root="$2"
  local lower="$root/src/mir/builder/resolved_lowering"
  local product="$root/src/mir/resolved_semantics/product.rs"
  local capability="$root/src/mir/compiler/capability.rs"

  guard_require_files "$tag" \
    "$lower/scope.rs" \
    "$lower/block_expr_tests.rs"
  for anchor in ResolvedScopeRegionPairV1 block_expr_scope_region_pair BlockExprPreludeRoot; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$product" \
      "B0-L3a exact sealed pair query drifted: $anchor"
  done
  for anchor in ResolvedScopeSessionV1 close_success close_error pair_reconsumed; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$lower/scope.rs" \
      "B0-L3a resolved scope transaction drifted: $anchor"
  done
  guard_expect_fixed_in_file "$tag" "values: BTreeMap<BindingRefV1, ValueId>" \
    "$lower/identity.rs" "canonical BindingRef value environment missing"
  for forbidden in build_expression build_variable_access build_assignment allocate_binding_id \
    variable_map binding_ctx LexicalScopeGuard; do
    if rg -n "$forbidden" "$lower/lowerer.rs"; then
      guard_fail "$tag" "canonical lowerer crossed a legacy identity seam: $forbidden"
    fi
  done
  for anchor in retired retire_scope_success retire_scope_error disposed_bindings; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$lower/identity.rs" \
      "B0-L3a BindingRef disposition drifted: $anchor"
  done
  for anchor in BodyChildRoleV1::BlockExprPrelude ExprChildRoleV1::BlockExprTail \
    lower_block_expr during_cleanup; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$lower/lowerer.rs" \
      "B0-L3a located BlockExpr Lower drifted: $anchor"
  done
  for anchor in LocatedBodyV1 LocatedStmtV1 LocatedExprV1 block_expr_count; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$capability" \
      "B0-L3a located preflight drifted: $anchor"
  done

  cargo test -q --manifest-path "$root/Cargo.toml" --features vm-reference --lib \
    mir::builder::resolved_lowering::block_expr_tests
}
