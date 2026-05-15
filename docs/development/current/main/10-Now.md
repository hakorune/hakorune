---
Status: SSOT
Date: 2026-05-15
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Self Current Task - Now (main)

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-293x mimalloc blueprint lane`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- task breakdown:
  `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`
- mimalloc blueprint SSOT:
  `docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md`
- mimalloc port purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
- current blocker token: `MIMAP-FACADE-CLEAN-001 facade result observer / reason-code SSOT cleanup`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- continue phase-293x after METADATA-CATALOG-001 MIR metadata catalog cleanup; next blocker is MIMAP-FACADE-CLEAN-001 facade result observer / reason-code SSOT cleanup; VM-LIM-001 is parked
- keep LoopRange on the Stage1 route; do not source-desugar range loops
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` inactive unless explicitly reopened

## Rules

- keep BoxShape and BoxCount separate
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not update current mirrors for every landed card
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
3. `docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md`
4. `docs/development/current/main/phases/phase-293x/README.md`
5. `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`
6. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
7. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
8. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```
