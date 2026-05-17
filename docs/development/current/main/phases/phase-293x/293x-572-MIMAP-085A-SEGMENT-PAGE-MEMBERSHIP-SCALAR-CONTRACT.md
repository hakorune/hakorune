# 293x-572 MIMAP-085A Segment Page Membership Scalar Contract

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-085A` is the next row selected by `MIMAP-084A`.

The allocator lane now has page-local state and segment lifecycle state. This
row should add a small scalar contract for page/slice membership in a segment
without opening raw pointer membership, segment-map lookup, arena backing,
atomic bitmap execution, OSVM execution, provider activation, or backend
matchers.

## Scope

- Add a `.hako` owner for scalar segment/page membership classification.
- Add a focused proof app and local-run guard.
- Use stable scalar reason vocabulary and explicit inactive flags.
- Keep the model proof-only and same-owner.

## Required Vocabulary

```text
segment_id
page_id
slice_index
slice_count
segment_state
page_used
page_capacity
```

## Stop Lines

- No raw pointer residence.
- No segment-map pointer membership.
- No arena backing allocation.
- No atomic bitmap claim/unclaim.
- No OSVM execution, unreserve, or release.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `085A.1` | Add accepted SSOT. | owner, reasons, membership facts, inactive flags are specified. | no behavior before docs |
| `085A.2` | Add `.hako` owner. | scalar membership classifier exists. | no raw pointer/map lookup |
| `085A.3` | Add proof app. | VM and EXE output prove valid and rejected membership facts. | no backend matcher |
| `085A.4` | Add guard/index/manifest/module docs. | local-run guard pins owner/proof/stop lines. | no dev-gate default growth |
| `085A.5` | Close row. | evidence is recorded and next closeout/selection row is chosen. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation Result

`MIMAP-085A` added:

```text
docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md
lang/src/hako_alloc/memory/segment_page_membership_scalar_box.hako
apps/hako-alloc-segment-page-membership-scalar-proof/
tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_guard.sh
```

Proof output:

```text
hako-alloc-segment-page-membership-scalar-proof
members=1,0,40,7,3,16,1,2,8
accepted_states=1,4
rejects=1,2,3,4,5,6,7,8,9,10
inactive=0,0,0,0,0,0,0,0,0
counts=12,2,10,1,1,1,1,1,1,1,1,1,1,17,10
check=1
summary=ok
```

Next row:

```text
MIMAP-086A segment page membership closeout guard
```

