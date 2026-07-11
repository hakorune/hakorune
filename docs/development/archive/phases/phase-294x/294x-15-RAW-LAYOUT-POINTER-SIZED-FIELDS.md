---
Status: Landed
Date: 2026-05-12
Scope: raw-layout pointer-sized field planning for exact usize semantics.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/runtime/substrate-capabilities.md
  - src/mir/raw_layout.rs
---

# 294x-15 Raw Layout Pointer-Sized Fields

## Decision

MIR raw-layout planning now accepts `usize` and `isize` field declarations by
resolving them through `NumericTarget` pointer-width rules.

This is still metadata-only. It does not add `.hako struct` syntax, source
`sizeof` / `offsetof`, backend-native raw layout execution, pointer fields, or
semantic Box fields.

## Scope

Accepted:

- `usize` raw-layout fields;
- `isize` raw-layout fields;
- target-resolved storage:
  - 32-bit pointer target: `usize -> u32`, `isize -> i32`;
  - 64-bit pointer target: `usize -> u64`, `isize -> i64`;
- natural alignment and padding using the resolved scalar storage;
- an explicit target-planning entry for cross-target layout tests.

Deferred:

- source syntax for raw layout declarations;
- backend-active native field load/store;
- pointer / handle / Box fields;
- native typed-object exact numeric slots;
- hako_alloc live field migration.

## Stop Line

The raw-layout owner is still `src/mir/raw_layout.rs`. Backends may only consume
a future MIR/JSON raw-layout plan; they must not infer layout from app field
names, semantic `box` declarations, or allocator-specific strings.

`usize` in raw-layout metadata is target-resolved layout information only. It
does not imply that live runtime values have left the current `Integer(i64)`
lane.

## Verification

```bash
cargo test -q raw_layout --lib
cargo test -q numeric_substrate --lib
```
