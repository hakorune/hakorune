# 293x-584 GUARD-MANIFEST-010 Closeout Or Return Selection

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-010` is the planning row selected by
`GUARD-MANIFEST-009`.

All public hako_alloc closeout wrappers covered by this cleanup burst now
delegate through `run_row_guard.sh`, with thick implementations behind
`tools/checks/impl/`.

This row decides to close the manifest-wrapper cleanup burst, keep helper
extraction parked, and return to the mimalloc row queue at `MIMAP-087A`.

The final cleanup is the no-growth guard itself: instead of repeating the 12
public closeout wrappers in a second table,
`tools/checks/k2_wide_manifest_wrapper_guard.sh` derives expected wrappers from
the `hako-alloc-closeout` profile in `tools/checks/guard_rows.toml`.

Closed options:

```text
1. close the guard manifest cleanup burst and return to the mimalloc row queue
2. leave implementation-side closeout helper extraction parked until a future
   guard family shows repeated non-wrapper logic
```

## Stop Lines

- No allocator `.hako` behavior.
- No compiler acceptance change.
- No dev-gate / allocator-wide manifest pilot wiring.
- No broad generator rewrite.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM010.1` | Review manifest-backed closeout wrapper state. | 12 public hako_alloc closeout wrappers are manifest-backed thin wrappers. | no implementation bundle |
| `GM010.2` | Remove duplicate closeout wrapper truth from the no-growth guard. | guard derives expected wrappers from `hako-alloc-closeout`. | no generator rewrite |
| `GM010.3` | Choose return-to-MIMAP or helper extraction. | `MIMAP-087A` remains the selected current mimalloc row. | no behavior |

## Required Evidence

```text
bash tools/checks/k2_wide_manifest_wrapper_guard.sh
tools/checks/run_row_guard.sh --profile hako-alloc-closeout --dry-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
