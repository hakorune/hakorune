---
Status: Landed
Date: 2026-04-26
Scope: JoinIR Case-A context-label string inventory
Related:
  - src/mir/join_ir/lowering/loop_to_join/case_a_entrypoints.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - docs/development/current/main/phases/phase-291x/291x-311-joinir-casea-name-policy-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-312-joinir-casea-target-descriptor-table-card.md
  - docs/development/current/main/phases/phase-291x/291x-313-joinir-casea-fallback-dispatch-descriptor-card.md
---

# 291x-314: JoinIR Case-A Context-label String Inventory

## Goal

Inventory the remaining exact function-name strings in Case-A wrapper entry
points before changing code.

This is audit-only. It does not change accepted Case-A targets, lowerer
selection, or Stage-B bridge behavior.

## Findings

The remaining strings in:

```text
src/mir/join_ir/lowering/loop_to_join/case_a_entrypoints.rs
```

are passed as `func_name` context labels into:

```text
LoopToJoinLowerer::lower(...)
```

Those labels are consumed for:

```text
strict/dev diagnostics
LoopScopeShape Case-A analyze routing
generic Case-A name filter when NYASH_JOINIR_LOWER_GENERIC is disabled
LoopViewBuilder fallback dispatch
```

The four Case-A minimal labels overlap with the descriptor-owned acceptance
set:

```text
Main.skip/1
FuncScannerBox.trim/1
FuncScannerBox.append_defs/2
Stage1UsingResolverBox.resolve_for_source/5
```

They should consume the descriptor table introduced in `291x-312`, not repeat
string literals in wrapper methods.

Two Stage-B wrapper labels are not Case-A minimal acceptance rows:

```text
StageBBodyExtractorBox.build_body_src/2
StageBFuncScannerBox.scan_all_boxes/1
```

They are bridge/generic-hook context labels. Keep them separate from
`CASE_A_MINIMAL_TARGETS`.

## Decision

Next implementation target:

```text
JoinIR Case-A context-label helper cleanup
```

The cleanup should:

- add a descriptor consumer helper for minimal target context labels
- keep Stage-B labels as a small local label table/constant set
- not add or remove accepted Case-A targets
- not fold Stage-B labels into Case-A minimal policy

## Non-Goals

- No Case-A target expansion.
- No Case-A target deletion.
- No Stage-B target reclassification.
- No route-shape behavior change.
- No strict/dev behavior change.

## Acceptance

```bash
cargo test -q case_a_minimal_target
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
