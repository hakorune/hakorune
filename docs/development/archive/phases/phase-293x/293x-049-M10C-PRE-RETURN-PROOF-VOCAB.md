---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-049-M10C-PRE-RETURN-PROOF-VOCAB
Scope: M10c-pre pointer/handle return proof vocabulary
---

# 293x-049 M10c-pre Return Proof Vocabulary

## Decision

M10c-pre is live as vocabulary only.

It separates runtime handle return classes from native pointer return classes
before any strong LLVM pointer attrs are allowed.

## Locked Return Classes

```text
imm_i64
handle_existing_borrowed
handle_existing_owned_ref
handle_fresh_owned
native_ptr_nonnull
native_ptr_nullable
native_ptr_dereferenceable(len, align)
```

## Locked Proof Vocabulary

```text
fresh
nonnull
dereferenceable_bytes
alignment
noalias_scope
no_refcount_mutation
no_registry_write
```

## Responsibility

- `docs/development/current/main/design/return-proof-vocabulary-v0.toml` owns
  the machine-readable vocabulary.
- `src/abi/return_proof.rs` mirrors the vocabulary for Rust-side future
  consumers.
- Active LLVM/runtime-decl export points still reject strong pointer attrs.
- Handle return classes must not feed LLVM pointer attrs.

## Non-Goals

- No `noalias`, `nonnull`, `dereferenceable`, or alignment export.
- No verifier-backed alias/lifetime proof yet.
- No runtime-decl manifest widening to strong attrs.
- No backend or `.inc` inference from helper names.

## Gates

```bash
bash tools/checks/k2_wide_return_proof_vocab_guard.sh
bash tools/checks/k2_wide_export_attrs_consistency_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
