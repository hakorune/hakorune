# Callable parameter contract

This module is the sole source-backed contract issuer for direct callable
method parameters in the normal semantic package.

```text
final syntax loan + resolved batch identity
  -> exact BindingRef and optional declared type spelling
  -> OpaqueHandle
     or ExactTrivial(i64)
     or ExactText(StringBox-as-Text)
  -> package-owned contract rows
```

Only explicit `i64` is classified as `ExactTrivial`; explicit `StringBox` is
classified as the semantic `ExactText` formal contract. An absent ordinary
spelling remains `OpaqueHandle`; an unsupported explicit spelling and every
non-ordinary transfer reject. `ExactText` is a source/formal demand only:
`HomeDemandV1::Handle` is a projection, not a runtime handle or physical wire.
The current Dynamic ingress explicitly rejects `ExactText` until the bounded
S6C/common-V2 route is admitted; it does not reinterpret or fall back.

The catalog is borrowed from one complete resolved batch, non-`Clone`, and has
no row selector, name lookup, `into_parts`, raw batch slot, AST/ValueId/MIR
authority, or fallback path. The normal package owns a private exact-contract
subset and exposes only its scoped lowering loan.
