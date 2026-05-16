# 293x-450 MIR-EMIT-SSOT-001 Pure-First MIR Artifact Exactness

Status: landed
Date: 2026-05-16

## Decision

`MIR-EMIT-SSOT-001` is the current compiler/selfhost BoxShape sidecar before
`MIMAP-029B`.

It fixes the pure-first route so a guard that preflights MIR and then builds an
EXE consumes the exact same MIR JSON artifact. It does not add allocator
behavior and does not change backend route capability.

SSOT:

```text
docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
```

## Scope

- Add `--mir-in FILE` to the selfhost build route.
- Add `--mir-out FILE` as the explicit source-to-MIR output spelling.
- Keep `--mir FILE` as a temporary compatibility alias for `--mir-out`.
- Ensure `--mir-in` consumes an existing MIR JSON file and does not re-emit MIR
  from source.
- Update pure-first guards so the checked MIR artifact is the EXE input
  artifact.
- Add or update a guard/probe that proves EXE build does not rewrite the MIR
  artifact after preflight.

## Non-Goals

- No allocator behavior.
- No `MIMAP-029A` owner widening.
- No `MIMAP-029B` row selection.
- No C shim / `.inc` name matcher shortcut.
- No route capability expansion.
- No broad return-type inference.
- No provider hook, host allocator replacement, or `#[global_allocator]`.

## Current Route Shape To Fix

Current problem shape:

```text
pure-first guard:
  emit MIR JSON
  inspect MIR JSON

selfhost_build.sh --mir <same path>:
  re-emit MIR JSON to that path
  build EXE from the second artifact
```

Required shape:

```text
emit:
  source -> MIR JSON artifact

preflight:
  inspect that MIR JSON artifact

EXE:
  selfhost_build.sh --mir-in <that MIR JSON artifact> --exe <out>
```

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `450.1` | Document selfhost MIR argument semantics. | `--mir-in`, `--mir-out`, and `--mir` compatibility meaning are written in the active card/SSOT. | do not change behavior yet |
| `450.2` | Implement selfhost route parsing for `--mir-in` / `--mir-out`. | `--mir-in` reaches the EXE lane as an existing MIR artifact; `--mir` still works as `--mir-out`. | no source re-emit under `--mir-in` |
| `450.3` | Change pure-first guard build route to use `--mir-in`. | The guard emits MIR once and builds EXE from that same file. | no guard-specific backend shortcut |
| `450.4` | Add schema sanity to the exactness guard. | The preflight artifact contains `functions[].metadata.lowering_plan` where current route checks expect it. | no route classifier yet |
| `450.5` | Add artifact exactness guard. | Guard fails if EXE build rewrites or regenerates the preflight MIR artifact. | no heavy regression pack by default |
| `450.6` | Re-run MIMAP-029A guard and quick gate. | Existing allocator proof remains green through exact same artifact route. | no allocator row selection |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/pure_first_mir_artifact_exactness_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
tools/checks/dev_gate.sh quick
```

If the exactness guard is introduced during this row, run it before the MIMAP
guard so a route mismatch fails near the cause.

## Return Condition

This row closes when:

```text
preflight MIR SHA == EXE input MIR SHA
selfhost --mir-in does not re-emit MIR
preflight artifact exposes functions[].metadata.lowering_plan
pure-first guards use --mir-in for EXE build
MIMAP-029A proof still passes
```

After closeout, continue to `MIR-ROUTE-PREFLIGHT-001` before returning to
`MIMAP-029B`, unless the exactness row exposes a smaller blocker that must be
split first.

## Landed Implementation

Implementation:

```text
tools/selfhost/selfhost_build.sh
tools/selfhost/lib/selfhost_build_route.sh
tools/selfhost/lib/selfhost_build_direct.sh
tools/selfhost/lib/selfhost_build_run.sh
tools/selfhost/lib/selfhost_build_exe.sh
tools/checks/lib/pure_first_exe_guard.sh
tools/checks/pure_first_mir_artifact_exactness_guard.sh
docs/tools/check-scripts-index.md
```

Landed behavior:

```text
--mir-in FILE:
  consumes an existing MIR JSON artifact for EXE/run routes and does not
  re-emit source.

--mir-out FILE:
  explicit source-to-MIR output spelling.

--mir FILE:
  compatibility alias for --mir-out.

pure_first_guard_build_exe:
  passes --mir-in to selfhost_build.sh and hashes the MIR artifact before and
  after EXE build.
```

The exactness guard also checks that a generated MIR artifact exposes
`functions[].metadata.lowering_plan`, keeping the 451 preflight schema sanity
visible before the route classifier is implemented.

Evidence:

```text
bash tools/checks/pure_first_mir_artifact_exactness_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

MIR-EMIT-SSOT-001 is closed. The active blocker moves to
`MIR-ROUTE-PREFLIGHT-001`.
