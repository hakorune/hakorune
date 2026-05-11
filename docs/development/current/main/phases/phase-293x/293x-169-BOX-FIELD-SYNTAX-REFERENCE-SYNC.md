---
Status: Complete
Date: 2026-05-12
Scope: Box field syntax reference sync.
Related:
  - docs/reference/language/EBNF.md
  - docs/reference/language/LANGUAGE_REFERENCE_2025.md
  - docs/reference/language/types.md
  - docs/reference/boxes-system/README.md
---

# 293x-169 Box Field Syntax Reference Sync

## Goal

Make the language reference match the current Box field design:

- `field` is the simple untyped stored field form.
- `field: Type` is a stored field with declared-type metadata.
- `init { field }` is legacy compatibility, not the new-code default.

This keeps Hakorune's "easy first, fast when explicit" surface clear without
turning every simple field into a mandatory type declaration.

## Changes

- Updated the EBNF stored-field grammar to include bare `field`.
- Clarified that `field: Type` metadata is not a general runtime assignment
  type check today; it is input for typed-object planning, optimization,
  verification, and documentation.
- Clarified that portable initialization should use `birth(...)` assignment.
- Updated the long-form language reference and Box docs to present the three
  forms as simple / explicit / legacy.

## Proof

```bash
cargo test -q get_identifier_on_previous_line_stays_stored_field
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

M167 should continue with Unified Members from the start. General `.hako` code
may use bare `field`; low-level `hako_alloc` state boxes may keep `field: Type`
where declared metadata helps future optimizer/verifier work.
