# 293x-004 Real Apps EXE Boundary Probe

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: document and gate the current VM -> EXE transition before starting
  the real allocator port.

## Finding

The three real apps are VM-green, but direct EXE build is not green yet.

The current direct route:

```text
source .hako -> direct MIR(JSON) -> ny-llvmc pure-first -> EXE
```

reaches the backend boundary and fail-fasts with:

```text
unsupported pure shape for current backend recipe
```

With route trace enabled, the first unsupported owner is:

```text
[llvm-pure/unsupported-shape] ... first_op=newbox owner_hint=mir_normalizer ...
```

## Decision

- Keep VM as the active real-app correctness gate.
- Add a separate EXE boundary probe suite instead of pretending EXE parity is
  green.
- Keep `HAKO_BACKEND_COMPAT_REPLAY=none`; harness replay is not accepted as
  mainline proof.
- The real allocator port should wait until the EXE boundary blocker has a
  concrete owner decision, or should explicitly remain VM-only if it is a
  policy/state prototype.

## Changes

- Added `tools/smokes/v2/profiles/integration/apps/real_apps_exe_boundary_probe.sh`.
- Added `tools/smokes/v2/suites/integration/real-apps-exe-boundary.txt`.
- Updated phase-293x docs/current pointers.

## Verification

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
