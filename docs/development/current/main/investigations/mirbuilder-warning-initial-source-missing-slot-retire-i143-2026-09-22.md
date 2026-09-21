---
Status: closed__2026-09-22__WarningInitialSourceMissingSlotRetireI143
Task: MIRBUILDER-WARNING-INITIAL-SOURCE-MISSING-SLOT-RETIRE-I143
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i142-2026-09-22.md
Implementation permission: true; one declaration-only reject variant
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I144
---

# MirBuilder warning initial-source missing-slot retirement I143

## Six-line brief

```text
Decision: delete MissingProgramSlotSet from the initial callable-source
  reject enum because the slot-set input is mandatory at the issuer boundary.
Source authority + canonical issuer: issue_initial_callable_program_source_v1
  and its required ProjectedProgramItemSlotSetV1 input.
Non-authority: warning text, default/empty slot fabrication, or a new reject
  path that weakens the source admission contract.
Fail-fast boundary: any source/reference/constructor, changed match coverage,
  focused red, or a new warning caused by the deletion.
Smallest next slice: remove the unreachable enum variant and run the existing
  initial-callable-source tests and quick warning gates.
Non-claims: no parser admission change, fallback, semantic receipt, backend
  change, or LegacyCallV0 retirement.
```

## Finite census and delete set

Boundary: `InitialCallableProgramSourceRejectV1` declaration ->
`issue_initial_callable_program_source_v1`.

`rg` census found exactly one occurrence of `MissingProgramSlotSet`, at the
enum declaration. There are no constructors, match arms, tests, active
fixtures, or current docs references. The issuer already requires a
`&ProjectedProgramItemSlotSetV1`; it cannot represent a missing slot set.

Delete exactly the one enum variant. Preserve all other typed reject reasons
and the existing required slot-set parameter.

## Acceptance

- existing `initial_callable_program_source` tests remain green;
- quick library check and test-binary build record measured warning counts;
- fmt, pointer guard, and diff check pass;
- qualified absence confirms no `MissingProgramSlotSet` reference remains;
- no `#[allow]`, fallback, or test deletion is introduced.

## Closeout receipt

The unreachable `MissingProgramSlotSet` variant was deleted. The required
`ProjectedProgramItemSlotSetV1` issuer input and every remaining typed reject
reason are unchanged. Qualified source/tools census is empty for the deleted
variant.

Measured local evidence:

- `initial_callable_program_source` focused suite: **6/6**;
- `cargo fmt --all -- --check`: **pass**;
- `CARGO_BUILD_JOBS=4 cargo check --profile quick --lib`: **pass**, lib
  warnings **1,675** (I142 baseline 1,676);
- `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib --no-run`: **pass**,
  lib-test warnings **543**;
- `current_state_pointer_guard.sh`: **pass**; `git diff --check`: **pass**.

No parser admission behavior or fallback path changed.
