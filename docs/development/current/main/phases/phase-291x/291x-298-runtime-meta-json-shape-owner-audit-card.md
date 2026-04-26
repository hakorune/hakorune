---
Status: Landed
Date: 2026-04-26
Scope: runtime/meta JsonShapeToMap owner audit
Related:
  - lang/src/runtime/meta/json_shape_parser.hako
  - lang/src/runtime/meta/README.md
  - src/mir/join_ir_vm_bridge_dispatch/targets.rs
  - src/mir/join_ir/frontend/ast_lowerer/route.rs
  - src/mir/join_ir/lowering/if_lowering_router.rs
  - src/mir/join_ir/lowering/mod.rs
  - src/tests/joinir_frontend_if_select.rs
---

# 291x-298: runtime/meta JsonShapeToMap Owner Audit

## Goal

Decide whether `JsonShapeToMap` is stale runtime/meta table debt or an active
support export that must be quarantined rather than deleted.

This is BoxShape cleanup only. It does not add a CoreMethod row, a JoinIR
accepted shape, or a lowering behavior change.

## Audit Target

```text
lang/src/runtime/meta/json_shape_parser.hako
selfhost.meta.JsonShapeToMap
JsonShapeToMap._read_value_from_pair/1
```

## Findings

`JsonShapeToMap` is active.

Active references:

```text
src/mir/join_ir_vm_bridge_dispatch/targets.rs
src/mir/join_ir/frontend/ast_lowerer/route.rs
src/mir/join_ir/lowering/if_lowering_router.rs
src/mir/join_ir/lowering/mod.rs
src/tests/joinir_frontend_if_select.rs
```

The active surface is specifically:

```text
JsonShapeToMap._read_value_from_pair/1
```

It is used as a JoinIR if-lowering / frontend fixture and bridge target. It is
not a CoreMethod semantic contract owner and not a `mir_call` route/need/surface
policy table.

## Decision

Do not delete `JsonShapeToMap` in this slice.

Classify it as:

```text
active support / JoinIR fixture utility
```

It should be quarantined away from the semantic-table root so that
`lang/src/runtime/meta/` remains visually dominated by compiler semantic
contracts (`CoreMethodContractBox` plus generated manifest) instead of support
utilities.

## Next Slice

Move or quarantine the implementation under a support path while preserving the
public export and bridge function name:

```text
selfhost.meta.JsonShapeToMap
JsonShapeToMap._read_value_from_pair/1
```

The move must refresh the stage1 embedded module snapshot and run the JoinIR
frontend test that covers `_read_value_from_pair/1`.

## Non-Goals

- No deletion of `JsonShapeToMap`.
- No rename of the public bridge function.
- No JoinIR route expansion.
- No `.inc` classifier growth.
- No CoreMethodContract row addition.

## Acceptance

```bash
rg -n "JsonShapeToMap|json_shape_parser|_read_value_from_pair" lang src tools crates apps docs/development/current/main/phases/phase-291x --glob '!target/**' --glob '!*.json'
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
