---
Status: Landed
Date: 2026-04-26
Scope: JoinIR simple-while main route gate helper
Related:
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - src/mir/naming.rs
  - docs/development/current/main/phases/phase-291x/291x-317-joinir-simple-while-main-gate-inventory-card.md
---

# 291x-318: JoinIR Simple-while Main Gate Helper

## Goal

Replace the simple-while `main` substring route gate with a structured helper.

This is behavior-narrowing BoxShape cleanup: the route still accepts real
`main` entries, but no longer treats arbitrary names containing `main` as
simple-while candidates.

## Change

Added:

```text
is_simple_while_main_route_candidate(name)
```

The helper accepts:

```text
canonical JoinIR function name: main
StaticMethodId whose method is exactly main
```

The helper rejects unrelated names where `main` is only a substring, such as:

```text
domain_loop/0
Main.remaining/0
Main.not_main/0
```

## Preserved Behavior

- `main` remains accepted.
- `Main.main/N` remains accepted.
- The pinned/carrier route checks are unchanged.
- `simple_while_minimal` lowering is unchanged.

## Non-Goals

- No route expansion.
- No Case-A descriptor change.
- No generic carrier-summary cleanup.
- No static method identity migration outside this gate.

## Validation

```bash
cargo test -q simple_while_main_route_gate
cargo test -q test_is_loop_lowered_function
```
