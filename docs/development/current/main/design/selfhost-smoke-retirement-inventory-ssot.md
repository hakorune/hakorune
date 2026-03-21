---
Status: SSOT
Scope: Selfhost smoke retirement inventory
Decision: accepted
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md
- tools/smokes/v2/run.sh
---

# Selfhost smoke retirement inventory

## Goal

Separate current selfhost gate coverage from old opt-in canaries and always-skipped Mini-VM
smokes so future cleanup does not delete live profile entries by mistake.

## Rules

- Do not delete selfhost smoke scripts only because repo grep shows few or zero callers.
- `tools/smokes/v2/run.sh` auto-discovers `*.sh` under a profile directory, but support buckets
  named `archive/lib/tmp/fixtures` are pruned from live discovery.
- Profile members stay live until they either:
  - move into a discovery-pruned support bucket or archive profile, or
  - are replaced by a documented semantic successor and removed from active docs/gates.
- Remove a selfhost smoke only after the script is either:
  - moved out of the active profile directory, or
  - replaced by a documented semantic successor and verified to be absent from active docs/gates.
- Always-skipped Mini-VM smokes may be tagged as `retire candidate`, but deletion still requires a
  dedicated retire slice.
- Stage-B opt-in canaries may be tagged as `opt-in archive candidate`, then moved to
  `tools/smokes/v2/profiles/archive/selfhost/` once archive/home is chosen.

## Categories

- `keep-current-entry`
  - active selfhost gate entry or daily smoke
- `retire candidate`
  - always skipped in the default lane and not referenced by active docs/gates
- `opt-in archive candidate`
  - manual/diagnostic canary behind explicit env flags, not part of the current daily lane

## Phase status

- Phase A: inventory fixed
- Phase B: Mini-VM always-skip trio moved to `tools/smokes/v2/profiles/archive/selfhost/`
- Phase C: opt-in Stage-B canaries moved to `tools/smokes/v2/profiles/archive/selfhost/`
- Phase D: `archive/selfhost` is the manual diagnostics home for selfhost-only legacy canaries

## Inventory (2026-03-07)

### Retired to archive profile

| Script | Evidence | Why it is not current |
| --- | --- | --- |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_mir_m2_eq_true_vm.sh` | moved out of `integration/selfhost`, still skip-only when run directly | Mini-VM compare polish canary, no longer discovered by active selfhost profile |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_mir_m3_branch_true_vm.sh` | moved out of `integration/selfhost`, still skip-only when run directly | Mini-VM branch canary, no longer discovered by active selfhost profile |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_mir_m3_jump_vm.sh` | moved out of `integration/selfhost`, still skip-only when run directly | Mini-VM jump canary, no longer discovered by active selfhost profile |

### Archived opt-in diagnostics

| Script | Evidence | Why it stays for now |
| --- | --- | --- |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_binop_vm.sh` | moved out of `integration/selfhost`; runs only when `SMOKES_ENABLE_STAGEB=1` | manual Stage-B binop diagnostic |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_if_vm.sh` | moved out of `integration/selfhost`; runs only when `SMOKES_ENABLE_STAGEB=1` | manual Stage-B if diagnostic |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_index_vm.sh` | moved out of `integration/selfhost`; runs only when `SMOKES_ENABLE_STAGEB=1` | manual Stage-B index diagnostic |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_oob_vm.sh` | moved out of `integration/selfhost`; runs only when `SMOKES_ENABLE_STAGEB_OOB=1` | manual OOB diagnostic canary |
| `tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_v1_compat_vm.sh` | moved out of `integration/selfhost`; runs only when `SMOKES_ENABLE_STAGEB_V1=1` | manual v1-compat diagnostic canary |

### Keep current entry

| Script | Evidence | Why it stays |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` | current selfhost subset gate | active daily/phase gate |
| `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` | current runtime-route smoke | active selfhost route contract |
| `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_identity_compat_route_guard_vm.sh` | pinned in active selfhost/runtime docs | active compat-route contract |

## Next retire phase

1. Keep `tools/smokes/v2/profiles/archive/selfhost/` as the manual/home for retired Mini-VM and opt-in Stage-B selfhost canaries.
2. Verify `tools/smokes/v2/run.sh --profile integration --filter 'selfhost_(mir_m[23]|stageb_)'` no longer discovers archived scripts.
3. Keep only active selfhost gate entrypoints under `profiles/integration/selfhost/`.
