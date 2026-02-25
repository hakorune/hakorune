---
Status: Ready
Scope: code（仕様不変・unconnected のまま）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P11: Normalizer が block_params を生成する最小ケース（ValueJoin “最初の生成”）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変（既定経路は触らない）。P10 で追加した `emit_frag(): block_params → PHI` の配線が壊れていないことを、**Normalizer が block_params を実際に生成する**最小ケースで固定する。

## 目的

- `Frag.block_params` を **Normalizer が生成する**最小のケースを追加する（テスト専用で OK）。
- その `Frag` を `emit_frag()` に通し、PHI が join block に挿入されることを unit test で固定する。
- strict/dev の verify（重複/0件/layout/len）も通ることを固定する。

## 非目的

- router/single_planner の経路変更（P11 は unconnected のまま）
- 既存 pattern（2/6/7/1883/263）の挙動/ログ変更
- ValueJoin を実アプリで使う（P12+）

## 実装方針（最小で確実）

“If2 の join（then/else → join）” を **test-only の Normalizer helper** で作る。

### 使う SSOT 要素

- `EdgeArgs.layout = JumpArgsLayout::ExprResultPlusCarriers`
- `EdgeArgs.values = [expr_result, carriers...]`
- `Frag.block_params[join] = BlockParams { layout, params: [expr_param, carrier_params...] }`
- `emit_frag()` が `block_params` を PHI として join block head に挿入する（P10）

## 手順

### Step 1: test-only の Normalizer helper を追加

新規ファイル（推奨）:
- `src/mir/builder/control_flow/plan/normalizer/value_join_demo_if2.rs`

中身（例）:
- `pub(super) fn build_demo_if2_valuejoin_frag(...) -> (Frag, Vec<BasicBlockId/ValueId>...)`
- 生成する CFG（概念）:
  - `header`（Branch）
  - `then_bb`（then_val を作る）
  - `else_bb`（else_val を作る）
  - `join_bb`（block_params を持つ + Return）

ポイント:
- then/else → join の wire は `ExprResultPlusCarriers` を使う（values は then_val / else_val の 1 個だけで良い）
- `join_bb` の Return は `CarriersOnly` で `values=[expr_param]` を返す（Return の SSOTは `emit_wires` に委譲）

### Step 2: unit test を追加（strict/dev の verify を有効化）

追加先（推奨）:
- `src/mir/builder/control_flow/plan/normalizer/mod.rs` の `#[cfg(test)]` 節、または新規 `value_join_demo_if2.rs` 内

テスト内容:
- `NYASH_JOINIR_DEV=1`（または `NYASH_JOINIR_STRICT=1`）をセットして strict/dev verify を有効化
- `MirFunction` を最小構成で作成し、`emit_frag(&mut func, &frag)` を呼ぶ
- `join_bb` の先頭に `MirInstruction::Phi { dst: expr_param, inputs: [(then_bb, then_val), (else_bb, else_val)] }` が入ることを検証
- `join_bb` が `Return { value: Some(expr_param) }` で終わることを検証

注意:
- env を触るので、テスト内で値を保存→復元（or remove）すること（既存テストの運用に合わせる）。

### Step 3: ドキュメント更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P11 完了/Next 更新）
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p11): normalizer demo generates block_params for valuejoin"`

## 次（P12）

P12 で初めて “実経路” に入る：
- 既存 normalizer の 1ケース（IfPhi or Loop after join のどちらか）に限定して block_params を生成し、fixture を 1 本緑にする（strict/dev のタグを SSOT 化）。
