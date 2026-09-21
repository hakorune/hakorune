---
Status: closed__2026-09-21__WarningVmReferenceTestImport__Execution
Task: MIRBUILDER-WARNING-VM-REFERENCE-TEST-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i61-2026-09-21.md
Implementation permission: true for the selected `CanonicalPublishedFamilyKindV1` import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I62
---

# Warning cleanup: VM-reference test import

## Six-line brief

```text
Decision: gate the unused CanonicalPublishedFamilyKindV1 import for the
  VM-reference test route classifier.
Source authority + canonical issuer: canonical publication owns the family kind;
  only the test classifier reads it through this module.
Non-authority: cargo-fix, wildcard imports, visibility changes, VM routing,
  publication ownership, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the single import item, then run the
  fixed gates sequentially.
Non-claims: no VM route, publication owner, backend, or test-body change.
```

## Preconditions and acceptance

I61 selected the unused import at
`src/mir/compiler/source_entry_vm_reference.rs:8`.
`CanonicalPublishedFamilyKindV1` is referenced only by the `#[cfg(test)]` route
classifier. `PublishedCanonicalSourceEntryOwnerV1` remains unconditional because
production result ownership uses it.

Split the import so only `CanonicalPublishedFamilyKindV1` receives `#[cfg(test)]`.
Do not edit VM routing, publication ownership, or the test body.

Expected result: one warning diagnostic is removed, so lib warnings
**1,760 → 1,759** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

`CanonicalPublishedFamilyKindV1` now receives `#[cfg(test)]`; the VM-reference
publication owner, route behavior, and test body were unchanged. The fixed gates
completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,759 | `/tmp/hakorune-warning-i0-vm-reference-test-import-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-vm-reference-test-import-lib-test-20260921.log` |

The expected one-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
