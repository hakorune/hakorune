---
Status: Ready
Scope: code（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P13: ValueJoin expr_result の実使用（IfPhiJoin merge join を block_params 化）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。P10 で用意した `Frag.block_params → emit_frag() の PHI` を、**実経路（IfPhiJoin、historical label 3）**で 1 箇所だけ使い始める。

## 目的

- IfPhiJoin の `then/else → merge_bb` の合流（これまで `CorePhiInfo` で表現していた PHI）を、
  **`Frag.block_params + EdgeArgs(values)`** に置き換える。
- これにより “expr_result 的な join 値（単一値）を EdgeCFG の SSOT（block params + edge-args）で表す” を、実経路で 1 件固定できる。

## 非目的

- loop header の 2 PHI（loop_var/carrier）を block_params 化する（今回は merge join の 1 本だけ）
- DomainPlan/Planner の拡張（P13 は Normalizer の内部表現だけ差し替え）
- 新 env var 追加、恒常ログ追加、エラー文字列変更

## 対象（最小）

- `src/mir/builder/control_flow/plan/facts/if_phi_join_facts.rs`
  - facts/current route naming の anchor
- current route anchor:
  - `src/mir/builder/control_flow/joinir/route_entry/router.rs`
- historical implementation file token:
  - `pattern3_if_phi.rs`
  - 当時の `CorePhiInfo` 3 本（header 2 本 + merge 1 本）のうち、**merge の 1 本だけ**を block_params へ移して `CorePhiInfo` から除去する

## 変換方針（SSOT）

### before

- `merge_bb` に `CorePhiInfo { dst: carrier_next, inputs: [(then_bb, carrier_then), (else_bb, carrier_else)] }`
- `then_bb → merge_bb` / `else_bb → merge_bb` の Normal exit args は空（`CarriersOnly + values=[]`）

### after

- `Frag.block_params[merge_bb] = BlockParams { layout: ExprResultPlusCarriers, params: [carrier_next] }`
- `then_bb → merge_bb` の Normal exit args:
  - `layout=ExprResultPlusCarriers`
  - `values=[carrier_then]`
- `else_bb → merge_bb` の Normal exit args:
  - `layout=ExprResultPlusCarriers`
  - `values=[carrier_else]`

注: `ExprResultPlusCarriers` は “先頭 1 slot + carriers” の SSOT 語彙だが、P13 では **join の単一値を運ぶ最小ラベル**として使う。
（layout/len が一致していれば、PHI は機械的に挿入できる。）

## 実装手順

### Step 1: merge PHI 1 本を CorePhiInfo から削除

- historical implementation file token `pattern3_if_phi.rs` の `phis` 生成で、`block: merge_bb` の 1 件を削除する
- `header_bb` の 2 PHI はそのまま維持する（仕様不変）

### Step 2: then/else の Normal args を “join 値” にする

- `then_bb → merge_bb` / `else_bb → merge_bb` の `EdgeStub.args` を `ExprResultPlusCarriers + values=[…]` に差し替える
- それ以外の branch/wire の args は既存どおり空のまま

### Step 3: frag に block_params を追加

- `frag.block_params.insert(merge_bb, BlockParams { layout: ExprResultPlusCarriers, params: vec![carrier_next] })`
- これにより P10 の `emit_frag()` が merge_bb に 1 本の PHI を挿入する

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

任意（IfPhiJoin の current semantic wrapper を強くしたい場合）:
- `./tools/smokes/v2/run.sh --profile integration --filter "if_phi_join_vm"`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p13): use block_params for if-phi join merge join"`
