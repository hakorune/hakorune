# 293x-453 MIR-EMIT-SSOT-002 Canonical Emit Wrapper

Status: landed
Date: 2026-05-16

## Decision

`MIR-EMIT-SSOT-002` is selected after same-artifact routing, route preflight,
and phase progress diagnostics landed.

It makes the external source-to-MIR authority explicit so guards and selfhost
tools do not each choose their own emit environment.

Decision:

```text
canonical external source-to-MIR route:
  tools/smokes/v2/lib/emit_mir_route.sh --route direct

thin facade:
  not added in this row
```

Existing route surface:

```text
tools/smokes/v2/lib/emit_mir_route.sh
  operational route SSOT for new smoke/check callers

tools/hakorune_emit_mir.sh
  internal compat-capsule implementation
  direct callers are guarded by tools/checks/hakorune_emit_mir_direct_caller_guard.sh
```

Therefore this row must not replace `tools/hakorune_emit_mir.sh` with another
large script. It should either promote `tools/smokes/v2/lib/emit_mir_route.sh`
as the canonical external entry, or add a thin facade over it if the command
shape needs a friendlier name.

SSOT:

```text
docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
```

## Scope

- Prefer promoting the existing route SSOT:

```text
tools/smokes/v2/lib/emit_mir_route.sh
```

- Optionally add a thin facade only if it delegates to the route SSOT:

```bash
tools/hako_emit_mir_json.sh --in app.hako --out app.mir.json
```

- Move pure-first guards and selfhost wrappers toward the canonical route
  entry.
- Document which direct CLI flags remain developer diagnostics.

## Stop Lines

- Do not remove direct CLI flags in this row.
- Do not replace `tools/hakorune_emit_mir.sh`; keep it as an internal
  compat-capsule unless a separate retirement card is selected.
- Do not bypass `tools/checks/hakorune_emit_mir_direct_caller_guard.sh`.
- Do not change Stage1 semantics.
- Do not add allocator behavior.
- Do not use this wrapper to hide route preflight failures.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `453.1` | Decide whether `emit_mir_route.sh` alone is the canonical external entry or whether a thin `hako_emit_mir_json.sh` facade is still useful. | No competing wrapper exists after the decision. | do not edit consumers yet |
| `453.2` | Document explicit env policy on the chosen entry. | One command owns guard/selfhost source-to-MIR emission. | no Stage1 semantics change |
| `453.3` | Migrate pure-first guard emit calls. | Guards stop invoking `target/debug/hakorune` directly for canonical source-to-MIR emission. | keep direct CLI for diagnostics |
| `453.4` | Migrate selfhost source-to-MIR calls. | Selfhost uses the canonical route for source emission and `--mir-in` for existing artifacts. | no duplicate emission |
| `453.5` | Add equivalence smoke. | Canonical route output and selfhost `--mir-out` output match under the same env. | no broad regression pack |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/pure_first_mir_artifact_exactness_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when guard/selfhost source-to-MIR emission has one documented
external route entry, `tools/hakorune_emit_mir.sh` remains an internal
compat-capsule, and direct compiler CLI routes are clearly diagnostic escape
hatches, not competing CI authorities.

## Landed Implementation

Files:

```text
tools/smokes/v2/lib/emit_mir_route.sh
tools/selfhost/lib/selfhost_build_direct.sh
tools/checks/lib/pure_first_exe_guard.sh
tools/checks/pure_first_mir_artifact_exactness_guard.sh
tools/checks/pure_first_route_preflight_guard.sh
tools/checks/canonical_mir_emit_route_guard.sh
docs/tools/check-scripts-index.md
```

Behavior:

- `emit_mir_route.sh --route direct` is the canonical external source-to-MIR
  route for pure-first/selfhost guards.
- The direct route owns `--backend mir --emit-mir-json`.
- `selfhost_build.sh --mir-out` delegates source emission to the canonical
  route.
- pure-first guard MIR emission delegates to the canonical route.
- same-artifact and route-preflight guards use the canonical route for source
  MIR emission fixtures.
- No `tools/hako_emit_mir_json.sh` facade was added; the existing route SSOT is
  sufficient for this row.

Evidence:

```text
bash -n tools/smokes/v2/lib/emit_mir_route.sh tools/selfhost/lib/selfhost_build_direct.sh tools/checks/lib/pure_first_exe_guard.sh tools/checks/pure_first_mir_artifact_exactness_guard.sh tools/checks/pure_first_route_preflight_guard.sh tools/checks/canonical_mir_emit_route_guard.sh
bash tools/checks/canonical_mir_emit_route_guard.sh
bash tools/checks/pure_first_mir_artifact_exactness_guard.sh
bash tools/checks/pure_first_route_preflight_guard.sh
bash tools/checks/selfhost_progress_diagnostics_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

Closeout:

```text
current blocker returns to MIMAP-029B.
```
