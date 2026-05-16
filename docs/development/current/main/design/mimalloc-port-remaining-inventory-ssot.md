---
Status: historical SSOT
Decision: accepted
Date: 2026-05-14
Scope: Remaining mimalloc `.hako` port inventory after M213 and the next row selection input.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/post-m213-next-lane-selection-ssot.md
  - docs/development/current/main/design/purge-lifecycle-ladder-closeout-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - lang/src/hako_alloc/
---

# Mimalloc Port Remaining Inventory SSOT

Current note:

```text
This D206 inventory selected the older M214/M215 direction and is historical
for current restart purposes. M214 and M215 have landed. The active row is now
read from CURRENT_STATE.toml; after MIMAP-050A, it is MIMAP-051A reclaim
owner-transfer contract inventory.
```

## Decision

D206 freezes the post-M213 mimalloc port inventory.

The current lane should not continue directly into reclaim execution. The next
step is D207, which must select exactly one implementation row from this
inventory.

Recommended D207 selection:

```text
M214 allocator options/defaults inventory
```

M214 should be a read-only `.hako` inventory row for mimalloc-shaped option and
initialization vocabulary. It must not add mutable runtime options, environment
toggles, provider activation, allocator replacement, reclaim execution, or
allocation behavior changes.

## Already complete

| Area | Current state | Notes |
| --- | --- | --- |
| capability substrate | complete through the M20-M44 mimalloc substrate proof ladder | raw pages, static tables, OSVM, TLS, atomic i64, native pointer atomic routes, remote-free sketches, and closeout guard exist |
| production facade | complete through M46-M51 | facade, local page policy, remote-free policy, OSVM page source, stress parity, and closeout guard exist |
| algorithm page/free/realloc lane | complete through M163-M190 | size classes, layout migration, page model, page queue, fast path, OSVM page source, local free, remote free, page map, realloc, alignment, huge pages, secure list, usize, object return, and nullable failure handle are covered |
| record and packed metadata lane | complete through C201-C212 for current allocator metadata goals | record declarations, layout plans, metadata record migration, non-escaping packed auto-use pilots, and backend fail-fast hardening are closed for now |
| verifier/stats/lifecycle/purge lane | complete through M191-M213 plus C194/C194b | stats snapshots, decommit/recommit/reuse lifecycle, verifier-owned invariants, bounded purge scheduler, and abandoned/reclaim inventory exist |
| provider/hook ladder | inactive after M103 | optional future host replacement work only; it does not gate `.hako` mimalloc completion |

## Remaining inventory

| Remaining area | State after M213 | Next action |
| --- | --- | --- |
| options/defaults/init vocabulary | not yet modeled as a `.hako` owner after stats landed | open M214 as read-only inventory |
| thread heap ownership | owner/page facts exist only as scalar inventory inputs around M213 | keep parked unless D207 chooses a read-only owner-token row |
| reclaim execution | explicitly inactive | do not open without a new guarded ladder for threads, atomic claim, remote-free drain, ownership transfer, and rollback |
| unreserve / OS release | explicitly inactive | keep closed until a page-source release/unreserve proposal row is accepted |
| secure randomness / entropy | secure-list encode/decode small path exists, but no randomness or cryptographic claim exists | future inventory only; no entropy route without capability design |
| visible record materialization | packed metadata pilots are non-escaping and backend fail-fast only | keep in language/CorePlan lane, not in D207 unless it blocks the selected allocator row |
| generic `PackedArray<T>` source surface | compiler auto-use exists for selected record-array shapes, but source-level generic container semantics remain parked | use language task breakdown when needed |
| brand/state/transition language semantics | documented as future minimal-surface rows | do not implement as part of allocator behavior rows |
| selfhost migration | parked behind allocator/language seam stability | resume through selfhost SSOTs only after a lane switch card |

## Recommended next row

```text
M214 allocator options/defaults inventory
```

Purpose:

```text
name allocator option/default/init vocabulary
prove defaults are observable as read-only facts
keep allocation behavior unchanged
keep runtime options/env toggles inactive
```

Suggested owner:

```text
lang/src/hako_alloc/memory/options_inventory_box.hako
```

Suggested proof app / guard naming:

```text
apps/hako-alloc-options-inventory-proof/
tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh
```

M214 acceptance should require structured report fields like:

```text
options_inventory_present = 1
mutable_options_enabled = 0
env_toggles_added = 0
would_change_allocation_policy = 0
would_select_provider = 0
would_install_hook = 0
would_replace_process_allocator = 0
would_execute_reclaim = 0
```

## Alternate future rows

These rows are not selected by D206. D207 may choose one only with a concrete
reason.

| Candidate | Scope | Why not first |
| --- | --- | --- |
| `M214 thread heap owner-token inventory` | read-only owner-token facts for future abandoned/reclaim rows | M213 already has scalar abandoned/reclaim vocabulary; options/defaults are the safer missing mimalloc surface |
| `M214 secure entropy source inventory` | name randomness/entropy needs for stronger secure-list policy | needs capability design and must avoid false cryptographic claims |
| `M214 page-source release/unreserve proposal` | docs/proposal for release and unreserve semantics | would touch OSVM lifetime and must remain explicit |
| `M214 reclaim execution proposal` | new guarded ladder for reclaim execution | too broad after M213; requires threads, atomics, ownership transfer, and remote-free drain |
| language minimal-surface row | brand, LoopRange, delegate, record literal, transition, uses | should run only if selected allocator row needs it |
| selfhost restart inventory | selfhost status refresh | should wait until allocator/language seam is stable or a lane switch card selects it |

## Stop lines

D206 and D207 must not add:

```text
new allocator behavior
mutable runtime options
environment option toggles
thread scheduling
atomic ownership claim
remote-free drain during reclaim
reclaim execution
page ownership migration
unreserve
OS release
provider activation
hooks
process allocator replacement
backend app/name matchers
language syntax implementation
selfhost parser/mirbuilder changes
```

## D207 contract

D207 must produce exactly one selected row.

Required D207 outputs:

```text
selected row token
why this row now
owned files and proof app
stop lines
first guard command
current pointer update target
```

If D207 cannot select a safe row, it must stop and keep the current blocker as
a docs/design blocker rather than widening allocator behavior.
