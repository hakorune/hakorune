---
Status: closed__2026-09-22__WarningLoopBreakFactsSourceKind__DeletedAndVerified
Task: MIRBUILDER-WARNING-LOOPBREAK-FACTS-SOURCE-KIND-I0
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i120-2026-09-22.md
Implementation permission: true for deleting the production-zero accessor only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I122
---

# Warning cleanup: unused LoopBreak source-kind accessor

## Six-line brief

```text
Decision: delete VerifiedCallableLoopBreakSourceFactsV1::source_kind; its
repository caller census is declaration-only and the field remains internal.
Source authority + canonical issuer: the existing LoopBreak source-Facts
owner and its source-kind field; no new semantic issuer is introduced.
Non-authority: warning suppression, cargo-fix, field deletion, route changes,
parser composite promotion, compatibility fallback, or old-edge retirement.
Fail-fast boundary: any caller, compile error, focused red, or warning-count
mismatch rejects the deletion.
Smallest next slice: remove the one accessor and run the source-Facts focused
tests, warning refresh, and pointer guard.
Non-claims: no LoopBreak admission, Recipe, physical consumer, production
switch, or LegacyCallV0 retirement change.
```

## Census and delete set

The selected warning is `dead_code` for
`VerifiedCallableLoopBreakSourceFactsV1::source_kind` at
`src/mir/builder/normal_callable_loop_source_facts/loop_break.rs:436`.
Repository search found no call outside the declaration. The enclosing
`source_kind` field is still consumed when the Facts product is constructed
and checked; the candidate projection's source-kind relation remains in use.
The delete set is exactly this accessor body. No field, issuer, candidate
relation, route, or test fixture is part of the slice.

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

The accessor was deleted with no replacement, field change, or route change.
The focused source-Facts filter executed **16/16** tests. The warning baseline
moved from lib **1,693** to **1,692** and from lib-test **550** to **549**;
the same one accessor diagnostic was present in both targets. The no-run
build, formatting, diff, and pointer guard all passed. No semantic receipt,
LoopBreak admission, or compatibility route changed.
