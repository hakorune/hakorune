# P18: Stage-B loop canary capture fix

Scope: repair the Stage-B loop JSON canary and route its
Program(JSON v0) capture through the shared extractor.

P39 later archived the canary at
`tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh`.

## Why

The dev canary had two stale issues:

- it extracted Stage-B stdout with an inline `awk '/^{/,/^}$/'` range
- its Python validation command had two here-doc redirections, so the actual
  validation body was not executed

That made the canary look green without proving the Program(JSON v0) loop shape.

## Decision

Use `tools/selfhost/lib/stageb_program_json_capture.sh` for capture and pass the
captured JSON file explicitly into Python.

This keeps the canary dev-only. It does not add a new gate.

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh
bash tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
