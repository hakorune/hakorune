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

`R0-CALLABLE-CATALOG-L0B-S0` and `R0-BARE-STATIC-RECOVERY0-P0` keep this module
disconnected. CUT0 later installs the catalog once per Builder root and
atomically retires `static_method_index` plus `lowered_method_asts`; this module
provides no compatibility fallback.

`BareStaticRecoveryDecisionV1` is the sole P0 selection owner:

```text
0 static candidates -> NoCandidate
1 static candidate  -> Unique(canonical key)
2+ static candidates -> Ambiguous
```

It owns no resolver priority, caller context, argument evaluation, call
emission, retry, or result representation. P0 keeps its production consumers
at zero.
