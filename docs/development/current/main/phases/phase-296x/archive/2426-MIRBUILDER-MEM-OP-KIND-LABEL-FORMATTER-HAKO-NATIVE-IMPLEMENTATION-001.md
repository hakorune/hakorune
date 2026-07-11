# 2426 MIRBUILDER-MEM-OP-KIND-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for `mem_op_kind_label_formatter`.

## Scope

The implementation accepts a `MemOpKind` token and returns display/json labels.

## Non-Claims

- This is not generated from Rust.
- Memory semantics, FastMem handling, lowering, and mutation remain Rust.

## Next

`MIRBUILDER-MEM-OP-KIND-LABEL-FORMATTER-PARITY-GATE-001`
