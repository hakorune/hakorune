---
Status: Active
Date: 2026-04-24
Scope: CoreBox surface catalog を ArrayBox から StringBox / MapBox へ広げる phase front。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/phases/phase-290x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md
  - docs/development/current/main/phases/phase-291x/291x-98-mapbox-content-enumeration-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md
  - docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md
  - docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-106-arraybox-element-result-publication-card.md
  - docs/development/current/main/phases/phase-291x/291x-107-string-semantic-owner-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-108-alias-ssot-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-109-map-compat-source-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-110-mapbox-get-existing-key-typing-card.md
  - docs/development/current/main/phases/phase-291x/291x-111-stringbox-case-conversion-card.md
  - docs/development/current/main/phases/phase-291x/291x-112-arraybox-clear-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-113-arraybox-contains-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-114-arraybox-indexof-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-115-arraybox-join-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-116-arraybox-reverse-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-117-arraybox-sort-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-118-arraybox-slice-result-receiver-card.md
  - docs/development/current/main/phases/phase-291x/291x-119-docs-status-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-120-mapbox-taskboard-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-121-doc-update-simplification-contract.md
---

# Phase 291x: CoreBox surface catalog

- Status: Active reference lane
- Date: 2026-04-24
- Purpose: phase-290x の `ArrayBox` catalog/invoke seam を、CoreBox surface の横断ルールへ上げる。
- Landed implementation targets:
  - `StringBox`
  - `MapBox` first current-vtable slice
- Latest landed cleanup target: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Next implementation target: `successor cleanup card selection` (pending)
- Sibling guardrail:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - phase-137x remains observe-only unless app work produces a real blocker

## Decision

ArrayBox で固定した読み方を CoreBox 全体へ広げる。

```text
surface contract
  -> canonical name / aliases / arity / slot / effect / return

execution dispatch
  -> one invoke seam per Box family

exposure state
  -> runtime / VM / std sugar / docs / smoke pinned state
```

ただし、最初の code slice で `StringBox` と `MapBox` を同時に触らない。
phase-291x の初回実装は `StringBox` だけに閉じる。

## Reading Order

1. `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
2. `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
3. `docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md`
4. `docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md`
5. `docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md`
6. `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
7. `docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md`
8. `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`
9. `docs/development/current/main/phases/phase-291x/291x-98-mapbox-content-enumeration-contract-card.md`
10. `docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md`
11. `docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md`
12. `docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md`
13. `docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md`
14. `docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md`
15. `docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md`
16. `docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md`
17. `docs/development/current/main/phases/phase-291x/291x-106-arraybox-element-result-publication-card.md`
18. `docs/development/current/main/phases/phase-291x/291x-107-string-semantic-owner-cleanup-card.md`
19. `docs/development/current/main/phases/phase-291x/291x-108-alias-ssot-cleanup-card.md`
20. `docs/development/current/main/phases/phase-291x/291x-109-map-compat-source-cleanup-card.md`
21. `docs/development/current/main/phases/phase-291x/291x-110-mapbox-get-existing-key-typing-card.md`
22. `docs/development/current/main/phases/phase-291x/291x-111-stringbox-case-conversion-card.md`
23. `docs/development/current/main/phases/phase-291x/291x-112-arraybox-clear-router-card.md`
24. `docs/development/current/main/phases/phase-291x/291x-113-arraybox-contains-router-card.md`
25. `docs/development/current/main/phases/phase-291x/291x-114-arraybox-indexof-router-card.md`
26. `docs/development/current/main/phases/phase-291x/291x-115-arraybox-join-router-card.md`
27. `docs/development/current/main/phases/phase-291x/291x-116-arraybox-reverse-router-card.md`
28. `docs/development/current/main/phases/phase-291x/291x-117-arraybox-sort-router-card.md`
29. `docs/development/current/main/phases/phase-291x/291x-118-arraybox-slice-result-receiver-card.md`
30. `docs/development/current/main/phases/phase-291x/291x-119-docs-status-closeout-card.md`
31. `docs/development/current/main/phases/phase-291x/291x-120-mapbox-taskboard-closeout-card.md`
32. `docs/development/current/main/phases/phase-291x/291x-121-doc-update-simplification-contract.md`

## Current Rule

- docs-first before code
- current docs update policy is
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`;
  do not append landed history to current mirrors for every card
- `StringBox.length()` is canonical; `len()` and `size()` are compatibility aliases
- `StringBox.indexOf(needle, start)` is stable; `find` is compatibility alias
- `StringBox.lastIndexOf(needle, start_pos)` is landed as a StringBox-only catalog row
- `apps/std/string.hako` is sugar, not the semantic owner
- `apps.std.string` is the exact manifest alias that pins the current public
  sugar smoke; this is not a broader `std.string` packaging decision
- alias ownership is split on purpose:
  - manifest alias / module lookup lives in `hako.toml`
  - imported static-box alias binding lives in the runner text-merge strip path
  - static receiver/type-name lowering lives in the MIR builder only
- `using apps.std.string as S` resolves `apps.std.string` as a manifest alias,
  then binds `S` to the exported `StdStringNy` static box for `S.method(...)`
  calls after merge
- imported static-box aliases are not namespace roots; they do not imply
  `new Alias.BoxName()` or `new apps.std.string.BoxName()`
- `apps/lib/boxes/string_std.hako` is an internal selfhost helper, not a
  public std owner
- `apps/std/string_std.hako` is dead scaffold and is removed by `291x-107`
- legacy `apps/std/string2.hako` diagnostic residue was deleted by an explicit cleanup card
- `MapBox` first slice cataloged current Rust vtable rows only
- do not add `length` as a Rust vtable alias in the first MapBox commit
- do not collapse `size` and `len` slots in the first MapBox commit
- do not normalize `set` / `delete` / `clear` return contracts in the first MapBox commit
- `MapBox.length` is now a separate contract-first alias slice; it must not
  promote `keys` / `values` / `delete` / `remove` / `clear`
- `MapBox` source-level write rows now have a contract decision: `set`,
  `delete` / `remove`, and `clear` return Rust-vtable-compatible receipt
  strings; bad-key normalization remains separate
- `MapBox` source-visible bad-key rows now have a contract decision:
  non-string `set/get/has/delete/remove` keys publish
  `[map/bad-key] key must be string`; field rows keep the field-name variant
- `MapBox.get(missing-key)` keeps the stable tagged read-miss text
  `[map/missing] Key not found: <key>`
- `291x-110` landed the conservative successful-read rule for
  `MapBox.get(existing-key)`: publish `V` only for receiver-local homogeneous
  Map facts with tracked literal keys; mixed, untyped, and missing-key reads
  stay `Unknown`
- `291x-111` landed StringBox case conversion as stable surface rows:
  `toUpper` / `toLower` live in the catalog and keep
  `toUpperCase` / `toLowerCase` as compatibility aliases
- `291x-112` landed `ArrayBox.clear()` as a catalog-backed receiver-only
  write-`Void` row on the Unified value path
- `291x-113` landed `ArrayBox.contains(value)` as a catalog-backed
  receiver-plus-value read-`Bool` row on the Unified value path
- `291x-114` landed `ArrayBox.indexOf(value)` as a catalog-backed
  receiver-plus-value read-`Integer` row on the Unified value path
- `291x-115` landed `ArrayBox.join(delimiter)` as a catalog-backed
  receiver-plus-delimiter read-`String` row on the Unified value path
- `291x-116` landed `ArrayBox.reverse()` as a catalog-backed receiver-only
  write-`String` receipt row on the Unified value path
- `291x-117` landed `ArrayBox.sort()` as a catalog-backed receiver-only
  write-`String` receipt row on the Unified value path
- `291x-118` landed the `ArrayBox.slice()` result-receiver pin: direct source
  `slice().length()` stays on the `ArrayBox` receiver path and does not degrade
  to `RuntimeDataBox.length`
- `291x-119` closed stale status/deferred wording as docs-only BoxShape
  cleanup; no CoreBox behavior changed
- `291x-120` closed stale MapBox taskboard follow-up wording as docs-only
  BoxShape cleanup; future-risk rows remain explicitly deferred
- `291x-121` simplified current docs update policy: current mirrors stay thin,
  and latest-card history lives in `CURRENT_STATE.toml` plus the active card
- `MapBox.keys()/values()` element publication is landed through the S0 state
  owner; `keys().get(i)` and `values().get(i)` are pinned in sorted-key order
- `MapBox.delete(key)` and `MapBox.remove(key)` use the catalog-backed Unified
  receiver-plus-key value path
- `MapBox.clear()` now uses the catalog-backed Unified receiver-only value path
- `ArrayBox.get/pop/remove` element-result publication is landed:
  publish `T` only when the receiver has a known `MirType::Array(T)`; keep
  `Unknown` for mixed or untyped receivers.
- alias SSOT cleanup is landed in `291x-108`
- Map compat/source cleanup is landed in `291x-109`: keep `OpsCalls.map_has(...)`
  as the only remaining selfhost-runtime `pref == "ny"` Map wrapper, and keep
  `crates/nyash_kernel/src/plugin/map_compat.rs` as compat-only legacy ABI
  quarantine
- next cleanup must be selected after the `latest_card_path` recorded in
  `CURRENT_STATE.toml`; do not reopen the landed
  ArrayBox.clear / contains / indexOf / join / reverse / sort rows or the older existing-key
  typing rule without an owner-path change.

## Implementation State

Landed first implementation card:

```text
String surface catalog
  -> StringMethodId
  -> StringBox::invoke_surface(...)
  -> thin registry / method-resolution / dispatch consumers
  -> stable String surface smoke
```

Landed smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`

Landed StringBox cleanup smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_lastindexof_start_vm.sh`

MapBox is now the next CoreBox catalog target.

Landed MapBox card:

```text
Map surface catalog
  -> MapMethodId
  -> MapBox::invoke_surface(...)
  -> thin registry / method-resolution / effect-analysis / VM dispatch consumers
  -> stable MapBox surface smoke for Rust catalog + hako-visible VM subset
```

Landed smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`

Landed MapBox follow-up:

- source-level vm-hako non-empty `MapBox.values().size()` state-owner shape is landed and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_values_vm.sh`.
- source-level vm-hako non-empty `MapBox.keys().size()` state-owner shape is landed and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_keys_vm.sh`.
- source-level vm-hako `MapBox.remove(key)` delete-owner alias is landed and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_remove_vm.sh`.
- source-level vm-hako `MapBox.clear()` state reset is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_clear_vm.sh`.
- source-level vm-hako `MapBox.set(...)` duplicate receiver stripping is landed
  and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh`.
- `keys()/values()` element publication is landed in source-level vm-hako and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_keys_values_elements_vm.sh`.
  Rust audit/fix (2026-04-23): `keys()` sorts deterministically and
  `values()` now follows the same sorted-key order for the promoted contract.
- `MapBox.set/delete/remove/clear` source-level write-return receipt contract
  is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh`.
- `MapBox` bad-key normalization is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_bad_key_vm.sh`
  and `tools/smokes/v2/profiles/quick/core/map/map_bad_key_has_vm.sh`.
- `MapBox.get(missing-key)` is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_get_missing_vm.sh`
  and `tools/smokes/v2/profiles/quick/core/map/map_missing_key_tag_vm.sh`.
- `MapBox.get(existing-key)` typing is landed and pinned by focused MIR tests in
  `src/tests/mir_corebox_router_unified.rs`; publish `V` only for
  receiver-local homogeneous Map facts with tracked literal keys.
- `StringBox` case conversion is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`
  plus focused MIR tests; `toUpperCase` / `toLowerCase` remain compatibility
  aliases on the same stable rows.
- legacy `apps/std/map_std.hako` JIT-only placeholder was deleted; it was not an active module-registry/prelude route.
- unused `lang/src/vm/hakorune-vm/map_keys_values_bridge.hako` prototype was deleted; it was not an active VM route.
- `apps/lib/boxes/map_std.hako` prelude/module-registry dependency was deleted by the phase-291x cleanup card.
- landed alias card:
  `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`
- landed extended-route card:
  `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`

Landed CoreBox router first slice:

- `src/mir/builder/router/policy.rs` now routes only the catalog-backed
  `StringBox.length` / `len` / `size` and `StringBox.substring` / `substr`
  and `StringBox.concat`, `StringBox.trim`, `StringBox.contains`,
  `StringBox.lastIndexOf` one-arg and two-arg, `StringBox.replace`, and
  `StringBox.indexOf` /
  `find`, plus `ArrayBox.length` / `size` / `len`, `ArrayBox.push`,
  `ArrayBox.slice`, `ArrayBox.get`, `ArrayBox.pop`, `ArrayBox.set`,
  `ArrayBox.clear`, `ArrayBox.contains`, `ArrayBox.indexOf`, `ArrayBox.join`,
  `ArrayBox.reverse`, `ArrayBox.sort`,
  `ArrayBox.remove`, `ArrayBox.insert`, `MapBox.size`, `MapBox.length`, `MapBox.len`,
  `MapBox.has`, `MapBox.get`, `MapBox.set`, `MapBox.keys`, and
  `MapBox.values`, `MapBox.delete`, `MapBox.remove`, and `MapBox.clear` rows
  through `Route::Unified`.
- `src/mir/builder/utils/boxcall_emit.rs` still bridges `MirType::String` to
  `StringBox` before route selection; uncovered methods remain on the BoxCall
  fallback.
- `ArrayBox.get` / `pop` / `remove` intentionally stayed `MirType::Unknown` in
  the route-only slice; `291x-106` landed the dedicated element-result
  publication card that narrows them only when the receiver has a known
  `Array<T>` MIR fact.
- `ArrayBox.set` follows the write-`Void` contract already used by
  `ArrayBox.push`.
- `ArrayBox.clear` follows the same receiver-only write-`Void` contract already
  used by `ArrayBox.push` / `set` / `insert`.
- `ArrayBox.contains` follows the read-only Bool-return contract already proven
  by `StringBox.contains`, with a receiver-plus-value Unified shape.
- `ArrayBox.indexOf` follows the read-only Integer-return contract already
  proven by StringBox search rows, with a receiver-plus-value Unified shape.
- `ArrayBox.join` follows the read-only String-return contract already proven
  by StringBox read rows, with a receiver-plus-delimiter Unified shape.
- `ArrayBox.reverse` follows the mutating String-receipt contract, with a
  receiver-only Unified shape.
- `ArrayBox.sort` follows the same mutating String-receipt contract, with a
  receiver-only Unified shape.
- `ArrayBox.slice()` result follow-up calls are pinned by `291x-118`; direct
  source `slice().length()` stays on `ArrayBox.length` and must not lower as
  `RuntimeDataBox.length`.
- `ArrayBox.insert` follows the same write-`Void` contract already used by
  `ArrayBox.push` / `ArrayBox.set`.
- `MapBox.get` intentionally stays `MirType::Unknown` because stored map values
  are data-dependent.
- `MapBox.set`, `MapBox.delete` / `remove`, and `MapBox.clear` write-return
  rows have a landed receipt-string contract in `291x-99`; source-level
  vm-hako publication and matching type hints are synced.
- `MapBox.delete` / `remove` router promotion is landed in `291x-104`;
  `MapBox.clear` is landed in `291x-105`.
- `MapBox.get(missing-key)` keeps its landed tagged read-miss contract in
  `291x-101`; successful `get(existing-key)` typing remains data-dependent.
- two-arg `lastIndexOf` is landed in `291x-103`; MapBox keys/values element
  publication is landed in `291x-102`.
- task cards:
  - `docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-116-arraybox-reverse-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-117-arraybox-sort-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-118-arraybox-slice-result-receiver-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-119-docs-status-closeout-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-120-mapbox-taskboard-closeout-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-121-doc-update-simplification-contract.md`
