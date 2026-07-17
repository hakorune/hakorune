# Same-module callable declaration catalog

This module seals one complete, immutable view of static-box callable
declarations before body lowering.

It owns:

- structured source identity: static box owner, method name, arity;
- ordered parameters and `ParamDecl` transport;
- optional declared return spelling;
- the source body paired with that identity;
- deterministic method+arity candidate lookup.

It does not own:

- call-result representation;
- body/type inference;
- MIR symbols as semantic identity;
- lowering or publication order;
- runtime/backend behavior;
- fallback resolution.

`R0-CALLABLE-RESULT-I64-CATALOG0-L0a` keeps this product disconnected.
L0b will migrate the existing partial `static_method_index` and
`lowered_method_asts` consumers only after parity is fixed by tests.
