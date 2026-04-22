---
Status: Active follow-up
Date: 2026-04-22
Scope: CoreBox receiver routing seam for Value World / Unified call migration.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
---

# CoreBox Router Unified Value Path Card

## Status

Landed route slices:

- `StringBox.length` / `StringBox.len` / `StringBox.size`
- `StringBox.substring` / `StringBox.substr`

This is not the active phase-292x `.inc` boundary-thinning blocker. Keep the
remaining method-family flips as CoreBox value-first cleanup candidates after
the current backend route-tag work has a clean stopping point.

## Finding

`src/mir/builder/router/policy.rs` currently forces the three CoreBox families
to the BoxCall route:

```text
StringBox | ArrayBox | MapBox -> Route::BoxCall
```

`src/mir/builder/utils/boxcall_emit.rs` also bridges a value-typed string
receiver into the same surface name before route selection:

```text
MirType::String -> "StringBox"
```

So a source call such as `s.substring(1, 4)` can be seen as a `StringBox`
receiver and then rejected from the Unified path by router policy. The fallback
path still emits canonical MIR method calls, but it is not the value-first
Unified path.

## Decision

Do not flip all CoreBoxes to `Route::Unified` in one edit.

The first implementation slice must be narrow:

- start with `StringBox` only
- allowlist one proven method family at a time
- keep `ArrayBox` and `MapBox` on `Route::BoxCall` until their receiver and
  return-type paths are proven
- keep the `MirType::String -> StringBox` bridge as a compatibility fallback
  until the Unified path owns the equivalent type publication

## Why It Is Not A One-Line Flip

The BoxCall fallback currently owns behavior that the Unified path must either
preserve or replace before route policy can change:

- receiver classification from `value_origin_newbox` / `value_types`
- plugin/core return-type publication for downstream MIR type users
- the `in_unified_boxcall_fallback` recursion guard
- canonical `Callee::Method` emission for legacy object-world paths

Changing `StringBox` routing before those contracts are covered can silently
move a working call into an incomplete path.

## First Safe Slice

Landed first code card:

```text
String value receiver
  -> router method allowlist
  -> Unified emission only for the selected method family
  -> return-type publication parity
  -> focused smoke
```

`length` / `len` / `size` was the first route flip because the method family is
already cataloged, read-only, and arity-zero. `substring` / `substr` followed
after the Unified receiver-argument shape and catalog-backed return-type
publication were fixed by focused fixtures.

## Implementation Snapshot

- `src/mir/builder/router/policy.rs` now allowlists the catalog-backed
  `StringMethodId::Length` and `StringMethodId::Substring` families to
  `Route::Unified`.
- `src/mir/builder/calls/unified_emitter.rs` computes method-result annotation
  arity without the duplicated receiver arg, preserving `StringBox.length/0`
  return-type publication.
- `src/mir/builder/types/annotation.rs` now reads `StringMethodId` for
  StringBox return-type publication, so aliases such as `substr` use the same
  return contract as their canonical method.
- `src/mir/join_ir/lowering/method_return_hint.rs` consumes the same builder
  return-type helper instead of duplicating the primitive method table.
- `src/backend/mir_interpreter/handlers/calls/method.rs` strips an exact
  duplicate receiver `ValueId` at the VM method boundary before slot dispatch.
- `src/tests/mir_corebox_router_unified.rs` pins direct string value receiver
  shape: `length`, `substring`, and `substr` use the Unified receiver-arg
  shape, while `concat` stays on the BoxCall fallback shape.

## Acceptance

- `choose_route` has focused tests that pin current fallback behavior for:
  - `UnknownBox`
  - user instance names that do not end with `Box`
  - non-allowlisted `StringBox` / `ArrayBox` / `MapBox` methods
- the allowlisted `StringBox` method families reach `Route::Unified`
  only when the Unified call env is enabled
- a direct `MirType::String` receiver fixture proves the chosen method no
  longer depends on the broad CoreBox BoxCall guard
- existing StringBox and MapBox surface smokes stay green
- no broad route change is made for `ArrayBox` or `MapBox`

## Verification Commands

```bash
cargo test -q router
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh
```

Use a narrower focused MIR-builder smoke if one exists by the time this card is
implemented.

## Remaining Work

- `StringBox.concat`
- `StringBox.indexOf` / `find`
- `StringBox.replace`
- `StringBox.trim`
- `StringBox.lastIndexOf`
- `StringBox.contains`
- `ArrayBox` and `MapBox` route flips

Each method family needs its own fixture and route assertion before the
family-wide CoreBox fallback can shrink further.

## Exit Condition

This card is done when the router policy can express:

```text
CoreBox method route = catalog-backed method decision
```

instead of the current family-wide rule:

```text
CoreBox family -> always BoxCall
```

The family-wide fallback can remain for uncovered methods, but the allowlisted
value-first path must be documented, tested, and visible from the router seam.
