---
Status: SSOT
Date: 2026-05-14
Scope: ARRAY-002A typed Array<T> method contract only.
Related:
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-315-ARRAY-002A-TYPED-ARRAY-METHOD-CONTRACT.md
---

# Typed Array Method Contract SSOT

## Decision

Decision: accepted.

A local declared as `Array<T>` has a Stage1 method-surface contract for the
canonical ordinary typed collection methods:

```text
push(value)
get(index)
set(index, value)
length()
```

This row owns method name and arity diagnostics only. Element type checking,
Array inference, and backend ArrayBox route guards remain separate rows.

## Canonical surface

```hako
local ids: Array<PageId> = []
ids.push(PageId(1))
local first = ids.get(0)
ids.set(0, PageId(2))
local n = ids.length()
```

## Stage1 contract

For a tracked local `x: Array<T>`:

- `x.push(value)` expects 1 argument.
- `x.get(index)` expects 1 argument.
- `x.set(index, value)` expects 2 arguments.
- `x.length()` expects 0 arguments.
- Other method names fail-fast with `[array/method-contract]`.

The method still lowers through the existing JSON v0 method-call shape. This
row does not introduce a new Array-specific JSON node.

## Non-goals

- No element type compatibility checks. `ARRAY-002B` owns direct element checks.
- No type inference for `local x = []`. `ARRAY-002C` keeps unsupported inference
  fail-fast.
- No `PackedArray<T>` fallback to ordinary `Array<T>` / `ArrayBox`.
- No backend or ArrayBox route proof. `ARRAY-002D` owns the backend guard.
- No aliases such as `len()` / `size()` in canonical typed `Array<T>` source.

## Guard

```bash
bash tools/checks/k2_wide_array_typed_method_contract_guard.sh
```
