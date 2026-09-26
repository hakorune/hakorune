# MIRBUILDER-LOOP-COND-NO-EXIT-ACCUM-FAMILY-DISJOINT-S10

**Row**: `MIRBUILDER-LOOP-COND-NO-EXIT-ACCUM-FAMILY-DISJOINT-S10`
**BoxShape**: production fix — restore the designed disjointness between
`loop_cond_no_exit` (LCBC `NoExitBody`) and the callable
`VariableAccumRecurrence` (VAR) family. Test-only changes are excluded and
queued to a sibling resync slice (S11).
**Mode**: `fast` — one responsibility, one production edge, focused gates.

## Context / defect

`a852d2a1bb` (LOOP-COND-NO-EXIT-S0) added `try_extract_loop_cond_no_exit_facts`,
a disjoint sibling extractor wired after `loop_cond_break_continue`, claiming
`loop(cond) { stmt* }` bodies with zero exit signals. Its overlap analysis
audited only route predicates (`LoopSimpleWhile` vs `LoopCondBreakContinue`)
and missed the callable-seam VAR product:

- `apps/tests/loop_simple_while_inline_explicit_step_min.hako`
  (`loop(i < 4) { acc = acc + i; i = i + 1 }`) is the accepted VAR source
  profile (M10b-I0-R0-VAR card: "exclusive ownership"; "its exact
  two-assignment loop body is disjoint"). `b579959a3d` requires
  `selection.matched_routes()` to be empty for `VariableAccumRecurrenceReady`.
- After S0, the same loop produces `LoopCondBreakContinueFacts`
  (`NoExitBody`) → `matched = [LoopCondBreakContinue]` →
  `[freeze:contract][callable-loop/route-not-front-selected]
  VariableAccumRecurrenceOverlap { routes: [LoopCondBreakContinue] }`.
- The freeze is the designed exclusivity guard working correctly; the defect
  is the extractor claiming a loop family that VAR already owns.
  Two lifecycle-module tests fail on this (`accepted_variable_accum_callable_completes_source_backed_mir_lifecycle`,
  `late_var_failure_discards_lowered_invocation_and_fresh_call_succeeds`
  — the latter freezes before its injected fault is ever reached).

## Decision

One meaning, one authority: an accumulator-over-induction recurrence is the
VAR family, exclusively. `loop_cond_no_exit` must defer it, not claim it.

## Source authority + canonical issuer

- `try_extract_loop_cond_no_exit_facts`
  (`src/mir/builder/control_flow/plan/facts/loop_cond_no_exit_facts.rs`)
  gains a bounded family guard: defer bodies matching the recurrence
  profile — exactly two statements, both `v = v <binop> …` self-recurrence
  assignments, distinct targets, the first statement's RHS reading the
  second's target, and the loop condition referencing the second's target.
- New `RejectReason::VariableAccumFamily => "variable_accum_family"`,
  handoff `OutOfScope` (FactsAbsent stays the named facts terminal; the
  callable-seam VAR attempt decides the family upstream).
- `collect_vars_from_expr`
  (`plan/loop_cond/break_continue_helpers.rs`) widened
  `pub(super)` → `pub(in crate::mir::builder)` — same precedent as the
  `matches_parse_string2_shape` /
  `build_loop_cond_break_continue_recipe` widening in S0.

## Non-authority

- No change to `CallableLoopRouteMatchV1`, the overlap guard, VAR
  observation/projection, or the recipe builder. The freeze contract stays:
  a body this guard does not recognise still produces whatever it produced
  before (other facts → overlap, nothing → FactsAbsent).
- No widening of the deferral beyond the exact two-assignment recurrence:
  `loop(i<n){i=i+1}` (S0 positive A), `loop(i<cap){free.push(i); i=i+1}`
  (S0 positive B), and `i=i+1;i=i+1` degenerate same-target bodies stay
  LCBC-claimed.
- Deferred near-family shapes that VAR declines land on `FactsAbsent`
  (honest terminal), never a second lowering path.

## Fail-fast boundary

- The guard never fabricates facts: it returns `Ok(None)`; unmatched
  shapes keep their existing terminals.
- No silent fallback: a VAR-eligible loop that VAR rejects keeps the
  established `variable-accum-recurrence` reject terminal; it does not
  drop to LCBC.

## Smallest next slice

This card's implementation: guard + reason + visibility widen +
extractor unit tests + focused lifecycle regression check + guard-script
pin update.

## Residual disposition census (lifecycle module, baseline reds)

All eight residual reds reproduced identically on parent HEAD (S9 card).
Dispositions:

| Test | Observed terminal | Disposition |
|---|---|---|
| `accepted_variable_accum_callable_completes_source_backed_mir_lifecycle` | `VariableAccumRecurrenceOverlap{[LoopCondBreakContinue]}` | live regression → this slice |
| `late_var_failure_discards_lowered_invocation_and_fresh_call_succeeds` | same overlap freeze (before injected fault) | live regression → this slice |
| `source_backed_loop_keeps_invocation_scope_and_ledger_route` | `[callable-loop/route-not-front-selected] LoopCondRouteRejected(SourceItemsMissing)` | stale pin (`facts-rejected` vocabulary retired; call-free bodies honestly lack bound method-call items) → S11 |
| `merged_parser_program_source_stops_at_named_publication_boundary` | fronts `ParserStringUtilsBox.i2s/1` `SourceCallOutsideSelectedFamily` (literal-receiver `substring`, not a lexical local) | stale pin (fronting order; designed freeze is honest) → S11 |
| `actual_string_helpers_general_result_row_reaches_its_first_loop_carrier` | `[raw-compat/runtime-box-fate-retired/static]` | stale pin (Compatibility root → raw lane deliberately retired by `7167aee18e`) → S11 |
| `source_bound_static_result_owner_reaches_the_raw_terminal` | same retired freeze | stale pin (same) → S11 |
| `source_backed_app_main_direct_call_consumes_affine_loan` | 0×`Call`/`LegacyCallV0`; lifecycle-bearing callee now emits `Invoke{operation:Call}` (`6b8efa6d8c`/`0acec9467e`) | stale pin (instruction-kind filter) → S11 |
| `source_backed_package_failure_is_terminal_before_builder_effects` | stage `RootExpansion` (root-execution consumption rejects selected-gate program before seal stage) | stale pin (stage fronting; exact error vocabulary to be pinned at fix time) → S11 |

## Pins (required before close)

- Positive: `loop(i<4){acc=acc+i; i=i+1}` inside a static-box method →
  `VariableAccumRecurrenceReady` (no LCBC fact minted); the two failing
  lifecycle tests go green.
- Negative A: `loop(i<n){i=i+1}` still produces `NoExitBody` facts
  (extractor unit test).
- Negative B: a two-statement body that is not a self-recurrence pair
  (`free.push(i); i = i + 1`) still produces `NoExitBody` facts.
- Negative C: `i=i+1; i=i+1` (same target) still produces `NoExitBody`
  facts — same-target is not the distinct-binding recurrence.
- Guard: `tools/checks/mirbuilder_qualified_route_scope_guard.sh` pins the
  family guard and new reject reason alongside the existing extractor pins.

## Non-claims

- Does not complete Gate-1; the suite remains externally blocked
  (D21 record unchanged).
- Does not claim VAR handles every recurrence shape; near-family declines
  keep the named `FactsAbsent`/`variable-accum-recurrence` terminals.
- Does not resync the six stale pins; that is S11's responsibility.

## Landed evidence (2026-09-27)

Implementation:
- `reject_reason.rs`: `VariableAccumFamily => "variable_accum_family"`,
  `for_loop_cond_no_exit` maps it to `OutOfScope`.
- `break_continue_helpers.rs`: `collect_vars_from_expr` widened
  `pub(super)` → `pub(in crate::mir::builder)` (same precedent as the S0
  widenings in this file).
- `loop_cond_no_exit_facts.rs`: `claims_variable_accum_family` +
  `self_recurrence_assign` guard inserted after the exit-signal check;
  doc-comment records the VAR disjointness contract; unit tests
  `defers_variable_accum_recurrence_family`,
  `keeps_same_target_self_recurrence`,
  `keeps_induction_free_accum_update`, `pipeline_defers_variable_accum_family`.
- `mirbuilder_qualified_route_scope_guard.sh`: pins the guard, reject
  reason, and new test names.

Focused gates:
- `cargo test --profile quick -p nyash-rust --lib
  mir::builder::control_flow::plan::facts::loop_cond_no_exit_facts::`
  → 13/13 green.
- `cargo test --profile quick -p nyash-rust --lib
  mir::builder::normal_default_root_catalog_lifecycle::variable_accum_tests`
  → 2/2 green (VAR selected once, `(1,0)` counts; injected-fault path
  reaches `failure-after-source-consume` again).
- `cargo test --profile quick -p nyash-rust --lib --
  mir::builder::normal_callable_loop_source_facts::
  mir::builder::normal_callable_loop_source_route` → 23/23 green.
- Full lifecycle module: 45 pass / 6 fail — exactly the six S11 stale pins;
  zero new failures.
- `mirbuilder_qualified_route_scope_guard.sh` → ok.
- `current_state_pointer_guard.sh` → ok.
