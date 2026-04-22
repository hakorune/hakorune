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
- `StringBox.concat`
- `StringBox.trim`
- `StringBox.contains`
- `StringBox.lastIndexOf` one-arg
- `StringBox.replace`
- `StringBox.indexOf` / `StringBox.find` one-arg and two-arg
- `ArrayBox.length` / `ArrayBox.size` / `ArrayBox.len`
- `ArrayBox.push`
- `ArrayBox.slice`
- `ArrayBox.get`
- `ArrayBox.pop`
- `ArrayBox.set`
- `MapBox.size`
- `MapBox.len`
- `MapBox.has`

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

- start with `StringBox` only, then move collection boxes one proven method
  family at a time
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
publication were fixed by focused fixtures. `concat` then proved the same
receiver-plus-one-argument shape for the first non-zero-arity string value call.
`trim` extends the arity-zero read-only String-return family beyond `length`.
`contains` proves the first Bool-return StringBox value-path family.
one-arg `lastIndexOf` proves an Integer-return reverse-search family while
leaving the two-arg overload explicitly deferred.
`replace` proves the first two-argument String-return mutation-like read
surface without changing StringBox immutability.
`indexOf` / `find` proves the current forward-search family, including the
stable start-position overload that already exists in the StringBox catalog and
runtime dispatch.
`ArrayBox.length` / `size` / `len` is the first collection slice because it is
read-only, arity-zero, and publishes a fixed `Integer` result without touching
generic element-return methods.
`ArrayBox.push` is the first collection write slice because it is a single
argument, has a catalog-backed `WriteHeap` effect, and returns `Void`.
`ArrayBox.slice` is the next read-only slice because its return type is an
explicit `ArrayBox`, without touching generic element-return methods.
`ArrayBox.get` is the first generic element-return slice; it moves routing to
the Unified receiver-plus-index shape while intentionally leaving the MIR
result type `Unknown` because the element type is data-dependent.
`ArrayBox.pop` uses the same generic element-return contract as `get`; it moves
to the Unified receiver-only shape while keeping the MIR result type
`Unknown`.
`ArrayBox.set` follows the write-`Void` contract already proven by
`ArrayBox.push`, with receiver-plus-index-plus-value Unified shape.
`ArrayBox.remove` follows the same generic element-return contract as
`get` / `pop`; it moves to the Unified receiver-plus-index shape while
keeping the MIR result type `Unknown`.
`ArrayBox.insert` follows the write-`Void` contract already proven by
`push` / `set`, with receiver-plus-index-plus-value Unified shape.
`MapBox.size` is the first MapBox route slice because it is read-only,
arity-zero, and publishes a fixed `Integer` result without collapsing the
separate `len` slot.
`MapBox.len` is a separate read-only current-vtable row with the same fixed
`Integer` result, kept distinct from `size` and without adding `length`.
`MapBox.has` is the first keyed MapBox read slice because it has a fixed
`Bool` result without touching stored value materialization.
`MapBox.get` is the first stored-value MapBox read slice; it moves to the
Unified receiver-plus-key shape while keeping the MIR result type `Unknown`.
`MapBox.set` is the first stored-value MapBox write slice; it moves to the
Unified receiver-plus-key-plus-value shape while keeping the current visible
write-return opaque as `Unknown`.

## Implementation Snapshot

- `src/mir/builder/router/policy.rs` now allowlists the catalog-backed
  `StringMethodId::Length`, `StringMethodId::Substring`, and
  `StringMethodId::Concat`, `StringMethodId::Trim`, and
  `StringMethodId::Contains`, `StringMethodId::LastIndexOf`, and
  `StringMethodId::Replace`, `StringMethodId::IndexOf`, and
  `StringMethodId::IndexOfFrom` families to `Route::Unified`.
- `src/mir/builder/router/policy.rs` also allowlists the catalog-backed
  `ArrayMethodId::Length`, `ArrayMethodId::Push`, `ArrayMethodId::Slice`, and
  `ArrayMethodId::Get`, `ArrayMethodId::Pop`, `ArrayMethodId::Set`, and
  `ArrayMethodId::Remove`, and `ArrayMethodId::Insert` families to
  `Route::Unified`.
- `src/mir/builder/router/policy.rs` allowlists the catalog-backed
  `MapMethodId::Size`, `MapMethodId::Len`, `MapMethodId::Has`, and
  `MapMethodId::Get`, and `MapMethodId::Set` rows to `Route::Unified`;
  remaining MapBox rows still use the family-wide fallback.
- `src/mir/builder/calls/unified_emitter.rs` computes method-result annotation
  arity without the duplicated receiver arg, preserving `StringBox.length/0`
  return-type publication.
- `src/mir/builder/types/annotation.rs` now reads `StringMethodId` for
  StringBox return-type publication, so aliases such as `substr` use the same
  return contract as their canonical method. It also reads `ArrayMethodId` for
  fixed ArrayBox return rows, starting with `length` / `size` / `len`.
- `src/mir/join_ir/lowering/method_return_hint.rs` consumes the same builder
  return-type helper instead of duplicating the primitive method table.
- `src/backend/mir_interpreter/handlers/calls/method.rs` strips an exact
  duplicate receiver `ValueId` at the VM method boundary before slot dispatch.
- `src/tests/mir_corebox_router_unified.rs` pins direct string value receiver
  shape: `length`, `substring`, and `substr` use the Unified receiver-arg
  shape; `concat` uses the same Unified receiver-plus-argument shape; `trim`
  uses the arity-zero Unified receiver-arg shape; `contains` uses the
  receiver-plus-needle shape and publishes `MirType::Bool`; one-arg
  `lastIndexOf` uses the receiver-plus-needle shape and publishes
  `MirType::Integer`; `replace` uses the receiver-plus-old-plus-new shape and
  publishes `MirType::String`; `indexOf` / `find` use receiver-plus-needle
  and receiver-plus-needle-plus-start shapes and publish `MirType::Integer`;
  `ArrayBox.length` / `size` / `len` use the arity-zero receiver shape and
  publish `MirType::Integer`; `ArrayBox.push` uses the receiver-plus-value
  shape and stays `Void`; `ArrayBox.slice` uses the
  receiver-plus-start-plus-end shape and publishes `Box(ArrayBox)`;
  `ArrayBox.get` uses the receiver-plus-index shape and intentionally stays
  `MirType::Unknown`; `ArrayBox.pop` uses the receiver-only shape and also
  intentionally stays `MirType::Unknown`; `ArrayBox.set` uses the
  receiver-plus-index-plus-value shape and stays `Void`; `ArrayBox.remove`
  uses the receiver-plus-index shape and intentionally stays
  `MirType::Unknown`; `ArrayBox.insert` uses the
  receiver-plus-index-plus-value shape and stays `Void`;
  `MapBox.size` and `MapBox.len` use the arity-zero receiver shape and publish
  `MirType::Integer`; `MapBox.has` uses the receiver-plus-key shape and
  publishes `MirType::Bool`; `MapBox.get` uses the receiver-plus-key shape and
  intentionally stays `MirType::Unknown`; `MapBox.set` uses the
  receiver-plus-key-plus-value shape and intentionally stays
  `MirType::Unknown`; `lastIndexOf/2` and `MapBox.delete` remain pinned as
  BoxCall fallback sentinels.

## Acceptance

- `choose_route` has focused tests that pin current fallback behavior for:
  - `UnknownBox`
  - user instance names that do not end with `Box`
  - non-allowlisted `StringBox` / `ArrayBox` / `MapBox` methods
- the allowlisted CoreBox method families reach `Route::Unified`
  only when the Unified call env is enabled
- a direct `MirType::String` receiver fixture proves the chosen method no
  longer depends on the broad CoreBox BoxCall guard
- existing ArrayBox, StringBox, and MapBox surface smokes stay green
- no broad route change is made for remaining `ArrayBox` rows or `MapBox`

## Verification Commands

```bash
cargo test -q router
bash tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh
```

Use a narrower focused MIR-builder smoke if one exists by the time this card is
implemented.

## Remaining Work

- remaining route-only CoreBox rows are closed for the current ArrayBox stable
  rows and MapBox `size` / `len` / `has` / `get` / `set`
- contract-first backlog: two-arg `StringBox.lastIndexOf(needle, start_pos)`,
  Array generic element-result publication (`get` / `pop` / `remove` as `T`
  instead of `Unknown`), `MapBox.length`, `MapBox.keys` / `values`,
  `MapBox.delete` / `remove` / `clear`, and MapBox write-return / bad-key
  normalization
- non-router cleanup backlog: String semantic owner cleanup, alias SSOT
  cleanup, and Map compat/source cleanup

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
