---
Status: Done
Date: 2026-06-08
Scope: SELFHOST-SURFACE-000.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-639-MIM-PORT-FMEM-140-SELFHOST-SURFACE-PREFLIGHT-TASK-ORDER.md
  - docs/development/current/main/phases/phase-296x/296x-640-OUTBOX-0-NARROW-LOWERING-LANDING.md
  - tools/checks/k2_wide_phase296x_selfhost_surface_check_guard.sh
---

# 296x-641 SELFHOST-SURFACE-000 Surface Check Landing

## Purpose

Record the selfhost surface gate refresh that keeps Stage1 selfhost sources on
report/check-only evidence and confirms that the outbox surface is now a
closed narrow transfer marker instead of a missing MIR surface.

## Implementation

```text
selfhost surface check:
  keep pending / transport-only / deferred / prohibited semantics out of the
  runtime surface
  require explicit report/check evidence
  treat outbox as landed narrow lowering, not as an unimplemented freeze
```

## Report / Check

```text
selfhost_pending_surface_use_count=0
selfhost_transport_surface_semantic_use_count=0
selfhost_guarded_surface_use_count
selfhost_forbidden_surface_use_count=0
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_selfhost_surface_check_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The selfhost surface gate refresh is now a report/check-only surface, and
the next active lane returns to the mimalloc AtomicRemoteHead selection row.
```

## Closeout

```text
next: MIM-PORT-FMEM-031 AtomicRemoteHead CAS lowering producer selection
```
