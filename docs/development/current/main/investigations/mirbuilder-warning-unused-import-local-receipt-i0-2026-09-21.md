---
Status: closed__2026-09-21__WarningUnusedImportLocalReceipt
Task: MIRBUILDER-WARNING-UNUSED-IMPORT-LOCAL-RECEIPT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i3-2026-09-21.md
Implementation permission: true for one private re-export import deletion only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I4
---

# Warning cleanup: unused local receipt import

## Six-line brief

```text
Decision: remove the unused parent-module import of
  drive_local_statement_with_receipt_v1; the implementation remains owned by
  local_statement_descent.rs and is not re-exported through this row.
Source authority + canonical issuer: Rust module resolution and the I3
  two-surface warning inventory.
Non-authority: cargo-fix, local-statement behavior, warning count alone,
  visibility widening, or semantic lowering interpretation.
Fail-fast boundary: any external reference, compile error, new warning, or
  changed failure name rejects the deletion.
Smallest next slice: remove only that name from stmts/mod.rs and rerun both
  fixed quick-profile checks plus fmt/pointer guards.
Non-claims: no statement lowering change, suppression, broad import cleanup,
  or production route retirement.
```

## Precondition and acceptance

The I3 inventory contains the import warning on both lib and lib-test surfaces.
The only other occurrences are the implementation and its internal call in
`local_statement_descent.rs`; no caller imports the parent re-export. Acceptance
requires lib warnings 1,842, lib-test warnings 561, both commands exit 0, and
fmt/diff/pointer guards remain green.

## Execution evidence

The parent-module import was removed while the implementation in
`local_statement_descent.rs` remained unchanged. `cargo check --profile quick
--lib -j4` completed with lib warnings 1,842 (down from 1,843), and
`cargo test --profile quick --lib --no-run -j4` completed with lib-test warnings
561 (down from 562). Both exit 0; fmt, diff, and pointer guards are green. No
statement-lowering behavior or failure inventory changed.
