---
Status: Landed
Date: 2026-04-24
Scope: Promote `StringBox.toUpper` / `toLower` into the stable String surface catalog and retire their TypeRegistry-only extras status.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - src/boxes/basic/string_surface_catalog.rs
  - src/runtime/type_registry.rs
  - src/tests/mir_corebox_router_unified.rs
  - tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh
---

# StringBox Case Conversion Card

## Decision

Treat StringBox case conversion as part of the same stable String surface story:

```text
StringBox catalog rows
  -> toUpper / toLower
  -> keep JS-style aliases toUpperCase / toLowerCase
  -> Route::Unified for known StringBox/String receivers
```

This card does not widen the String family beyond case conversion. It only
promotes the already implemented rows from TypeRegistry extras into the catalog,
router, and smoke-locked stable surface.

## Current Facts

- runtime support already exists in `src/boxes/string_box.rs`
- core method specs already know `StringUpper` / `StringLower`
- `src/runtime/type_registry.rs` still carried `toUpper` / `toLower` in
  `STRING_METHOD_EXTRAS`
- `291x-91` still deferred the uppercase/lowercase name family after the first
  stable slice

## Implementation Slice

- add `toUpper` / `toLower` rows to `STRING_SURFACE_METHODS`
- keep `toUpperCase` / `toLowerCase` as compatibility aliases on those rows
- wire `StringBox::invoke_surface(...)`, router allowlists, return-type
  inference, and VM arg-shape checks through the catalog-backed methods
- remove the remaining String TypeRegistry extras fallback for these rows
- extend focused MIR tests and the stable String surface smoke to pin canonical
  plus alias behavior

## Non-Goals

- do not widen into `split`, `startsWith`, `endsWith`, `charAt`, or `equals`
- do not change string semantic ownership or alias SSOT
- do not add new slots or rename the public compatibility aliases
- do not reopen MapBox follow-up cards

## Acceptance

```bash
cargo test --lib string_surface_catalog --quiet
cargo test --lib invoke_surface_routes_string_aliases_and_values --quiet
cargo test --release string_value_case_conversion_uses_unified_receiver_arg_shape_and_string_return -- --nocapture
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh
```

## Exit Condition

`toUpper` / `toLower` are no longer a TypeRegistry-only side path. They live in
the same StringBox surface catalog, Unified route allowlist, and stable smoke as
the rest of the landed String stable rows, while `toUpperCase` /
`toLowerCase` remain compatibility aliases.
