---
Status: closed__2026-09-21__WarningBaselineRefresh__NoSafeSliceForSelectedImport
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I0
Date: 2026-09-21
Parent: mirbuilder-cleanup-retirement0-d0-task-map-2026-08-04.md
Implementation permission: false; exact-head diagnostics and classification only
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0__next_caller_zero_cohort
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

## Exact-head observation and disposition

At `999a7ba541`, both fixed surfaces completed successfully:

| surface | command | warnings | lint classification |
| --- | --- | ---: | --- |
| lib | `cargo check --profile quick --lib -j4` | 1,845 | dead_code 1,729; unused_imports 103; private_interfaces 12; unused_variables 1 |
| lib-test | `cargo test --profile quick --lib --no-run -j4` | 563 | dead_code 541; unused_imports 10; private_interfaces 12 |

The streams are preserved outside the repository for this observation and the
exit status is zero for both commands. The prior `1793/523` figures remain a
historical comparison only. They are not merged into the current receipt, and
no red test baseline was changed.

The first candidate was the single-looking import `std::num::NonZeroU64` at
`src/mir/builder/module_invocation_identity.rs:8`. A lib-only source search
missed the `#[cfg(test)]` constructor at line 59. The fixed lib-test compile
therefore rejected its deletion with `use of undeclared type NonZeroU64`.
The import is restored and this candidate is `NoSafeSlice`; the parent census
must require both surfaces before selecting a caller-zero row. The other 102
lib unused-import rows, all lib-test-only rows, and every dead-code/private-
interface row remain unselected with their existing owner or parked role.

## Acceptance and stop conditions

This I0 closes with both reproducible command receipts, stable lint/file
classification, and a named rejected candidate. No unused-import deletion is
selected until a cohort is proven against both surfaces. A changed failure
name, unclassified row, or command/profile drift reopens the classification
boundary.

No `#[allow]`, suppression, visibility change, test deletion/ignore, semantic
route change, or production caller switch is permitted from this card.
