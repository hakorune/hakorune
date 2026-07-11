---
Status: Landed
Date: 2026-04-28
Scope: centralize parser member typed-header and property-body parsing
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/members/syntax.rs
  - src/parser/declarations/box_def/members/fields.rs
  - src/parser/declarations/box_def/members/properties.rs
---

# 291x-656: Parser Member Syntax SSOT

## Goal

Make the unified-member parser keep one small owner for typed member headers
and property body parsing.

This is BoxShape cleanup. It must not add syntax, change synthetic property
method names, or alter MIR lowering.

## Evidence

After the unified-member property cleanup burst, `fields.rs` and
`properties.rs` still each parsed parts of the same member shape:

```text
name : Type
=> expr
{ block } [catch|cleanup]
```

That left three local sources for the same syntax boundary:

- stored/computed fields parsed optional declared types in `fields.rs`;
- `get` reused the field parser but owned its own required body error;
- `once` / `birth_once` parsed the typed name and body in `properties.rs`.

## Decision

Add a parser-member syntax helper that owns:

- optional and required declared type parsing;
- required typed member names;
- property body parsing for arrow and block forms;
- the policy for whether postfix handlers attach only to block bodies or to
  both arrow and block bodies.

Keep emission ownership in `property_emit.rs`.

## Boundaries

- Do not change `computed` / `once` / `birth_once` synthetic method names.
- Do not change stored-field metadata behavior.
- Do not add a new accepted member syntax.
- Do not touch MIR property-read lowering.

## Acceptance

```bash
cargo fmt
cargo test parser_unified_members_get --lib
cargo test parser_unified_members_property_emit --lib
cargo test parser_weak_field_contract --lib
cargo test parser_birth_once_cycle --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added `members::syntax` as the parser-side SSOT for typed member headers and
  property bodies.
- Reused that syntax helper from stored/computed field parsing and
  once/birth_once property parsing.
- Kept property emission in `property_emit.rs` and left MIR lowering untouched.
