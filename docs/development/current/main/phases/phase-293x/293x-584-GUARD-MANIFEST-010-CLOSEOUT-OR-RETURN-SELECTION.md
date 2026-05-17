# 293x-584 GUARD-MANIFEST-010 Closeout Or Return Selection

Status: selected current
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-010` is the planning row selected by
`GUARD-MANIFEST-009`.

All public hako_alloc closeout wrappers covered by this cleanup burst now
delegate through `run_row_guard.sh`, with thick implementations behind
`tools/checks/impl/`.

This row should decide whether to:

```text
1. close the guard manifest cleanup burst and return to the mimalloc row queue
2. add one final helper-extraction row for implementation-side closeout logic
```

## Stop Lines

- No allocator `.hako` behavior.
- No compiler acceptance change.
- No dev-gate / allocator-wide manifest pilot wiring.
- No broad generator rewrite.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM010.1` | Review manifest-backed closeout wrapper state. | next row is selected. | no implementation bundle |
| `GM010.2` | Choose return-to-MIMAP or helper extraction. | selected card exists. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

