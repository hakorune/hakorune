---
Status: Ready
Scope: docs-only
---

# Phase 29ba P2: Closeout (docs-only)

## Goal

Phase 29ba を closeout し、FlowBox schema への収束が完了したことを SSOT として固定する。

## Acceptance

- `rg -n "\\[plan/fallback:" src/mir` が 0 件
- Gate が緑:
  - `./tools/smokes/v2/run.sh --profile quick`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- release 既定挙動・恒常ログは不変（FlowBox tags は strict/dev only）

## Steps

1. 進捗更新
   - `docs/development/current/main/phases/phase-29ba/README.md`
     - `Status: Complete`
     - Plan の P1 ✅ / P2 ✅ を反映し、Summary を短く追加

2. Now/Backlog を次フェーズへ
   - `docs/development/current/main/10-Now.md` の `Next:` を `TBD`（次フェーズ選定中）に寄せる
   - `docs/development/current/main/30-Backlog.md` の Active/Planned を最小更新
     - （次フェーズが決まり次第、Phase を差し替える）

3. 参照導線の最終確認（docs-only）
   - `docs/development/current/main/design/flowbox-observability-tags-ssot.md`
   - `docs/development/current/main/design/flowbox-fallback-observability-ssot.md`
   - `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md`

