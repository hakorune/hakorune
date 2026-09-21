---
Status: closed__2026-09-21__WarningBaselineRefreshI112__SelectedSlotRows
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I112
Date: 2026-09-21
Parent: mirbuilder-warning-build-gate-brand-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-SLOT-ROWS-I0
---

# MirBuilder warning baseline refresh I112

## Six-line brief

```text
Decision: refresh the warning surface after removing the redundant BuildGate
brand projection, then select at most one bounded row.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
warning classification policy, with the current owner census as the boundary.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or guessed
source/production callers.
Fail-fast boundary: a new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,697/551, classify one warning, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad cleanup, route switch, parser expansion, fallback repair,
or LegacyCallV0 retirement.
```

## Refresh evidence

The preceding BuildGate row removed the unused outer `brand` field and
accessor while preserving row-level parser identity. Its focused suite passed
10/10. The sequential check produced **1,697 lib warnings** and the test build
produced **551 lib-test warnings**; formatting, diff, and pointer guards pass.

The old-edge lane remains `NoSafeSlice`: no named successor or caller-zero
proof authorizes legacy-edge deletion in this refresh. The I112 refresh
reproduced **1,697 lib warnings** and selected one bounded warning cohort.

The selected warning is the unused `ProjectedProgramItemSlotSetV1::rows`
accessor in `src/parser/build_cfg/program_item_slots.rs:48`. It has no
production caller; the production consumer already takes the owned rows via
`into_rows`, and two parser tests only borrowed the accessor. The bounded row
switches those tests to `into_rows` or `exact_final_slot` and deletes the
redundant borrow projection.

The old-edge lane remains `NoSafeSlice`; this selection does not authorize a
legacy route or compatibility deletion.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```
