---
Status: Active Inventory
Date: 2026-04-22
Scope: phase-290x docs-first start pointとしての `ArrayBox` surface / dispatch / exposure touchpoint inventory。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-290x/README.md
  - docs/development/current/main/phases/phase-290x/290x-90-arraybox-surface-canonicalization-design-brief.md
  - apps/std/array.hako
  - apps/kilo_nyash/enhanced_kilo_editor.hako
---

# Phase 290x ArrayBox Surface Inventory Ledger

## Decision

This ledger is the docs-first inventory for phase-290x.
It records where `ArrayBox` truth is currently split, so the implementation slice
can converge without reopening the same search.

## Implementation Update: 2026-04-22

The first stable surface catalog is now landed:

```text
src/boxes/array/surface_catalog.rs
  -> ArrayMethodId
  -> ArrayBox::invoke_surface(...)
```

Cataloged stable methods:

- `length` canonical, with `size` compatibility alias and `len` legacy slot alias
- `get`
- `set`
- `push`
- `pop`
- `slice`
- `remove`
- `insert`

Thin readers now consume this catalog for TypeRegistry exposure, static method
resolution, effect classification, and MIR interpreter ArrayBox dispatch.
Extended methods (`clear/contains/indexOf/join/sort/reverse`) are still outside
the first catalog slice.

Stable smoke is now pinned at:

```text
tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
```

It locks the Rust catalog/invoke seam and the hako VM visible-owner route. Direct
source follow-up calls on a `slice()` result still lower through a
`RuntimeDataBox` union receiver; that is a separate return-type cleanup topic,
not the ArrayBox surface owner for this phase.

## Three-Layer Inventory

### 1. Surface Contract

Current surface implementations live primarily in:

- `src/boxes/array/ops/access.rs`
  - `get`
  - `set`
  - `get_index_i64`
  - `set_index_i64`
  - `has_index_i64`
- `src/boxes/array/ops/capacity.rs`
  - `push`
  - `pop`
  - `length`
  - `size`
- `src/boxes/array/ops/mutation/remove.rs`
  - `remove`
- `src/boxes/array/ops/insert.rs`
  - `insert`
  - `insert_index_i64`
  - `slot_insert_box_raw`
- `src/boxes/array/ops/sequence/slice.rs`
  - `slice`
- `src/boxes/array/ops/sequence/membership.rs`
  - `indexOf`
  - `contains`
- `src/boxes/array/ops/sequence/order.rs`
  - `join`
  - `sort`
  - `reverse`

Problem:

- these files are the implementation truth
- the stable public method rows now have a catalog owner, but extended sequence
  methods still need a later catalog decision

### 2. Execution Dispatch

Stable dispatch now has the `ArrayBox::invoke_surface(...)` seam. Current
dispatch touchpoints are:

- `src/backend/mir_interpreter/handlers/boxes_array.rs`
  - method-name bridge for cataloged stable methods
- `src/backend/mir_interpreter/handlers/calls/method/dispatch.rs`
  - slot-based dispatch for cataloged stable methods
- `src/runtime/type_registry.rs`
  - slot exposure from the catalog for stable methods, plus legacy extras
- `src/mir/builder/calls/method_resolution.rs`
  - known-method surface checks from the catalog
- `src/mir/builder/calls/effects_analyzer.rs`
  - read/write effect classification from the catalog

Problem:

- extended methods and non-VM consumers still need separate follow-up decisions

### 3. Exposure State

Current exposure truth is split across:

- `apps/std/array.hako`
  - user-facing sugar surface
- `apps/smokes/std/array_smoke.hako`
  - app-facing smoke surface
- `src/tests/nyash_abi_basic.rs`
  - slot exposure sanity checks
- `src/tests/core13_smoke_array.rs`
  - still thin compared to the full current array surface
- `apps/kilo_nyash/enhanced_kilo_editor.hako`
  - real app proving ground that now depends on native `insert`

Problem:

- runtime / std / smoke / app usage do not read from one explicit exposure table

## Current Surface Snapshot

| Method | Runtime impl | VM dispatch | Std wrapper | Smoke | Notes |
| --- | --- | --- | --- | --- | --- |
| `length` | yes | yes | yes | yes | canonical name |
| `size` | yes | yes | no direct std sugar | indirect only | compatibility alias in catalog |
| `len` | yes | yes | no | indirect only | legacy slot alias in catalog |
| `get` | yes | yes | implicit direct use | yes | stable |
| `set` | yes | yes | implicit direct use | indirect | stable but lightly surfaced |
| `push` | yes | yes | yes | yes | stable |
| `pop` | yes | yes | yes | yes | stable |
| `slice` | yes | yes | yes | yes | stable |
| `remove` | yes | yes | yes | yes | cataloged in 290x-1 |
| `insert` | yes | yes | yes | yes | cataloged in 290x-1 |

## Canonicalization Decisions Needed

### `length` vs `size`

Decision locked in phase-290x:

- canonical: `length`
- alias: `size`
- legacy slot alias: `len`

### Exposure Vocabulary

Phase-290x should treat these as distinct:

- implemented in runtime
- routed in dispatch
- exposed in std sugar
- pinned in smoke
- described in docs

This is the difference between “exists” and “stable user-facing surface”.

## Current App Anchor

`apps/kilo_nyash/enhanced_kilo_editor.hako` is the active proving ground.

Why it matters:

- it forced `insert()` to become real
- it shows that ArrayBox truth is app-lane infrastructure, not just runtime internals
- it gives phase-290x a practical stop-line: reduce future app rediscovery cost

## Next Inventory-to-Code Seam

The first implementation slice has converged on:

```text
surface_catalog.rs
  -> ArrayMethodId
  -> ArrayBox::invoke_surface(...)
```

Next follow-up:

- return to kilo editor feature slices unless ArrayBox drift reappears
- decide whether extended methods join this catalog in a later small card
