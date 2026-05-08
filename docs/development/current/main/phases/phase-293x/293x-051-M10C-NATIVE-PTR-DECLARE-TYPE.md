---
Status: done
Date: 2026-05-09
Card: 293x-051-M10C-NATIVE-PTR-DECLARE-TYPE
Scope: M10c-native-ptr-declare-type `.hako` ll_emit type spelling
---

# 293x-051 M10c Native Ptr Declare Type

## Decision

M10c-native-ptr-declare-type is live as type spelling only.

The `.hako` runtime-decl reader maps native pointer return/value classes to
LLVM `ptr` declarations so future rows do not silently fall through to the
generic `i64` spelling.

## Boundary

Owned here:

```text
native_ptr_nonnull -> ptr
native_ptr_nullable -> ptr
native_ptr_dereferenceable -> ptr
```

Not owned here:

```text
active native pointer runtime-decl rows
ret_proofs in active runtime-decl manifest
nonnull / dereferenceable / noalias / align attrs
public ABI manifest widening
C shim pointer-proof inference
```

## Acceptance

```text
bash tools/checks/k2_wide_native_ptr_decl_type_guard.sh
tools/checks/dev_gate.sh quick
```

## Files

```text
lang/src/shared/backend/ll_emit/runtime_decl_registry_box.hako
tools/checks/k2_wide_native_ptr_decl_type_guard.sh
docs/development/current/main/design/return-proof-vocabulary-ssot.md
```
