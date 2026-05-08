---
Status: done
Date: 2026-05-09
Card: 293x-053-HAKO-MEM-REALLOC-RUNTIME-DECL
Scope: M10c-hako-mem-realloc-row active nullable native pointer row
---

# 293x-053 Hako Mem Realloc Runtime-Decl

## Decision

M10c-hako-mem-realloc-row is live as the second active native pointer runtime-decl row.

The row covers the existing C ABI symbol and `.hako` substrate facade:

```text
hako_mem_realloc(ptr, new_size) -> native_ptr_nullable
```

## Boundary

Owned here:

```text
runtime-decl row for hako_mem_realloc
generated `.hako` runtime-decl defaults sync
nullable native pointer argument spelling
nullable native pointer return spelling
guard that only hako_mem_alloc / hako_mem_realloc are active native pointer rows
```

Not owned here:

```text
nonnull / dereferenceable / noalias / align attrs
ret_proofs in the active runtime-decl manifest
hako_mem_free row
void / no-return-value runtime-decl class
pointer lifetime / alias verifier
C shim proof inference
```

## Manual Update

No public manual update is required for this card because the row is
backend-private runtime-decl metadata. The public substrate capability manual
already names the `hako.mem` allocation facade. The next manual trigger is
either a public value-class surface change or a source-level `hako.mem`
signature change.

## Acceptance

```text
bash tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
tools/checks/dev_gate.sh quick
```

## Files

```text
docs/development/current/main/design/runtime-decl-manifest-v0.toml
lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako
tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
```
