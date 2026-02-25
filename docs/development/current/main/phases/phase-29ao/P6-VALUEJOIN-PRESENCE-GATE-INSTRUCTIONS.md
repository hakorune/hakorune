---
Status: Ready
Scope: code（未接続・仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/joinir-plan-frag-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P6: ValueJoin presence（direct skeleton の安全ゲート + 次の wire 足場）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変（未接続のまま）、ValueJoin が絡むケースを “direct skeleton” が飲み込まないように固定する

## 目的

- `CanonicalLoopFacts.value_join_needed` の存在は、将来 “expr result + carriers” の post-phi 表現に繋がる。
- 現時点の direct skeleton（Pattern1 subset）は value join を扱えないので、**誤って採用されない** ことを SSOT として固定する。
- ここでは “wire 実装” ではなく、P7（ValueJoin wire）へ進むための **安全ゲート**と境界テストを先に固める。

## 非目的

- expr result の EdgeArgs への実配線（`JumpArgsLayout::ExprResultPlusCarriers` を満たす args 構築）
- value join の抽出ロジック拡張（Facts 側の detect は別タスクに切る）
- 既存ルーティング/挙動/ログ/エラー文字列の変更

## 実装手順

### Step 1: direct skeleton を value_join_needed で gate する

対象:
- `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`

変更:
- `normalize_loop_skeleton_from_facts(...)` の冒頭で
  - `if facts.value_join_needed { return Ok(None); }`

方針:
- `Ok(None)` で fallback を維持（未接続なので観測差分なし）
- strict/dev の Fail-Fast は P7 以降（wire が入った段階）で入れる

### Step 2: unit test（value join の境界を固定）

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`（direct compose のテスト群）
  - もしくは `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`

追加テスト:
- `LoopFacts.features.value_join = Some(ValueJoinFacts { needed: true })` をセットして canonicalize し、
  - `try_compose_core_plan_direct(...)` が `None` になることを固定

注:
- value_join は現状 “未接続” の前提なので、テストは “合成側の安全ゲート” を固定する目的に限定する。

### Step 3: docs 更新

更新:
- `docs/development/current/main/phases/phase-29ao/README.md`（P6 完了の記録 + Next を P7 へ）
- `docs/development/current/main/10-Now.md`（Next 更新）
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p6): gate direct skeleton on valuejoin presence (unconnected)"`

## 次（P7 の入口）

P7 で ValueJoin の “最小の wire” を入れる：
- `JumpArgsLayout::ExprResultPlusCarriers` の SSOT を PlanFrag/CorePlan 側で表現できる場所を決める
- その上で局所 verify を追加（`post-phi-final-form-ssot.md` と整合）
