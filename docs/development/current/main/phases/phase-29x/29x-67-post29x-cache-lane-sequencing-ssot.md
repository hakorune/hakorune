---
Status: SSOT
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x 完了後に実施する post-29x extension（X41-X46）の順序と受け入れ基準を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - docs/development/current/main/design/selfhost-stageb-json-streaming-design.md
---

# 29x-67 Post-29x Cache Lane Sequencing SSOT

## 0. Positioning

- X1-X40 は「de-rust runtime integration」の本線として完了済み。
- X41-X46 は post-29x extension（性能/運用改善）として扱う。
- 本 extension は言語仕様変更ではなく build orchestration の改善のみを対象にする。

## 1. Task order (fixed)

### X41: post-29x closeout sync（docs）

- 目的:
  - X1-X40 の closeout 状態を固定し、次レーン（cache build）への入口を一本化する。
- 受け入れ:
  - `README` / `29x-90` / `29x-91` / `CURRENT_TASK.md` に X41-X46 導線が整合している。
  - X1-X40 を壊さない（完了判定は維持）。

### X42: cache key determinism（CB-1）

- 目的:
  - `ModuleCompileKey` / `ObjectKey` / `LinkKey` の決定規則を実装境界で固定する。
- 受け入れ:
  - 同一入力で key が安定再現される。
  - 差分時は key が変化し、理由を観測可能。

### X43: L1 MIR cache（CB-2）

- 目的:
  - module 単位の MIR/ABI artifact 保存・再利用を導入する。
- 受け入れ:
  - 2回目実行で L1 hit が観測される。
  - 依存差分時に必要 module のみ L1 miss する。

### X44: L2 object cache（CB-3）

- 目的:
  - backend compile の object 生成を module 単位で再利用する。
- 受け入れ:
  - MIR 不変時は object 再生成を回避できる。
  - backend/ABI/target 変更時は miss して再生成される。

### X45: L3 link cache（CB-4）

- 目的:
  - entry + ordered object set 不変時の link 再利用を導入する。
- 受け入れ:
  - link input が同一なら link を再実行しない。
  - ABI 境界変更時は link cache が無効化される。

### X46: cache gate integration + done sync（CB-5）

- 目的:
  - daily/milestone で cache hit/miss を観測し、運用契約として固定する。
- 受け入れ:
  - check/gate が cache lane を含めて再現可能。
  - rollback 条件と fail-fast 契約が docs で同期される。

## 2. Dependency graph

- X40 -> X41 -> X42 -> X43 -> X44 -> X45 -> X46

## 3. Guard rails

- strict/dev では非canonical ABI 混入を fail-fast で止める。
- cache lane の導入で Rust compat lane の暗黙再活性化を許可しない。
- 一時回避フラグを追加する場合は default OFF + 撤去条件を docs に明記する。
