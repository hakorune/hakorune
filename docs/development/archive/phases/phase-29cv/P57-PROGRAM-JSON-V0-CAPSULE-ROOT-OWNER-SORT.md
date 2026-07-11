---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: sort root Program(JSON v0) capsule owners after PHI probe cleanup.
Related:
  - tools/ROOT_SURFACE.md
  - tools/dev/program_json_v0/README.md
  - tools/selfhost/README.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
---

# P57 Program(JSON v0) Capsule Root Owner Sort

## Goal

Close the root hold queue item for Program(JSON v0) capsules without widening or
deleting the remaining compat routes.

This is a BoxShape cleanup slice. It separates weak manual dev probes from
broadly referenced route helpers.

## Owner Reading

- `dev_stagea.sh` and `dev_stageb.sh` are manual Stage-A/Stage-B Program(JSON
  v0) probes with no current root-owner need.
- `hakorune_emit_mir.sh` remains a shared helper with live route docs, smoke
  helpers, and probe integrations.
- `hakorune_emit_mir_compat.sh` and `hakorune_emit_mir_mainline.sh` remain thin
  preset wrappers owned by the bootstrap route SSOT.
- `selfhost_exe_stageb.sh` remains the explicit EXE bridge capsule used by
  selfhost build/probe routes.

## Decision

- move the Stage-A/Stage-B manual dev probes to
  `tools/dev/program_json_v0/`
- add `tools/dev/program_json_v0/README.md` as their capsule owner
- keep `hakorune_emit_mir*` and `selfhost_exe_stageb.sh` at tools root because
  they still have broad active route owners
- remove the Program(JSON v0) capsule group from the root hold queue

## Non-goals

- do not delete Program(JSON v0) compatibility support
- do not change Stage-A/Stage-B probe behavior
- do not move `hakorune_emit_mir*` or `selfhost_exe_stageb.sh` in this slice
- do not touch smoke shortcut owners

## Acceptance

```bash
bash -n tools/dev/program_json_v0/dev_stagea.sh
bash -n tools/dev/program_json_v0/dev_stageb.sh
bash -n tools/hakorune_emit_mir.sh
bash -n tools/hakorune_emit_mir_compat.sh
bash -n tools/hakorune_emit_mir_mainline.sh
bash -n tools/selfhost_exe_stageb.sh
test ! -e tools/dev_stagea.sh
test ! -e tools/dev_stageb.sh
rg -n 'tools/dev/program_json_v0/dev_stage(a|b).sh' \
  tools/dev/program_json_v0/README.md docs/development/current/main/phases/phase-132x
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/dev_stagea.sh' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/dev_stageb.sh' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
