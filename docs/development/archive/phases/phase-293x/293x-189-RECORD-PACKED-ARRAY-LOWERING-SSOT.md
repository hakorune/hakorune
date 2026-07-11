---
Status: Complete
Date: 2026-05-12
Scope: record and packed-array lowering planning SSOT.
Related:
  - docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
---

# 293x-189 Record Packed-Array Lowering SSOT

## Goal

Fix the design vocabulary before allocator metadata grows more hand-written
parallel scalar arrays.

The user-facing word is `record`. The compiler/runtime reading is an
identity-free aggregate/value lane: local scalar replacement, packed record
columns, and materialization only at boundaries that need object semantics.

## Decision

- `record` is the explicit source-level aggregate/value contract.
- Ordinary `box` keeps identity-capable object semantics.
- User-box field-index fast paths may remove lookup cost, but must not erase
  identity or imply packed storage.
- M178 scalar metadata arrays remain the current truthful implementation.
- C205 is the first row allowed to migrate allocator metadata to `record`
  syntax on top of packed storage.

## Next Rows

```text
C201 ordinary user-box field-index fast path
C202 record surface and semantics
C203 record local scalar replacement
C204 ArrayBox inline-record storage
C205 allocator metadata record migration
```

## Stop Line

Do not flatten ordinary user boxes, infer record legality from typed field fast
paths, move `ArrayBox` authority into generic value storage, or introduce
allocator-specific record syntax.

M179 remains the next allocator algorithm row. This card does not implement
huge routing, huge page models, secure lists, or compiler/runtime record
lowering.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
