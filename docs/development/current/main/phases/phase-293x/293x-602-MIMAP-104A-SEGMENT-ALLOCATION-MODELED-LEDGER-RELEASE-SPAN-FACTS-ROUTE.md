# 293x-602 MIMAP-104A Segment Allocation Modeled Ledger Release Span Facts Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-104A` is the allocator behavior row selected by `MIMAP-103A`.

The scalar modeled allocation ledger already releases live tokens. This row
extends the release report with scalar span facts for the released allocation:

```text
modeled block start
request block count
modeled block end
allocation-time new page used
allocation-time remaining blocks
```

This is still ledger-local metadata. It does not mutate a real page free-list or
execute segment free.

## Result

Successful modeled ledger releases now expose scalar release span facts:

- `old_page_used_at_allocation`
- `page_capacity`
- `request_blocks`
- `new_page_used_at_allocation`
- `remaining_blocks_at_allocation`
- `modeled_block_end`
- `released_blocks`
- `release_span_present`

`MIMAP-104A` selects
`MIMAP-105A post-release-span-facts row selection` as the next planning row.

## Scope

- Extend `HakoAllocSegmentAllocationModeledLedgerReleaseReport` with release
  span fact fields.
- Populate those fields only on a successful live-token release.
- Add a proof app and guard that verify the span facts for normal and recycled
  releases.
- Keep existing MIMAP-097A / MIMAP-100A behavior compatible.

## Stop Lines

- No real segment allocation/free execution.
- No free-list mutation.
- No page state mutation outside the modeled ledger.
- No arena backing allocation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `104A.1` | Add release span facts to the ledger release report. | successful release reports expose deterministic span fields. | no real free |
| `104A.2` | Add proof app / SSOT / guard. | VM, MIR JSON, and pure-first EXE evidence pass. | no backend matcher |
| `104A.3` | Update ledgers and current pointers. | current pointer guard passes. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_span_facts_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/debug/hakorune --backend vm apps/hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof/main.hako
bash tools/checks/run_proof_app.sh --only MIMAP-104A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_span_facts_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
