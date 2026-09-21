---
Status: closed__2026-09-22__WarningLoopBreakCandidateAccessors__DeletedAndVerified
Task: MIRBUILDER-WARNING-LOOPBREAK-CANDIDATE-ACCESSORS-I0
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i124-2026-09-22.md
Implementation permission: true for deleting the five production-zero accessors only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I126
---

# Warning cleanup: unused LoopBreak candidate accessors

## Six-line brief

```text
Decision: delete the five unused accessors on
VerifiedCallableLoopBreakSourceCandidateV1.
Source authority + canonical issuer: the existing LoopBreak candidate owner;
the physical adapter consumes the fields directly after move.
Non-authority: field deletion, projection relation changes, warning
suppression, cargo-fix, parser promotion, compatibility fallback, or old-edge
retirement.
Fail-fast boundary: any accessor caller, compile error, focused red, or
warning-count mismatch rejects the deletion.
Smallest next slice: remove only owner/function_origin/source_kind/outcome/
terminality accessors; retain projection for its focused test caller.
Non-claims: no LoopBreak admission, Recipe, physical ownership, production
switch, or LegacyCallV0 retirement change.
```

## Census and delete set

The selected warning group is the unused accessor impl at
`src/mir/builder/normal_callable_loop_source_facts/loop_break.rs:63-84`:

```text
VerifiedCallableLoopBreakSourceCandidateV1::{
  owner, function_origin, source_kind, outcome, terminality
}
```

Repository search found no call to these five methods. The retained
`projection` method has one focused test caller. The candidate's private
fields are destructured directly by `SourceLoopBreakPhysicalInputV1::from_candidate`
and remain in the product. The delete set is exactly the five method bodies;
no field, constructor, projection check, or physical adapter changes.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib normal_callable_loop_source_facts
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout evidence

Five unused accessors were deleted. The retained `projection` accessor stayed
because `loop_break_source_tests.rs:181` calls it. The focused source-Facts
filter executed **16/16** tests. Lib warnings remained **1,691** because the
retained accessor keeps the diagnostic group; lib-test warnings moved from
**548** to **547**. The no-run build, formatting, diff, and pointer guard
passed. No field, projection relation, semantic receipt, or route changed.
