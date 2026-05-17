# 293x-604 MIMAP-ROW-CADENCE-001 Mimalloc Row Validation Cadence SSOT

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-ROW-CADENCE-001` is the process cleanup row selected by `MIMAP-105A`.

The current mimalloc lane is intentionally proof-heavy. This row clarifies which
evidence level each row type needs so development can keep moving without
weakening stop-line safety.

## Scope

- Add a durable validation-cadence SSOT for mimalloc / hako_alloc rows.
- Define evidence levels for planning rows, allocator behavior rows,
  compatibility checks, closeout rows, and broad gates.
- Point current docs to that SSOT.
- Keep existing proof apps and guard entrypoints intact.

## Stop Lines

- No allocator behavior.
- No parser/compiler behavior.
- No guard weakening.
- No removal of landed proof apps or guards.
- No dev_gate / allocator-wide default growth.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `CADENCE.1` | Add validation-cadence SSOT. | row types and required evidence are explicit. | no guard weakening |
| `CADENCE.2` | Update task pointers / taskboard. | future rows can cite the SSOT. | no broad gate growth |
| `CADENCE.3` | Close the row and select the next planning row. | current pointer guard passes. | no allocator behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
