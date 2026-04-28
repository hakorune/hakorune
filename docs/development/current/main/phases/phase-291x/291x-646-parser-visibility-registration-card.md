---
Status: Landed
Date: 2026-04-28
Scope: centralize parser visibility member registration
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/members/fields.rs
  - src/tests/parser_unified_members_get.rs
---

# 291x-646: Parser Visibility Registration

## Goal

Keep visibility tracking for unified members in one parser-side helper.

This is BoxShape cleanup. It does not change `public`/`private` syntax or
stored/get property classification.

## Evidence

`try_parse_visibility_block_or_single(...)` had repeated `public_fields` /
`private_fields` insertion logic across visibility blocks, `weak` sugar,
contextual `get`, and ordinary header-first members.

That made the member parser hold duplicate partial truth for one decision:

```text
visibility keyword + member name -> public_fields/private_fields entry
```

## Decision

Add a small `record_visibility(...)` helper in `fields.rs` and route all
visibility member-name recording through it.

Keep contextual `get` handling unchanged:

- `public get size: Type => expr` records property name `size`;
- `private get hidden: Type => expr` records property name `hidden`;
- `public get: Type` remains a stored field named `get`.

## Boundaries

- Do not change property getter emission.
- Do not change stored-field parsing.
- Do not move visibility parsing to a new module in this card.
- Do not alter gate behavior.

## Acceptance

```bash
cargo fmt
cargo test parser_unified_members_get --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Centralized visibility member-name recording in `record_visibility(...)`.
- Removed the dead fallback branch after header-first visibility parsing.
- Added parser coverage for `private get ...` and visible stored field
  `public get: Type`.
