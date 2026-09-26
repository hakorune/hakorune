# MIRBUILDER-EXE-ACCEPTANCE-COND-SUBSTRING-COVERAGE-S2

Status: landed__2026-09-26
Date: 2026-09-26
Parent: MIRBUILDER-EXE-ACCEPTANCE-SOURCE-CALL-COVERAGE-D14 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: D14 Decision. `(StringSubstring,2)` at
  `Condition` is a bounded vocabulary gap on the existing
  CoreMethod contract authority — physical consume is
  placement-agnostic; only the two placement pins gate it.

## Slice

1. `src/mir/source_call_target/core_method.rs`
   `supported_placement`: per-(op,arity) single expected
   placement → allowed placement set. `StringLen/0` =
   {Condition}; `StringSubstring/2` = {Body, Condition};
   `ArrayPush/1` = {Body} (unchanged arms otherwise). The
   contract issue passes the candidate loop site's ACTUAL
   resolved placement, not a table constant.
2. `src/mir/resolved_semantics/
   resolver_core_method_callable_contract.rs`
   `required_target_placement`: verify `placement` is in
   the (op,arity) allowed set — keep the same named reject
   vocabulary (`TargetPlacementMismatch` /
   `TargetOperationMismatch`).

## Overlap analysis (required, source-read)

- Body-position `substring` (S6C scan-with-init family)
  keeps `placement == Body` — allowed-set includes it;
  candidates unchanged.
- `StringLen` stays Condition-only; `ArrayPush` stays
  Body-only — no behavior change for either.
- `nearest_loop` semantics unchanged: candidates = loops
  whose resolved placement for the site is in the allowed
  set; ambiguity still `NoUniqueLoopSite`.
- Every contract sub-site (call/receiver/args/result)
  must still resolve to the sealed placement —
  `PlacementMismatch` unchanged.

## Pins (required before close)

- Positive: trim-header fixture (`loop(i<n &&
  s.substring(i,i+1)==" "){i=i+1}`) issues a contract with
  `placement == Condition`; probe covers both condition
  sites; `into_selected_relation` yields a CoreMethod
  relation.
- Positive: body-position substring fixture still
  contracts `placement == Body`.
- Negative: `StringLen` at Body / `ArrayPush` at
  Condition remain unarmed; `TargetPlacementMismatch`
  preserved for drifted claims.
- Guard: extend `mirbuilder_qualified_route_scope_guard.sh`
  — pin the allowed-set vocabulary and the `Condition`
  arm; register touched files (800-line list).
- Real app: json_stream_aggregator EXE advances past
  `SourceCallOutsideSelectedFamily` on `trim/1` — record
  the next honest terminal. VM `ingest/1` stays the
  recorded ledger-less boundary.

## Fail-fast boundary

- No receiver widening — `UnsupportedReceiver` still
  drops qualified/current-owner/other receivers.
- No new (op,arity) arms — `TargetOperationMismatch`
  unchanged for everything outside {StringLen/0,
  StringSubstring/2, ArrayPush/1}.
- `ConditionalUpdateIf` (ingest) and continue-only loops
  remain their existing named boundaries.

## Evidence (landed — filled at close)

- Source arm: `supported_placement` generalized to
  `allowed_placements` (core_method.rs) — `StringLen/0` =
  {Condition}, `StringSubstring/2` = {Body, Condition},
  every other (op,arity) unarmed. Candidate loops carry
  `(loop_site, placement)` so the issued contract seals
  the site's actual resolved placement; `nearest_loop` is
  payload-generic (named-array uses `()`).
- Verify mirror: `required_target_placement` generalized
  to `allowed_target_placements`
  (resolver_core_method_callable_contract.rs) — membership
  check with `ArrayPush/1` = {Body}; named rejects
  (`TargetPlacementMismatch`, `TargetOperationMismatch`,
  `PlacementMismatch`, `UnsupportedReceiver`) unchanged.
- NamedArray: `== Some(Body)` filter keeps `ArrayPush`
  Body-only; a condition-position `push` is skipped
  before any requirement issue.
- Focused pins (cargo test --lib, all green):
  `condition_position_substring_contracts_and_consumes_exact_row`
  (Condition seal + existing header port consume),
  `body_position_length_stays_unarmed`,
  `condition_position_push_stays_unarmed`,
  `resolver_callable_contract_co_seals_condition_substring_and_generated_target`,
  `resolver_callable_contract_rejects_body_length_placement`
  (named `TargetPlacementMismatch`); existing
  `..._co_seals_body_substring_...` still `Body`.
- Test split: `resolver_callable_contract_*` moved to
  `callable_source_ledger_contract_tests.rs` (998-line
  file → 630 + 436, both under the 800 hard stop).
- Guards: S2 section added to
  `mirbuilder_qualified_route_scope_guard.sh` and touched
  files registered in the 800-line list; both guards
  green; rustfmt clean on touched files.
- Real app (debug binary `target/debug/hakorune`):
  - EXE json_stream_aggregator advances past
    `SourceCallOutsideSelectedFamily` on `trim/1` — next
    honest terminal `[mir/callable-semantic-package/port]
    DeclaredInstanceLocatorNotConsumed` at package
    `complete()` (all selected coverage consumed;
    declared-instance call locators unconsumed —
    separate authority family).
  - VM unchanged: `ingest/1` ledger-less spine decline
    (`loop winner selection declined` [Body(2)]) —
    recorded non-claim.

## Non-claims

- Does not fix `ingest` F2 `ConditionalUpdateIf` or its
  VM ledger absence.
- Does not arm static-publication gaps (`binary_trees`,
  mimalloc `seedBlocks` keep their terminals).
- Does not widen receiver vocabulary or add non-StringBox
  core methods.
- Does not touch S6C target plans, the probe coverage
  arithmetic, or the route token contract.
