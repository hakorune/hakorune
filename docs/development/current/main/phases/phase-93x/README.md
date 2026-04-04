# Phase 93x — archive-later engineering helper sweep

目的:
- `tools/selfhost/` 直下に残っていた legacy engineering helpers を `tools/archive/legacy-selfhost/engineering/` に退避する
- current/doc の参照を archive 側へ repoint して、live surface を薄く保つ
- `delete-ready` は出ていない前提で、archive-later だけを整理する

## Current

- **Current (LANDED)**: `archive-later engineering helper sweep`（handoff complete）
- **Phase 92x（LANDED）**: `selfhost proof/compat caller rerun`（`93x archive-later engineering helper sweep` に handoff）

## Scope

### In Scope

- `tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh`
- `tools/archive/legacy-selfhost/engineering/pre_promote_legacy_main_removal.sh`
- `tools/archive/legacy-selfhost/engineering/promote_tier2_case.sh`
- `tools/archive/legacy-selfhost/engineering/program_analyze.sh`
- `tools/archive/legacy-selfhost/engineering/program_analyze.hako`
- `tools/archive/legacy-selfhost/engineering/gen_v1_min.sh`
- `tools/archive/legacy-selfhost/README.md`
- current/design docs の live path repoint

### Out of Scope

- `keep-now` wrappers
- `vm-hako` reference lane
- `rust-vm` residual explicit keep
- source delete-ready removal

## Read Next

1. `docs/development/current/main/phases/phase-94/README.md`
2. `CURRENT_TASK.md`
