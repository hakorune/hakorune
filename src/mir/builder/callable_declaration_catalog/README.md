# Same-module callable declaration catalog

This module seals one complete, immutable view of same-module callable
declarations before body lowering.

The durable lookup catalog owns:

- canonical lookup keys: namespace, box owner, method name, arity;
- ordered parameters and `ParamDecl` transport;
- optional declared return spelling;
- the source body paired with that identity;
- deterministic static method+arity candidate lookup.

The admitted inventory is exactly:

- every function method in a non-sync, non-record static box;
- every non-static method in a non-sync, non-record ordinary box.

Constructors, top-level functions, ordinary-box static methods, record methods,
and sync-box methods stay outside this catalog. Instance rows never enter
static candidate lookup.

It does not own:

- parser declaration identity;
- call-result representation;
- body/type inference;
- MIR symbols as semantic identity;
- lowering or publication order;
- runtime/backend behavior;
- fallback resolution.

For a `VerifiedFinalCallableProgramSourceV1`, `source_backed.rs` is the sole
issuer. It borrows the final HRTB syntax loan and creates two inseparable
pieces:

```text
installable lookup catalog
+ selected lookup key -> parser-issued opaque declaration identity
```

The key is selection/lookup vocabulary, never source identity. Final slots,
statement indexes, method ordinals, names, arity, and AST addresses may
navigate inside the same final-source loan, but cannot repair a missing or
mismatched opaque identity. The legacy AST-only seal remains a compatibility
origin and cannot enter the source-backed semantic package.

CUT0 installs the catalog exactly once per legacy Builder root before the
remaining declaration-index effects. Query-before-install and duplicate
install are typed session failures. The old lowering-order static index and
body snapshot store have been retired without a compatibility fallback.

`BareStaticRecoveryDecisionV1` is the sole P0 selection owner:

```text
0 static candidates -> NoCandidate
1 static candidate  -> Unique(canonical key)
2+ static candidates -> Ambiguous
```

It owns no resolver priority, caller context, argument evaluation, call
emission, retry, or result representation. Exactly two unresolved-global
entrypoints consume this decision. `Ambiguous` never falls through to the
legacy dev tail resolver; `NoCandidate` preserves that pre-existing priority.
