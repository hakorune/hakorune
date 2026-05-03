---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive closed phase29ck small-entry perf diagnostics probes from active tools/dev
Related:
  - docs/development/current/main/phases/phase-29ck/P10-SMALL-PERF-REENTRY-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P11-SMALL-ENTRY-STARTUP-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md
  - docs/development/current/main/phases/phase-29ck/P13-SMALL-ENTRY-RAW-NET-REFRESH.md
  - docs/development/current/main/phases/phase-29cv/README.md
  - tools/checks/phase29ck_small_entry_probe_surface_guard.sh
  - tools/archive/legacy-selfhost/engineering/README.md
---

# P356A: Phase29ck Small-Entry Probe Archive

## Intent

Move closed small-entry perf diagnostics out of active `tools/dev`.

P10-P13 already closed the small-entry lane as monitor-only. The probes now
serve as historical evidence for that decision, not as active daily keepers.

## Archived

- `tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_startup_probe.sh`
- `tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_gc_sections_experiment.sh`

Both scripts keep runnable repo-root resolution from the archive bucket.

## Boundary

Allowed:

- move closed small-entry diagnostics probes to archived engineering evidence
- update P10-P13 references to archived paths
- add a no-regrowth guard for the archived active paths

Not allowed:

- change perf methodology
- change current runtime proof smoke ownership
- reopen phase21_5/kilo perf implementation
- archive phase29ck compat or Stage1 dialect keepers

## Replacement

The live phase-level anchor remains:

- `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

Historical small-entry probes remain runnable from the archive bucket when that
old measurement path needs to be inspected.

## Guard

`tools/checks/phase29ck_small_entry_probe_surface_guard.sh` fails if the
archived small-entry probes return to active `tools/dev`, and confirms the
runtime-proof smoke anchor remains present.

## Acceptance

```bash
tools/checks/phase29ck_small_entry_probe_surface_guard.sh
bash -n tools/checks/phase29ck_small_entry_probe_surface_guard.sh tools/checks/dev_gate.sh \
  tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_startup_probe.sh \
  tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_gc_sections_experiment.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
