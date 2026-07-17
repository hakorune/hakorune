# Same-module callable declaration catalog

This module seals one complete, immutable view of same-module callable
declarations before body lowering.

It owns:

- structured source identity: namespace, box owner, method name, arity;
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

- call-result representation;
- body/type inference;
- MIR symbols as semantic identity;
- lowering or publication order;
- runtime/backend behavior;
- fallback resolution.

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
