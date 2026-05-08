---
Status: SSOT
Decision: accepted M10c-pre vocabulary lock
Date: 2026-05-08
Scope: pointer-return / handle-return ownership proof vocabulary before strong LLVM attrs.
Related:
  - docs/development/current/main/design/runtime-decl-manifest-v0.toml
  - docs/development/current/main/design/return-proof-vocabulary-v0.toml
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/reference/runtime/substrate-capabilities.md
---

# Return Proof Vocabulary SSOT

## Decision

M10c-pre is a vocabulary/proof row only. It does not export new LLVM attrs.

The critical separation is:

```text
handle return proof:
  runtime value / ownership proof
  not an LLVM pointer attr target

native pointer return proof:
  native pointer proof
  only future class family that may feed LLVM pointer attrs
```

Backends must not attach `nonnull`, `dereferenceable`, alignment, or `noalias`
because a return class is handle-like or because a helper name looks pointer-like.

## Return Classes

Locked vocabulary:

```text
imm_i64
handle_existing_borrowed
handle_existing_owned_ref
handle_fresh_owned
native_ptr_nonnull
native_ptr_nullable
native_ptr_dereferenceable(len, align)
```

Handle classes describe runtime handles and ownership. Native pointer classes
describe native address values.

## Proof Vocabulary

Locked vocabulary:

```text
fresh
nonnull
dereferenceable_bytes
alignment
noalias_scope
no_refcount_mutation
no_registry_write
```

`no_refcount_mutation` and `no_registry_write` are side-effect proofs. They are
not LLVM pointer attrs.

## Export Rule

Current rule:

```text
handle_* + any proof:
  never export LLVM pointer attrs

native_ptr_nullable:
  no pointer attrs in M10c-pre

native_ptr_nonnull + nonnull:
  eligible only after M10c verifier/export gates

native_ptr_dereferenceable(len, align):
  eligible only after M10c verifier/export gates
```

M10c-pre therefore enables future verification vocabulary, not optimizer
behavior.

## Machine Truth

The machine-readable vocabulary lock is:

```text
docs/development/current/main/design/return-proof-vocabulary-v0.toml
```

The Rust vocabulary mirror is:

```text
src/abi/return_proof.rs
```

The guard must keep these in sync and must keep strong attrs absent from active
runtime declaration export surfaces until M10c lands with verifier-owned proof.

## Non-Goals

- No new LLVM attrs.
- No `runtime-decl` strong attrs.
- No backend `.inc` pointer-attr inference.
- No public ABI manifest widening.
- No aliasing/lifetime verifier yet.
