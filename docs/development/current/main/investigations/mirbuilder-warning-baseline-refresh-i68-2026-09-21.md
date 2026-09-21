---
Status: design_stop__2026-09-21__WarningBaselineRefreshI68__NextCohortSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I68
Date: 2026-09-21
Parent: mirbuilder-warning-static-method-crow-facade-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0
---

# MirBuilder warning baseline refresh I68

## Six-line brief

```text
Decision: refresh both warning surfaces after the C-row facade-chain deletion.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: diagnostic totals alone, cargo-fix, blanket allow, or dead-code
  guesses; grouped import items need source census evidence.
Fail-fast boundary: a new warning, owner-local move, command drift, or an
  unclassified test-only reference stops the next selection.
Smallest next slice: compare lib/lib-test against 1,754/558 and select at most
  one caller-zero cohort, counting grouped lint items separately when needed.
Non-claims: no broad warning deletion, suppression, semantic refactor, or route
  change.
```

## Acceptance

Run these commands once each, sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record lint, file and line, owner, role, and whether a grouped diagnostic masks
multiple import items. Select one finite caller-zero import cohort or write
`NoSafeSlice`. Keep dead-code and private-interface rows with their owners.
