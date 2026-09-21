---
Status: closed__2026-09-22__WarningLoopBreakFactsSourceKindField__DeletedAndVerified
Task: MIRBUILDER-WARNING-LOOPBREAK-FACTS-SOURCE-KIND-FIELD-I0
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i122-2026-09-22.md
Implementation permission: true for deleting the unread outer Facts field only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I124
---

# Warning cleanup: unread outer LoopBreak Facts source-kind field

## Six-line brief

```text
Decision: delete the unread `source_kind` field from
VerifiedCallableLoopBreakSourceFactsV1 and its constructor assignment.
Source authority + canonical issuer: the existing LoopBreak source-Facts
owner; the candidate projection retains its own source-kind relation.
Non-authority: warning suppression, cargo-fix, candidate-field deletion,
parser composite promotion, compatibility fallback, or old-edge retirement.
Fail-fast boundary: any field read, compile error, focused red, or warning
count mismatch rejects the deletion.
Smallest next slice: remove exactly the outer field and assignment, then run
the source-Facts focused tests, warning refresh, and pointer guard.
Non-claims: no LoopBreak admission, Recipe, physical consumer, production
switch, or LegacyCallV0 retirement change.
```

## Census and delete set

The selected warning is `dead_code` for the outer field
`VerifiedCallableLoopBreakSourceFactsV1::source_kind` at
`src/mir/builder/normal_callable_loop_source_facts/loop_break.rs:423`.
Repository search found the field's constructor assignment only and no read.
The candidate struct's separate `source_kind` field and projection checks are
outside the delete set. The exact delete set is one field plus one constructor
assignment.

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

The unread outer field and its constructor assignment were deleted. The
candidate-level source-kind field and projection relation remain unchanged.
The focused source-Facts filter executed **16/16** tests. The warning baseline
moved from lib **1,692** to **1,691** and from lib-test **549** to **548**;
the no-run build, formatting, diff, and pointer guard all passed. No semantic
receipt, LoopBreak admission, or compatibility route changed.
