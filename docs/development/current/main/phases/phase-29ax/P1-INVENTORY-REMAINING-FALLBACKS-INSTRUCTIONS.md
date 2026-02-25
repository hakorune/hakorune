---
Status: SSOT
Scope: CorePlan purity Stage-2 fallback inventory
Related:
- docs/development/current/main/phases/phase-29ax/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# P1: 残 fallback 棚卸し＆境界固定

## 目的

- gate 実行で「落ちる/落ちない」を確定し、strict/dev の fallback を分類する。
- 1件につき fixture/smoke を最低1本で固定する。

## 手順

1. Gate 実行（SSOT）

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

2. 棚卸し（README に追記）

分類は以下のいずれかで短く記載:

- Ok(None)（対象外）
- Freeze（曖昧・矛盾・禁止形）
- subset 拡張（Facts/Planner/Composerで吸う）
- CorePlan 語彙追加（最小のみ）

3. fixture/smoke の固定

- 既存 fixture/smoke がある場合は流用し、gate で再現することを明記。
- 新規の場合は 1件につき最低1本追加。

4. Next 更新

- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## 検証

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
