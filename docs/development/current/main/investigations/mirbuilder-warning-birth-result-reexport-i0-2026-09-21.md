---
Status: closed__2026-09-21__WarningBirthResultReexport__Execution
Task: MIRBUILDER-WARNING-BIRTH-RESULT-REEXPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i63-2026-09-21.md
Implementation permission: true for the selected caller-zero `BirthResultAbiV1` re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I64
---

# Warning cleanup: caller-zero birth-result re-export

## Six-line brief

```text
Decision: remove the unused BirthResultAbiV1 ordinary_new_coseal re-export from the
  semantic-package facade.
Source authority + canonical issuer: birth_abi_handoff owns the type; no caller
  enters through the ordinary_new_coseal facade.
Non-authority: cargo-fix, wildcard imports, visibility changes, birth ABI
  semantics, or warning guesses.
Fail-fast boundary: any caller discovery, compile error, changed test warning,
  or warning-count mismatch rejects the slice.
Smallest next slice: delete the single caller-zero re-export, then run the fixed
  gates sequentially.
Non-claims: no birth ABI owner, lifecycle admission, or test-body change.
```

## Preconditions and acceptance

I63 selected the caller-zero re-export at
`src/mir/normal_callable_semantic_package/ordinary_new_coseal.rs:17`.
`BirthResultAbiV1` has no repository caller through the ordinary_new_coseal facade; its only
uses are internal to `birth_abi_handoff`.

Remove only `BirthResultAbiV1` from the ordinary_new_coseal re-export. Do not edit the birth
ABI owner, lifecycle admission, or test body.

Expected result: one warning diagnostic is removed, so lib warnings
**1,757 → 1,756** and lib-test warnings **561 → 560**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```


## Closeout evidence

The caller-zero `BirthResultAbiV1` re-export was removed from
`ordinary_new_coseal`; the birth ABI owner, lifecycle admission, and test body
were unchanged. The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,756 | `/tmp/hakorune-warning-i0-birth-result-reexport-lib-20260921.log` |
| lib test | 560 | `/tmp/hakorune-warning-i0-birth-result-reexport-lib-test-20260921.log` |

The expected one-warning reduction was observed on both surfaces.
