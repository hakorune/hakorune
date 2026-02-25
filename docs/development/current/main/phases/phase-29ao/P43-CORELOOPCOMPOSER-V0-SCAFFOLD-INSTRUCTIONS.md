---
Status: Ready
Scope: CoreLoopComposer v0 の “実装足場” を追加し、既存の pattern 別 from_facts を置き換える入口を用意する（未接続・仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/P42-STAGE3-CORELOOPCOMPOSER-V0-DESIGN-INSTRUCTIONS.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreloop-exitmap-composition-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P43: CoreLoopComposer v0 scaffold (unconnected, behavior-preserving)

## 目的

P42 で固めた Stage-3 の設計（Skeleton+Feature 合成）に沿って、
`CoreLoopComposer v0` の実装足場（入口・型・責務境界）を先に用意する。

- **入口を 1 箇所に固定**し、以後の移行を “pattern 列挙” ではなく composer に閉じ込める
- 既定挙動は不変（未接続、または `Ok(None)` のみ返す）
- strict/dev の Fail-Fast/Freeze を増やさない（このステップでは出さない）

## 非目的

- CorePlan の語彙拡張
- Facts 抽出の拡張（P44+）
- router/planner の挙動変更

## 実装

### Step 1: composer に v0 モジュールを追加（未接続）

追加:

- `src/mir/builder/control_flow/plan/composer/coreloop_v0.rs`

公開（crate内）API（例）:

- `pub(in crate::mir::builder) fn try_compose_core_loop_v0(...) -> Result<Option<CorePlan>, String>`

要求:

- 入力は `CanonicalLoopFacts`（projection 済み）を使う（直接 `LoopFacts` を再解析しない）
- v0 の定義域外（skeleton != Loop / value_join_needed=true 等）は `Ok(None)`（未接続のため）
- v0 は “合成だけ” を守る（builder mutation/emit/merge は呼ばない）

### Step 2: `composer/mod.rs` に module 宣言だけ追加

- `pub(super) mod coreloop_v0;` を追加
- まだ呼び出さない（このステップでは未接続）

### Step 3: 最小ユニットテスト（None境界固定）

追加（例）:

- `skeleton_kind != Loop` なら `Ok(None)`
- `value_join_needed == true` なら `Ok(None)`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md` の Next を P43 に更新
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` の Next を P43 に揃える

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p43): add coreloop composer v0 scaffold"`
