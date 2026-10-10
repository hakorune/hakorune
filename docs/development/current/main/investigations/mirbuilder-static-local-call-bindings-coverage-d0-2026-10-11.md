# MIRBUILDER-STATIC-LOCAL-CALL-BINDINGS-COVERAGE-D0

Status: design_stop; source/physical owner audit required
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

## Decision required

```text
Decision: pending read-only owner audit.
Source authority + canonical issuer: determine from the original Static
  call rows, verified Completion/exit flow and existing local-call binding
  group issuer; do not create a second call or exit authority.
Non-authority: the aggregate IncompleteOrdinaryNewCoverage error, slot order
  alone, and the disposable debug print.
Fail-fast boundary: no unconsumed binding group may be treated as covered;
  missing exit, wrong site/owner, partial group, or double consumption must
  retain a named refusal.
Smallest next slice: decide whether these eligible Static exits must issue
  the existing exit entry, or whether an existing sole physical owner must
  consume the group without it; name exact production callers and tests.
Non-claims: no whole SizeClassBox publication, no mixed physical acceptance,
  no Bool physical Invoke, no mimalloc-lite EXE PASS.
```

Audit the `root_local_call_bindings` producer, each relevant Completion exit,
the selected physical exit construction, and the `is_empty()` consumer. Use
the unchanged source plus focused positive/negative evidence. The earlier
mixed S0 package checkpoint remains valid, but its physical acceptance is
open. Do not change the coverage predicate merely to make the probe green.
