---
Status: closed__2026-09-22__WarningBaselineRefreshI124__SelectedLoopBreakCandidateAccessors
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I124
Date: 2026-09-22
Parent: mirbuilder-warning-loopbreak-facts-source-kind-field-i0-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-LOOPBREAK-CANDIDATE-ACCESSORS-I0
---

# MirBuilder warning baseline refresh I124

## Six-line brief

```text
Decision: refresh the warning surface and select exactly one bounded row;
keep the old-edge lane parked unless a new caller-zero proof exists.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the existing LoopBreak retirement card.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
or a new semantic receipt without a named owner.
Fail-fast boundary: a new warning, unclassified red, nonzero old-edge caller,
or missing absence guard stops selection and records NoSafeSlice.
Smallest next slice: census one caller-zero old-edge candidate, or one
production-zero warning item only when that deletion is proven finite.
Non-claims: no broad warning cleanup, parser semantic change, VM repair, or
LegacyCallV0 retirement beyond one explicitly selected slice.
```

## Baseline and old-edge boundary

I123 deleted the unread outer LoopBreak Facts field and recorded lib **1,691**
and lib-test **548**. The I120 caller census remains binding: compatibility
registry, ledger-free legacy, shared `lower_loop_or_freeze_v1`, and parser
composite successor evidence still prevent old-edge deletion.

The refreshed diagnostics remain lib **1,691** and lib-test **548**. The
selected finite warning group is the five production-zero accessors on
`VerifiedCallableLoopBreakSourceCandidateV1` at
`normal_callable_loop_source_facts/loop_break.rs:64-84`:
`owner`, `function_origin`, `source_kind`, `outcome`, and `terminality`.
The `projection` accessor has one focused test caller and is retained.
Repository search found no other call to the five selected accessors; the
candidate fields themselves remain consumed directly by the physical adapter.
I124 therefore selects deletion of these five accessor bodies only. The
old-edge `NoSafeSlice` remains unchanged.

```text
cargo check --profile quick --lib -j4          passed; lib 1,691 warnings
cargo test --profile quick --lib --no-run -j4  passed; lib-test 548 warnings
```

The next card owns the five-method deletion and its focused source-Facts
acceptance. Do not convert the old-edge state into a source disposition.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record the complete selected owner/caller inventory and keep the next row
bounded to one physical responsibility.
