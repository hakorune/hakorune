# 293x-600 HAKO-ALLOC-SRC-CLEAN-001 Segment Counter Compound Assignment Cleanup

Status: selected current
Date: 2026-05-17

## Decision

`HAKO-ALLOC-SRC-CLEAN-001` is the cleanup sidecar selected by `MIMAP-102A`.

The C199 compound assignment surface is already live for field targets. This
row uses that existing syntax to reduce noisy same-field counter increments in
the current segment allocation modeled lane.

This is not a new language feature row.

## Scope

Rewrite only exact same-field increments:

```hako
me.counter = me.counter + 1
```

to:

```hako
me.counter += 1
```

Allowed files:

- `lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako`
- `lang/src/hako_alloc/memory/segment_allocation_modeled_consume_box.hako`
- `lang/src/hako_alloc/memory/segment_allocation_readiness_scalar_box.hako`
- `lang/src/hako_alloc/memory/segment_page_membership_scalar_box.hako`
- `lang/src/hako_alloc/memory/segment_lifecycle_scalar_state_box.hako`

## Stop Lines

- No parser/compiler change.
- No new compound-assignment semantics.
- No allocator behavior change.
- No proof-app or source-wide formatting bundle.
- No local counter rewrite.
- No `me.FIELD = me.FIELD - 1` rewrite.
- No non-segment hako_alloc memory rewrite.
- No real segment allocation/free execution.
- No arena backing allocation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling or worker spawning.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `CLEAN.1` | Rewrite same-field segment counter increments to `+= 1`. | no remaining exact same-field `+ 1` matches in allowed files. | no broader formatting |
| `CLEAN.2` | Re-run compound-assignment syntax guard. | C199 guard passes. | no compiler changes |
| `CLEAN.3` | Re-run focused segment allocator guards. | current segment lane guards pass. | no behavior change |
| `CLEAN.4` | Close the row and select the next planning row. | current pointer guard passes. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_compound_assignment_surface_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_readiness_scalar_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_consume_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
