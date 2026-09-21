---
Status: closed__2026-09-21__WarningGenericG0AdmissionTestImport__Execution
Task: MIRBUILDER-WARNING-GENERIC-G0-ADMISSION-TEST-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i56-2026-09-21.md
Implementation permission: true for the selected `issue_generic_g0_physical_emitter_admission_v1` re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I57
---

# Warning cleanup: Generic G0 admission test re-export

## Six-line brief

```text
Decision: gate the unused Generic G0 admission issuer re-export for test-only
  fixtures.
Source authority + canonical issuer: emitter_admission owns the issuer;
  production lowering_input consumes the reject/prepared/source-parent items.
Non-authority: cargo-fix, wildcard imports, visibility changes, admission logic,
  lowering behavior, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the single re-export item, then run the
  fixed gates sequentially.
Non-claims: no Generic G0 admission, lowering input, physical layout, or test
  body change.
```

## Preconditions and acceptance

I56 selected the unused re-export at
`src/mir/compiler/generic_g0_physical_operation_cohort.rs:14`.
`issue_generic_g0_physical_emitter_admission_v1` is referenced by the
`#[cfg(test)]` emitter-admission fixtures and the resolved-lowering session
fixtures. Production code uses the source-parent issuer and the reject/prepared
admission types through the same module, so those exports remain unconditional.

Split the re-export so only `issue_generic_g0_physical_emitter_admission_v1`
receives `#[cfg(test)]`. Do not edit admission logic, lowering input, physical
session code, or test bodies.

Expected result: one warning diagnostic is removed, so lib warnings
**1,767 → 1,766** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

`issue_generic_g0_physical_emitter_admission_v1` now receives `#[cfg(test)]` on
the parent re-export; admission logic, lowering input, physical session code,
and test bodies were unchanged. The fixed gates completed sequentially with exit
0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,766 | `/tmp/hakorune-warning-i0-generic-g0-admission-test-import-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-generic-g0-admission-test-import-lib-test-20260921.log` |

The expected one-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
