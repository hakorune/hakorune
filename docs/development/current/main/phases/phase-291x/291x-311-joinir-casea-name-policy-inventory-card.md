---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A name-policy inventory
Related:
  - src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/join_ir/lowering/loop_to_join/case_a_entrypoints.rs
  - src/mir/join_ir_vm_bridge_dispatch/targets.rs
  - docs/development/current/main/phases/phase-291x/291x-310-joinir-frontend-route-descriptor-table-card.md
---

# 291x-311: JoinIR Case-A Name-policy Inventory

## Goal

Inventory the remaining Case-A loop name-policy seam before changing code.

This is audit-only. It does not change accepted Case-A targets or fallback
dispatch behavior.

## Findings

Case-A minimal target acceptance lives in:

```text
src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
is_case_a_minimal_target(...)
```

The same four exact function names are duplicated in fallback dispatch:

```text
src/mir/join_ir/lowering/loop_view_builder.rs
dispatch_by_name(...)
```

Current Case-A minimal target subset:

```text
Main.skip/1
FuncScannerBox.trim/1
FuncScannerBox.append_defs/2
Stage1UsingResolverBox.resolve_for_source/5
```

This subset overlaps with `JOINIR_TARGETS`, but it is not identical. The loop
target table also contains Stage-B lowerers:

```text
StageBBodyExtractorBox.build_body_src/2
StageBFuncScannerBox.scan_all_boxes/1
```

Do not reuse `JOINIR_TARGETS` directly as the Case-A minimal policy. Case-A
needs its own smaller owner because it maps each row to a specific generic
Case-A lowerer.

`loop_to_join/case_a_entrypoints.rs` also repeats function names, but those are
context labels passed into the lowerer wrappers, not the acceptance owner. Keep
that seam separate from the first cleanup.

## Decision

Next implementation target:

```text
JoinIR Case-A target descriptor table split
```

Create a Case-A local descriptor table that owns:

```text
function name
Case-A route/lowerer kind
```

Initial consumers:

```text
is_case_a_minimal_target(...)
```

Later consumer:

```text
loop_view_builder::dispatch_by_name(...)
```

Split these into two cards so acceptance predicate cleanup and fallback
dispatch cleanup do not mix.

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No Stage-B target reclassification.
- No shape-based lowering change.
- No bridge target table change.

## Acceptance

```bash
cargo test -q test_is_loop_lowered_function
cargo test -q joinir_frontend_
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
