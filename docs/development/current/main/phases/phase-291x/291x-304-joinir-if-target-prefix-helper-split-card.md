---
Status: Landed
Date: 2026-04-26
Scope: JoinIR if-target prefix helper split
Related:
  - src/mir/join_ir_vm_bridge_dispatch/targets.rs
  - src/mir/join_ir/lowering/if_lowering_router.rs
  - src/mir/join_ir/lowering/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-303-joinir-if-target-prefix-policy-inventory-card.md
---

# 291x-304: JoinIR If-target Prefix Helper Split

## Goal

Move JoinIR if-target prefix strings out of lowering call sites and into the
target policy owner.

This is behavior-preserving BoxShape cleanup.

## Change

Added helper functions in `targets.rs`:

```text
is_if_lowering_prefix_target(name, stage1_enabled)
is_if_toplevel_prefix_target(name)
```

Updated call sites:

```text
try_lower_if_to_joinir(...)
is_joinir_if_toplevel_target(...)
```

## Preserved Behavior

The lowering router still accepts:

```text
IfSelectTest.*
IfSelectLocalTest.*
IfMergeTest.*
IfToplevelTest.*
Stage1JsonScannerTestBox.*
Stage1* gated by HAKO_JOINIR_STAGE1
```

The toplevel target check still accepts only its previous prefix subset:

```text
IfSelectTest.*
IfToplevelTest.*
IfMergeTest.*
```

## Non-Goals

- No target expansion.
- No target deletion.
- No type-hint policy cleanup.
- No bridge execution route change.

## Validation

```bash
cargo test -q joinir_frontend_
cargo test -q mir_joinir_if_select
cargo test -q test_is_loop_lowered_function
tools/checks/dev_gate.sh quick
git diff --check
```
