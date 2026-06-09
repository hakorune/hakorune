---
Status: Active
Date: 2026-06-08
Scope: worker-friendly remaining mimalloc migration ladder after MIR-FMEM-008E.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-614-MIM-PORT-FMEM-115-REMAINING-FASTMEM-MIGRATION-TARGET-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - docs/development/current/main/phases/phase-296x/296x-473-MIR-FMEM-008C-HANDOFF-ORDER.md
---

# Mimalloc Post-008E Worker Ladder

## Purpose

`MIR-FMEM-008E` closed the producer-neutral readiness gate for the fastmem
proof/lowering ladder. This note does not reopen that proof ladder.

It exists to keep the remaining mimalloc migration work in small worker-sized
families so that body migration can continue without growing the active
restart card.

## Current Restart Surface

The active maintenance lane in `CURRENT_STATE.toml` remains:

```text
MIM-PORT-FMEM-109 source-syntax smoke manifest runner
```

That lane keeps the source-syntax smoke runner compact. It is not where new
allocator semantics should be reintroduced.

## Worker Ladder

### 1. Source-syntax maintenance

Current role:

```text
keep the manifest runner thin
keep new body fixtures manifest-backed
avoid shell block growth
```

Representative card:

```text
MIM-PORT-FMEM-109 source-syntax smoke manifest runner
```

### 2. Source-body migration families

These are the body-migration surfaces that should be handled as separate
worker rows instead of one broad migration bucket.

```text
page_meta_local_free_to_free_refill_counter_body_box.hako
page_meta_free_head_alloc_body_box.hako
page_meta_refill_then_free_head_alloc_body_box.hako
page_meta_page_local_alloc_route_cfg_preflight_box.hako
```

Open discipline:

```text
one body family = one manifest/report/check surface
proof/report first, lowering second, smoke third
```

### 3. Publication / remote-owner substrate

Keep the publication and remote-owner families separate from the source-body
migration rows.

```text
AtomicRemoteHead
remote-owner branch routing
TLS backing transfer
owner slot reuse
abandoned reclaim
```

Current policy:

```text
proof/report and preflight rows first
CAS / drain / retry lowering only after the proof surface is explicit
```

### 4. Activation family

Keep the product activation surfaces separate from the publication and
owner-lifecycle surfaces.

```text
product activation
hook install
global allocator claim
winner claim
```

Current policy:

```text
preflight / producer evidence first
real activation side effects stay closed until the activation row opens
```

### 5. Allocator-model surfaces

These are not source-syntax smoke rows. They are model / runtime / bridge
surfaces and should stay in their own worker lane.

```text
page_map_release
page_map_bridge
page_map_realloc_same_class
page_map_realloc_alloc_copy_release
page_map_realloc_failure_contract
allocator_facade
page_box
page_heap
page_map_box
size_class_box
remote_free_policy
worker_tls_*
provider_*
segment_*
reclaim_*
purge_*
```

## Worker Order

1. Keep `MIM-PORT-FMEM-109` compact.
2. Keep source-body families in the manifest runner, one family at a time.
3. Keep publication / remote-owner rows separate from body migration.
4. Keep activation rows separate from publication / owner rows.
5. Keep allocator-model surfaces separate from source-body rows.

## No-Go

```text
do not reopen MIR-FMEM-008B..008E proof work
do not merge AtomicRemoteHead with activation
do not merge allocator-model surfaces into the source-syntax smoke runner
do not expand the active taskboard with historical detail
do not claim product/provider allocator activation
```

## Notes

The detailed historical row list already lives in the phase cards and the
workstream archive. This note only provides a worker-friendly ladder for the
remaining migration shape after 008E.

