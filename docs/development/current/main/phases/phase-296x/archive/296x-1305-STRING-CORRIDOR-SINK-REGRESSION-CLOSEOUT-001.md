# 296x-1305 STRING-CORRIDOR-SINK-REGRESSION-CLOSEOUT-001

Status: closed  
Date: 2026-06-19  
Output contract: `string-corridor-sink-regression-closeout-v0`

## Decision

`STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001` is closed.

The touched-module regression was fixed without benchmark/source/function-name
branches:

- benchmark tests now assert the existing semantic region-mapping contract
  instead of raw `ValueId` numbers.
- method-call receiver physical ABI differences are normalized through a
  read-only `MethodCallOperandView`.
- string corridor recognition remains the string-shape owner; physical method
  operand encoding is owned by `mir::ssot::method_call`.

## Evidence

```bash
cargo test -q operand_view
cargo test -q string_corridor_sink
cargo test -q string_kernel_plan
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

All commands were green for the closeout slice.

## Next

Open the next upstream cleanup:

```text
PHI-INPUT-REMAT-OPERAND-MEMO-001
```

Scope:

- add predecessor-local memoization to PHI input rematerialization.
- same predecessor + same original `ValueId` must reuse the same materialized
  `ValueId`.
- cycle handling remains fail-closed.
- do not expand accepted rematerialization shapes in this row.

## Stop Line

- do not add string-specific route branches.
- do not restore raw `ValueId` benchmark assertions.
- do not make backend consumers read diagnostic optimization hints as
  correctness evidence in this row.
- do not change app-front / rust-subset-to-hako behavior in this row.

summary=ok
