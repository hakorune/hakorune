---
Status: closed__2026-09-21__WarningPublishedBackendTestImports__Execution
Task: MIRBUILDER-WARNING-PUBLISHED-BACKEND-TEST-IMPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i57-2026-09-21.md
Implementation permission: true for the selected two test-only re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I58
---

# Warning cleanup: published-backend test re-exports

## Six-line brief

```text
Decision: gate two unused published-backend re-exports used only by test
  fixtures.
Source authority + canonical issuer: c_transport and compiled_entry_contract
  own the types; production consumers use their canonical internal paths.
Non-authority: cargo-fix, wildcard imports, visibility changes, backend view or
  cleanup semantics, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the two selected re-export items, then run
  the fixed gates sequentially.
Non-claims: no backend route, transport, cleanup, ABI, or test-body change.
```

## Preconditions and acceptance

I57 selected the two unused re-exports at
`src/mir/compiler/normal_default_pipeline/published_backend_view.rs:39-42`.
`PublishedCallKindV1` is referenced by test-only historical function-view
fixtures, while `CompiledEntryCleanupKindV1` is referenced by the compiled-entry
contract fixture. Production code uses the canonical transport and compiled-entry
contract paths directly.

Split the re-export so only these two types receive `#[cfg(test)]`; leave the
other published call rows and compiled-entry contract types unconditional. Do
not edit backend view logic, transport, cleanup, ABI, or test bodies.

Expected result: two warning diagnostics are removed, so lib warnings
**1,766 → 1,764** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

`PublishedCallKindV1` and `CompiledEntryCleanupKindV1` now receive `#[cfg(test)]`
on the parent re-exports; backend view, transport, cleanup, ABI, and test bodies
were unchanged. The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,764 | `/tmp/hakorune-warning-i0-published-backend-test-imports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-published-backend-test-imports-lib-test-20260921.log` |

The expected two-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
