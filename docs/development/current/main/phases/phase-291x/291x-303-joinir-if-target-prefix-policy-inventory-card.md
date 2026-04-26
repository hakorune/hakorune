---
Status: Landed
Date: 2026-04-26
Scope: JoinIR if-target prefix policy inventory
Related:
  - src/mir/join_ir_vm_bridge_dispatch/targets.rs
  - src/mir/join_ir/lowering/if_lowering_router.rs
  - src/mir/join_ir/lowering/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-302-joinir-if-target-exact-allowlist-ssot-card.md
---

# 291x-303: JoinIR If-target Prefix Policy Inventory

## Goal

Inventory the remaining prefix-based JoinIR if-lowering target policy after the
exact target mirror was removed.

This is audit-only. It does not change accepted targets.

## Findings

Exact targets are now owned by:

```text
src/mir/join_ir_vm_bridge_dispatch/targets.rs
JOINIR_IF_TARGETS
```

Remaining prefix policy still lives in lowering call sites:

```text
src/mir/join_ir/lowering/if_lowering_router.rs
src/mir/join_ir/lowering/mod.rs
```

Current prefix families:

```text
IfSelectTest.*
IfSelectLocalTest.*
IfMergeTest.*
IfToplevelTest.*
Stage1JsonScannerTestBox.*
Stage1* gated by HAKO_JOINIR_STAGE1
```

`is_joinir_if_toplevel_target(...)` only uses the toplevel subset:

```text
IfSelectTest.*
IfToplevelTest.*
IfMergeTest.*
```

This difference should be preserved until a dedicated behavior decision changes
it.

## Decision

Next implementation target:

```text
JoinIR if-target prefix policy helper split
```

Move the prefix strings into small helpers in `targets.rs`:

```text
is_if_lowering_prefix_target(name, stage1_enabled)
is_if_toplevel_prefix_target(name)
```

Callers keep their current behavior by choosing the appropriate helper.

## Non-Goals

- No target expansion.
- No target deletion.
- No change to the Stage1 env gate.
- No type-hint policy cleanup in this slice.

## Acceptance

```bash
cargo test -q joinir_frontend_
cargo test -q mir_joinir_if_select
cargo test -q test_is_loop_lowered_function
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
