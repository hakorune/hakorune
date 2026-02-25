---
Status: Ready
Scope: code（仕様不変・unconnected から “接続可能” へ）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/edgecfg-fragments.md
  - docs/development/current/main/design/join-explicit-cfg-construction.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P10: ValueJoin minimal wiring（block_params → MIR PHI の 1 箇所接続）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 既定挙動は不変。`Frag.block_params` が **非空のときだけ**有効化される “受け口→PHI 生成” を `emit_frag()` に追加し、ValueJoin の実配線に入る準備を完了する。

## 目的

- Phase 29ao P9 で導入した `Frag.block_params`（join 受け口）を、MIR 側の SSOT（`MirInstruction::Phi`）へ **1 箇所だけ**接続する。
- join 値（expr_result + carriers）を `EdgeArgs(layout+values)` で運び、target block の `block_params` が受け取る、という **post-phi SSOT** の “最小の勝ち筋” を確立する。
- 既定挙動不変を守るため、現行経路は `block_params` を生成しない（空）前提を維持し、実使用は P11+ へ回す。

## 非目的

- Normalizer/Composer が `block_params` を実際に生成して回帰を通す（P11+）
- `CorePhiInfo` の撤去（CorePlan 側の PHI は現状維持）
- join の「どのケースを ValueJoin にするか」の拡張（P10 は “受け口→PHI” の wiring のみ）
- 新 env var 追加、恒常ログ追加

## 仕様（P10 で固定する SSOT）

### 1) 受け口の定義

- `Frag.block_params[target] = BlockParams { layout, params }`
- `layout` は `EdgeArgs.layout` と一致し、`params.len()` は `EdgeArgs.values.len()` と一致する。

### 2) PHI の生成規則（局所・機械的）

`target` に入ってくる edge の集合（`frag.wires` と `frag.branches`）から、
各 param を “pred→value” の PHI 入力へ写像する。

- `params[i]` に対して:
  - `inputs = incoming_edges.map(|(pred, edge_args)| (pred, edge_args.values[i]))`
  - `MirInstruction::Phi { dst: params[i], inputs, type_hint: None }` を `target` の block head に挿入

### 3) Fail-Fast

- `Frag.block_params` が存在する target に対して incoming edge が 0 の場合は contract violation（strict/dev）
- 同一 `(pred,target)` が複数存在する場合は contract violation（strict/dev）
- `EdgeArgs.layout/len` が `BlockParams.layout/len` と不一致は contract violation（strict/dev）
- `block_params` が空のときは完全に no-op（既定挙動不変）

## 実装手順

### Step 1: emit_frag に “block_params → PHI” を追加（唯一の接続点）

対象:
- `src/mir/builder/control_flow/edgecfg/api/emit.rs`

やること:
- `emit_frag()` の `verify_frag_invariants_strict(frag)?;` の後、terminator emit の前に:
  - `emit_block_params_as_phis(function, frag)?;` を呼ぶ
- `emit_block_params_as_phis` は `frag.block_params.is_empty()` の場合 `Ok(())` で即 return（仕様不変）

PHI 挿入 SSOT:
- `crate::mir::ssot::cf_common::insert_phi_at_head_spanned(function, block, dst, inputs, span)`
  - span は `Span::unknown()` で良い（P10 は wiring 固定だけ）

incoming edge の収集（再解析禁止）:
- `frag.wires`: `stub.target == Some(target)` のもの（Return は target=None なので対象外）
- `frag.branches`: `then_target/else_target` を 각각収集（args は then_args/else_args）

### Step 2: strict/dev 検証の置き場所は verify.rs に寄せる

対象:
- `src/mir/builder/control_flow/edgecfg/api/verify.rs`

やること:
- P9 で追加した “ExprResultPlusCarriers の受け口検証” を再利用し、P10 の emit 側は「検証済みを前提」で小さく保つ
- emit 側でも最小の `debug_assert!` は OK（Fail-Fast）

### Step 3: unit test（1本で良い）

対象（推奨）:
- `src/mir/builder/control_flow/edgecfg/api/emit.rs`

テスト内容:
- `MirFunction` を最小構成で作り、block を 3 つ用意（pred1/pred2/target）
- `Frag` を手で作る:
  - `wires` に `pred1→target` と `pred2→target` を追加（args は `ExprResultPlusCarriers` で values=[vX, vC1, ...]）
  - `block_params[target] = BlockParams { layout: ExprResultPlusCarriers, params=[p0, p1, ...] }`
- `emit_frag(func, &frag)` を呼ぶ
- `target` block の先頭に `Phi(dst=p0)` が入り、inputs が (pred1,vX1)/(pred2,vX2) になっていることを検証

## 検証（必須）

- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p10): emit block params as phis (valuejoin wiring)"`

## 次（P11）

P11 で初めて “実使用” を入れる：
- Normalizer の 1 ケースだけ（If2 か Loop after join）で `block_params` を生成し、`ExprResultPlusCarriers` を通して回帰 fixture を 1 本緑にする。
