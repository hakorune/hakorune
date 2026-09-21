---
Status: closed__2026-09-21__WarningBaselineRefreshI110__SelectedBuildGateBrand
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I110
Date: 2026-09-21
Parent: mirbuilder-warning-raw-physical-brand-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-BUILD-GATE-BRAND-I0
---

# MirBuilder warning baseline refresh I110

## Six-line brief

```text
Decision: refresh the warning surface after deleting the raw physical brand
accessor, then select at most one caller-zero or equivalent bounded row.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
warning classification policy, with the current owner census as the boundary.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or guessed
source/production callers.
Fail-fast boundary: a new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,699/551, classify one warning, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad cleanup, route switch, parser expansion, fallback repair,
or LegacyCallV0 retirement.
```

## Refresh evidence

The preceding raw-physical row deleted
`CompletedRawRootBatchPhysicalV1::brand`. Focused raw-root acceptance passed
15/15. The sequential check produced **1,699 lib warnings** and the test build
produced **551 lib-test warnings**; formatting, diff, and pointer guards pass.

The old-edge lane remains `NoSafeSlice`: no named successor or caller-zero
proof authorizes legacy-edge deletion in this refresh. The I110 refresh
reproduced **1,699 lib warnings** and selected one bounded warning cohort.

The selected warning is the redundant outer `brand` field and accessor on
`PreparedBuildGateDecisionSetV1` (`src/parser/build_cfg/decision_set.rs:49,127`).
Neither has a production caller; each issued `BuildGateDecisionRowV1` already
carries and validates the parser brand. One parser unit test reads the
accessor only as a convenience assertion. The next row removes that redundant
projection and makes the test assert the row-level brand used by the consumer.

The old-edge lane remains `NoSafeSlice`; this selection does not authorize a
legacy route or compatibility deletion.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```
