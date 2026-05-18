# Hako Alloc Segment Arena Backing No-Escape Address Capability SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Inventory the no-escape address capability boundary before any real raw pointer
residence or pointer-derived lookup opens.

MIMAP-244A consumes the MIMAP-240A requirement matrix report and records scalar
owner/lifetime/address-carrier facts plus escape and closed-substrate blockers.
The row deliberately models an address-like scalar carrier, not pointer
residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_no_escape_address_capability_box.hako
```

The owner may:

- observe an accepted requirement matrix report;
- record segment id, arena id, lifetime generation, and address-like scalar
  carrier facts;
- reject return/storage/alias escapes;
- reject real pointer residence, pointer lookup, arena backing, segment-map,
  atomic bitmap, OSVM, worker, provider, and backend matcher requirements.

The owner must not:

- create raw pointer residence;
- perform pointer-derived lookup;
- allocate arena backing;
- mutate a real segment-map;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- infer anything from owner names or backend matchers.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | no-escape address capability accepted |
| `1` | requirement matrix missing, rejected, or not marked present |
| `2` | invalid lifetime generation |
| `3` | invalid address-like scalar carrier |
| `4` | return escape would be required |
| `5` | storage escape would be required |
| `6` | alias escape would be required |
| `7` | real pointer residence would be required |
| `8` | pointer-derived lookup would be required |
| `9` | real arena backing allocation would be required |
| `10` | real segment-map mutation would be required |
| `11` | atomic bitmap execution would be required |
| `12` | OSVM/page-source execution would be required |
| `13` | worker/TLS/source-level concurrency would be required |
| `14` | provider activation / hook / host allocator replacement would be required |
| `15` | backend matcher or `.inc` owner-name shortcut would be required |

## Validation

MIMAP-244A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_guard.sh --level L2
```

The guard must:

- prove accepted scalar no-escape address capability publication;
- prove matrix, lifetime, address carrier, escape, and closed-substrate reject
  reasons;
- prove inactive execution flags remain zero;
- prove the MIR JSON has typed report fields and the expected route surface.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.
