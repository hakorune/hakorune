---
Status: Complete
Date: 2026-05-12
Scope: unsupported backend fail-fast for exact numeric runtime-check contracts.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-06f Backend Runtime-Check Contract Failfast

## Purpose

Prevent non-VM backend routes from silently ignoring
`ExactNumericRuntimeCheckContract::DynamicIntegerRange`.

294x-06d made the Rust VM execute existing contracts. 294x-06e made MIR
semantic refresh attach those contracts for real exact numeric `FieldSet`
producers. This row adds the complementary backend stop line: if a route cannot
execute/lower the contract, it must fail fast before emitting or executing code.

## Landed

- Added the shared backend guard to
  `src/mir/exact_numeric_field_contracts.rs`:
  - `exact_numeric_runtime_check_contract_count(...)`;
  - `enforce_exact_numeric_runtime_checks_supported(...)`;
  - stable tag
    `[freeze:contract][exact-numeric/runtime-check-unsupported-backend]`.
- Guarded ny-llvmc executable/object emit and llvmlite object emit before MIR
  JSON is handed to backend compilers.
- Guarded LLVM product dev/test routes that would otherwise execute a mock,
  PyVM harness, or legacy object path without consuming exact numeric
  contracts.
- Guarded WASM native-shape emit, WASM WAT/module codegen, and the WASM v2
  scaffold route. AOT inherits the stop line through the WASM compile path.
- Kept MIR JSON emit diagnostic routes open. They may serialize metadata; they
  do not claim to execute or lower the runtime-check contract.

## Contract

If a `MirModule` contains one or more
`DynamicIntegerRange` runtime-check contracts:

```text
backend != Rust VM interpreter:
  fail fast before code emission/execution
  tag = [freeze:contract][exact-numeric/runtime-check-unsupported-backend]
  include backend=<route> and contracts=<count>
```

If no runtime-check contract exists, the backend route is unchanged.

## Non-Goals

- non-VM lowering/execution of `DynamicIntegerRange`;
- typed-object exact numeric storage;
- exact `usize` arithmetic, comparison, or shift semantics;
- blocking MIR JSON diagnostic export;
- allocator/hako_alloc migration.

## Verification

- `cargo test -q exact_numeric_field_contracts --lib`
- `cargo check --bin hakorune`

## Next

1. add exact numeric operation policy rows;
2. add VM exact `usize` values/ops where policy is fixed;
3. lower backend support only when the route can actually preserve the exact
   numeric semantics.
