# Hako Alloc Segment Arena Backing No-Escape Address Capability Diagnostics SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Publish observer-only diagnostics for the MIMAP-244A no-escape address
capability inventory.

MIMAP-245A consumes the capability inventory counters plus the last capability
report, then records scalar summary facts for matrix, lifetime, address-carrier,
escape, and closed-substrate reject categories.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_no_escape_address_capability_diagnostic_box.hako
```

The owner may:

- observe MIMAP-244A inventory counters;
- summarize matrix, lifetime, address-carrier, return/storage/alias escape,
  pointer residence, pointer lookup, arena backing, segment-map, atomic bitmap,
  OSVM, worker, provider, and backend matcher reject categories;
- publish last capability report facts and inactive execution flags.

The owner must not:

- record capability rows itself;
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
| `0` | diagnostic summary accepted |
| `1` | inventory or capability report missing / not applicable |
| `2-15` | reserved for the MIMAP-244A no-escape address capability reason surface |

The accepted diagnostic report mirrors the last MIMAP-244A capability reason in
`reason`; observer owner counters still use `0`/`1` for its own accept/reject
state.

## Validation

MIMAP-245A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_diagnostics_guard.sh --level L2
```

The guard must:

- prove diagnostic observation after the MIMAP-244A inventory;
- prove category flags for matrix, lifetime, address, escape, and every closed
  substrate requirement;
- prove empty inventory is rejected;
- prove inactive flags remain zero.

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
