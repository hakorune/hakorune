---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: root tool surface manifest and archive metadata schema.
Related:
  - tools/ROOT_SURFACE.md
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
  - tools/archive/manual-smokes/README.md
  - tools/archive/manual-tools/README.md
---

# P53 Root Tool Surface Manifest

## Goal

Make the remaining root-level tool surface visible before moving more legacy
helpers.

The post-P52 state is no longer a thin-wrapper cleanup hunt. The next safe step
is classification:

- protected root entrypoints stay in root
- compat capsules keep explicit owners
- debug probes get an owner or later move to debug/archive
- manual/archive entries get a deletion lifecycle

## Decision

- Add `tools/ROOT_SURFACE.md` as the root tool manifest.
- Link the lifecycle SSOT to the manifest.
- Add archive entry metadata fields to manual archive READMEs.
- Do not move AOT, native LLVM, PHI, or Program(JSON v0) capsule helpers in
  this slice.

## Current Reading

- `tools/mir13-migration-helper.sh` was already archived in P52.
- `tools/native_llvm_builder.py` is not an immediate archive candidate because
  `tools/ny_mir_builder.sh` still owns it under the explicit native backend
  canary route.
- `tools/selfhost_exe_stageb.sh` remains a compat capsule, not the
  `selfhost_build.sh` mainline facade.
- The CI/golden MIR chain remains protected:
  `core_ci.sh -> ci_check_golden.sh -> compare_mir.sh -> snapshot_mir.sh`.

## Non-goals

- Do not delete archived files.
- Do not move root helpers other than already-archived P52 files.
- Do not claim Program(JSON v0) delete-last work is complete.
- Do not change compiler behavior.

## Acceptance

```bash
test -f tools/ROOT_SURFACE.md
rg -n "tools/native_llvm_builder.py" tools/ROOT_SURFACE.md
rg -n "tools/selfhost_exe_stageb.sh" tools/ROOT_SURFACE.md
rg -n "original_path" tools/archive/manual-tools/README.md tools/archive/manual-smokes/README.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
