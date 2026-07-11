---
Status: Complete
Date: 2026-05-12
Scope: Box stored field initializer syntax and lowering.
Related:
  - docs/reference/language/EBNF.md
  - docs/reference/language/LANGUAGE_REFERENCE_2025.md
  - src/parser/declarations/box_def/members/fields.rs
  - src/parser/declarations/box_def/members/property_emit.rs
  - apps/tests/unified_members_field_initializer.hako
---

# 293x-171 Box Stored Field Initializer

## Decision

Accepted: Box stored fields may initialize directly at declaration site:

```hako
box Counter {
  count = 41
  name: StringBox = "Nya"
}
```

Both forms lower to constructor prologue assignments before the user `birth`
body, in declaration order:

```hako
me.count = 41
me.name = "Nya"
```

This keeps the simple path short while preserving the explicit `field: Type`
metadata path for optimizer/verifier work.

## Non-goals

- No runtime field type enforcement for `field: Type`.
- No initializer support for `weak field`.
- No change to `init { field }`; it remains legacy field-slot compatibility.

## Proof

```bash
cargo test -q stored_field_initializers_generate_birth_prologue
NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --backend vm apps/tests/unified_members_field_initializer.hako
NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --backend vm apps/tests/unified_members_basic.hako
```
