# MIRBUILDER-LIFECYCLE-BASELINE-CONTRACT-PIN-RESYNC-S11

**Row**: `MIRBUILDER-LIFECYCLE-BASELINE-CONTRACT-PIN-RESYNC-S11`
**BoxShape**: test-only resync — repin six lifecycle-module contract pins to
the honest terminals the landed contracts now produce. No production
authority, route selection, Recipe, physicalization, or publication change.
**Mode**: `fast` — one responsibility (contract-pin resync), focused gates.

## Context

The S10 census (`mirbuilder-loop-cond-no-exit-accum-family-disjoint-s10`)
classified the eight lifecycle-module residual reds: two were the
`VariableAccumRecurrenceOverlap` live regression (fixed in S10); the
remaining six are stale pins — each test still asserts a vocabulary, stage,
fronting order, or instruction kind that a deliberately landed contract
changed. All six reproduced identically on parent HEAD before any of this
work.

## Resync table

| Test | Retired premise | Honest current terminal | Fix |
|---|---|---|---|
| `source_backed_loop_keeps_invocation_scope_and_ledger_route` | error contains `[callable-loop/facts-rejected]` | `[freeze:contract][callable-loop/route-not-front-selected] LoopCondRouteRejected(SourceItemsMissing)` — call-free loop bodies bind no method-call source items, so the armed LCBC lane stops at `SourceItemsMissing` | repin the vocabulary; keep the `!callable-ledger-missing` and `current_module.is_some()` scope pins |
| `merged_parser_program_source_stops_at_named_publication_boundary` | fronts `StringHelpers.split_lines/1` at `[Body(5), LoopBody(1), IfThen(0)]` | fronts `ParserStringUtilsBox.i2s/1` at `[Body(4), LoopBody(1), Value, Lhs]` — `"0123456789".substring(...)` is a literal receiver, honestly outside the lexical-local core-method family | repin function + site |
| `actual_string_helpers_general_result_row_reaches_its_first_loop_carrier` | `.expect("… reach GenericLoop")` — compat static box lowers | `[freeze:contract][raw-compat/runtime-box-fate-retired/static]` — `7167aee18e` retired the raw runtime-box-fate lane; a Compatibility root is exactly that lane | repin to `expect_err` + retired terminal |
| `source_bound_static_result_owner_reaches_the_raw_terminal` | `.expect("… must lower")` | same retired freeze — reaching the raw terminal now means the retired freeze itself | repin to `expect_err` + retired terminal |
| `source_backed_app_main_direct_call_consumes_affine_loan` | counts `Call`/`LegacyCallV0` only and expects `CanonicalTyped` route | lifecycle-bearing callees (`helper(value: i64): i64`) route through `Invoke{operation: Call}` per `6b8efa6d8c`/`0acec9467e`; the published backend view classifies Invoke as a lifecycle instruction, so the module honestly reports `UnsupportedBeforeObject` with zero corridor rows | widen the instruction match to include `Invoke{Call}`; repin route to `UnsupportedBeforeObject` and `static_method_calls().is_empty()` |
| `source_backed_package_failure_is_terminal_before_builder_effects` | stage `CallableSemanticSeal` + `_source.is_none()` | stage `RootExpansion` — root-execution consumption rejects the selected-gate program at `[mir/normal-root/consume]` before semantic seal, and retains the `RootExecution` rejected owner (same preflight-retains-source contract as the compatibility arm) | repin stage, assert `normal-root/consume` vocabulary and retained `RootExecution` owner |

## Fail-fast boundary

- Repins assert the *named* terminals only — no silent-fallback paths are
  added, no assertions weakened beyond what the landed contracts produce.
- Each repinned terminal was reproduced on parent HEAD and traced to its
  owning commit before being classified stale (S10 card census table).

## Landed evidence

- `normal_default_root_catalog` module (lifecycle + loop_scope + merged_route
  + variable_accum + map_consumer + final_validation + root_source_handoff):
  **51 passed / 0 failed**.
- VAR lifecycle: 2/2 green (`accepted_variable_accum_callable_completes_source_backed_mir_lifecycle`,
  `late_var_failure_discards_lowered_invocation_and_fresh_call_succeeds` — S10 fix retained).
- No production file changed in this slice; `git diff` touches only the
  three test files plus this card / CURRENT_STATE / workstream.

## Pins

- Focused: `normal_default_root_catalog` module 51/51 green.
- Guards: `mirbuilder_qualified_route_scope_guard.sh`,
  `current_state_pointer_guard.sh` green.

## Non-claims

- Does not change Gate-1 suite state (externally blocked, D21 unchanged).
- Does not assert the retired raw lane is gone — the retired freeze is the
  honest terminal and stays pinned.
- Does not weaken `parser_scan_package`'s no-fallback assertion shape.
