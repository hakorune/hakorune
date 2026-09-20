---
Status: design_stop__2026-09-21__WarningBaselineRefresh__ClassificationOnly
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I0
Date: 2026-09-21
Parent: mirbuilder-cleanup-retirement0-d0-task-map-2026-08-04.md
Implementation permission: false; exact-head diagnostics and classification only
NextCard: one caller-zero unused-import cohort after the baseline is stable
---

# MirBuilder warning baseline refresh I0

## Six-line brief

```text
Decision: refresh the warning inventory at the current head before deleting
  anything; warning count alone is not a zero-warning or semantic claim.
Source authority + canonical issuer: fixed Cargo quick-profile diagnostics,
  the existing warning parser, and the named source owner for each row.
Non-authority: a historical count, a filtered test run, blanket `allow`,
  visibility widening, compiler semantics, or warning-driven source deletion.
Fail-fast boundary: an unclassified diagnostic, changed command/profile, or
  missing file:line/owner/role stops the row before any cleanup edit.
Smallest next slice: produce separate current `lib` and `lib-test` inventories,
  classify lint code/file:line/owner/production-test-compat-generated role,
  and select one finite caller-zero unused-import cohort.
Non-claims: no warning removal, suppression, semantic refactor, red-baseline
  change, whole-library green claim, or legacy retirement.
```

## Boundary

The census covers the fixed `nyash-rust` quick `--lib` and `--lib --tests`
diagnostic surfaces at the current commit. It includes warning code, source
location, owning module/authority, and production/test/compat/generated role.
It excludes release warnings, VM or backend parity, known test failures,
chronic measurement thresholds, and any source edit.

The workstream already records the historical observations `lib=1793` and
`lib-test=523` from 2026-09-12. The latest qualified Main slice recorded an
exact-head quick-check observation of 1,845 warnings. These numbers are
navigation evidence only; they must not be merged into one baseline or treated
as a regression until the two fixed commands are rerun and their diagnostic
names are compared.

## Required evidence

1. Run each fixed command once, with one Cargo process and the repository quick
   profile. Preserve the complete diagnostic stream and command line.
2. Parse each warning into a stable row keyed by lint, file, line, owner, and
   role. Deduplicate only identical diagnostic rows; retain duplicate counts.
3. Compare the current rows with the prior committed inventory and classify
   additions, removals, and unchanged rows as current-change, known baseline,
   or informational census.
4. Select at most one unused-import cohort whose callers are zero and whose
   deletion has no semantic owner change. Everything else remains parked with
   an explicit owner and reopen condition.

The fixed-order policy is `surface census -> baseline refresh -> one
caller-zero unused-import cohort`. Dead-code, private-interface, and
private-bound rows stay with their semantic owner; this card does not authorize
their deletion. The existing `MIRBUILDER-WARNING-SURFACE-CENSUS-R0` task map
and quick-profile baseline comparator remain the reference rather than a new
warning authority.

## Acceptance and stop conditions

Close this I0 only when both surfaces have reproducible command receipts,
stable inventories, explicit red classification, and a single selected
caller-zero cohort (or a named `NoSafeSlice`). A changed failure name,
unclassified row, or command/profile drift keeps the card at `design_stop`.

No `#[allow]`, suppression, visibility change, test deletion/ignore, semantic
route change, or production caller switch is permitted from this card.
