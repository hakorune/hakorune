---
Status: Ready
Scope: code（未接続・仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P7: ValueJoin wire（EdgeArgs layout の語彙固定 + 局所 verify）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変（未接続）。ValueJoin を “post-phi 最終形” に繋げるための EdgeArgs 語彙を先に固定する

## 目的

- `JumpArgsLayout::ExprResultPlusCarriers`（expr_result + carriers）の語彙を PlanFrag/CorePlan 側で扱うための SSOT を作り、
  **局所 verify**（最低限の shape 検証）を追加する。
- P6 で direct skeleton は `value_join_needed` を採用しないように gate した。P7 では “wire を入れる前に”
  layout の不整合（offset=1 なのに値がない等）を、PlanVerifier で早期に検出できるようにする。

## 非目的

- value join の実配線（join 値の生成・PHI生成・boundary配線）
- router/既存挙動/ログ/エラー文字列の変更
- 新 env var 追加

## 実装手順

### Step 1: EdgeArgs builder（SSOT）を plan/normalizer 側に追加

対象:
- `src/mir/builder/control_flow/plan/normalizer/`

追加ファイル（推奨）:
- `src/mir/builder/control_flow/plan/normalizer/value_join_args.rs`

内容（例）:
- `pub(super) fn expr_result_plus_carriers_args(expr_result: ValueId, carriers: Vec<ValueId>) -> EdgeArgs`
  - layout = `JumpArgsLayout::ExprResultPlusCarriers`
  - values = `[expr_result] + carriers`

注意:
- 既存の `common::empty_args()` は `CarriersOnly` のまま維持（回帰しない）

### Step 2: PlanVerifier に EdgeArgs layout の局所検証を追加

対象:
- `src/mir/builder/control_flow/plan/verifier.rs`

追加する最小ルール（案: [V13]）:
- `JumpArgsLayout::ExprResultPlusCarriers` のとき、`EdgeArgs.values.len() >= 1` を必須
  - 理由: offset=1 の語彙なのに expr_result が無いのは contract violation
- `CarriersOnly` は 0 個も許可（既存の empty_args を壊さない）

検証対象:
- `CoreLoopPlan.frag.wires[*].args`
- `CoreLoopPlan.frag.exits[*][*].args`
- `CoreLoopPlan.frag.branches[*].then_args/else_args`

エラーメッセージ:
- 既存の verifier の prefix 形式（`[V??] ...`）に揃える

### Step 3: unit tests（OK/NG を固定）

対象:
- `src/mir/builder/control_flow/plan/verifier.rs`

追加テスト:
- `ExprResultPlusCarriers` で `values=[]` → verify が Err
- `ExprResultPlusCarriers` で `values=[ValueId(..)]` → verify が Ok

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p7): verify edgeargs layout for valuejoin (unconnected)"`

## 次（P8）

P8 で “最小の value join 実配線” を入れる（strict/dev で Fail-Fast）:
- `post-phi-final-form-ssot.md` に沿って、join 入力（expr_result + carriers）の対応表と順序を固定
- その上で direct skeleton の `value_join_needed` gate を “対応できる subset” だけ解除する
