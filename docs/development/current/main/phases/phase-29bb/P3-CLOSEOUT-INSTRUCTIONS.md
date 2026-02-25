---
Status: Ready
Scope: docs-only
---

# Phase 29bb P3: Closeout (docs-only)

## 目的

Phase 29bb の成果（CoreLoopComposer 単一入口の導入と shadow_adopt の集約）を SSOT に固定し、
次フェーズ選定へ渡す。

## やること

1) `docs/development/current/main/phases/phase-29bb/README.md`
   - Status を Complete に変更
   - P3 を ✅ で完了扱いにする
2) `docs/development/current/main/10-Now.md`
   - Current Focus を次フェーズ選定へ更新（TBD でOK）
3) `docs/development/current/main/30-Backlog.md`
   - Phase 29bb を Complete に移す
4) `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
   - Active/Next を次フェーズ選定に更新

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
