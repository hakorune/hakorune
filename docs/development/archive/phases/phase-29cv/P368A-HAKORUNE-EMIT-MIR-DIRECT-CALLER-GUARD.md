---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: lock direct callers of the root `hakorune_emit_mir.sh` helper behind the route SSOT
Related:
  - tools/hakorune_emit_mir.sh
  - tools/smokes/v2/lib/emit_mir_route.sh
  - tools/checks/hakorune_emit_mir_direct_caller_guard.sh
---

# P368A: Hakorune Emit MIR Direct Caller Guard

## Intent

Keep `tools/hakorune_emit_mir.sh` as an internal compat-capsule
implementation while preventing new smoke/check/dev scripts from calling it
directly.

The route SSOT for smoke/check/perf/dev callers is
`tools/smokes/v2/lib/emit_mir_route.sh`.

## Boundary

Allowed:

- migrate the remaining smoke direct call to `emit_mir_route.sh`
- update route environment diagnostics to print the route helper command
- add a no-growth guard for direct `hakorune_emit_mir.sh` shell execution
- wire the guard into quick gate

Not allowed:

- delete `tools/hakorune_emit_mir.sh`
- change `hakorune_emit_mir.sh` route semantics
- delete top-level compatibility preset wrappers
- change selfhost runtime temp-MIR behavior

## Direct Caller Contract

Only these owners may execute `tools/hakorune_emit_mir.sh` directly:

- `tools/hakorune_emit_mir_mainline.sh`
- `tools/hakorune_emit_mir_compat.sh`
- `tools/selfhost/lib/selfhost_run_routes.sh`

All smoke/check/perf/dev routes must go through
`tools/smokes/v2/lib/emit_mir_route.sh`.

## Acceptance

```bash
bash tools/checks/hakorune_emit_mir_direct_caller_guard.sh
bash tools/smokes/v2/lib/emit_mir_route.sh --route hako-helper --timeout-secs 60 --out /tmp/phase29cl_launcher_cutover.mir.json --input lang/src/runner/entry/launcher_native_entry.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The full `phase29cl_by_name_lock_vm.sh` smoke also covers an unrelated
`phase29cl_by_name_mainline_guard.sh` owner allowlist. This card validates the
changed MIR emit route command directly.
