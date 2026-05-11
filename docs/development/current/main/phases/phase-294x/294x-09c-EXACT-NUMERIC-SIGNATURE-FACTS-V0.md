---
Status: Complete
Date: 2026-05-12
Scope: MIR-owned exact numeric signature facts for declared parameters and
  return annotations.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-09c Exact Numeric Signature Facts V0

## Purpose

Bridge the remaining metadata gap between AST-preserved parameter/return type
annotations and MIR-owned exact numeric facts before VM reference exact `usize`
execution starts.

This row keeps the same owner order:

```text
AST annotation text
MIR declared signature metadata
MIR-owned exact numeric facts
VM reference execution later
backend fail-fast/lowering remains visible
hako_alloc migration waits
```

## Landed Shape

- MIR function metadata preserves declared parameter names and declared type
  annotation text where AST lowering already has it.
- MIR function metadata preserves accepted return type annotation text.
- `src/mir/exact_numeric_value_facts.rs` seeds exact numeric value facts for
  declared exact numeric parameters.
- Return annotations publish a function-level exact numeric return fact, but do
  not validate return values or change runtime lowering in this row.
- Static and instance method lowering pass AST `ParamDecl` / return annotation
  metadata into MIR without changing `FunctionSignature` or `MirType`.

## Non-Goals

- VM exact `usize` runtime representation;
- VM arithmetic/compare/shift execution;
- backend lowering;
- typed-object exact numeric storage;
- exact numeric return-value verification;
- hako_alloc field migration.

## Verification

- `cargo test exact_numeric_ --lib`
- `cargo check --release --bin hakorune`
