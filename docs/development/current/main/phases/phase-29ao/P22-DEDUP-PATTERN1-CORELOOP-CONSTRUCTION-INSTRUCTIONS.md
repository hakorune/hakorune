---
Status: Ready
Scope: refactor+tests（意味論不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - src/mir/builder/control_flow/plan/normalizer/pattern1_simple_while.rs
  - src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs
  - src/mir/builder/control_flow/joinir/patterns/router.rs
---

# Phase 29ao P22: Pattern1 CoreLoop 構築の重複排除（DomainPlan/Fact Skeleton の SSOT 統一）

Date: 2025-12-30  
Status: Ready for execution  
Goal: Pattern1（SimpleWhile）の CoreLoop 構築コードが `DomainPlan` 経路と `Facts→CorePlan(skeleton)` 経路で二重化しているため、1 箇所に統一して divergence を防ぐ。

## 背景

- 現在 Pattern1 の “CorePlan::Loop を作る” 実装が2ヶ所に存在する:
  - `src/mir/builder/control_flow/plan/normalizer/pattern1_simple_while.rs`（DomainPlan→CorePlan）
  - `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`（Facts→CorePlan(skeleton), strict/dev shadow adopt で使用）
- 構造が同じなのに分岐があると、将来の修正が片方だけに入り、意味論/契約がズレる危険が高い。

P22では “CoreLoop を構築するロジック” を 1 箇所に集約し、両経路が同一実装を通るようにする。

## 非目的

- Pattern1 の対応範囲拡張（subset を広げない）
- strict/dev shadow adopt の適用範囲拡張（Pattern1以外へ広げない）
- exit/cleanup/value_join の合成拡張（P20 SSOT は docs のみ。実装拡張は P23+）

## 実装方針

### 1) CoreLoop 構築ヘルパーを新設（SSOT）

新規ファイル（推奨）:
- `src/mir/builder/control_flow/plan/normalizer/pattern1_coreloop_builder.rs`

中身（例）:
- `pub(super) fn build_pattern1_coreloop(builder: &mut MirBuilder, loop_var: &str, condition: &ASTNode, loop_increment: &ASTNode, ctx: &LoopPatternContext) -> Result<CoreLoopPlan, String>`

責務:
- blocks allocate（Standard5）
- ValueId allocate（loop_var_current/cond_loop/loop_var_next）
- compare/binop lowering（既存 helper を使用）
- header/step effects 作成
- header phi 作成
- frag（branches + wires）作成
- final_values（loop var mapping）作成

注意:
- `frag.exits` は Pattern1 subset では常に空のままにする（P20の ExitMap 合成は P23+ で扱う）。
- 既存の `pattern1_simple_while.rs` の debug trace は維持（必要なら wrapper 側で出す）。

### 2) DomainPlan 経路をヘルパー呼び出しに差し替え

対象:
- `src/mir/builder/control_flow/plan/normalizer/pattern1_simple_while.rs`

やること:
- `normalize_pattern1_simple_while()` の “Step 1〜13” 相当の CoreLoop 構築部分を `build_pattern1_coreloop()` に置き換える。
- 返り値 `CorePlan::Loop(loop_plan)` はそのまま。

### 3) Facts→CorePlan(skeleton) 経路をヘルパー呼び出しに差し替え

対象:
- `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`

やること:
- `normalize_loop_skeleton_from_facts()` の Pattern1 subset で CoreLoop を構築する部分を `build_pattern1_coreloop()` に置き換える。
- `value_join_needed` gate / skeleton gate / pattern1 facts gate は維持。
- `frag.exits` はヘルパーで空固定（ここで presence から作らない）。

### 4) テストで境界を固定（最小）

- `pattern1_simple_while.rs` のテスト（あるなら）/ もしくは新規ユニットテストで、ヘルパーが
  - branches=1, wires=2, phis=1 を生成する
  - `frag.exits.is_empty()` を維持する
  を固定。

※ block id/value id は比較しない（allocator の都合で不安定）。

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P22完了追記、NextをP23へ）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p22): dedup pattern1 coreloop construction"`

