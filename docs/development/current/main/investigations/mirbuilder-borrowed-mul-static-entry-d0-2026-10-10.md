# MIRBUILDER-BORROWED-MUL-STATIC-ENTRY-D0

Status: active design stop
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
borrowed-formal definition. The strongest source candidate is
`SizeClassBox.bin_size(bin)`'s `bin * me.word_size()` at source line 25,
because `bin` is the borrowed formal; the current error string omits owner
and site, so the candidate is not yet an accepted exact-site finding.

The current static transport seed accepts qualified observations or a single
CurrentOwner i64 initializer, with no other nonqualified sibling for the
callee. `bin_size` also has calls in `size_to_bin`, `good_size`, and
`bin_size_usize`. The existing target-scoped two-caller source cohort is for
`size_to_bin`; its source-shape test cannot be reused as `bin_size` authority.
The detached Eq WIP is not part of this D0 and must remain protected.

## Decision brief to close

```text
Decision: pending exact-site diagnosis and complete bin_size incoming closure.
Source authority + canonical issuer: source-owned Mul site and retained
  BorrowedFormalUses draft; executable entry must be issued from the
  complete incoming call/actual and formal contract evidence, with the
  current selected callable/transport owner.
Non-authority: the freeze string alone, the size_to_bin two-caller cohort,
  source-only inventory, a successful isolated operand test, and caller names.
Fail-fast boundary: retain source-only-entry for any unsupported incoming
  caller, missing actual/formal proof, mixed entry profile, or ambiguous site.
Smallest next slice: diagnose the exact Mul owner/site, census every incoming
  bin_size call and its actual/profile, then select one bounded issuer or
  preserve NoSafeSlice with a concrete reopen condition.
Non-claims: no Bool physical result, Eq physical completion, executable
  bin_size entry, whole SizeClassBox publication, or mimalloc-lite EXE PASS.
```

The read-only worker independently confirmed the source-only materialization
gate and corrected an initially tempting but false analogy: the existing
two-caller cohort belongs to `size_to_bin`, not `bin_size`. Do not broaden
the global transport seed or accept source-only definitions unconditionally.

Acceptance for this D0 is one owner/site diagnosis, an all-caller table with
source location, actual origin, formal class and existing claim/packet owner,
and a single issuer decision with positive/negative acceptance for the first
execution slice. No implementation is authorized by the current design stop.
