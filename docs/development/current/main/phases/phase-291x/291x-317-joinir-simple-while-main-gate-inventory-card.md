---
Status: Landed
Date: 2026-04-26
Scope: JoinIR simple-while main route gate inventory
Related:
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/naming.rs
  - docs/development/current/main/phases/phase-291x/291x-316-current-pointer-thinning-card.md
---

# 291x-317: JoinIR Simple-while Main Gate Inventory

## Goal

Inventory the `LoopViewBuilder` simple-while `main` route gate before changing
code.

This is audit-only. It does not change route behavior.

## Finding

`LoopViewBuilder::try_loop_simple_while(...)` currently gates the route with:

```text
name.contains("main")
```

This is real routing policy, not a log/debug label. The route is attempted
before Case-A shape detection, so an overly broad name check can over-capture
unrelated functions whose names merely contain the substring `main`.

The intended behavior is narrower:

```text
JoinIR canonical main function
static Box method whose method name is exactly main
```

The repo already has a static method identity SSOT:

```text
src/mir/naming.rs
StaticMethodId::parse(...)
```

## Decision

Next implementation target:

```text
JoinIR simple-while main route gate helper
```

The helper should:

- accept canonical JoinIR `main`
- accept static methods where `StaticMethodId.method == "main"`
- reject unrelated names that only contain `main` as a substring
- keep the existing pinned/carrier route checks unchanged

## Non-Goals

- No simple-while route expansion.
- No `simple_while_minimal` lowering change.
- No Case-A descriptor change.
- No generic carrier-summary cleanup.

## Acceptance

```bash
cargo test -q simple_while_main_route_gate
cargo test -q test_is_loop_lowered_function
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
