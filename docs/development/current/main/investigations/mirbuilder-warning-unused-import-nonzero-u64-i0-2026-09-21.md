---
Status: rejected__2026-09-21__WarningUnusedImportNonZeroU64__TestSurfaceReference
Task: MIRBUILDER-WARNING-UNUSED-IMPORT-NONZERO-U64-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i0-2026-09-21.md
Implementation permission: false; candidate rejected by lib-test compile
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0__next_caller_zero_cohort
---

# Warning cleanup: unused `NonZeroU64` import

## Six-line brief

```text
Decision: remove one caller-zero unused import proven unused at exact HEAD.
Source authority + canonical issuer: Rust name resolution and the two fixed
  quick-profile diagnostic surfaces selected by the parent baseline I0.
Non-authority: warning count, cargo-fix suggestions, visibility changes,
  dead-code ownership, test-only imports, or semantic route inference.
Fail-fast boundary: any remaining reference, changed warning family, compile
  error, or new failure stops before accepting the deletion.
Smallest next slice: delete only `std::num::NonZeroU64` from
  `src/mir/builder/module_invocation_identity.rs` and rerun both fixed checks.
Non-claims: no broad import cleanup, dead-code cleanup, suppression, semantic
  change, red-baseline update, or production cutover.
```

## Acceptance

The import is used by the `#[cfg(test)]` constructor in the same module, so it
is not a caller-zero row across both fixed surfaces. The attempted deletion
was rejected by `cargo test --profile quick --lib --no-run -j4` with
`use of undeclared type NonZeroU64`; the source import is restored. No deletion
is accepted by this card.

Do not run cargo-fix or alter neighboring imports. If the import is referenced
after all cfg/test surfaces are compiled, stop and return to the parent census
with a `NoSafeSlice` classification.
