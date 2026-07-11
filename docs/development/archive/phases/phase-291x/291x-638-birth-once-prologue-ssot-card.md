---
Status: Landed
Date: 2026-04-28
Scope: centralize birth_once constructor prologue emission
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/members/README.md
  - src/parser/declarations/box_def/members/property_emit.rs
  - src/parser/declarations/box_def/members/methods.rs
  - src/parser/declarations/box_def/members/mod.rs
  - src/tests/parser_unified_members_property_emit.rs
---

# 291x-638: Birth Once Prologue SSOT

## Goal

Make `birth_once` construction-time initialization match the unified member
contract through one parser-side owner.

This is BoxShape cleanup. It does not add syntax, change synthetic method names,
or add runtime property lookup.

## Evidence

After 291x-637, `property_emit.rs` owns synthetic property methods:

```text
birth_once -> __compute_birth_<name> + __get_birth_<name>
```

The construction-time prologue was still emitted manually in `methods.rs`.
That left two structural problems:

- synthetic `birth_once` method bodies and eager initializer AST construction
  had separate owners;
- canonical `birth(...)` constructors are parsed through `constructors.rs`, so
  method-level prologue injection does not cover the actual user-birth path.

## Decision

`property_emit.rs` owns both parts of parser-side `birth_once` lowering:

- synthetic compute/getter methods;
- constructor prologue statements that call `__compute_birth_<name>` and store
  the value in the existing hidden `__birth_<name>` field.

Box parsing applies the prologue after the full member list is parsed, using the
collected declaration-order `birth_once` property list. This keeps properties
declared before or after `birth(...)` on the same path.

## Boundaries

- Keep accepted syntax unchanged.
- Keep hidden storage names unchanged.
- Keep declaration-order evaluation.
- Do not add MIR optimization or direct property IR.
- Do not move `once` cache/poison behavior in this card.

## Acceptance

```bash
cargo fmt
cargo test parser_unified_members_property_emit --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Moved `birth_once` eager initializer AST construction into
  `property_emit.rs`.
- Removed the method-parser special case for `birth_once` prologue injection.
- Applied `birth_once` prologues after the full box member list is parsed, so
  properties declared before or after `birth(...)` share declaration-order
  lowering.
- Synthesized `birth/0` when a box has `birth_once` properties but no user
  birth constructor.
- Added parser tests for canonical `birth(...)` prologue insertion and the
  no-user-birth constructor synthesis path.
