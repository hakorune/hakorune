---
Status: Landed
Date: 2026-04-28
Scope: delete empty joinir::api facade and remove current-doc placement hint
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/edgecfg-fragments.md
  - src/mir/builder/control_flow/joinir/mod.rs
  - src/mir/builder/control_flow/joinir/api/mod.rs
---

# 291x-623: Empty JoinIR API Facade Prune

## Goal

Remove the empty `joinir::api` module now that it has no Rust items and no code
callers.

This is BoxShape-only cleanup. It does not change JoinIR routing, merging,
parity checks, planner/lowerer wiring, or accepted control-flow shapes.

## Evidence

The module contained only doc comments and was declared from `joinir/mod.rs`.

Current code references were zero:

```bash
rg -n "control_flow::joinir::api|joinir::api|crate::mir::builder::control_flow::joinir::api" src/mir/builder/control_flow src/mir/builder src/mir -g'*.rs'
```

The only current design mention was a placement suggestion in
`edgecfg-fragments.md`; historical investigation/archive mentions remain
unchanged.

## Boundaries

- Delete only the empty module declaration and doc-only file.
- Remove the current-doc suggestion to place EdgeCFG APIs beside `joinir/api`.
- Do not edit historical investigation/archive notes.
- Do not move any EdgeCFG, JoinIR, planner, verifier, or lowerer code.

## Acceptance

- No code references to `joinir::api` remain.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed the empty `joinir::api` facade.
- Kept current API placement guidance pointed at `edgecfg/api`.

## Verification

```bash
rg -n "control_flow::joinir::api|joinir::api|crate::mir::builder::control_flow::joinir::api" src/mir/builder/control_flow src/mir/builder src/mir -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
