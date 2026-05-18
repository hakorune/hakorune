# Hako Alloc Segment Arena Backing Modeled No-Escape Address Residence SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Record an accepted MIMAP-244A no-escape address capability as a scalar/model
residence row without creating real raw pointer residence or pointer-derived
lookup.

The row treats the address carrier as a non-dereferenceable model token. It is
an allocator ledger fact, not a runtime pointer.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_no_escape_address_residence_box.hako
```

The owner may:

- observe an accepted no-escape address capability report;
- record segment id, arena id, lifetime generation, and address carrier facts;
- publish a modeled residence token equal to the scalar address carrier;
- reject missing / rejected / invalid capability reports;
- reject escape blockers and closed-substrate requirements.

The owner must not:

- create real raw pointer residence;
- perform pointer-derived lookup or dereference;
- allocate real arena backing;
- mutate a real segment-map;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- infer anything from owner names or backend matchers.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | modeled no-escape address residence accepted |
| `1` | capability report missing |
| `2` | capability report was rejected |
| `3` | no-escape address capability flag missing |
| `4` | address carrier invalid |
| `5` | return/storage/alias escape blocker present |
| `6` | real pointer residence would be required |
| `7` | pointer-derived lookup would be required |
| `8` | real arena backing allocation would be required |
| `9` | real segment-map mutation would be required |
| `10` | atomic bitmap execution would be required |
| `11` | OSVM/page-source execution would be required |
| `12` | worker/TLS/source-level concurrency would be required |
| `13` | provider activation / hook / host allocator replacement would be required |
| `14` | backend matcher or `.inc` owner-name shortcut would be required |

## Validation

MIMAP-248A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_guard.sh --level L2
```

The guard must:

- prove accepted modeled residence publication from a no-escape address
  capability report;
- prove missing / rejected / invalid / escape / closed-substrate reject reasons;
- prove inactive execution flags remain zero;
- prove the MIR JSON has typed report fields and the expected route surface.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
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
