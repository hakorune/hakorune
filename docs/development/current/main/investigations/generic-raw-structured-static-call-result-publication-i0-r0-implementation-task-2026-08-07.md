# Generic raw structured static-call result publication I0/R0

Status: `completed caller-zero implementation 2026-08-07; production activation remains 0`

Parent design:

- `generic-raw-structured-static-call-result-publication-d0-task-2026-08-07.md`

## Change

Add one disconnected publication product under the callable-result
representation family.  It accepts a sealed exact source requirement and a
successful `CompletedUnifiedValueCallEmissionV1`, then issues one non-Clone,
single-use publication receipt whose destination is taken only from the
physical call receipt.  The receipt consumer writes `MirType::Integer` to
`type_ctx` exactly once after physical success.

The product is not wired into `RawInvocationChildPortV1`, method-call route
selection, GenericLoop, or production lowering.  Existing receipt helpers
remain the sole physical Call authority.

## Contract

- Source proof owns exact `(Cataloged caller, SourceExprSite)` selection.
- Physical `CompletedUnifiedValueCallEmissionV1` owns the final `ValueId`.
- Publication is non-Clone and consume-once; source proof cannot publish.
- Foreign caller/site/catalog, unavailable result, alternate route, missing
  destination, failed emission, duplicate consume, and guessed destination
  reject before or without a type write.
- `CALLABLE-RESULT-NESTED-REP0` and the later production activation remain
  separate owners.
- No retry, fallback, name dispatch, GenericLoop backfill, or local inference.

## Done

- New source/result publication module and focused unit tests are below the
  800-line source/test limit.
- Focused tests cover source identity retention, successful publication,
  failed physical emission, and duplicate publication rejection. Foreign-site
  and alternate-route cases remain I1 activation negatives, not I0 claims.
- `cargo test --lib static_call_result_publication` (1 test) and
  `cargo test --lib static_result_publication` (3 tests) are green.
- `cargo build --bin hakorune` and `cargo build --release --bin hakorune`
  are green; existing warning volume is unchanged.
- `git diff --check`, current-state guard, and MirBuilder replacement guard are
  green.
- The canonical strict probe is rerun and still stops at
  `MissingTransientType { init: ValueId(3) }`; this is recorded as a
  pre-production boundary, not a green production receipt.
- The implementation commit updates the exact `docs/reference/**` row, this
  task closeout, and current mirrors. An immutable probe receipt is deferred
  to the I1 whole-source activation row because I0 intentionally has no
  production caller.

## Stop

Return to design if a source plan must carry `ValueId`/Builder state, if a
physical receipt cannot be obtained without opening an alternate route, if
`type_ctx` gains a second publication owner, or if production wiring becomes
necessary to prove this isolated product.

## Next row

`GENERIC-RAW-STRUCTURED-STATIC-CALL-RESULT-PUBLICATION-I1-D0` designs the
whole-source activation and the route-transaction rollback owner. It is a
separate design stop; I0 does not authorize a production caller.
