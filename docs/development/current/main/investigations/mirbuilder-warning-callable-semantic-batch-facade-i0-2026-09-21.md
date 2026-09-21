---
Status: closed__2026-09-21__WarningCallableSemanticBatchFacade__Execution
Task: MIRBUILDER-WARNING-CALLABLE-SEMANTIC-BATCH-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i52-2026-09-21.md
Implementation permission: true for the four test-only facade exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I53
---

# Warning cleanup: callable semantic batch test facade

## Six-line brief

```text
Decision: gate four unused forwarding exports whose observed consumers are
  test-only callable-semantic fixtures.
Source authority + canonical issuer: callable_semantic_batch/issuer.rs owns the
  issuer and policy; mod.rs is only a forwarding facade.
Non-authority: cargo-fix, wildcard imports, visibility changes, issuer logic,
  semantic batch behavior, or warning guesses.
Fail-fast boundary: any production facade consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: gate the four mod.rs exports under cfg(test), then run
  the fixed check and lib-test no-run gates sequentially.
Non-claims: no source admission, resolver policy, or production route change.
```

## Preconditions and acceptance

I52 selected the unused import group at
`src/mir/callable_semantic_batch/mod.rs:20-24`. The four exports are
`issue_resolved_callable_semantic_batch_v1`,
`issue_resolved_callable_semantic_batch_with_brand_catalog_v1`,
`issue_resolved_callable_semantic_batch_with_policy_v1`, and
`DirectCallObservationBatchPolicyV1`. Source census found no production caller
through this facade: issuer internals use child-module names directly, and the
common-v2 reference is inside a `#[cfg(test)]` module.

Gate only those four forwarding exports with `#[cfg(test)]`. Do not edit the
issuer, model, S6C relation, tests, or production call path.

Expected result: the four imports form one warning diagnostic, so lib
warnings **1,772 → 1,771** and lib-test warnings remain **561**. Both fixed gates must exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```


## Closeout evidence

The four forwarding exports received `#[cfg(test)]`; the issuer, model, S6C
relation, semantic batch behavior, and production call path were unchanged.
The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,771 | `/tmp/hakorune-warning-i0-callable-semantic-batch-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-callable-semantic-batch-facade-lib-test-20260921.log` |

The expected one-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
