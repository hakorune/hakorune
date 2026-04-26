---
Status: Landed
Date: 2026-04-26
Scope: post-runtime-meta cleanup inventory
Related:
  - src/mir/join_ir_vm_bridge_dispatch/targets.rs
  - src/mir/join_ir/lowering/if_lowering_router.rs
  - src/mir/join_ir/lowering/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-300-runtime-meta-root-closeout-card.md
---

# 291x-301: Post-runtime-meta Cleanup Inventory

## Goal

Select the next small BoxShape cleanup after the `runtime/meta` root closeout.

## Findings

The next cleanup seam is JoinIR if-lowering target ownership.

Exact active targets are listed in `JOINIR_IF_TARGETS`:

```text
JsonShapeToMap._read_value_from_pair/1
Stage1JsonScannerBox.value_start_after_key_pos/2
```

The same exact names are also mirrored in `try_lower_if_to_joinir(...)`.
That means the target table says it is SSOT, but the router still carries a
manual exact allowlist.

## Decision

Next implementation target:

```text
JoinIR if-target exact allowlist SSOT
```

Use the existing target table consumer:

```text
crate::mir::join_ir_vm_bridge_dispatch::is_if_lowered_function(...)
```

to remove the exact string mirror in the lowering router.

## Non-Goals

- Do not change test-prefix behavior in this slice.
- Do not expand JoinIR accepted shapes.
- Do not rename `JsonShapeToMap._read_value_from_pair/1`.
- Do not change bridge execution routes.

## Acceptance

```bash
rg -n "JsonShapeToMap\\._read_value_from_pair/1|Stage1JsonScannerBox\\.value_start_after_key_pos/2|JOINIR_IF_TARGETS|is_if_lowered_function" src/mir src/tests --glob '!target/**'
bash tools/checks/current_state_pointer_guard.sh
cargo test -q joinir_frontend_
git diff --check
```
