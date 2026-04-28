---
Status: Landed
Date: 2026-04-28
Scope: centralize Box member postfix catch/cleanup gate checks
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/config/env/parser_flags.rs
  - src/parser/declarations/box_def/members/postfix.rs
  - src/parser/declarations/box_def/members/constructors.rs
  - src/tests/parser_method_postfix.rs
---

# 291x-643: Member Postfix Gate SSOT

## Goal

Make Box member postfix `catch/cleanup` acceptance pass through one gate owner.

This is BoxShape cleanup. It does not add syntax; it removes accidental gate
bypass for method/constructor postfix handlers.

## Evidence

After 291x-640, `postfix.rs` owned the TryCatch parser, but two required
postfix entrypoints still accepted `catch/cleanup` whenever the token appeared:

- method-level postfix after the last parsed method;
- `init(...)` constructor postfix via `wrap_with_required_postfix`.

`wrap_with_optional_postfix` already checked the Stage-3 parser gate, so the
gate truth was split.

## Decision

`postfix.rs` owns the Box member postfix gate. Required postfix entrypoints
fail fast when member postfix syntax is disabled.

The gate uses the existing `method_catch()` compatibility function, which is
enabled by Stage-3 or by the legacy explicit method-catch env.

## Boundaries

- Keep enabled behavior unchanged.
- Keep legacy explicit method-catch env behavior unchanged.
- Do not change expression/block postfix parsing outside Box members.
- Do not alter TryCatch AST shape.

## Acceptance

```bash
cargo fmt
cargo test parser_method_postfix --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added a single member-postfix gate check in `postfix.rs`.
- Routed required method/constructor postfix parsing through that gate.
- Kept enabled behavior intact via existing `method_catch()` compatibility
  policy.
- Added environment-locked parser tests for enabled and disabled method/init
  postfix paths.
