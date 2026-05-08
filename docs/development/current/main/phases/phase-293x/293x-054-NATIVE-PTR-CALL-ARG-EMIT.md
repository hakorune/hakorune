---
Status: done
Date: 2026-05-09
Card: 293x-054-NATIVE-PTR-CALL-ARG-EMIT
Scope: M10c-native-ptr-call-arg-emit ll_emit manifest arg handling
---

# 293x-054 Native Ptr Call Arg Emit

## Decision

M10c-native-ptr-call-arg-emit is live for `.hako` ll_emit.

`hako_mem_realloc` makes a native pointer argument active in the runtime-decl
manifest. Therefore the ll_emit path must treat `native_ptr_*` manifest
arguments as LLVM `ptr` operands instead of falling back to `i64`.

## Boundary

Owned here:

```text
manifest extern arg class validation
native_ptr_* arg classes accepted only from native pointer values
LLVM call operand text emits `ptr %rN` for native_ptr_* args
hako_mem runtime-decl guard locks the arg emission seam
```

Not owned here:

```text
LLVM pointer attrs
ret_proofs in active runtime-decl rows
inttoptr / ptrtoint recovery casts
hako_mem_free / void runtime-decl row
C shim native pointer interpretation
```

## Acceptance

```text
bash tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
tools/checks/dev_gate.sh quick
```

## Files

```text
lang/src/shared/backend/ll_emit/call_policy_box.hako
lang/src/shared/backend/ll_emit/runtime_decl_registry_box.hako
lang/src/shared/backend/ll_emit/recipe_facts_v0_box.hako
lang/src/shared/backend/ll_emit/ll_text_emit_box.hako
tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
```
