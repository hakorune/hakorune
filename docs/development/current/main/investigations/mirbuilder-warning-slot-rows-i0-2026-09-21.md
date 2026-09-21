---
Status: closed__2026-09-21__WarningSlotRows__DeletedAndVerified
Task: MIRBUILDER-WARNING-SLOT-ROWS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i112-2026-09-21.md
Implementation permission: true for the production-zero slot-row borrow accessor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I114
---

# Warning cleanup: redundant projected-slot row borrow

## Six-line brief

```text
Decision: delete ProjectedProgramItemSlotSetV1::rows; retain the existing
into_rows ownership transfer used by the production consumer.
Source authority + canonical issuer: src/parser/build_cfg/program_item_slots.rs;
the projection owner seals the slot set and the normal source consumer owns rows.
Non-authority: warning suppression, cargo-fix, slot-row schema changes, or a
new placement projection.
Fail-fast boundary: any production caller, focused parser red, or warning-count
mismatch rejects the row.
Smallest next slice: change one projection test to consume into_rows, delete
one accessor, and run the BuildGate focused gate plus warning checks.
Non-claims: no slot semantics, source-path changes, AST rewrite, fallback,
route switch, or LegacyCallV0 retirement.
```

## Census boundary

The bounded inventory is `ProjectedProgramItemSlotSetV1::rows`, its two test
callers in `build_cfg/projection_tests.rs` and
`source_seal_finalizer_tests.rs`, and the existing production `into_rows`
consumer. It excludes `OpenProjectedProgramItemSlotsV1`, row validation, and
all unrelated parser `rows` accessors.

## Caller census and acceptance

`rg` shows no production caller of the borrow accessor. The two tests now use
`into_rows` or the existing `exact_final_slot` contract and retain their slot
and source-path checks.
The focused gate is:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib parser::build_cfg
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Expected warning refresh: lib **1,697 → 1,696** and lib-test **551**
unchanged. No `#[allow]`, slot-row schema change, or unrelated parser change
is allowed.

## Closeout evidence

The redundant borrow accessor was deleted. The projection test now consumes
the existing `into_rows` owner, while the source-seal finalizer test uses the
existing `exact_final_slot` contract; both retain their placement assertions.
The focused BuildGate parser family passed **10/10**. Sequential validation:

```text
cargo check --profile quick --lib -j4          -> lib warnings 1,696
cargo test --profile quick --lib --no-run -j4  -> lib-test warnings 551
cargo fmt --all -- --check                     -> pass
git diff --check                                -> pass
bash tools/checks/current_state_pointer_guard.sh -> pass
```

No slot semantics, source path, fallback, or route behavior changed. The next
action is a warning baseline refresh before selecting another cohort.
