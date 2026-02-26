---
Status: Ready
Scope: code + gate（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
---

# Phase 29ao P16: ValueJoin exit の実使用（Pattern5 Infinite Early-Exit の after join を block_params 化）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。P10 の `Frag.block_params → emit_frag() の PHI` を、**実経路（Pattern5: infinite loop with early-exit）**で 1 箇所だけ使い始める。

## 目的

- Pattern5 の loop exit 合流（`after_bb` の PHI）を **`Frag.block_params + EdgeArgs(values)`** に置き換える。
- `CorePhiInfo` に残っている “exit join の PHI” をもう 1 件減らし、join の表現を EdgeCFG の SSOT に寄せる。
- 可能なら、回帰パック（phase29ae）に Pattern5 の VM 代表を追加して “実経路” をゲートで固定する。

## 非目的

- loop header の PHI を block_params 化する（emit_frag は「frag 内の incoming」しか観測しないため段階未到達）
- DomainPlan/Planner/Facts の拡張（P16 は Normalizer 内部表現の差し替えのみ）
- 新 env var 追加、恒常ログ追加、エラー文字列変更

## 対象（最小）

- `src/mir/builder/control_flow/plan/normalizer/pattern5_infinite_early_exit.rs`
  - 現状: `CorePhiInfo` 3 本（header 2 本 + after 1 本）
  - 変更: **after の 1 本だけ**を block_params へ移し、`CorePhiInfo` から除去する

## 変換方針（SSOT）

### before

- `after_bb` に `CorePhiInfo { dst: carrier_out, inputs: [(break_then_bb, carrier_current)] }`
- `header_bb → after_bb` の else_args は空（CFG上は存在、条件は `true` なので実行はされないが incoming には含まれる）
- `break_then_bb → after_bb` の wire args も空

### after

- `Frag.block_params[after_bb] = BlockParams { layout: ExprResultPlusCarriers, params: [carrier_out] }`
- `header_bb → after_bb`（unreachable 側）の else_args:
  - `layout=ExprResultPlusCarriers`
  - `values=[carrier_current]`
- `break_then_bb → after_bb`（break 側）の wire args:
  - `layout=ExprResultPlusCarriers`
  - `values=[carrier_current]`

注: unreachable な incoming でも `emit_frag()` の block_params 検証（strict/dev）では “incoming edge” として扱われるため、
args を揃えておく（Fail-Fast を安定させる）。

## 実装手順

### Step 1: after PHI 1 本を CorePhiInfo から削除

- `pattern5_infinite_early_exit.rs` の `phis` 生成で、`block: after_bb` の 1 件を削除する
- `header_bb` の 2 PHI はそのまま維持する（仕様不変）

### Step 2: after_bb に incoming する edges の args を “join 値” にする

- `header_bb` の loop branch（`cond=true`）で `else_target=after_bb` の `else_args` を差し替える
- `break_then_bb → after_bb` の `EdgeStub.args` を差し替える（ExitKind::Break の wire）

### Step 3: frag に block_params を追加

- `frag.block_params.insert(after_bb, BlockParams { layout: ExprResultPlusCarriers, params: vec![carrier_out] })`
- これにより P10 の `emit_frag()` が after_bb に 1 本の PHI を挿入する

## 回帰ゲート（任意だが推奨）

P16 の実使用を “JoinIR regression gate” で固定したい場合は、phase29ae pack に Pattern5 VM 代表を 1 本足す:

- 対象: `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern5_break_vm.sh`
- pack 追加案:
  - `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に
    `run_filter "pattern5_break_vm" "phase286_pattern5_break_vm" || exit 1` を追加
  - `docs/development/current/main/phases/phase-29ae/README.md` の Regression pack に追記

※ LLVM EXE は時間/環境依存が大きいので pack には入れない（VM のみ）。

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

任意:
- `./tools/smokes/v2/run.sh --profile integration --filter "phase286_pattern5_break_vm"`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p16): use block_params for pattern5 exit join"`

