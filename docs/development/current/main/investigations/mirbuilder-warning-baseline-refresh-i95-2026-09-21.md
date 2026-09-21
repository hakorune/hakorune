---
Status: closed__2026-09-21__WarningBaselineRefreshI95__SelectedBranchCountFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I95
Date: 2026-09-21
Parent: mirbuilder-warning-source-build-gate-raw-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-ADMITTED-REGISTRY-BRANCH-COUNT-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I95

## Six-line brief

```text
Decision: refresh the post-source-gate-facade warning surface and select one
bounded row; keep the LoopBreak old-edge lane parked without a successor.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the current LoopBreak owner census.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or a guessed
external/source caller.
Fail-fast boundary: new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,709/552, census one candidate, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad cleanup, LoopBreak deletion, publication, VM repair, or
LegacyCallV0 retirement.
```

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record the warning class, owner, cfg/production role, and caller inventory.
Keep `admitted_registry::branch_count` and the live compatibility route in
their existing owner rows. Select at most one implementation row, or record
`NoSafeSlice`, before changing code.

## Handoff

I95 closed the test-only SourceBuildGate raw projection at lib **1,709** and
lib-test **552** with source-session tests **6/6**. The old-edge deletion lane
remains blocked because `route_loop_break_recipe` is still the live registry
compatibility handler; a future deletion row requires a named successor and
caller-zero proof.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,709** warnings and lib-test
**552** warnings. The old-edge census remains `NoSafeSlice`: the live
compatibility registry entry still has no named successor. The warning
`AdmittedTextScanRegistryV1::branch_count` is a separate test observation
accessor. Its repository census also found the unused production-side
`PreparedAotExecutableAdmissionV1::registry_branch_count` facade, which is
called only by a test. The two accessors are therefore one coupled test-only
chain. Other `branch_count` symbols in the interpreter, MIR facts, and JSON
metadata are independent production or reporting data and remain untouched.

The next bounded cohort is
`MIRBUILDER-WARNING-ADMITTED-REGISTRY-BRANCH-COUNT-TEST-FACADE-I0`.
