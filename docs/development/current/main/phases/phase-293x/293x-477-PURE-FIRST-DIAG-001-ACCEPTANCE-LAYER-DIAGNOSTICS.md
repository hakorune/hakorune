# 293x-477 PURE-FIRST-DIAG-001 Acceptance Layer Diagnostics

Status: landed
Date: 2026-05-16

## Decision

`PURE-FIRST-DIAG-001` is a compiler diagnostics sidecar selected by
`MIMAP-040B`.

It makes the pure-first acceptance layer flow explicit and improves preflight
failure output so unsupported shapes are classified before backend emission
where possible.

## Scope

- Add a pure-first acceptance layer flow SSOT.
- Extend `tools/checks/pure_first_route_preflight.py` failure output with
  stable `layer` and `contract` fields.
- Add preflight classification for `return_shape=object_handle` routes that do
  not publish `target_result_box_name`.
- Extend the preflight guard with a minimal object-handle route fixture.

## Stop Lines

- Do not change backend lowering behavior.
- Do not change route generation semantics.
- Do not change allocator behavior.
- Do not add provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- Do not add app/box-name backend classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `DIAG.1` | Document acceptance layer flow. | Docs name parser/MIR/semantic-route/preflight/backend responsibilities. | no code first |
| `DIAG.2` | Add stable `layer`/`contract` fields to preflight failures. | Existing failures remain classified, but output is easier to triage. | no backend behavior |
| `DIAG.3` | Add object-handle concrete-box preflight. | Missing `target_result_box_name` fails before LLVM. | no route generation change |
| `DIAG.4` | Update guard. | Minimal fixture proves the new diagnostic vocabulary. | no app-specific matcher |

## Required Evidence

```text
python3 -m py_compile tools/checks/pure_first_route_preflight.py
bash tools/checks/pure_first_route_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row closes when preflight failures include layer/contract diagnostics and
the current blocker moves to the next planning row.

## Landed Implementation

```text
owners:
  tools/checks/pure_first_route_preflight.py
  tools/checks/pure_first_route_preflight_guard.sh
  docs/development/current/main/design/pure-first-acceptance-layer-flow-ssot.md
```

Closeout:

```text
current blocker moves to MIMAP-040C post-diagnostics row selection.
```
