# MIRBUILDER-BORROWED-MUL-STATIC-ENTRY-D0

Status: D0 accepted; selected MIRBUILDER-BORROWED-MUL-STATIC-ENTRY-S0
Date: 2026-10-10
Scope: exact source and incoming authority for `borrowed-mul/source-only-entry`
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-bool-literal-terminal-d0-2026-10-10.md
  - lang/src/hako_alloc/memory/size_class_box.hako
  - src/mir/normal_callable_semantic_package/ordinary_new_borrowed_mul_materialization.rs
  - src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_source_drafts.rs

## Observed first stop

At `597d844b31`, the unchanged `size_class_box.hako` published-view probe,
using protected uncommitted Static Eq work copied into detached diagnostic
checkout `/mnt/workdisk/hako-bool-eq-integration-probe-20261010`, moved from
`ordinary-new/local-commit/root-exit-source-missing` to
`ordinary-new/borrowed-mul/source-only-entry`. This is a red integration
observation, not publication acceptance. The Bool literal source relation S0
is complete; Bool physical result and call transport remain open.

`prepare_borrowed_mul_source_v1` emits this refusal only when an exact Mul site
is retained in `source_only_definitions` but its owner lacks an executable
borrowed-formal definition. A temporary env-gated diagnostic in the detached
checkout identified owner slot 4 and exact site
`[Body(1), IfThen(0), Value]`: `SizeClassBox.bin_size(bin)`'s
`bin * me.word_size()` at source line 25. The diagnostic was removed after the
red focused probe; the original WIP files were not changed by it. Build took
5m15s, test execution 0.01s; this is compiler build cost, not suite cost.

The same original incoming inventory held these three successful rows for
the selected probe. Each targets the same `StaticBoxMethod bin_size/1`,
target slot 4, `OpaqueHandle` formal binding 0, and CurrentOwner `ExactI64`
result claim with required integer argument `[0]`:

| Caller | Exact call site | Source actual origin | Existing source/finish owner |
| --- | --- | --- | --- |
| `size_to_bin/1` | slot 5 `[Body(2), LoopBody(0), IfCondition, Rhs]` | loop-local `bin`, candidate `Scalar(binding 2, Integer)` | `StaticIncomingSourceV1`; existing loop-scalar source/packet path owns per-iteration physical read |
| `good_size/1` | slot 7 `[Body(2), Value]` | local `bin` from `me.size_to_bin(size)`, candidate `Scalar(binding 1, Integer)` | `StaticIncomingSourceV1`; caller's result-local source and canonical physical read still need joining |
| `bin_size_usize/1` | slot 9 `[Body(0), Value]` | annotated `usize` formal `bin`, candidate `Scalar(binding 0, Integer)` | `StaticIncomingSourceV1`; declared scalar formal and canonical physical read still need joining |

An env-gated second diagnostic at `prepare_static_source_actuals_v1` observed
these exact `Scalar(_, Integer)` candidates; `good_size` and
`bin_size_usize` were visited twice but retained the same source site and
binding. That build took 5m17s, test 0.01s, and remained red at the same
source-only gate. The diagnostic was removed. Candidate classification is
source evidence, not an executable actual or physical ValueId.

The current static transport seed accepts qualified observations or a single
CurrentOwner i64 initializer, with no other nonqualified sibling for the
callee. The existing target-scoped two-caller source cohort is for
`size_to_bin`; its source-shape test cannot be reused as `bin_size` authority.
`StaticOneInputFinishV1`, its source projection and the target entry view
currently admit only one or that exact two-caller cohort; the finisher also
requires a `Forwarded` opaque actual. The three `bin_size` actual sources
must be joined at their existing provenance owners before entry is issued.
The detached Eq WIP is not part of this D0 and must remain protected.

## Decision

```text
Decision: extend the existing target-scoped Static cohort/actual/entry owner
  from the fixed two-caller source profile to one complete selected incoming
  set. Its cardinality is inventory-derived, not a new three-caller grammar.
  Each admitted Scalar Integer caller must finish its own actual against the
  same formal, result, signature and Completion before the callee entry can
  borrow the original Mul source. The existing two-caller forward cohort
  retains its own stricter path; no global transport-owner promotion occurs.
Source authority + canonical issuer: the original `StaticIncomingSourceV1`
  rows and `BorrowedIncomingInventoryV1::project` own the all-caller set;
  `BorrowedCallActualCandidateV1` and retained `SourceStatic` identity own
  each scalar's source class/site; the existing static one-input finisher
  joins those with the physical signature, result and Completion; the
  existing borrowed-entry owner view issues the sole executable entry.
  `BorrowedMulSourceV1` and its checked Normal guard remain the operand owner.
Non-authority: the freeze string, caller/variable names, hardcoded cohort
  size, the `size_to_bin` two-caller cohort, SourceStatic candidates alone,
  and a scalar's runtime payload without its exact source/physical read.
Fail-fast boundary: unsupported/vetoed incoming caller, wrong target/formal,
  duplicate or missing sibling, Bool/Unknown/noninteger candidate, or absent
  Completion/signature leaves the entry source-only. A failed canonical
  physical read rejects later publication independently.
Smallest next slice: MIRBUILDER-BORROWED-MUL-STATIC-ENTRY-S0, using the one
  inventory-derived cohort and same finisher/entry owners to complete the
  three original Scalar Integer incoming sites. Preserve the prior two-caller
  forward tests. Do not change the `.hako` source or issue a parallel carrier.
Non-claims: no Bool physical result, Eq physical completion, executable
  bin_size entry before S0 tests, whole SizeClassBox publication, or
  mimalloc-lite EXE PASS.
```

The read-only worker independently confirmed the source-only materialization
gate and corrected an initially tempting but false analogy: the existing
two-caller cohort belongs to `size_to_bin`, not `bin_size`. Do not broaden
the global transport seed or accept source-only definitions unconditionally.

S0 positive acceptance is the unchanged `size_class_box.hako` focused
published-view probe moving past `borrowed-mul/source-only-entry`, with the
three exact original call sites and one completed callee entry. Its negative
acceptance changes one caller candidate to Bool/Unknown, removes one sibling
actual, or drifts source target/site: none may leave a partly executable
cohort. Reuse the existing static source/finish/packet tests and the focused
probe; add only an unrepresented scalar all-caller case. The independent
physical ValueId/final MIR checks remain mandatory downstream.
