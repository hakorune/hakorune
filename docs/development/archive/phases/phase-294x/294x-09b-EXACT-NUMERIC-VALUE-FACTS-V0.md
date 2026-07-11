---
Status: Complete
Date: 2026-05-12
Scope: first MIR-owned exact numeric per-value facts for field reads, copies,
  and conservative control merges.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-09b Exact Numeric Value Facts V0

## Purpose

Publish the first exact numeric value facts before VM reference exact `usize`
execution starts.

This keeps the owner order explicit:

```text
MIR-owned facts first
VM reference execution later
backend fail-fast/lowering remains visible
hako_alloc migration waits
```

## Landed Shape

- `src/mir/exact_numeric_value_facts.rs` owns exact numeric per-value fact
  refresh.
- `FunctionMetadata::exact_numeric_value_facts` stores exact numeric facts for
  values produced by exact numeric `FieldGet` sites.
- `Copy` propagates existing exact numeric facts without reclassifying source
  syntax.
- `Phi` and `Select` publish exact numeric facts only when every incoming exact
  type is identical.
- mixed exact/dynamic and exact/exact mismatch control merges are recorded in
  `FunctionMetadata::exact_numeric_value_fact_rejections`.
- exact/dynamic merge rejection is order-independent, so both exact→dynamic
  and dynamic→exact incoming orders fail the same policy.

## Non-Goals

- VM exact `usize` runtime representation;
- VM arithmetic/compare/shift execution;
- backend lowering;
- typed-object exact numeric storage;
- hako_alloc field migration;
- param/return exact numeric route facts.

## Verification

- `cargo test exact_numeric_ --lib`
