# MIRBUILDER-REVIEW-MERGE-READINESS-R0

**Row**: `MIRBUILDER-REVIEW-MERGE-READINESS-R0`
**BoxShape**: structural cleanup only — no new accepted shape, no
behavior change. Moves code/tests across file boundaries, widens
test-thread stacks, shares one shape vocabulary, refreshes the
accepted red receipt.
**Mode**: `fast` — focused gates + receipt refresh.

## Review findings and dispositions

1. **800-line hard stop violated (critical)** — fixed.
   - `normal_callable_semantic_lowering_state.rs` (809 -> 749): the
     S8 ledger-publication APIs moved to
     `normal_callable_semantic_lowering_state/loop_value_publication.rs`
     — same impl-split pattern as `source_call_publication` /
     `map_local_tests`. Visibility is `pub(in crate::mir::builder)`,
     matching the sibling callers.
   - `published_backend_view_tests.rs` (848 -> 493) +
     `published_backend_view_drift_tests.rs` (360): definition-drift
     pins moved to a sibling test module; shared fixtures promoted to
     `pub(super)` (`intrinsic_array_module` lives in the
     `include!`d intrinsic-array file).
   - `normal_default_root_catalog_lifecycle_tests.rs` (1221 -> 732) +
     `normal_default_root_catalog_main_selection_tests.rs` (497):
     main-selection pins moved to a sibling module.
   - Guard coverage: all nine files are registered in
     `mirbuilder_qualified_route_scope_guard.sh`'s 800-line list under
     `MIRBUILDER-EXE-ACCEPTANCE-REVIEW-LINE-BOUNDARY-R0` so the
     boundary cannot silently regrow.

2. **New S8 tests SIGABRT under default `cargo test` (critical)** —
   fixed. The deep source-backed lowering recursion exceeds the
   default 8 MiB test-thread stack; affected pins now run on a
   32 MiB `run_on_test_thread` (the repo's existing pattern):
   - `callable_source_return_in_body_loop_reads_bind_header_phi_dst`
   - `callable_source_loop_exit_read_binds_after_phi_dst`
   - `source_backed_loop_keeps_invocation_scope_and_ledger_route`
   - `source_bool_call_feeds_existing_composite_condition`
   - `merged_parser_program_source_stops_at_named_publication_boundary`
   - `named_array_source_reaches_retained_typed_write_and_c_frame`
   - all 6 `raw_public_cutover_parity_success_p0` tests
   - all 6 `raw_public_ingress_p0` tests
   - all 6 `tests::joinir::mainline_phase49` tests
   Verified: full `--lib` suite completes under a default environment
   with **zero** stack-overflow aborts (was aborting at baseline too —
   verified on stash — the wraps remove that abort surface entirely).
   Production recursion depth is unchanged; the robustness note stays
   visible — lowering recursion in `lower_exit_if_state_core` /
   `lower_callable_loop_source_parts_block` is a known debug-build
   depth risk parked for a dedicated iterative-lowering slice.

3. **Red baseline receipt stale (warning)** — refreshed.
   `cargo_lib_red_baseline.*` re-recorded under the fixed gate
   (quick profile, `--test-threads=1`, RUST_MIN_STACK=16MiB):
   `passed 7869 -> 7981`, `failed 167 -> 127`, inventory 8164.
   Delta vs the Sep-25 receipt: +5 failure names — all verified
   session debt already red at remote HEAD before this slice
   (`loopfacts_ok_none_*` x2, `unarmed_nested_loop_stops_*`,
   `birth_receiver_non_escape_rejects_*`,
   `unsupported_parameter_types_*` — recorded in the post-S14 census);
   -45 failure names now green via S9-S14 repins, corridor and env-pin
   fixes, and the stack wraps. Verifier: `KNOWN BASELINE ... ok`.

4. **Document-lane verify loosening (warning, accepted)** — kept as
   carded D19/S6-S7 decision; no code change in this slice. The
   document lane skipping the Integer-domain call-edge check and
   reclassifying non-Integer-return cataloged calls as
   `UnsupportedBeforeObject` is the accepted boundary; a
   String-returning helper downgrading the whole module is the
   priced-in blast radius documented in D19.

5. **`env` magic receiver in one more owner (warning, noted)** —
   bounded interception of unbound `Variable("env")` in
   `plan_member_call_route`; bound locals still shadow it and unknown
   methods keep a named terminal. No code change.

## Suggestions implemented

- `resolve_named_assignment` no longer inlines
  `Other { kind: "BindingAssignmentTarget" }` — the row is now issued
  by `ShadowBodyShapeDraftV0::record_binding_assignment_target`,
  shared with `record_assignment_target_shape`'s `Variable` arm so
  both call sites keep one vocabulary.
- The name-resolution limitation of
  `publish_source_loop_final_value_named` (diagnostic-name index;
  shadowed same-name bindings freeze) is stated explicitly in the S8
  card's Evidence section.

## Non-claims

- No production behavior change: the splits, visibility widening,
  stack wraps, and shared shape vocabulary are BoxShape moves only.
- The 127-name red list is the accepted baseline, not a fix
  inventory — every failure is owned debt (LEGACY-TESTS-RETIRE-R0,
  loop-planner freeze family, or the five session-debt pins above).
- Gate-1 stays externally blocked per D21; this slice does not claim
  overall MirBuilder completion.

## Evidence

- `cargo test --lib --no-run`: clean (0 errors; warnings are
  pre-existing).
- `env -u RUST_MIN_STACK <test binary> --test-threads=4` (full
  suite): completes — no SIGABRT; residual failures are env-race
  flakes under parallel threads plus the baseline debt recorded in
  the receipt.
- `python3 tools/checks/lib/cargo_lib_red_baseline.py --root .`:
  `KNOWN BASELINE ... ok`.
- `bash tools/checks/mirbuilder_qualified_route_scope_guard.sh`: ok.
- `bash tools/checks/current_state_pointer_guard.sh`: ok.
