---
Status: Complete
Date: 2026-05-12
Scope: dev gate profile ownership cleanup for phase-294x work
Related:
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/tools/check-scripts-index.md
---

# 294x-09e DEV-GATE-QUICK-PROFILE-SPLIT

## Decision

`tools/checks/dev_gate.sh quick` is the day-to-day slim gate. It must keep
surface drift, compile, ABI/decl drift, and narrow K2 first-row sentinels, but
it must not run the full allocator/mimalloc/provider proof ladder.

The full allocator proof ladder is owned by:

```bash
tools/checks/dev_gate.sh allocator-wide
```

`hotpath` owns the integration/perf smoke steps that were making quick feel
like a milestone profile.

## Boundaries

- `quick` keeps one provider inactive sentinel so hook/provider/activation
  hazards stay visible without running every historical row guard.
- `allocator-wide` runs `quick`, then the full
  `tools/checks/k2_wide_allocator_gate.sh` ladder and metal keep inventory.
- `hotpath` runs `quick`, then MapLookup/Chip8 integration smoke and the
  phase21.5 hotpath perf bundle.
- This row does not change language semantics, MIR exact numeric facts, VM
  reference behavior, hako_alloc behavior, or allocator-provider activation.

## Acceptance

```bash
bash tools/checks/allocator_provider_inactive_sentinel_guard.sh
bash tools/checks/dev_gate.sh --list
bash tools/checks/dev_gate.sh quick
```

Run this explicitly at allocator/mimalloc/provider closeout points:

```bash
bash tools/checks/dev_gate.sh allocator-wide
```
