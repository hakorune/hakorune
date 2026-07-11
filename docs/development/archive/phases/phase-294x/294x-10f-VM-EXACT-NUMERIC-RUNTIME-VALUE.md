---
Status: Complete
Date: 2026-05-12
Scope: VM reference runtime value representation for exact numeric results.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-10f VM Exact Numeric Runtime Value

## Decision

Exact numeric VM reference rows must not round successful exact numeric
results back into `VMValue::Integer(i64)` as their only live representation.

The VM gains a tagged exact numeric value that carries:

- the declared exact numeric source name, such as `usize`;
- the exact value as a wide integer payload.

This keeps VM reference execution honest when `usize` values are valid but no
longer fit the current dynamic `Integer(i64)` lane.

## Contract

- Exact numeric arithmetic and logical shift routes produce tagged exact
  numeric values.
- Exact numeric operands may come from either legacy dynamic `Integer(i64)`
  values or already-tagged exact numeric values with the same declared type.
- Negative unsigned operands still fail fast.
- Overflow still fails fast through the MIR numeric substrate policy.
- Generic dynamic integer consumers do not silently treat tagged exact numeric
  values as `Integer(i64)`.

## Non-Goals

- No non-VM backend lowering.
- No native typed-object exact numeric slots.
- No production `hako_alloc` field migration.
- No implicit wrapping arithmetic.

## Acceptance

```bash
cargo test -q --lib exact_numeric_
cargo check --release --bin hakorune
```
