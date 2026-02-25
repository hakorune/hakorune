---
Status: Active
Scope: docs-first（仕様不変）
Related:
- docs/development/current/main/phases/phase-29al/README.md
- docs/development/current/main/design/effect-classification-ssot.md
- src/mir/effect.rs
---

# Phase 29al P2: Effect classification SSOT（docs-first）

Date: 2025-12-29  
Status: Ready for execution  
Scope: effect 分類と最小の変形規約を SSOT 化する（仕様不変）

## Objective

- “どの変形が許されるか” を effect で固定し、JoinIR/PlanFrag/CorePlan/最適化/RC insertion が相互に壊さない境界を作る
- 実装者が `PURE` を都合で付け替える事故を防ぐ（SSOT + 参照導線）

## Non-goals

- 新しい最適化の追加
- 既存の effect 実装変更（コードは触らない）
- 新 env var 追加
- release 挙動/ログの変更

## Steps

### Step 1: effect SSOT を design に追加

Add:
- `docs/development/current/main/design/effect-classification-ssot.md`

Must include:
- primary categories（Pure/Mut/Io/Control）
- PlanFrag/CorePlan の effect 境界（`CoreEffectPlan::MethodCall.effects` 等）
- “許される変形” の最小法典（DCE/CSE/再順序）
- RC insertion と effect の扱い（削除禁止・順序保持）

### Step 2: 参照導線を追加

Update:
- `docs/development/current/main/design/planfrag-ssot-registry.md`
- 必要なら `docs/development/current/main/design/joinir-plan-frag-ssot.md` に関連リンク

### Step 3: Phase 入口と運用を更新

Update:
- `docs/development/current/main/phases/phase-29al/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Verification

- docs-only のため必須なし
- 任意: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "docs(phase29al): effect classification ssot"`

