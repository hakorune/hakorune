# MIRBUILDER-BORROWED-MUL-STATIC-ENTRY-D0

Status: D0 accepted; selected MIRBUILDER-BORROWED-MUL-STATIC-ACTUAL-ENTRY-S0
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
The selected loop-body caller's detached physical packet is built after the
static actual finisher. A finisher cannot require that later packet as its
own entry prerequisite. This order makes source-cohort admission and
executable actual/entry finishing separate slices; final publication still
requires the packet's independent canonical physical proof.
An attempted source-only scalar cache passed a focused positive/mixed-route
test and 12 static-source family cases after its initializer-only flag was
separated; the existing two-caller tests also passed. It was **discarded
before commit** because `static_incoming_cohort_v1` already projects the
complete original caller set, including vetoes. Retaining a second map would
duplicate a source proof without moving the real first stop. The final code
diff contains no scalar-cache implementation or extra test.
The detached Eq WIP is not part of this D0 and must remain protected.

## Decision

```text
Decision: the complete source cohort is already available through
  `static_incoming_cohort_v1`; do not retain another map. Finish the
  inventory-derived CurrentOwner Scalar Integer actual cohort atomically in
  the existing borrowed-formal ledger, after source candidate, Completion,
  result contract and physical signature are available. Lend the callee entry
  only from that completed cohort. The Mul source-only gate accepts only
  this checked entry for its exact owner/site; it does not promote the global
  transport set. The loop's later physical packet remains a separate final
  publication obligation.
Source authority + canonical issuer: the original `StaticIncomingSourceV1`
  rows and `BorrowedIncomingInventoryV1::project`/`static_incoming_cohort_v1`
  own the all-caller set. `BorrowedCallActualCandidateV1`/`SourceStatic` own
  exact scalar source/site. `StaticOneInputFinishV1` currently owns finished
  input evidence but only supports one or a special two-caller forwarding
  cohort; preserve that path. A sibling same-ledger scalar finisher issues
  completed scalar actuals from the original inventory. `borrowed-entry`
  checks the full completed cohort before entry loan. `BorrowedMulSourceV1`
  remains the guarded operand owner. The detached loop packet and canonical
  binding read stay with the later physical owner.
Non-authority: the freeze string, caller/variable names, hardcoded cohort
  size, the `size_to_bin` two-caller cohort, SourceStatic candidates alone,
  and a scalar's runtime payload without its exact source/physical read.
Fail-fast boundary: unsupported/vetoed incoming caller, wrong target/formal,
  duplicate or missing sibling, or mixed Forwarded/Scalar route mints no
  scalar cohort. Wrong source site/binding/ordinal, Bool/Unknown/noninteger
  candidate, or absent/mismatched Completion, result or signature leaves the
  later entry source-only. A failed canonical physical read rejects final
  publication independently.
Smallest next slice: `MIRBUILDER-BORROWED-MUL-STATIC-ACTUAL-ENTRY-S0` joins
  the existing full source projection with each exact Scalar Integer
  candidate and Completion/signature/result, atomically finishes all actuals,
  lends only the checked entry and moves the unchanged probe past
  `borrowed-mul/source-only-entry`. Keep the existing one/two-caller
  Forwarded path green and the later physical packet check independent.
Non-claims: no Bool physical result, Eq physical completion, executable
  bin_size entry before S0 tests, whole SizeClassBox publication, or
  mimalloc-lite EXE PASS.
```

The read-only worker independently confirmed the source-only materialization
gate and corrected an initially tempting but false analogy: the existing
two-caller cohort belongs to `size_to_bin`, not `bin_size`. The worker also
confirmed that source scalar preparation precedes actual finishing, while the
loop's detached physical packet and canonical read are checked later. Use
`source_incoming.project` to check the complete veto-aware set; if the entry
view needs borrowed rows, derive their references from that same inventory's
`exact_rows` after projection rather than retaining a second cohort map.
Do not broaden the global transport seed or accept source-only definitions
unconditionally.

The next accepted slice must retest the unchanged published-view probe and
move past `borrowed-mul/source-only-entry`, while retaining a negative
missing or noninteger sibling. Existing two-caller forward tests must stay
green. Physical ValueId/final MIR checks remain independent.
