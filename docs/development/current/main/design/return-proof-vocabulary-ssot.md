---
Status: SSOT
Decision: accepted M10c-pre vocabulary lock
Date: 2026-05-08
Scope: pointer-return / handle-return ownership proof vocabulary before strong LLVM attrs.
Related:
  - docs/development/current/main/design/runtime-decl-manifest-v0.toml
  - docs/development/current/main/design/return-proof-vocabulary-v0.toml
  - docs/development/current/main/design/runtime-decl-return-proof-fixture-v0.toml
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

## Runtime-Decl Return Proof Row

Decision: accepted M10c-proof-row schema lock.

This section owns the runtime-decl return proof row schema.

Runtime-decl return proof rows may use the vocabulary above, but in the current
lane they are schema/proof records only. They do not emit LLVM attrs.

Row shape:

```toml
symbol = "..."
ret = "native_ptr_nonnull"
ret_proofs = ["nonnull"]
ret_proof_export = "disabled"
```

Allowed `ret_proof_export` values:

```text
disabled
verifier_required
exported
```

Current rule:

```text
disabled:
  allowed as schema/proof fixture only; emits no attrs

verifier_required:
  may be validated for native pointer rows only; emits no attrs

exported:
  blocked until M10c lands with verifier-owned export gates
```

The active `runtime-decl-manifest-v0.toml` must not grow `ret_proofs` or strong
attrs until a later card wires verifier/export consumption. The fixture file is
the only accepted schema host for M10c-proof-row.

## Native Pointer LLVM Type Name

Decision: accepted M10c-native-ptr-declare-type lock.

The `.hako` ll_emit runtime declaration registry may map native pointer return
classes to LLVM `ptr` type names:

```text
native_ptr_nonnull -> ptr
native_ptr_nullable -> ptr
native_ptr_dereferenceable -> ptr
```

This mapping is type spelling only. It does not prove ownership, nonnull,
dereferenceability, alignment, noalias, or lifetime. It also does not activate
any runtime-decl row.

The purpose is to prevent future native pointer rows from silently falling back
to the generic `i64` type spelling before M10c export gates are ready.

## First Active Native Pointer Runtime-Decl Row

Decision: accepted M10c-hako-mem-alloc-row lock.

The first active native pointer runtime-decl row is:

```text
hako_mem_alloc -> native_ptr_nullable
```

Contract:

```toml
symbol = "hako_mem_alloc"
args = ["imm_i64"]
ret = "native_ptr_nullable"
attrs = ["nounwind", "willreturn"]
memory = "readwrite"
lanes = ["hako-ll-min-v0", "compare"]
```

This row is intentionally nullable because the C ABI contract permits OOM to
return `NULL`. Therefore this row must not export `nonnull`,
`dereferenceable`, alignment, or `noalias`.

No `ret_proofs` are active in `runtime-decl-manifest-v0.toml` yet. The row only
locks the backend-private declaration/type seam for the existing `hako.mem`
facade and C ABI symbol.

## Machine Truth

The machine-readable vocabulary lock is:

```text
docs/development/current/main/design/return-proof-vocabulary-v0.toml
```

The runtime-decl row schema fixture is:

```text
docs/development/current/main/design/runtime-decl-return-proof-fixture-v0.toml
```

The Rust vocabulary mirror is:

```text
src/abi/return_proof.rs
```

The runtime-decl proof-row validator is:

```text
src/abi/runtime_decl_return_proof.rs
```

The `.hako` type-name consumer for runtime-decl value classes is:

```text
lang/src/shared/backend/ll_emit/runtime_decl_registry_box.hako
```

The guard must keep these in sync and must keep strong attrs absent from active
runtime declaration export surfaces until M10c lands with verifier-owned proof.

## Non-Goals

- No new LLVM attrs.
- No `runtime-decl` strong attrs.
- No active `runtime-decl` `ret_proofs` rows.
- No backend `.inc` pointer-attr inference.
- No public ABI manifest widening.
- No aliasing/lifetime verifier yet.
