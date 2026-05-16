# 293x-452 SELFHOST-PROGRESS-001 Phase Progress Diagnostics

Status: ready
Date: 2026-05-16

## Decision

`SELFHOST-PROGRESS-001` follows `MIR-ROUTE-PREFLIGHT-001`.

It makes pure-first/selfhost wrappers report enough phase progress that a user
can distinguish slow compilation, no-output timeout, unsupported route, and
backend/link failure.

SSOT:

```text
docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
```

## Scope

- Add stable phase start/done lines around long selfhost steps.
- Include elapsed time for completed phases where the shell wrapper can measure
  it cheaply.
- Add timeout/no-output closeout that prints the last known phase.
- Keep text progress stable first; JSONL events are a later extension.

## Phase Tags

Initial required phase names:

```text
selfhost.emit_mir
selfhost.route_preflight
selfhost.nyllvmc
selfhost.link
selfhost.run
```

Example text output:

```text
[selfhost] phase=emit_mir start
[selfhost] phase=emit_mir done elapsed_ms=123
[selfhost] phase=route_preflight start
[selfhost] phase=nyllvmc start
```

## Stop Lines

- Do not change compiler behavior.
- Do not add verbose dumps by default.
- Do not add unguarded multi-line debug spam.
- Do not mix allocator behavior with diagnostics.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `452.1` | Define phase tag names in docs and wrapper helpers. | Tags are stable and short. | no JSONL yet |
| `452.2` | Add phase start/done output to selfhost route helpers. | Long phases are visible in normal guard logs. | no behavior change |
| `452.3` | Add timeout/no-output closeout. | Failure report includes the last phase. | no heavy log dump |
| `452.4` | Add a diagnostics guard. | Guard can distinguish a route-preflight failure from a backend/link phase. | no allocator app dependency |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/selfhost_progress_diagnostics_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when a failed or timed-out pure-first build leaves a concise
last-phase breadcrumb in the log.
