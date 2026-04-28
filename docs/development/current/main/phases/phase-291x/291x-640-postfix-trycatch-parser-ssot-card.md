---
Status: Landed
Date: 2026-04-28
Scope: centralize Box member postfix catch/cleanup parsing
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/members/postfix.rs
  - src/parser/declarations/box_def/members/constructors.rs
  - src/tests/parser_method_postfix.rs
---

# 291x-640: Postfix TryCatch Parser SSOT

## Goal

Keep Box member postfix `catch/cleanup` parsing in one owner.

This is BoxShape cleanup. It does not add syntax or change Stage-3 runtime
semantics.

## Evidence

`postfix.rs` already owned shared wrapping for unified member bodies and
method-level postfix handlers, but `init(...)` constructor parsing still had a
hand-written copy of the `catch/cleanup` parser.

That duplicated:

- catch parameter parsing;
- duplicate-catch rejection;
- cleanup parsing;
- `ASTNode::TryCatch` construction.

## Decision

`members/postfix.rs` is the parser-side SSOT for Box member postfix
`catch/cleanup` parsing and TryCatch wrapping.

Constructor parsing may decide when postfix is allowed, but it must delegate the
actual parser and AST construction to `postfix.rs`.

## Boundaries

- Keep accepted syntax unchanged.
- Keep method-level duplicate-wrapper detection unchanged.
- Keep constructor names and arity keys unchanged.
- Do not change block-expression postfix parsing outside Box members.

## Acceptance

```bash
cargo fmt
cargo test parser_method_postfix --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added one shared parser in `postfix.rs` for Box member postfix
  `catch/cleanup` and `TryCatch` construction.
- Routed `init(...)` constructor postfix parsing through the shared helper.
- Routed method-level postfix wrapping through the same parser while preserving
  duplicate-wrapper detection.
- Added a constructor postfix regression test.
