# 293x-628 MIMAP-124A Post-Route-Diagnostics-Cleanup Row Selection

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-124A` is a planning-only row after the route diagnostics cleanup wave:

```text
ROUTE-FIXPOINT-001
ROUTE-DIAG-VOCAB-001
ROUTE-DIAG-VOCAB-002
```

The next step should return to the mimalloc / hako_alloc implementation lane
unless the current allocator row exposes another concrete compiler acceptance
blocker.

## Scope

- Review the current segment allocation modeled lane and recent compiler
  cleanup evidence.
- Select exactly one next row.
- Classify the selected row as allocator behavior, closeout, compiler
  acceptance, language ergonomics, or cleanup.
- Keep provider activation and host allocator replacement closed.

## Stop Lines

- No allocator behavior in this row.
- No compiler route behavior.
- No source syntax.
- No guard bundle.
- No provider activation.
- No host allocator replacement.
- No backend matchers.
- No silent fallback.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `124A.1` | Read the taskboard and granularity SSOT. | next candidates are concrete. | no implementation |
| `124A.2` | Pick exactly one next row. | selected card exists with owner/proof/guard/stop lines. | no bundle |
| `124A.3` | Update current pointers. | pointer guard and diff check pass. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
