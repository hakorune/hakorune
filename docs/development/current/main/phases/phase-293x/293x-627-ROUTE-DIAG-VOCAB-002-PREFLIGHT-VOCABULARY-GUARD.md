# 293x-627 ROUTE-DIAG-VOCAB-002 Preflight Vocabulary Guard

Status: selected current
Date: 2026-05-18

## Decision

`ROUTE-DIAG-VOCAB-002` is the next BoxShape cleanup row after
`ROUTE-DIAG-VOCAB-001`.

`ROUTE-DIAG-VOCAB-001` created the SSOT:

```text
docs/reference/mir/route-diagnostics-vocabulary.md
```

The next narrow cleanup is to prevent drift between the SSOT and
`tools/checks/pure_first_route_preflight.py`.

## Scope

- Add one lightweight static guard for route diagnostic vocabulary drift.
- Check that stable preflight reason tokens emitted by
  `pure_first_route_preflight.py` are named by the SSOT.
- Check that the guard itself is listed in `docs/tools/check-scripts-index.md`
  if a new public check script is added.
- Keep behavior unchanged.

## Stop Lines

- No new route acceptance shape.
- No proof vocabulary change.
- No Python preflight behavior change except static inspection plumbing.
- No backend allowlist changes.
- No allocator behavior.
- No source syntax.
- No broad guard generator work.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `RDVG.1` | Add static vocabulary drift guard. | guard reads the preflight script and SSOT without running EXE builds. | no behavior |
| `RDVG.2` | Wire guard into check index / manifest only if needed. | public command is discoverable. | no broad gate |
| `RDVG.3` | Verify existing preflight guard still passes. | old guard remains green. | no reason rewrite |
| `RDVG.4` | Close current pointers. | pointer guard and diff check pass. | no task bundle |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/pure_first_route_preflight_guard.sh
git diff --check
```
