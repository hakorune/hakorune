# 293x-631 MIMAP-125A Post-Source-Cleanup Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-125A` is a planning-only row after the focused source-cleanup slices:

```text
RUNTIME-UNWRAP-001
WASM-LOG-001
```

The next step should return to the mimalloc / hako_alloc implementation lane
unless the next allocator row exposes a concrete compiler or language
acceptance blocker.

Selected row:

```text
MIMAP-126A
  segment allocation modeled local-free reuse route
```

Rationale:

```text
MIMAP-119A / MIMAP-121A closed the chain from released-span facts to an
explicit page-local local_free mutation through HakoAllocPageModel.releaseLocal.

The smallest allocator behavior that advances that lane is to prove that the
released local_free blocks can be collected by the existing page-local
HakoAllocPageModel.acquire path and reused without opening real segment
allocation/free, raw pointer residence, segment-map lookup, atomics, OSVM, or
provider activation.
```

## Scope

- Review the current segment allocation modeled lane.
- Select exactly one next mimalloc / hako_alloc or Hakorune compiler row.
- Keep provider activation and host allocator replacement closed.

## Stop Lines

- No allocator behavior in this row.
- No compiler route behavior.
- No source syntax.
- No cleanup bundle.
- No provider activation.
- No host allocator replacement.
- No backend matchers.
- No silent fallback.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `125A.1` | Read the taskboard and granularity SSOT. | next candidates are concrete. | no implementation |
| `125A.2` | Pick exactly one next row. | selected card exists with owner/proof/guard/stop lines. | no bundle |
| `125A.3` | Update current pointers. | pointer guard and diff check pass. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`MIMAP-125A` selected `MIMAP-126A segment allocation modeled local-free reuse
route`.

The selected row is intentionally narrow:

- owner: `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako`
- proof app:
  `apps/hako-alloc-segment-allocation-modeled-local-free-reuse-proof/`
- guard:
  `tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_guard.sh`
- validation level: `L2 proof`, plus focused compatibility only if the row
  changes existing report surfaces
- stop lines: no real segment allocation/free, direct page array mutation,
  raw pointer residence, segment-map lookup, arena backing, atomic bitmap,
  page-source/OSVM call, thread scheduling, provider activation, host
  replacement, or backend matcher
