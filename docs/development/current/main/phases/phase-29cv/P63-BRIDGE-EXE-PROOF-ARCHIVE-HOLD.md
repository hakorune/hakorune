---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: keep the standalone bridge-to-EXE proof after checking the nearest replacement candidate.
Related:
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/development/current/main/phases/phase-29cv/P62-BRIDGE-CAPSULE-CALLER-OWNERSHIP-SPLIT.md
  - tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
  - tools/selfhost_exe_stageb.sh
---

# P63 Bridge EXE Proof Archive Hold

## Goal

Avoid archiving the standalone Program(JSON v0)->MIR->EXE proof before a real
replacement is green.

## Evidence

The closest replacement candidate was:

```bash
HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate \
  bash tools/selfhost_exe_stageb.sh apps/tests/hello_simple_llvm.hako -o /tmp/phase29cv_bridge_probe.exe
```

It entered the bridge capsule but failed in the selected ny-llvmc driver with:

```text
unsupported pure shape for current backend recipe
```

The existing standalone bridge proof still passes:

```bash
bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
```

## Decision

- keep `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
- do not archive it behind `tools/selfhost_exe_stageb.sh stageb-delegate` yet
- require a green replacement proof before removing the standalone bridge-to-EXE
  probe from the live capsule table

## Non-goals

- do not change backend recipe support here
- do not change `selfhost_exe_stageb.sh` route defaults
- do not widen Program(JSON v0) public compatibility

## Acceptance

```bash
bash -n tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh tools/selfhost_exe_stageb.sh
bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
