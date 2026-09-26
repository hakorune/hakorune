# MIRBUILDER-EXE-ACCEPTANCE-COMPOSITE-RETURN-EXIT-S0

Status: landed__2026-09-26
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D12 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: D12 Decision. The composite LoopBreak lane is the
  sole physical owner for `loop(cond){…return…}`; the resolver
  exit ledger → projection → candidate → parts lowering chain
  already carries `Return` end-to-end.

## Slice

1. `src/mir/compiler/loop_break_composite_source_projection.rs`
   `validate_exit_ledger`: count `ResolvedControlTransferV1::
   Return{..}` exits under the root as root control evidence in
   addition to root-targeted `Break`. Rename the reject
   `RootBreakMissing` -> `RootExitMissing` (the invariant is
   now "root carries explicit exit evidence").
2. `src/mir/builder/normal_callable_loop_source_facts/
   loop_break.rs`: update the tolerated composite reject arm to
   the renamed `RootExitMissing` (no behavior change — an
   exit-free root still yields no candidate).

## Overlap analysis (required, source-read)

- `forest.exits()` already collects
  `function.resolved_exits()` under the root — `ExplicitReturn`
  records are present today; only `ExplicitBreak`+`Break{root}`
  were counted.
- `issue_statement_role` (`loop_break_composite_body_role.rs:
  418-443`) maps `Return` stmts to `Exit{record}` and accepts
  `Return{..}` transfers unconditionally — no role change
  needed.
- `build_body_block` (`composite.rs`) emits
  `RecipeItem::Exit{ExitKind::Return}` and
  `IfV2{ExitOnly{ExitIf}}` for `if{return}`; loop root is
  `LoopV0{WhileLike, body_contract: ExitAllowed}` — unchanged.
- `(ExitAllowed, OpaqueExit)` -> `lower_opaque_exit` ->
  `CoreExitPlan::Return` (`parts/exit.rs`) — unchanged.
- `CallableLoopSoleFamilyV1`, winner spine, and the retired
  `features/*_pipeline`/`loop_cond_unified` are untouched —
  composite is consumed before the generic route issuer runs.

## Pins (required before close)

- Positive (projection unit): `loop(k<m){if{return 0};
  k=k+1}` fixture issues a composite projection — exits seen,
  `RootExitMissing` not returned; `issue_composite_source_
  candidate_v1` produces a non-empty recipe.
- Positive (existing): `composite_projection_retains_nested_
  loop_and_root_exit` + `single_structured_projection_is_
  admitted_by_composite_owner` unchanged (break roots still
  admit).
- Negative: an exit-free `loop(cond){i=i+1}` fixture still
  returns `RootExitMissing` (no candidate, typed decline path
  preserved).
- Guard: extend `mirbuilder_qualified_route_scope_guard.sh` —
  pin the Break|Return counting, `RootExitMissing`, the new
  test names; register touched files in the 800-line list.
- Real app: json_stream_aggregator advances past
  `loop winner selection declined` for `starts_with` — record
  the next honest terminal.

## Fail-fast boundary

- Exit-free roots (no root-Break AND no Return) still fail
  `RootExitMissing` — `ingest`/`trim` loops unchanged, next
  card material.
- Continue-only loops are NOT admitted (no physical evidence
  probed for continue-only roots in this slice).
- Return transfers are accepted unconditionally by the role
  map today (`target_function` is always known); a return
  whose record is absent still fails `MissingExit`.

## Evidence (landed)

- Positive unit: `return_in_body_projection_is_admitted_by_
  composite_owner` — `loop(k<m){if{return 0}; k=k+1}` issues a
  composite projection (1 loop member, 1 exit, control-bearing
  roles) and `issue_composite_source_candidate_v1` produces a
  candidate. 6/6 projection tests green.
- Negative unit: `exit_free_loop_declines_with_root_exit_
  missing` — exit-free `loop(cond)` still returns
  `RootExitMissing` (typed decline, no candidate).
- Guard: `mirbuilder_qualified_route_scope_guard.sh` S0
  section green — pins `Return { .. }` counting in
  `validate_exit_ledger`, `RootExitMissing`, tolerate arm, and
  both test names; touched files registered in the 800-line
  list.
- Real app EXE (`NYASH_BIN=target/debug/hakorune`,
  pure-first): `StringHelpers.starts_with/3` return-in-body
  loop lowers — the compile advances to the next honest
  terminal
  `[plan/freeze:contract] loop winner selection declined:
  zero selected family candidates fn=StringHelpers.trim/1
  site=[Body(2)]` — an exit-free `loop(i<n && (…))` (no
  explicit exits; D12 non-claim boundary, next card).
- Real app VM (debug binary): advances to
  `fn=JsonStreamAggregator.ingest/1 site=[Body(2)]`
  `loop(start < n)` — exit-free, ledger-less lane; same
  expected boundary as before the slice.
- EXE suite (debug binary): 4 pass / 8 fail —
  `binary_trees_exe` verified baseline-identical via
  stash+rebuild (`route-not-front-selected` /
  `SourceCallOutsideSelectedFamily`, `iterationCheck` exit-free
  loop — no composite candidates before or after this slice).
  Remaining fails are untouched-lane authorities
  (`NamedArray`, `UnsupportedDeclaredType`,
  `SourceCallOutsideSelectedFamily`, LLVM driver/toolchain,
  `return_type_strategy` panic — prior session verified as
  baseline debt).
- Verification correction: an earlier run of the EXE suite
  without `NYASH_BIN` used the stale 9/25 `target/release`
  binary and reported `main-import-view/selected-header-
  missing`; that terminal is a stale-binary artifact, not a
  reachable state of this build. All recorded evidence uses
  `NYASH_BIN=target/debug/hakorune`.
- Kept diagnostics: `router.rs` `Declined` now reports
  `fn=`, `site=`, `cond=` on winner-spine decline — contract
  error context only, no eprintln.

## Non-claims

- Does not fix `ingest`'s ledger absence (separate ownership
  question).
- Does not extend `sole_family` / spine vocabulary.
- Does not resurrect the retired ReturnInBody pipeline.
- Does not claim `StringHelpers`/`json_stream_aggregator`
  green.
