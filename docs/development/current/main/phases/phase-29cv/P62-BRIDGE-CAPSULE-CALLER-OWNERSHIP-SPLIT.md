---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify the remaining Program(JSON v0)->MIR bridge callers before archive/replacement work.
Related:
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/development/current/main/phases/phase-29cv/P37-PROGRAM-JSON-V0-COMPAT-CAPSULE-SSOT.md
  - tools/selfhost/lib/program_json_mir_bridge.sh
  - tools/selfhost_exe_stageb.sh
  - tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
---

# P62 Bridge Capsule Caller Ownership Split

## Goal

Prevent the bridge capsule from looking smaller than it is before deletion or
archive work starts.

## Decision

- record the direct `program_json_mir_bridge_emit()` callers:
  - `tools/selfhost_exe_stageb.sh`
  - `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
  - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
- document that `tools/selfhost_exe_stageb.sh` is route-split:
  `stageb-delegate` is the Program(JSON v0) bridge capsule, while `direct` is
  a MIR-first probe route
- keep indirect `tools/selfhost_exe_stageb.sh` callers owned by the selected
  emit route instead of counting them as independent bridge-helper callers
- classify the standalone phase29ci bridge proof as archive-after-replacement,
  not archive-now
- classify the phase29cg PHI/LLVM proof as keep until that proof no longer
  needs Program(JSON v0) input

## Non-goals

- do not delete or move bridge callers in this card
- do not change `HAKORUNE_STAGE1_EMIT_ROUTE` defaults
- do not claim Rust/public Program(JSON v0) delete-last is unblocked

## Acceptance

```bash
rg --fixed-strings "program_json_mir_bridge_emit" tools/selfhost_exe_stageb.sh tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
bash -n tools/selfhost/lib/program_json_mir_bridge.sh tools/selfhost_exe_stageb.sh tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
