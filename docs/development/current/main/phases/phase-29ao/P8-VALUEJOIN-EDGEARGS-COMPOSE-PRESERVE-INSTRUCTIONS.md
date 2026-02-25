---
Status: Ready
Scope: code（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/join-explicit-cfg-construction.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P8: ValueJoin wire の前提固定（compose が EdgeArgs を壊さないことをテストでSSOT化）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。ValueJoin の “expr_result + carriers” を Normal exit の EdgeArgs で運ぶために、compose が EdgeArgs を保存することを固定する。

## 目的

- ValueJoin を `JumpArgsLayout::ExprResultPlusCarriers` で表す前提で、EdgeCFG の合成則（`seq/if_/cleanup`）が
  **EdgeArgs(layout+values) を落とさずに上位へ伝播/内部配線へ移す**ことを unit test で固定する。
- これにより、次フェーズで Normal exit を “値を持つ出口（ValueJoin）” として使っても、compose が値を捨てないことが保証される。

## 非目的

- join 値の抽出・配線（実ワイヤリング）
- router/実行経路の変更
- 新 env var の追加

## 実装手順

### Step 1: compose::seq が EdgeArgs を保存することをテストで固定

対象:
- `src/mir/builder/control_flow/edgecfg/api/compose/seq.rs`

追加テスト（例）:
- `a.exits[Normal]` に `EdgeStub { args: EdgeArgs{layout=ExprResultPlusCarriers, values=[ValueId(10)]}}` を入れる
- `b.entry` へ内部配線された `wire` の `args.layout` と `args.values` が一致することを検証する

### Step 2: compose::if_ が then/else の EdgeArgs を保存することをテストで固定

対象:
- `src/mir/builder/control_flow/edgecfg/api/compose/if_.rs`

追加テスト（例）:
- then/else の `Frag` の Normal exit にそれぞれ異なる `EdgeArgs`（ExprResultPlusCarriers）を入れる
- `if_` の結果 `wires` に移った stubs の `args` が保持されることを検証する

### Step 3: compose::cleanup が Return の EdgeArgs を保存することをテストで固定

対象:
- `src/mir/builder/control_flow/edgecfg/api/compose/cleanup.rs`

追加テスト（例）:
- cleanup_frag の `Return` exit stub に `EdgeArgs{layout=ExprResultPlusCarriers, values=[ValueId(10)]}` を入れる
- `cleanup(..)` の結果 `wires` に同じ `args` で入ることを検証する

注: cleanup は P3 時点で Normal/Return のみを扱うので、この範囲で十分。

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p8): ssot compose preserves edgeargs for valuejoin"`

## 次（P9の入口）

P9 で “最小の ValueJoin 実配線” に入る：
- Normal exit の `EdgeArgs` を expr_result + carriers として扱う場所（CorePlan/Frag/merge）を 1 箇所に固定し、
  post-phi SSOT（pred→入力対応 + layout）と整合させる。
