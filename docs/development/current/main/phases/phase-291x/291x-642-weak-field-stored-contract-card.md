---
Status: Landed
Date: 2026-04-28
Scope: keep weak member parsing on stored-field syntax only
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/reference/language/EBNF.md
  - src/parser/declarations/box_def/members/fields.rs
  - src/tests/parser_weak_field_contract.rs
---

# 291x-642: Weak Field Stored Contract

## Goal

Keep `weak` member parsing aligned with the language contract:

```text
weak_stored := 'weak' IDENT ( ':' TYPE )?
```

This is BoxShape cleanup. It does not add syntax or change weak runtime
semantics.

## Evidence

`parse_weak_field` delegated to the generic header-first
field-or-property parser. That parser also accepts computed property bodies
under unified members (`=> expr` or `{ ... }`), so a weak member could travel
through the computed-property path and still be recorded in `weak_fields`.

That mixes two property classes:

- `weak` is stored-field metadata;
- computed/get properties synthesize getter methods and are not slots.

## Decision

`parse_weak_field` parses only the weak stored-field head and rejects initializer
or computed bodies immediately.

Visibility sugar (`public weak x`, `private weak x`) continues to delegate to
the same weak-field parser.

## Boundaries

- Keep `weak name` and `weak name: Type` accepted.
- Keep visibility weak sugar accepted.
- Do not change `init { weak name }` compatibility parsing.
- Do not add weak computed/once/birth_once semantics.

## Acceptance

```bash
cargo fmt
cargo test parser_weak_field_contract --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Replaced `parse_weak_field`'s generic header-first field/property delegation
  with a weak stored-field parser.
- Kept `weak name` and `weak name: Type` accepted.
- Rejected weak initializer/computed tails (`=`, `=>`, block,
  `catch/cleanup`) before they can synthesize getter methods.
- Added focused parser tests for stored weak metadata, visibility weak sugar,
  and computed-tail rejection.
