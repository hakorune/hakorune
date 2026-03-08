---
Status: Ready
Scope: code（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P12: ValueJoin の最初の実使用（SplitScan step join を block_params 化 / historical label 7）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。P10/P11 で用意した `Frag.block_params → emit_frag() の PHI` を、**実経路（SplitScan route）**で 1 箇所だけ使い始める。

## 目的

- SplitScan route の `then/else → step_bb` の合流（これまで `CorePhiInfo` で表現していた PHI）を、
  **`Frag.block_params + EdgeArgs(values)`** に置き換える。
- これにより “join の合流は EdgeCFG の SSOT（block params + edge-args）で表す” を、実経路で 1 件固定できる。
- JoinIR 回帰 SSOT（phase29ae pack）を緑のまま維持する。

## 非目的

- ループ header の carrier PHI を block_params 化する（今回は step join のみ）
- DomainPlan/Planner の拡張（P12 は Normalizer の内部表現だけ差し替え）
- 新 env var 追加、恒常ログ追加

## 対象（最小）

- `src/mir/builder/control_flow/plan/normalizer/pattern_split_scan.rs`（historical implementation file token; current route family is SplitScan）
  - 現状: `CorePhiInfo` 4 本（header 2 本 + step 2 本）
  - 変更: **step の 2 本だけ**を block_params へ移し、`CorePhiInfo` から除去する

## 変換方針（SSOT）

### before

- `step_bb` に `CorePhiInfo { dst: i_next, inputs: [(then_bb, start_next_then), (else_bb, i_next_else)] }`
- `step_bb` に `CorePhiInfo { dst: start_next, inputs: [(then_bb, start_next_then), (else_bb, start_current)] }`

### after

- `Frag.block_params[step_bb] = BlockParams { layout: ExprResultPlusCarriers, params: [i_next, start_next] }`
- then の `Normal` exit edge args:
  - `layout=ExprResultPlusCarriers`
  - `values=[start_next_then, start_next_then]`（i_next, start_next の順）
- else の `Normal` exit edge args:
  - `layout=ExprResultPlusCarriers`
  - `values=[i_next_else, start_current]`

注: `ExprResultPlusCarriers` は “先頭 1 slot + carriers” の SSOT 語彙だが、P12 では **join の 2 値を運ぶ最小ラベル**として使う。
（layout/len が一致していれば、PHI は機械的に挿入できる。）

## 実装手順

### Step 1: step PHI 2 本を CorePhiInfo から削除

- historical implementation file token `pattern_split_scan.rs` の `phis` 生成で、`block: step_bb` の 2 件を削除する
- `header_bb` の 2 PHI はそのまま維持する（既定挙動不変）

### Step 2: compose::if_ に渡す then/else frag の Normal args を “join 値” にする

現状の `build_body_if_frag(.., empty_args)` は “空 args” を前提にしているため、P12 では split-scan 内で直接 frag を組む:

- `then_frag`: `ExitKind::Normal` の stub.args を `then_join_args` にする
- `else_frag`: `ExitKind::Normal` の stub.args を `else_join_args` にする
- `join_frag`（entry=step_bb）に `block_params` を載せる
- `compose::if_(body_bb, cond_match, then_frag, empty_args, else_frag, empty_args, join_frag)` を呼ぶ

### Step 3: join_frag に block_params を追加

- `join_frag.block_params.insert(step_bb, BlockParams { layout: ExprResultPlusCarriers, params: vec![i_next, start_next] })`
- これにより P10 の `emit_frag()` が step_bb に 2 本の PHI を挿入する

### Step 4: 最小の固定（テスト/回帰）

必須の確認:
- `split_scan_*` current semantic wrapper lane が PASS のまま（historical replay token family は regression pack / retirement SSOT 側で担保）

任意（もし不安なら）:
- `pattern_split_scan.rs` に unit test を 1 本追加し、`frag.block_params.contains_key(&step_bb)` と、
  `phis` に `block: step_bb` が存在しないことを固定する

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- 任意: `./tools/smokes/v2/run.sh --profile integration --filter "split_scan_regression_pack_vm"`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p12): use block params for split scan step join"`

## 次（P13）

P13 では “ValueJoin を expr_result として使う” 最小ケースへ進む:
- if/loop の **最終 join 値（Return へ返す値）** を `ExprResultPlusCarriers` の先頭 slot として扱い、fixture を 1 本追加して SSOT 固定する。
