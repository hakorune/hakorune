# Callable parameter contract

This module is the sole source-backed contract issuer for direct callable
method parameters in the normal semantic package.

```text
final syntax loan + resolved batch identity
  -> exact BindingRef and optional declared type spelling
  -> OpaqueHandle
     or ExactTrivial(i64)
  -> package-owned contract rows
```

Only explicit `i64` is classified as `ExactTrivial`. An absent ordinary
spelling remains `OpaqueHandle`; an unsupported explicit spelling and every
non-ordinary transfer reject. `HomeDemandV1` is a one-way projection used by
the current Dynamic ingress consumer and is not stored as the authority.

The catalog is borrowed from one complete resolved batch, non-`Clone`, and has
no row selector, name lookup, `into_parts`, raw batch slot, AST/ValueId/MIR
authority, or fallback path. The normal package owns a private exact-contract
subset and exposes only its scoped lowering loan.
