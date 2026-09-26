# MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-HANDLE-ARG-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D9
  (accepted — the `argument_rows().is_err()` precheck is the
  designed complete-or-reject admission contract (pinned test
  fb6fd52886); the real gaps are upstream: `argument_observations`
  are only issued on gated coseal lanes so every other claim gets
  a synthesized `Err(SourceMismatch)`, and `SelectedNewArgumentKindV1`
  lacks a `Handle` kind so handle-typed args are `ArgumentNotTrivial`)
Owner: workstream row H / unified resume gate 1
Authority: D9 Decision. `SelectedNewArgumentObservationV1` → claim
  `argument_rows` → `materialize_arguments`. `OrdinaryObservation::
  Handle` already tracks the root binding; parameter contracts
  already exist for every admitted declaration.

## Slice

1. `resolved_semantics/selected_new_arguments.rs`: add
   `SelectedNewArgumentKindV1::Handle { binding: BindingRefV1 }`.
2. `resolved_semantics/home_prefix_local_flow.rs`
   `into_selected_argument`: map `Handle(root)` →
   `Some(Handle { binding: root })` (non-consumed handles only —
   the `Consumed` arm already returns `None`).
3. `resolved_semantics/home_new_prefix_arguments.rs` (new
   submodule; `home_new_prefix.rs` was at the 800-line hard stop):
   sibling entry `issue_new_home_prefixes_with_arguments_v1(input,
   selected, parameters)` returning `scan_new_home_flow`'s `.0`
   (prefixes) AND `.3` (observations), with the caller's real
   `parameter_contracts` fed to `install_parameters`.
   `issue_new_home_prefixes_v1` keeps its empty-params/.0 shape —
   the `child_new_ready` probe at `coseal_issue.rs:156` is
   deliberately untouched.
4. `normal_callable_semantic_package/ordinary_new_coseal_issue.rs`:
   the fallback branch (`:300`) and error branch (`:297`) use the
   sibling with `parameter_contracts` filtered by `batch_slot`,
   producing real `argument_observations` for every claim-bearing
   function instead of synthesized `Err(SourceMismatch)`.
5. `normal_callable_semantic_package/ordinary_new_arguments.rs`:
   add `OrdinaryNewTrivialArgumentKindV1::Handle { binding }`;
   `coseal_helpers::convert_selected_new_arguments` maps it.
6. `builder/ordinary_new_admission/selected.rs`
   `materialize_arguments`: `Handle { binding }` arm — same
   `value_for_exact_binding` + `observe_variable_site` mechanics
   as `Local`.

## Overlap analysis (required, source-read)

- `OrdinaryObservation::Handle(root)` already carries the root
  binding for parameter-installed (`StoredLocal::Handle(r)` with
  `r == root`) and copied handles; only `Consumed`/`Uninitialized`
  decline. `is_self_rooted_handle` exists for the self-rooted
  check if a narrower admission is ever needed — not used here.
- `materialize_arguments` `Handle` arm is byte-identical in shape
  to `Local`: `value_for_exact_binding(owner, binding)` +
  `observe_variable_site(site, binding, value)` — physically a
  borrow (the caller's handle stays live — `me.users.push(user)`
  uses `user` after `new UserStats(user)`).
- Feeding `parameter_contracts` on the fallback path can only
  unblock: every claim there currently hard-fails on the
  synthesized `Err`, so no working claim can regress. Functions
  whose params lack complete contracts keep `EntryDemandMissing`
  → `home_prefix` Err → `prepare` declines → `RetainedUnavailable`
  → `Ordinary` route — a designed non-root lifecycle.
- `install_parameters` also feeds the homes walk — params becoming
  visible can turn `home_prefix` Ok → `prepare` may SELECT claims
  that would previously decline. The selected path is the
  designed preferred lane; its `materialize_arguments` Handle arm
  is part of this slice.

## Pins (required before close)

- Existing `selected_new_rejects_nontrivial_argument_before_raw_
  descent` keeps failing with
  `[ordinary-new/argument-source-unavailable]` (negative —
  `ArgumentNotTrivial` unchanged for BinOp etc.).
- Existing `selected_new_arguments_reach_birth_in_issued_order`
  stays green (positive — trivial args unaffected).
- Positive: a cataloged non-main function with `local x = new
  Box(handle_param)` gets `argument_rows = Ok([Handle])` — the
  claim passes the precheck and the site lowers (selected or
  `RetainedUnavailable`→ordinary — assert it does NOT hit
  `argument-source-unavailable`).
- Positive: `into_selected_argument(Handle)` → `Handle{root}`;
  `materialize_arguments` Handle arm emits the binding's value
  (focused unit/route test if a cheap harness exists).
- Negative: a param whose contract is missing/unsupported keeps
  `ArgumentNotTrivial` or `EntryDemandMissing` — same freeze, no
  silent claim.
- Real app: json_stream_aggregator advances past
  `ordinary-new/argument-source-unavailable` —
  `local stats = new UserStats(user)` gets provable rows; next
  honest terminal is whatever it is.

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`:
  pin `SelectedNewArgumentKindV1::Handle`,
  `into_selected_argument`'s `Handle` arm,
  `issue_new_home_prefixes_with_arguments_v1`, the coseal fallback
  passing `parameter_contracts`, and
  `OrdinaryNewTrivialArgumentKindV1::Handle` +
  `materialize_arguments` Handle arm.

## Fail-fast boundary

- `argument-source-unavailable` still fires for `ArgumentNotTrivial`
  (non-trivial expr), `SourceMismatch` (missing navigation), and
  `ArgumentOrdinalOverflow` — unchanged on the claim admission.
- `argument-row-count`/`argument-row-drift`/`argument-ordinal-
  overflow`/`argument-binding-unavailable` unchanged.
- `install_parameters` failure keeps `EntryDemandMissing` — no
  param is invented; only declared contracts install.

## Evidence (landed 2026-09-25)

- Focused tests green: `cargo test --lib ordinary_new` 60/60,
  `selected_new` 3/3 (incl. pinned negative
  `selected_new_rejects_nontrivial_argument_before_raw_descent`),
  `brand_catalog` 19/19.
- New positive pin `ordinary_new_claim_records_parameter_handle_
  argument` (brand_catalog_tests.rs): `helper(value) { local item =
  new Page(value) }` — a cataloged non-main function — issues
  `argument_rows = Ok([Handle{binding}])` with `binding` proven
  equal to `Parameter{index:0}`'s declared binding, plus
  `home_prefix().is_ok()`. This is the `new UserStats(user)`
  motivating shape.
- Fixture repin in `ordinary_new_unknown_home_prefix_is_not_an_
  empty_cleanup_plan`: `helper(value)` now gets a real `Ok` prefix
  (declared contract installs; `EntryDemandMissing` was the old
  empty-contract artifact), and `new Page(first)`'s second claim is
  `Ok` (`first` observes as `Handle` — a live Home passed to `new`
  is now covered, previously `ArgumentNotCovered`). Both outcomes
  are the designed H3b semantics, not regressions.
- Guard: `mirbuilder_qualified_route_scope_guard.sh` extended —
  pins both `Handle` variants, the `into_selected_argument` Handle
  arm, `issue_new_home_prefixes_with_arguments_v1` (own submodule +
  re-export), the `parameter_contracts` `batch_slot` feed, the
  `materialize_arguments` Handle arm (`value_for_exact_binding` +
  `observe_variable_site`), the `scalar_actual_kind`/`emission_
  validation` Handle arms, and the new positive test name. All new
  files added to the 800-line hard-stop list
  (`home_new_prefix.rs` = 792 after the split).
- `cargo build` dev profile green.
- Real app `json_stream_aggregator` (debug binary): advanced past
  `[ordinary-new/argument-source-unavailable]` —
  `local stats = new UserStats(user)` now has provable rows — to
  `[static-result-ingress/no-exact-static-target]` at `env.get/1`
  inside `using`-imported `lang/src/shared/common/string_helpers.
  hako` (`StringHelpers` box, debug branch). `env` is not a
  declared box in the same-module catalog — a different authority
  (builtin/imported receiver), the honest next boundary.
- 12-entry EXE boundary suite: 4 pass / 7 fail — every other
  terminal identical to baseline (`NamedArray`, `callable-loop/
  route-not-front-selected` x3, `mir_call_no_route`, LLVM
  toolchain, untyped-field panic). No regressions.
- `tools/checks/current_state_pointer_guard.sh` green;
  `git diff --check` clean.

## Non-claims

- Does not rename `OrdinaryNewTrivialArgument*` (a Handle kind in a
  `Trivial`-named enum is honest naming debt — separate hygiene
  task, the semantic is "selected argument").
- Does not extend `Handle` args to other observation consumers.
- Does not claim `statsFor`/`ingestLine` green — `home_prefix`,
  `construction`, local materialization, or `me.stats.set` may hit
  their own named terminals.
- The `child_new_ready` probe at `coseal_issue.rs:156` is out of
  scope (feeding params there would change seed-completion
  eligibility).
- Root-vs-site asymmetry: `into_selected_argument` emits
  `Handle { binding: root }` (the chain root), while `materialize_
  arguments` requires `observe_variable_site(site, binding, …)` —
  the exact site↔binding match. An alias-of-handle argument
  (`local alias = user; new X(alias)`) fails closed at
  `argument-local-observation`, same as before — no source-alias
  authority was added.
- Does not admit `env`/builtin/imported static receivers —
  `no-exact-static-target` at `env.get/1` is the next named
  boundary and needs its own D-card.
