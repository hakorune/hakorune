# P7: run route doc/fmt cleanup

Scope: close the small cleanup left after P6 direct `--run`.

## Why

P6 moved normal `selfhost_build.sh --run` to direct source -> MIR(JSON) ->
`--mir-json-file`. Two follow-ups should be kept separate from that behavior
change:

- `cargo fmt --check` failed on an existing CLI arg test formatting shape.
- Older phase/route docs still described P1/P4 from the pre-P6 viewpoint.

## Decision

Format the CLI args test and add explicit status notes instead of rewriting old
phase history. The current route SSOT should name the direct run route so future
cleanup does not confuse normal `--run` with diagnostic Program(JSON v0)
artifact routes.

## Acceptance

```bash
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
