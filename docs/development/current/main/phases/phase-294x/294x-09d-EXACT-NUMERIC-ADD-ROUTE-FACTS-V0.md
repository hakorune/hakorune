---
Status: Complete
Date: 2026-05-12
Scope: MIR-owned exact numeric `BinOp::Add` route facts before VM reference
  exact `usize` execution.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-09d Exact Numeric Add Route Facts V0

## Purpose

Publish the first MIR-owned exact numeric arithmetic route fact before VM
reference execution consumes exact `usize` operations.

This keeps VM rows from rediscovering semantics by scanning generic integer
operations:

```text
MIR-owned exact numeric value facts
MIR-owned exact add route fact
VM reference execution later
backend fail-fast/lowering remains visible
hako_alloc migration waits
```

## Landed Shape

- `BinOp::Add` publishes an exact numeric result value fact only when both
  operands already have the same exact numeric value fact.
- A matching route fact records the add site, operands, result, and declared
  exact numeric source name.
- exact/dynamic and exact/exact mismatches publish rejection metadata and do not
  create result facts.

## Non-Goals

- VM exact `usize` add execution;
- checked runtime overflow handling at the VM/backend instruction site;
- `Sub`, `Mul`, division, modulo, bitwise, compare, or shift route facts;
- typed-object exact numeric storage;
- hako_alloc field migration.

## Verification

- `cargo test exact_numeric_ --lib`
- `cargo check --release --bin hakorune`
