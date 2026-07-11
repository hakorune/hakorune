---
Status: Landed
Date: 2026-04-28
Scope: centralize unified member synthetic property method emission
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/members/README.md
  - src/parser/declarations/box_def/members/property_emit.rs
  - src/parser/declarations/box_def/members/fields.rs
  - src/parser/declarations/box_def/members/properties.rs
  - src/parser/declarations/box_def/members/mod.rs
  - src/mir/builder.rs
  - src/mir/builder/decls.rs
  - src/mir/builder/fields.rs
  - src/tests/parser_unified_members_get.rs
  - src/tests/parser_unified_members_property_emit.rs
---

# 291x-637: Unified Member Property Emitter SSOT

## Goal

Make unified member internals cleaner by separating syntax parsing from
synthetic property method construction.

This is BoxShape cleanup. It does not add syntax, reserve `get`, change JSON
v0/MIR shape, or add runtime property lookup.

## Evidence

The previous layout mixed two responsibilities:

- `fields.rs` parsed fields and also constructed computed getter functions.
- `properties.rs` parsed once/birth_once and duplicated synthetic method bodies
  between header-first and block-first paths.

MIR builder also duplicated the getter-name contract:

- `decls.rs` parsed `__get_*` method names by local string-prefix checks.
- `fields.rs` reconstructed getter method names from `PropertyKind` locally.

## Decision

`property_emit.rs` is the parser-side SSOT for synthetic property method bodies
and names:

```text
computed   -> __get_<name>
once       -> __compute_once_<name> + __get_once_<name>
birth_once -> __compute_birth_<name> + __get_birth_<name>
```

Parser entry modules decide which syntax was written. They do not own the
synthetic method AST bodies.

`PropertyKind` owns the MIR-side getter-name contract used by registration and
field-read lowering.

## Boundaries

- Keep accepted syntax unchanged.
- Keep synthetic method names unchanged.
- Keep once cache/poison and birth_once storage behavior unchanged.
- Do not add optimization or `PropertyRead` IR in this card.
- Do not reserve `get` in the tokenizer.

## Result

- Added `members/property_emit.rs` and `members/README.md`.
- Reused one emitter for computed, once, and birth_once synthetic methods.
- Removed duplicated once/birth_once synthetic method bodies from
  `properties.rs`.
- Routed block-first computed members through the computed emitter instead of
  the birth_once path.
- Added `PropertyKind` helper methods for getter-name parsing/emission and
  used existing `CompilationContext` registration/read helpers.
- Added focused parser tests for the shared emitter and block-first computed
  contract.

## Verification

```bash
cargo fmt
cargo test parser_unified_members_get --lib
cargo test parser_unified_members_property_emit --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
