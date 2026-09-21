---
Status: closed__2026-09-22__WarningBaselineRefreshI122__SelectedLoopBreakFactsField
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I122
Date: 2026-09-22
Parent: mirbuilder-warning-loopbreak-facts-source-kind-i0-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-LOOPBREAK-FACTS-SOURCE-KIND-FIELD-I0
---

# MirBuilder warning baseline refresh I122

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
production-zero warning accessor only when that deletion is proven finite.
Non-claims: no broad warning cleanup, parser semantic change, VM repair, or
LegacyCallV0 retirement beyond one explicitly selected slice.
```

## Baseline and old-edge boundary

I121 deleted one production-zero LoopBreak Facts accessor and recorded lib
**1,692** and lib-test **549**. The old-edge census from I120 remains binding:
the compatibility registry, ledger-free legacy route, shared
`lower_loop_or_freeze_v1` caller, and parser composite successor still prevent
`MIR-RETIRE-FIRST-OLD-EDGE-R0` from being selected without new evidence.

The refreshed diagnostics identify one finite follow-up: the
`VerifiedCallableLoopBreakSourceFactsV1::source_kind` field at
`loop_break.rs:423` is assigned but never read. Its candidate-level
source-kind relation is a distinct, still-retained field. I122 selects only
the outer Facts field and its constructor assignment; no semantic issuer or
candidate relation changes. The old-edge `NoSafeSlice` remains unchanged.

```text
cargo check --profile quick --lib -j4          passed; lib 1,692 warnings
cargo test --profile quick --lib --no-run -j4  passed; lib-test 549 warnings
```

The next card owns the field deletion and its focused source-Facts acceptance.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record the lib/lib-test warning counts and the complete selected owner/caller
inventory. The next implementation card must state its exact delete set,
source authority, non-claims, and focused acceptance before code changes.
