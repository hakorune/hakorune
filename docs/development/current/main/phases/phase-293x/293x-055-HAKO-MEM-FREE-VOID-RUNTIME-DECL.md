---
Status: done
Date: 2026-05-09
Scope: M10c-hako-mem-free-void-row runtime-decl + ll_emit void call
---

# 293x-055 Hako Mem Free Void Runtime Decl

## Decision

M10c-hako-mem-free-void-row is live as the third active hako.mem
runtime-decl row.

## Why

`MemCoreBox.free_i64` already exposes the public memory capability free seam,
and the C ABI symbol is:

```text
hako_mem_free(void*) -> void
```

The runtime-decl manifest must not lie by declaring an invented `i64` result.
`.hako` ll_emit therefore needs a narrow `void` return class and `call void`
emission for direct extern calls.

## Owned

- runtime-decl row for `hako_mem_free`
- `void` runtime-decl type spelling in `.hako` ll_emit
- direct-call emission as `call void @"hako_mem_free"(ptr ...)`
- nullable native pointer argument validation through the manifest arg class
- guard that keeps free in the hako.mem runtime-decl seam

## Not Owned

- LLVM pointer attrs
- active `ret_proofs`
- ownership / aliasing / lifetime proof
- `hako_mem_free` return-value use as a valid expression
- C shim semantic branching

## Acceptance

```bash
bash tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Files

```text
docs/development/current/main/design/runtime-decl-manifest-v0.toml
lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako
lang/src/shared/backend/ll_emit/runtime_decl_registry_box.hako
lang/src/shared/backend/ll_emit/ll_text_emit_box.hako
tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
```
