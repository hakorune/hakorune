---
Status: closed__2026-09-21__WarningDirectAccumTestImports__Execution
Task: MIRBUILDER-WARNING-DIRECT-ACCUM-TEST-IMPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i59-2026-09-21.md
Implementation permission: true for the selected direct-accum test-only imports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I60
---

# Warning cleanup: DirectAccum test imports

## Six-line brief

```text
Decision: gate the DirectAccum snapshot seam's unused family-plan, preflight,
  and resolved-input imports.
Source authority + canonical issuer: capability and lowering_input own the
  types; only the snapshot seam consumes these imported forms.
Non-authority: cargo-fix, wildcard imports, visibility changes, production
  DirectAccum cutover, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the selected import groups, then run the
  fixed gates sequentially.
Non-claims: no DirectAccum production lowering, cutover, admission, or test-body
  change.
```

## Preconditions and acceptance

I59 selected the unused imports at
`src/mir/compiler/resolved_direct_accum_cutover.rs:10,15`.
`CanonicalFirstFamilyPlanV1`, `CanonicalLoweringPreflightV1`, and
`ResolvedModuleLoweringInputV1` are referenced only by the `#[cfg(test)]`
snapshot seam. Production direct-accum lowering uses the canonical plan and
prepared external-commit path.

Split the imports so only these selected items receive `#[cfg(test)]`; leave
`CanonicalResolvedCutoverStageErrorV1` and other production imports unconditional.
Do not edit DirectAccum lowering, cutover, admission, or test bodies.

Expected result: two warning diagnostics are removed, so lib warnings
**1,763 → 1,761** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

The DirectAccum snapshot-only family-plan, preflight, and resolved-input imports
now receive `#[cfg(test)]`; production lowering, cutover, admission, and test
bodies were unchanged. The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,761 | `/tmp/hakorune-warning-i0-direct-accum-test-imports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-direct-accum-test-imports-lib-test-20260921.log` |

The expected two-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
