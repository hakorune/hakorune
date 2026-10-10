# MIRBUILDER-STATIC-LOCAL-CALL-BINDINGS-COVERAGE-D0

Status: decided; bounded S0 and later loop design split
Date: 2026-10-11
Scope: unchanged `SizeClassBox` Static local-call binding groups at final package coverage
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-mixed-cohort-root-entry-d0-2026-10-11.md
  - docs/development/current/main/investigations/mirbuilder-bool-currentowner-entry-d0-2026-10-11.md
  - src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_home.rs
  - src/mir/normal_callable_semantic_package/ordinary_new_terminal_access.rs
  - src/mir/normal_callable_semantic_package/install/lowering_port.rs

## Current first stop

The completed Bool entry S0 moves the unchanged SizeClassBox published-view
probe past `accepts/1 borrowed-entry/entry-values-missing`. The final WIP
source still stops at install `IncompleteOrdinaryNewCoverage`, before the
physical ABI callback. The probe uses protected Eq WIP only in a disposable
worktree; no Eq change is included in the production slice.

Disposable-only diagnostics of the same source show
`local_call_bindings_consumed=false` as the sole failing top-level ledger
condition (`/tmp/hako-bool-entry-coverage-diag-20261011.log`). Binding groups
for owner slots 4, 5 and 7 have no `root_exits` entries, while slot 10 has
an emitted exit entry carrying its local groups
(`/tmp/hako-bool-entry-coverage-owner-20261011.log`). Source order maps
those slots to `bin_size`, `size_to_bin`, `good_size` and `accepts`.
`local_call_bindings_consumed` currently requires each group to be attached
to an emitted exit entry or a retained unavailable/finalized exit. The
absence of an exit entry does not itself prove whether the group was never
eligible, lost during issuance, or belongs to a different physical owner.

Read-only owner audit: `terminal_call.rs` is the sole local-call binding
group issuer. The generic `return` in `statement_surface.rs` emits no
`RootHomeExitEntry` and therefore cannot consume its groups. The selected
return path enters `prepare_root_home_exit`, which requires every sibling
exit in the same Completion to have a proven Home row before it can issue
the per-exit entry. `finalized_lexical_call_visit` checks finished Call
producers only after the package preflight; it does not prove per-exit Home
cleanup. Do not bypass the group-consumption gate or force one exit entry
while sibling exits remain unavailable.

The next disposable run confirms each of owner slots 4 (`bin_size`), 5
(`size_to_bin`), and 7 (`good_size`) has a `root_flow`, but
`all_exits_ready=false`; slot 10 (`accepts`) has `all_exits_ready=true`
(`/tmp/hako-bool-entry-coverage-flow-20261011.log`). Thus the missing
entries are a consequence of the whole-owner exit readiness gate, not a
missing `root_flow`. The exact disposable per-exit census is
`/tmp/hako-bool-entry-coverage-exits-20261011.log`:

| Owner | Unready exits | Ready exits |
| --- | --- | --- |
| `bin_size` slot 4 | Body(0)/IfThen(0) and Body(2)/IfThen(0): `ReturnValueNotCovered` for `return -1` | Body(1)/IfThen(0), Body(11) |
| `size_to_bin` slot 5 | Body(2)/LoopBody(0)/IfThen(0): `TerminalNotCovered`; Body(3): `PrefixNotCovered` at the loop | none |
| `good_size` slot 7 | Body(1)/IfThen(0): `ReturnValueNotCovered` for `return -1` | Body(2) |
| `accepts` slot 10 | none | both exits |

The two `return -1` owners share one missing source classifier. The search
loop owner needs a different Completion/Home rule and remains outside S0.

## Decision

```text
Decision: issue one exact negative integer immediate through the existing
  Home terminal IntegerLiteral relation, then allow the already selected
  root Home exit path to consume covered groups for bin_size/good_size.
Source authority: exact Unary Minus(Integer literal) and checked value from
  expression_source::negative_integer_immediate; source site, Completion
  exit and Home flow remain the existing authority. Canonical issuer is
  home_new_prefix terminal/scalar observation, not a new return or Call
  issuer. Physical Plain exit and integer emission are existing owners.
Non-authority: the aggregate IncompleteOrdinaryNewCoverage error, slot order
  alone, and the disposable debug print.
Fail-fast boundary: no unconsumed binding group may be treated as covered;
  overflow, wrong unary shape/site, missing sibling exit, wrong owner,
  partial group, or double consumption retain a named refusal.
Smallest next slice: NEGATIVE-INTEGER-RETURN-HOME-S0, covering exact signed
  immediate return source for unchanged bin_size/good_size. Positive:
  their entire owner exit flow ready and local groups consumed via existing
  Plain/Call exit entries. Negative: wrong unary/value/overflow and sibling
  exit still unavailable refuse. Reuse current focused family and original
  SizeClassBox probe; add only missing independent negative coverage.
Follow-up: SEARCH-LOOP-EARLY-RETURN-HOME-D0 for size_to_bin; its loop
  prefix/terminal failures are not a signed-immediate problem.
Non-claims: no whole SizeClassBox publication, no mixed physical acceptance,
  no Bool physical Invoke, no mimalloc-lite EXE PASS.
```

Audit the `root_local_call_bindings` producer, each relevant Completion exit,
the selected physical exit construction, and the `is_empty()` consumer. Use
the unchanged source plus focused positive/negative evidence. The earlier
mixed S0 package checkpoint remains valid, but its physical acceptance is
open. Do not change the coverage predicate merely to make the probe green.
