---
Status: Landed
Date: 2026-04-28
Scope: document and implement canonical get syntax for computed unified members
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/reference/language/EBNF.md
  - docs/reference/language/LANGUAGE_REFERENCE_2025.md
  - docs/reference/language/README.md
  - docs/reference/ir/json_v0.md
  - src/parser/declarations/box_def/mod.rs
  - src/parser/declarations/box_def/members/fields.rs
  - src/tests/parser_unified_members_get.rs
  - apps/tests/unified_members_basic.hako
---

# 291x-636: Unified Member Get Computed Syntax

## Goal

Make `get name: Type { ... }` / `get name: Type => expr` the canonical
surface syntax for computed unified members.

This is one accepted shape. It keeps the existing `name: Type { ... }` /
`name: Type => expr` shorthand as compatibility syntax and does not change
AST, JSON v0, MIR, getter naming, or property read lowering.

## Decision

Internal property classification remains:

```text
stored / computed / once / birth_once
```

User-facing docs should introduce the surface as:

```text
field / get / once / birth_once
```

`get` is contextual only at a Box member head. These shapes keep their existing
meaning:

- `get: Type` is a stored field named `get`.
- `get(...) { ... }` is a method named `get`.
- Legacy `name: Type { ... }` remains a computed property shorthand.

## Boundaries

- Do not reserve `get` globally in the tokenizer.
- Do not add generic runtime property lookup.
- Do not change synthetic getter names (`__get_name`).
- Do not change once or birth_once lowering.
- Do not add optimization rules in this card.

## Result

- Updated language docs to mark `get` as canonical computed syntax.
- Added parser support for contextual `get` computed members.
- Updated the unified members basic smoke fixture to use canonical `get`.
- Added parser regression tests for `get` computed properties, stored field and
  method compatibility, line-seam compatibility, and `public get` visibility.

## Verification

```bash
cargo fmt
cargo test parser_unified_members_get --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
