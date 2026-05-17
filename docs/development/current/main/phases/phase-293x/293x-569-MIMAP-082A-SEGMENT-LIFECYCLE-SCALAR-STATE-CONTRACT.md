# 293x-569 MIMAP-082A Segment Lifecycle Scalar State Contract

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-082A` is the next row selected by `MIMAP-081A`.

The segment / arena / bitmap boundary inventory is closed. This row should add
a small allocator-owned scalar contract for the segment lifecycle vocabulary
from the mimalloc lifecycle rewrite blueprint, without opening raw pointer,
atomic bitmap, OSVM, provider, or backend execution.

## Scope

- Add a `.hako` owner for scalar segment lifecycle state classification and
  transition checks.
- Add a focused proof app and local-run guard.
- Use stable scalar reason/state vocabulary and explicit inactive flags.
- Keep the model proof-only and same-thread/same-owner.

## Required Segment State Vocabulary

```text
Reserved
Active
PurgeScheduled
Purged
Abandoned
Reclaimed
Freed
```

## Required Transition Vocabulary

```text
Reserved -> Active
Active -> PurgeScheduled
PurgeScheduled -> Purged
Active -> Abandoned
Abandoned -> Reclaimed
Reclaimed -> Active
Active -> Freed
Purged -> Freed
```

## Stop Lines

- No raw pointer residence.
- No atomic bitmap claim/unclaim.
- No OSVM execution, unreserve, or release.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No arena backing allocation.
- No segment map pointer membership.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `082A.1` | Add accepted SSOT. | owner, states, transitions, reasons, inactive flags are specified. | no behavior before docs |
| `082A.2` | Add `.hako` owner. | scalar transition checker exists. | no OSVM/atomic/raw pointer |
| `082A.3` | Add proof app. | VM and EXE output prove valid and rejected transitions. | no backend matcher |
| `082A.4` | Add guard/index/manifest/module docs. | local-run guard pins owner/proof/stop lines. | no dev-gate default growth |
| `082A.5` | Close row. | evidence is recorded and next closeout/selection row is chosen. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation Result

`MIMAP-082A` added:

```text
docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md
lang/src/hako_alloc/memory/segment_lifecycle_scalar_state_box.hako
apps/hako-alloc-segment-lifecycle-scalar-state-proof/
tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_guard.sh
```

Proof output:

```text
hako-alloc-segment-lifecycle-scalar-state-proof
transitions=10,11,12,13,14,15,16,17
rejects=1,2,3,4,5,6,7,8
inactive=0,0,0,0,0,0,0
counts=16,8,8,1,1,1,1,1,1,1,1,34,8,0
check=1
summary=ok
```

Next row:

```text
MIMAP-083A segment lifecycle scalar state closeout guard
```

