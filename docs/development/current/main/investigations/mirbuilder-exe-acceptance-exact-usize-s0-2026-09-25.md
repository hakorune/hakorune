# MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-D0 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: usize-semantic-foundation-ssot.md (usize = exact source
  type on the i64 lane) + D0 Decision (parameter-scoped arm only).

## Slice

1. `src/mir/exact_trivial_parameter_abi.rs`: replaced the
   `ExactTrivialScalarAbiV1` field with a param-scoped scalar kind
   enum `ExactTrivialParameterScalarV1 { I64, Usize }`;
   `classify` admits `"usize"`; `mir_type()` = `MirType::Integer`;
   `source_type_name()` = `"usize"`; `is_i64()` distinguishes the
   i64 arm. The shared `ExactTrivialScalarAbiV1` (return ABI axis)
   is untouched.
2. `src/mir/normal_callable_semantic_package/instance_constructor_semantic/formal_contract.rs`:
   `classify_declaration` keys `ExactI64` on `== Some(I64)`; usize
   birth formals classify as `ExplicitUnsupported` (named boundary,
   not mislabeled i64).
3. `src/mir/compiler/normal_source_plan/instance_function_plan.rs`:
   the I64ParameterReturn family gates on `abi != I64` so usize is
   rejected with `ParameterDeclarations` (Source stage).

## Consumer audit (all exact-I64 gated, no silent usize-as-i64)

- `issuer.rs`: admits usize -> `ExactTrivial(USIZE)` (intended).
- `dynamic_admission.rs`: `abi != I64` -> `ParameterContractMismatch`.
- `normal_callable_semantic_package/model.rs`,
  `direct_call_lifecycle.rs`: `== ExactTrivial(I64)` checks; usize
  does not match, stays out of i64 lanes.
- `a_prime_i64_physical_capability/issuer.rs`: `!= I64` reject.
- `home_prefix_local_flow.rs`: `ExactTrivial(_) => return false`.
- `resolved_value_profile` (`parameter_entry.rs` -> `coverage.rs`
  `record_parameter_entry`): `abi != I64` ->
  `ParameterEntryContractMismatch` before any `InlineI64` push.

## Pins added

- `exact_trivial_parameter_abi::tests::accepts_exact_i64_and_usize_source_spellings`
- `exact_trivial_parameter_abi::tests::usize_projects_to_the_integer_lane_without_i64_alias`
- `callable_parameter_contract::tests::usize_parameter_projects_exact_trivial_not_unsupported_or_handle`
- `instance_constructor_semantic::tests::birth_formal_usize_declaration_stays_explicit_unsupported_not_exact_i64`
- `normal_source_plan::tests`: `usize_param` case in the family
  reject loop (Source stage `ParameterDeclarations`).

## Evidence

- Focused: `cargo test --profile quick --lib -- parameter_abi
  parameter_contract formal_contract instance_function usize`
  -> 42 pass / 1 fail; the single fail
  (`declared_box_names_project_as_handle_not_opaque_or_exact`,
  MapBox classification) is recorded in
  `tools/checks/manifests/cargo_lib_red_baseline.failures.txt`
  (pre-existing baseline debt, unrelated to usize).
- `cargo check --profile quick` green (existing warnings only).
- `git diff --check` clean; `current_state_pointer_guard.sh` ok.

## Suite rerun (quick binary, NYASH_BIN=target/quick/hakorune)

4 pass / 7 fail. Both usize-family entries moved PAST
`UnsupportedDeclaredType` to downstream named terminals:

- `boxtorrent_mini_exe`:
  `ParameterContract{UnsupportedDeclaredType}` ->
  `main-import-view/selected-header-missing` (different family).
- `mimalloc_lite_exe`:
  `ParameterContract{UnsupportedDeclaredType}` ->
  `ordinary-new/birth-global-legacy-stopped` (ordinary-new claim
  authority boundary).

Unchanged terminals: `json_stream_aggregator`
(`main-import-view/selected-header-missing`), `binary_trees`
(`birth-global-legacy-stopped`), `allocator_stress`
(`NamedArray(TextSourceMissing)`), `typed_object_newbox_min`
(build rc=1 toolchain), `untyped_field`
(`return_type_strategy` inference panic rc=101).

## Exit

- [x] usize admitted parameter-scoped; no scalar/return widening.
- [x] Mislabel sites pinned to i64-only.
- [x] Focused tests green; new reds classified vs baseline.
- [x] Card/pointer/workstream synced; commit+push.

## Non-claims

- usize CALLS are still unadmitted (no usize argument lane).
- usize RETURNS / literals / generic metadata keep their existing
  rejections (shared scalar enum untouched).
- The two moved terminals are boundary movement, not green; each
  downstream family needs its own design authority.
