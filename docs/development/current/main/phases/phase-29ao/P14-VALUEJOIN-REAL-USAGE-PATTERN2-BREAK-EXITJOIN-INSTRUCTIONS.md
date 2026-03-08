---
Status: Ready
Scope: code（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P14: ValueJoin exit の実使用（LoopBreak route / historical label 2 の after join を block_params 化）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。P10 の `Frag.block_params → emit_frag() の PHI` を、**exit join（loop の出口合流）**で 1 箇所だけ使い始める。

## 目的

- LoopBreak route の loop exit 合流（`after_bb` の PHI）を **`Frag.block_params + EdgeArgs(values)`** に置き換える。
- `CorePhiInfo` の “exit 側の PHI” を減らし、join の表現を EdgeCFG の SSOT に寄せる。
- JoinIR 回帰 SSOT（phase29ae pack）を緑のまま維持する（LoopBreak lane は pack に入っている）。

## 非目的

- loop header の 2 PHI（loop_var/carrier）を block_params 化する
- break/continue の ExitMap 合成や cleanup の拡張（今回の対象は “after join 1 本” のみ）
- 新 env var 追加、恒常ログ追加、エラー文字列変更

## 対象（最小）

- `src/mir/builder/control_flow/plan/normalizer/loop_break.rs`
  - historical implementation file token: `pattern2_break.rs`
  - 現状: `CorePhiInfo` 3 本（header 2 本 + after 1 本）
  - 変更: **after の 1 本だけ**を block_params へ移し、`CorePhiInfo` から除去する

## 変換方針（SSOT）

### before

- `after_bb` に `CorePhiInfo { dst: carrier_out, inputs: [(header_bb, carrier_current), (break_then_bb, carrier_break)] }`
- `header_bb → after_bb` の else_args は空（`CarriersOnly + values=[]`）
- `break_then_bb → after_bb` の wire args も空

### after

- `Frag.block_params[after_bb] = BlockParams { layout: ExprResultPlusCarriers, params: [carrier_out] }`
- `header_bb → after_bb`（loop 終了）の else_args:
  - `layout=ExprResultPlusCarriers`
  - `values=[carrier_current]`
- `break_then_bb → after_bb` の Normal wire args:
  - `layout=ExprResultPlusCarriers`
  - `values=[carrier_break]`

注: `ExprResultPlusCarriers` は “先頭 1 slot + carriers” の SSOT 語彙だが、P14 では **exit join の単一値を運ぶ最小ラベル**として使う。

## 実装手順

### Step 1: after PHI 1 本を CorePhiInfo から削除

- current implementation file `loop_break.rs` の `phis` 生成で、`block: after_bb` の 1 件を削除する
- `header_bb` の 2 PHI はそのまま維持する（仕様不変）

### Step 2: header の loop exit edge args を “join 値” にする

- `header_bb` の loop condition branch（`cond_loop`）で、`else_target=after_bb` の `else_args` を差し替える

### Step 3: break_then → after の args を “join 値” にする

- `break_then_bb → after_bb` の `EdgeStub.args` を差し替える

### Step 4: frag に block_params を追加

- `frag.block_params.insert(after_bb, BlockParams { layout: ExprResultPlusCarriers, params: vec![carrier_out] })`
- これにより P10 の `emit_frag()` が after_bb に 1 本の PHI を挿入する

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

任意（LoopBreak lane だけ回す）:
- `bash tools/smokes/v2/profiles/integration/joinir/loop_break_release_adopt_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p14): use block params for loop break exit join"`
