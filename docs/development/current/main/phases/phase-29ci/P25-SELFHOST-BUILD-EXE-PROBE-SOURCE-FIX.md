---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: keep the `phase29ci_selfhost_build_exe_consumer_probe.sh` helper-local proof sourcing helper files directly after `selfhost_build.sh` became a thin facade.
Related:
  - docs/development/current/main/phases/phase-29ci/P24-STAGE1-CONTRACT-MODE-ALIAS-PRUNE.md
  - tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
  - tools/selfhost/lib/selfhost_build_exe.sh
---

# P25 Selfhost Build EXE Probe Source Fix

## Goal

`tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh` used to extract a
prelude from `selfhost_build.sh`. That stopped working once `selfhost_build.sh`
became a thin facade over helper files: sourcing the extracted prelude now calls
`selfhost_build_main` and exits with `--in <file.hako> is required`.

Source the exact helper files directly instead. The probe remains a
helper-local proof for the Program(JSON v0)->MIR(JSON)->EXE consumer seam and
does not reopen the public `selfhost_build.sh --exe` route.

## Acceptance

```bash
bash -n tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
bash tools/checks/current_state_pointer_guard.sh
```
