---
Status: Landed
Date: 2026-04-26
Scope: JoinIR if-target exact allowlist SSOT
Related:
  - src/mir/join_ir_vm_bridge_dispatch/targets.rs
  - src/mir/join_ir/lowering/if_lowering_router.rs
  - docs/development/current/main/phases/phase-291x/291x-301-post-runtime-meta-cleanup-inventory-card.md
---

# 291x-302: JoinIR If-target Exact Allowlist SSOT

## Goal

Remove the exact function-name mirror from `try_lower_if_to_joinir(...)`.

`JOINIR_IF_TARGETS` already owns the exact if-lowering target list, so the
lowering router should consume the table instead of repeating exact names.

## Change

Replaced the router-local exact match:

```text
JsonShapeToMap._read_value_from_pair/1
Stage1JsonScannerBox.value_start_after_key_pos/2
```

with:

```rust
crate::mir::join_ir_vm_bridge_dispatch::is_if_lowered_function(&func.signature.name)
```

## Boundary

This slice intentionally keeps existing prefix gates unchanged:

```text
IfSelectTest.*
IfSelectLocalTest.*
IfMergeTest.*
IfToplevelTest.*
Stage1JsonScannerTestBox.*
Stage1* with HAKO_JOINIR_STAGE1
```

Those are a separate policy cleanup because they are not exact target rows.

## Non-Goals

- No JoinIR accepted-shape expansion.
- No bridge execution route change.
- No public function rename.
- No test-prefix policy change.

## Validation

```bash
cargo test -q joinir_frontend_
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
