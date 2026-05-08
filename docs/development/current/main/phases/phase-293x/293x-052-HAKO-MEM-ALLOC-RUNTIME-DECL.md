---
Status: done
Date: 2026-05-09
Card: 293x-052-HAKO-MEM-ALLOC-RUNTIME-DECL
Scope: M10c-hako-mem-alloc-row active nullable native pointer row
---

# 293x-052 Hako Mem Alloc Runtime-Decl

## Decision

M10c-hako-mem-alloc-row is live as the first active native pointer runtime-decl row.

The row covers the existing C ABI symbol and `.hako` substrate facade:

```text
hako_mem_alloc(size) -> native_ptr_nullable
```

## Boundary

Owned here:

```text
runtime-decl row for hako_mem_alloc
generated `.hako` runtime-decl defaults sync
nullable native pointer return spelling
guard that this is the only active native pointer row
```

Not owned here:

```text
nonnull / dereferenceable / noalias / align attrs
ret_proofs in the active runtime-decl manifest
hako_mem_realloc / hako_mem_free rows
pointer lifetime / alias verifier
C shim proof inference
```

## Acceptance

```text
bash tools/checks/k2_wide_hako_mem_alloc_runtime_decl_guard.sh
tools/checks/dev_gate.sh quick
```

## Files

```text
docs/development/current/main/design/runtime-decl-manifest-v0.toml
lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako
tools/checks/k2_wide_hako_mem_alloc_runtime_decl_guard.sh
```
