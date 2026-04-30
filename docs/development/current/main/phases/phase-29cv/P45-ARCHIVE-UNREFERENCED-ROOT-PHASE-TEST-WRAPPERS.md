---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive unreferenced root phase/test smoke wrappers in one faster-cleanup batch.
Related:
  - docs/development/current/main/phases/phase-29cv/P44-ARCHIVE-WINDOWS-NY-PARSER-WRAPPERS.md
  - tools/archive/manual-smokes/README.md
---

# P45 Archive Unreferenced Root Phase/Test Wrappers

## Goal

Apply the faster cleanup rule to root-level phase/test wrappers that have no
current active reference:

```text
active refなし + current PASSなし + capsule ownerなし = archive/delete
```

## Decision

Move these root wrappers to `tools/archive/manual-smokes/`:

- `tools/tlv_roundtrip_smoke.sh`
- `tools/mir_builder_exe_smoke.sh`
- `tools/phase24_comprehensive_smoke.sh`
- `tools/test_joinir_freeze_inventory.sh`
- `tools/test_loopssa_breakfinder_min.sh`
- `tools/test_loopssa_breakfinder_slot.sh`
- `tools/test_phase132_phi_ordering.sh`
- `tools/test_phase133_console_llvm.sh`

They are preserved as manual/historical evidence only. Active proof should stay
on role-first smoke lanes, current phase gates, or named compat capsules.

## Non-goals

- do not archive build helpers or broad utility scripts in this batch
- do not run heavy historical phase suites as acceptance
- do not change compiler behavior

## Acceptance

```bash
bash -n \
  tools/archive/manual-smokes/tlv_roundtrip_smoke.sh \
  tools/archive/manual-smokes/mir_builder_exe_smoke.sh \
  tools/archive/manual-smokes/phase24_comprehensive_smoke.sh \
  tools/archive/manual-smokes/test_joinir_freeze_inventory.sh \
  tools/archive/manual-smokes/test_loopssa_breakfinder_min.sh \
  tools/archive/manual-smokes/test_loopssa_breakfinder_slot.sh \
  tools/archive/manual-smokes/test_phase132_phi_ordering.sh \
  tools/archive/manual-smokes/test_phase133_console_llvm.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P45-ARCHIVE-UNREFERENCED-ROOT-PHASE-TEST-WRAPPERS.md' --fixed-strings \
  -e 'tools/tlv_roundtrip_smoke.sh' \
  -e 'tools/mir_builder_exe_smoke.sh' \
  -e 'tools/phase24_comprehensive_smoke.sh' \
  -e 'tools/test_joinir_freeze_inventory.sh' \
  -e 'tools/test_loopssa_breakfinder_min.sh' \
  -e 'tools/test_loopssa_breakfinder_slot.sh' \
  -e 'tools/test_phase132_phi_ordering.sh' \
  -e 'tools/test_phase133_console_llvm.sh' \
  docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
