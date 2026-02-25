---
Status: Ready
Scope: code（仕様不変・strict/dev のみ Fail-Fast）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/design/join-explicit-cfg-construction.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P9: ValueJoin minimal wire（BlockParams 足場 + strict/dev verify）

Date: 2025-12-30  
Status: Draft for execution  
Scope: 既定挙動は不変。ValueJoin の実配線の前に「join の受け口（block params）」を SSOT として用意し、strict/dev で contract 違反を局所で落とせるようにする。

## 目的

- EdgeCFG の北極星（`edgecfg-fragments.md`）どおりに、**join 値の合流**を `block params + edge-args` で表現する足場を追加する。
- Phase 29ao P7/P8 で確立した `EdgeArgs(layout+values)` を前提に、**target 側が受け口を持たない / 長さがズレる**などの事故を strict/dev で Fail-Fast に固定する。
- まだ ValueJoin を「本当に使う」段階ではないので、既定挙動（release 既定・回帰 pack のログ/挙動）は不変のままにする。

## 非目的

- ValueJoin の本実配線（join block の PHI 生成や expr_result の読み出し）
- router/single_planner の経路変更（採用範囲の拡大）
- 新 env var の追加
- 恒常ログの追加（strict/dev のみタグは可）

## 提案する最小アーキテクチャ（P9 の成果物）

- **EdgeCFG API に join 受け口（BlockParams）を追加**する（未接続でも SSOT 型が存在する）。
- **compose は BlockParams を破壊しない**（合成は写像だけ、allocation は caller）。
- **strict/dev の verify で “edge-args と block params の整合” をチェック**する（release は不変）。

## 実装手順

### Step 1: EdgeCFG に BlockParams 型を追加

新規ファイル（推奨）:
- `src/mir/builder/control_flow/edgecfg/api/block_params.rs`

例（方向性）:
- `pub struct BlockParams { pub layout: JumpArgsLayout, pub params: Vec<ValueId> }`
  - `layout` は `EdgeArgs.layout` と同一 SSOT を参照する
  - `params.len()` は `EdgeArgs.values.len()` と一致する（expr_result + carriers の長さ）

### Step 2: Frag に block_params を追加（既定は空）

対象:
- `src/mir/builder/control_flow/edgecfg/api/frag.rs`

やること:
- `Frag { .. }` に `pub block_params: BTreeMap<BasicBlockId, BlockParams>` を追加
- `Frag::new` / `Frag::with_single_exit` の既定は `BTreeMap::new()`

### Step 3: compose が block_params を合成しても壊れないことを固定

対象:
- `src/mir/builder/control_flow/edgecfg/api/compose/seq.rs`
- `src/mir/builder/control_flow/edgecfg/api/compose/if_.rs`
- `src/mir/builder/control_flow/edgecfg/api/compose/cleanup.rs`

やること:
- `Frag` を返すときに `block_params` を merge（衝突は strict/dev で Fail-Fast）
  - SSOT 方針: `BasicBlockId` が同じなら同じ join のはずなので、二重定義は contract violation

### Step 4: strict/dev verify を追加（ValueJoin 事故の早期検知）

対象（候補）:
- `src/mir/builder/control_flow/edgecfg/api/verify.rs`

追加する検証（strict/dev のみ）:
- `wires` / `branches` に `EdgeArgs.layout == ExprResultPlusCarriers` が現れる場合:
  - `target` block に `block_params[target]` が存在すること
  - `block_params[target].layout == ExprResultPlusCarriers`
  - `block_params[target].params.len() == edge_args.values.len()`

注:
- Phase 29ao P7 で PlanVerifier 側に V13 があるので、P9 は EdgeCFG 側の “受け口” を固定する（責務分離）。

### Step 5: unit tests（Some/None 境界と Fail-Fast の固定）

推奨:
- `src/mir/builder/control_flow/edgecfg/api/verify.rs` に `verify_frag_invariants_strict()` に並ぶ strict 検証を追加するなら、その関数単体の unit tests をここに追加
- `compose` 側の unit tests で `block_params` が保持されることを確認（P8 と同じスタイル）

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p9): scaffold block params for valuejoin (strict/dev verify)"`

## 次（P10 の方向性）

P10 では、最小ケース（If2 か loop の after join のどちらか 1つ）に対して:
- `block_params` を normalizer が生成する
- join block で PHI（または block param lowering）を 1 箇所に固定
- `post-phi-final-form-ssot.md` の “局所 verify” と整合させる
