# 293x-718 PURE-FIRST-GUARD-RELEASE-BIN-001

Status: landed
Date: 2026-05-18

## Decision

Use the release `hakorune` binary by default for pure-first guard VM runs and
MIR emit.

Debug `hakorune` remains available for diagnostics:

```bash
PURE_FIRST_VM_BIN=debug ...
PURE_FIRST_MIR_EMIT_BIN=debug ...
```

## Context

MIMAP-194A exposed a validation latency problem. The
`segment-map-local-free-reuse-ledger-bridge` proof app is large enough that
debug `hakorune` dominates both VM execution and MIR emit.

Measured on the MIMAP-192A proof app:

```text
debug VM:
  elapsed=114.05s

release VM:
  elapsed=7.20s

release direct MIR emit:
  elapsed=7.31s

MIMAP-192A L2 before VM helper switch:
  elapsed=119.84s
  selfhost.emit_mir elapsed_ms=8653
  remaining time dominated by debug VM

MIMAP-192A L2 after release VM + release MIR emit:
  elapsed=16.10s
  selfhost.emit_mir elapsed_ms=8671

MIMAP-194A closeout after release VM + release MIR emit:
  elapsed=35.70s
```

Before this sidecar, `pure_first_guard_emit_mir()` forced
`NYASH_BIN=target/debug/hakorune`, and current proof guards also ran VM through
`target/debug/hakorune`.

## Scope

- Add a shared `pure_first_guard_run_vm()` helper.
- Make `pure_first_guard_emit_mir()` use release `hakorune` by default.
- Keep debug override environment variables for diagnostic reproduction.
- Migrate the active MIMAP-192A guard and MIMAP-194A closeout guard to the VM
  helper.

## Stop Lines

- No MIR schema changes.
- No route/preflight vocabulary changes.
- No allocator behavior changes.
- No proof-app source changes.
- No EXE backend behavior changes.
- No hidden fallback: invalid binary mode fails fast.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Follow-Up

Older hand-written guards still contain direct `target/debug/hakorune --backend
vm` calls. They do not block MIMAP-195A, but future guard templates should use
`pure_first_guard_run_vm()` so new rows do not reintroduce debug-binary
validation latency.
