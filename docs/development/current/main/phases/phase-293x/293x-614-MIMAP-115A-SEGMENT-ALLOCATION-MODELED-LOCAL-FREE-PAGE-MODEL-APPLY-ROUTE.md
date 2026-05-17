# 293x-614 MIMAP-115A Segment Allocation Modeled Local-Free Page-Model Apply Route

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-115A` is the allocator behavior row selected by `MIMAP-114A`.

The scalar local-free lane now has:

```text
released-span ledger
  -> local-free candidate ledger
  -> local-free apply-plan ledger
```

This row opens one narrow page-local mutation seam:

```text
successful local-free apply-plan report
  + explicit HakoAllocPageModel
  -> release each block in the span through HakoAllocPageModel.releaseLocal
```

The page model already owns page-local `free` / `local_free` / `block_used`
state. This row must not introduce a second page-state owner.

## Result

`MIMAP-115A` landed by adding:

- `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako`
- `docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md`
- `apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/`
- `tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_model_apply_guard.sh`

It selects:

```text
MIMAP-116A post-local-free-page-apply row selection
```

## Validation Cadence

Cadence level:

```text
L2 proof row
```

Expected evidence:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-115A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_model_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Compatibility:

```text
bash tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh
```

is required only if this row changes `HakoAllocPageModel.releaseLocal` or page
model counters.

## Scope

Allowed:

- add a new page-model apply route owner under `lang/src/hako_alloc/memory/`;
- consume a successful `HakoAllocSegmentAllocationModeledLocalFreeApplyPlanReport`-shaped value;
- accept an explicitly supplied `HakoAllocPageModel`;
- validate page id, block span, and shape;
- call `HakoAllocPageModel.releaseLocal(block_id)` for each block in the plan
  span;
- expose scalar proof fields for applied block count, first failed block,
  page used/local-free counts before and after, and inactive substrate flags;
- reject invalid, source-rejected, page-id mismatch, partial apply, and
  unsupported substrate requests;
- add one focused proof app, manifest entry, SSOT, README/export wiring, and
  local guard.

Stop lines:

- no real segment allocation/free execution beyond the existing page-local
  `releaseLocal` model;
- no raw pointer residence;
- no segment-map pointer membership or lookup;
- no arena backing allocation;
- no atomic bitmap execution;
- no page-source or OSVM execution;
- no real thread scheduling or worker spawning;
- no source-level concurrency feature change;
- no provider activation, hook, host allocator replacement, or
  `#[global_allocator]`;
- no backend `.inc` app/name matcher;
- no direct mutation of page arrays outside `HakoAllocPageModel.releaseLocal`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `115A.1` | Add page-model apply route SSOT and owner boundary. | owner and stop lines are documented. | no raw pointer / segment-map |
| `115A.2` | Implement the narrow apply route over `releaseLocal`. | span releases are deterministic through page model API. | no direct page array mutation |
| `115A.3` | Add focused proof app and manifest entry. | `run_proof_app.sh --only MIMAP-115A` passes. | no broad gate |
| `115A.4` | Add public guard and current closeout docs. | dedicated guard and pointer guard pass. | no allocator-wide default growth |

## Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/debug/hakorune --backend vm apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/main.hako
bash tools/checks/run_proof_app.sh --only MIMAP-115A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_model_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
