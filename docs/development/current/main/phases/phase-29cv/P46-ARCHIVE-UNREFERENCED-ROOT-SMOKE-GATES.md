---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive unreferenced root singleton smoke/gate wrappers.
Related:
  - docs/development/current/main/phases/phase-29cv/P45-ARCHIVE-UNREFERENCED-ROOT-PHASE-TEST-WRAPPERS.md
  - tools/archive/manual-smokes/README.md
---

# P46 Archive Unreferenced Root Smoke Gates

## Goal

Continue faster cleanup for root-level singleton smoke/gate wrappers that have
no active reference and no current capsule owner.

## Decision

Move these root wrappers to `tools/archive/manual-smokes/`:

- `tools/nyfmt_smoke.sh`
- `tools/selfhost_emitter_usings_gate_smoke.sh`
- `tools/selfhost_progress_guard_smoke.sh`
- `tools/smoke_provider_modes.sh`
- `tools/test_filebox_fallback_smoke.sh`
- `tools/using_prefix_strict_smoke.sh`
- `tools/vm_filebox_smoke.sh`

They are manual/historical evidence only. Active coverage should come from
role-first smoke lanes, current phase gates, or named compat capsules.

## Non-goals

- do not archive broad diagnostic utilities in this batch
- do not archive active using/filebox smoke lanes that have current references
- do not change compiler behavior

## Acceptance

```bash
bash -n \
  tools/archive/manual-smokes/nyfmt_smoke.sh \
  tools/archive/manual-smokes/selfhost_emitter_usings_gate_smoke.sh \
  tools/archive/manual-smokes/selfhost_progress_guard_smoke.sh \
  tools/archive/manual-smokes/smoke_provider_modes.sh \
  tools/archive/manual-smokes/test_filebox_fallback_smoke.sh \
  tools/archive/manual-smokes/using_prefix_strict_smoke.sh \
  tools/archive/manual-smokes/vm_filebox_smoke.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P46-ARCHIVE-UNREFERENCED-ROOT-SMOKE-GATES.md' --fixed-strings \
  -e 'tools/nyfmt_smoke.sh' \
  -e 'tools/selfhost_emitter_usings_gate_smoke.sh' \
  -e 'tools/selfhost_progress_guard_smoke.sh' \
  -e 'tools/smoke_provider_modes.sh' \
  -e 'tools/test_filebox_fallback_smoke.sh' \
  -e 'tools/using_prefix_strict_smoke.sh' \
  -e 'tools/vm_filebox_smoke.sh' \
  docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
