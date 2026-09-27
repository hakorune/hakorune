# MIRBUILDER-ADJACENT-MODULE-CONTRACT-PIN-RESYNC-S14

**Row**: `MIRBUILDER-ADJACENT-MODULE-CONTRACT-PIN-RESYNC-S14`
**BoxShape**: test-only resync. Repin stale assertions to the honest
terminals produced by already-landed contracts. Zero production
changes.
**Mode**: `fast` — focused gates only.

## Census

Adjacent-module run after S12/S13 surfaced 22 residual reds. Every one
traces to a landed contract change whose test pin was never updated —
the same disposition class as S9/S11. Live gaps found during the census
were split out and already fixed (S12 Match/QMark draft coverage, S13
Me-Upvar seal arm). What remains here is purely stale pins.

### Group A — `runtime-box-fate` retired boundary (compat lane)

`7167aee18e` retired the raw runtime-box-fate lane for compatibility
boxes; fixtures entering through `compile_with_source`/`compile_normal`
on script/compat sources now hit the honest retired terminal
(`[freeze:contract][raw-compat/runtime-box-fate-retired/{static,instance}]`)
instead of producing the legacy artifact the pins expected.

- `callable_semantic_source` parity tests ×3 (`*_matches_legacy*`):
  repin to `expect_err` + retired-terminal vocabulary, preserving the
  semantic-source assertions that precede the physical boundary.
- `normal_script_runtime_work` ×4 and `normal_script_record_literal`
  ×3: repin expected diagnostics to the retired terminal.

### Group B — dynamic-lane fixture recut

`49b502da41` (2026-08-11) intentionally recut
`production_skip_while_*` fixtures from dynamic to mixed-typed
carriers; the two tests still assert the old dynamic-local inventory:

- `production_skip_while_*` ×2: repin fixtures/assertions to the
  mixed-typed contract the app actually exercises (or point the
  dynamic assertion at a synthetic dynamic fixture if one is needed).

### Group C — narrow admission / terminal renames

- `binding_rebind` ×3: `9a55f1e0ca` (typed pure Script parser
  admission) deliberately narrowed the admitted Script statement
  vocabulary; statement-level rebind forms now fail parse admission.
  Repin to parse-admission negative pins; do not re-broaden admission.
- enum tests ×3: enum declaration handling lives in the selected
  (package-bearing) lane; the compat `compile_normal(for_mir_mode)`
  path declines `EnumDeclaration` per the ongoing compat retirement.
  Repin to `expect_err` + unsupported-node terminal.
- `qualified_call_map_argument...`: `8ba705505e` corroborated-edge
  binding reclassified the terminal; repin expected variant to the
  observed `BorrowedEntryEscape`.
- `main_static_child` `IncompleteOrdinaryNewCoverage`: drain the
  ordinary-new claim ledger before `complete()` per the claim-drain
  contract.
- `accepted_vocabulary_is_closed_and_reviewable`: `0ac2b93e52` added
  `"Program"` to `SHADOW_ACCEPTED_STATEMENTS_V0`; add it to the pin.
- `forest_parent_rejects_unsupported_ancestry_and_orphan_scope`: same
  `IfThen`-supported-ancestry drift as S9 (`440c16216d`); move the
  fixture's unsupported ancestor to a still-unsupported segment.

## Pins

- Each touched test green under its new honest pin.
- `mir::resolved_semantics` module 348/348 green.
- Touched builder test modules green.
- No production file modified (`git diff` shows test files only).

## Non-claims

- Does not change any contract, admission breadth, or seal invariant.
- Does not reopen Gate-1, Gates 2-4, or the retired compat lane.
- Binding-rebind/enum narrowing stays as designed; this batch never
  re-broadens admission to satisfy a stale pin.
