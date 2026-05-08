---
Status: done
Date: 2026-05-09
Card: 293x-050-M10C-RUNTIME-DECL-RETURN-PROOF-ROW
Scope: M10c-proof-row runtime-decl return proof row schema
---

# 293x-050 M10c Runtime-Decl Return Proof Row

## Decision

M10c-proof-row is live as schema/validator only.

This card does not emit strong LLVM attrs. It creates the backend-private shape
that later M10c export work may consume after verifier gates are present.

## Boundary

Owned here:

```text
runtime-decl return proof row schema
Rust validator for row legality
schema fixture for handle/native pointer separation
guard that active runtime-decl and .inc remain unchanged
```

Not owned here:

```text
noalias / nonnull / dereferenceable / align export
active runtime-decl ret_proofs rows
public ABI manifest widening
alias/lifetime verifier
.inc symbol-name inference
```

## Acceptance

```text
cargo test -q runtime_decl_return_proof
bash tools/checks/k2_wide_runtime_decl_return_proof_row_guard.sh
```

## Files

```text
docs/development/current/main/design/return-proof-vocabulary-ssot.md
docs/development/current/main/design/runtime-decl-return-proof-fixture-v0.toml
src/abi/runtime_decl_return_proof.rs
tools/checks/k2_wide_runtime_decl_return_proof_row_guard.sh
```
