# 293x-452 SELFHOST-PROGRESS-001 Phase Progress Diagnostics

Status: landed
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

`selfhost.link` is reserved for the future point where the shell wrapper owns a
separate link command. Today `ny-llvmc --emit exe` is a combined compiler/linker
phase from this wrapper's perspective, so the active shell boundary is
`selfhost.nyllvmc`.

Example text output:

```text
[selfhost] phase=selfhost.emit_mir start
[selfhost] phase=selfhost.emit_mir done elapsed_ms=123
[selfhost] phase=selfhost.route_preflight start
[selfhost] phase=selfhost.nyllvmc start
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

## Landed Implementation

Files:

```text
tools/selfhost/lib/selfhost_progress.sh
tools/selfhost/selfhost_build.sh
tools/selfhost/lib/selfhost_build_direct.sh
tools/selfhost/lib/selfhost_build_exe.sh
tools/selfhost/lib/selfhost_build_run.sh
tools/checks/lib/pure_first_exe_guard.sh
tools/checks/selfhost_progress_diagnostics_guard.sh
docs/tools/check-scripts-index.md
```

Behavior:

- `selfhost_progress.sh` owns stable phase start/done/fail lines.
- `selfhost_build.sh --mir-out` reports `selfhost.emit_mir`.
- `selfhost_build.sh --mir-in --exe` reports `selfhost.nyllvmc`.
- `selfhost_build.sh --mir-in --run` reports `selfhost.run` and records fail
  state when the MIR input is invalid.
- pure-first guards report `selfhost.route_preflight` before `--mir-in` EXE
  build.
- pure-first build failure prints the last `HAKO_SELFHOST_PROGRESS_FILE`
  breadcrumb when available.

Evidence:

```text
bash -n tools/selfhost/lib/selfhost_progress.sh tools/selfhost/selfhost_build.sh tools/selfhost/lib/selfhost_build_direct.sh tools/selfhost/lib/selfhost_build_exe.sh tools/selfhost/lib/selfhost_build_run.sh tools/checks/lib/pure_first_exe_guard.sh tools/checks/selfhost_progress_diagnostics_guard.sh
bash tools/checks/selfhost_progress_diagnostics_guard.sh
bash tools/checks/pure_first_mir_artifact_exactness_guard.sh
bash tools/checks/pure_first_route_preflight_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

Closeout:

```text
current blocker moves to MIR-EMIT-SSOT-002.
```
