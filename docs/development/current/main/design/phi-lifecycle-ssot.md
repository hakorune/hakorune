---
Status: SSOT
Scope: MIR builder における PHI の “予約→定義→入力確定” ライフサイクルの SSOT。責務混線（Reserve-only PHI dst の露出）を構造で防ぐ。
Related:
- docs/development/current/main/design/compiler-pipeline-ssot.md
- docs/development/current/main/design/phi-input-strategy-ssot.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- src/mir/ssot/cf_common.rs
- src/mir/builder/control_flow/plan/lowerer/loop_lowering.rs
- src/mir/builder/control_flow/plan/features/loop_cond_bc.rs
- src/mir/builder/builder_emit.rs
---

# PHI Lifecycle (SSOT)

目的: PHI/SSA 周りの “時間的依存” をコメントや Step 番号で維持しない。PHI dst の「予約」と「定義」を混線させず、責務境界を綺麗にする。

## 背景（なぜ今やる？）

今回の selfhost ブロッカー（`call/arg_out_of_function_scope v=%36`）は、
PHI dst を `alloc_typed()` で先に予約して `variable_map` に公開したのに、PHI 命令の定義が後段（body emit 後）だったことが原因だった。

修正により挙動は直ったが、現状は “provisional PHI insert / PHI patch” が複数ファイルに散り、次の事故が再発しやすい:

- 「予約（ValueId確保）」と「定義（def_blocksに載る）」の混線
- 例外的な callsite が増えて SSOT から逸脱
- タグ/ガードがバラけ、診断が遅くなる

## 用語（この文書の SSOT）

- **Reserve**: `alloc_typed()` / `next_value_id()` で ValueId を確保する（定義ではない）。
- **Define**: MIR 上で “dst が定義された” 状態にする（PHI/Copy/Const 等の命令 emit を伴う）。
- **Expose**: その ValueId を `variable_map`（または “式評価が参照する bindings”）へ公開する。
- **Populate**: PHI inputs を確定する（pred→ValueId の組を揃える）。
- **Finalize/Seal**: CFG が確定し、以後 “入力の穴埋め” をしない状態にする。

## 変えてはいけない契約（MUST）

1. `variable_map` が指して良いのは **Define 済み ValueId のみ**（Reserve-only を公開しない）。
2. Reserve-only の PHI dst を body/effect で参照する必要がある場合は、**Populate 前でも Define だけ先に行う**（provisional PHI を許可）。
3. PHI の Insert/Update は “単一入口（SSOT）” に集約する（直書き禁止）。
4. 失敗は silent no-op で握りつぶさず、`Result` を伝播して fail-fast する（strict/dev(+planner_required) のポリシーに従う）。

## Provisional PHI Transaction Contract（SSOT）

provisional PHI（`inputs.len() == 0` の PHI 定義）は、時間的結合を避けるために必要な「Define-only」の表現です。
ただし、これが “途中エラーで残留” すると ghost ValueId / undefined operand を誘発し、診断距離が大きくなります。

このため、provisional PHI は **transaction（取引）**として扱い、以下の契約を SSOT とします。

### Scope

- provisional PHI は transaction 内でのみ許可する（transaction 外へ逃がさない）。
- release 既定の挙動は変えない（この契約は strict/dev(+planner_required) の fail-fast のみで可観測化する）。

### Invariants

- acceptance 変更なし（受理形は増やさない）
- AST rewrite 禁止
- transaction を抜ける前に、provisional PHI は必ず次のいずれかになる:
  - Populate 済み（inputs が埋まる）または
  - 破棄（abort/rollback; 必要なら後続で導入）

### Adapter / API Plan（推奨の単一入口）

- `phi_lifecycle` は低レベル API を提供し続ける:
  - `define_provisional_phi(...)`
  - `patch_phi_inputs(...)`
- その上に “transaction wrapper（SSOT）” を置き、呼び出し側の時間的結合を 1 箇所へ集約する:
  - `PhiTxn::begin(builder)`
  - `PhiTxn::define_provisional_phi(...) -> PhiToken`
  - `PhiTxn::patch_phi_inputs(token, ...)`
  - `PhiTxn::commit()`
  - `PhiTxn::abort_on_err(err) -> Err(freeze:contract)`（strict/dev(+planner_required) のみ）

### Failure Mode（Fail-Fast）

transaction が Err で中断し、provisional PHI が残留した場合は、strict/dev(+planner_required) で
`[freeze:contract][phi_lifecycle/provisional_left_unpatched] ...` として fail-fast する（panic 禁止）。

## 現状の PHI の居場所（JoinIR との関係）

- JoinIR は “計画（spec）” を返すのが役割で、PHI 命令そのものを emit する層ではない（`compiler-pipeline-ssot.md` 参照）。
- 今回の問題は **MIR builder（plan lowerer）内の PHI ライフサイクル**の話で、JoinIR が “なんとかする” 類の問題ではない。
  - JoinIR が正しくても、「PHI dst を先に公開した」瞬間に `def dominates use` が破れる。

## 実装状況（完了）

この SSOT は “設計だけ” ではなく、実装が完了している。

- SSOT入口: `src/mir/builder/emission/phi_lifecycle.rs`
  - `define_provisional_phi`（Define-only, inputs empty）
  - `patch_phi_inputs`（Populate/patch）
  - `define_phi_final` / `define_phi_final_fn`（single-step）
- LoopLowerer: provisional insert / patch を SSOT入口経由に移行済み（Step 1.5 / Step 4）。
- 直接呼び出しの抑制: `cf_common::insert_phi_at_head*` / `builder.update_phi_instruction` の直書きを減らし、SSOT入口へ寄せる。

## 推奨リファクタ（BoxShape / 挙動不変）

狙い: “Step 1.5 / Step 4” という時系列の知識を、LoopLowerer の中に分散させない。PHI 操作は 1 箇所から呼ぶ。

### Step 0（docs-only）

- 本書を追加（このファイル）。
- `docs/development/current/main/design/ai-handoff-and-debug-contract.md` に PHI patch の freeze tag を追記（後述）。
- `CURRENT_TASK.md` に「次の構造整備」として予定を追記。

### Step 1（SSOT入口の新設）

PHI のライフサイクル操作を “1つの入口” に集約する。

候補: `src/mir/builder/emission/phi_lifecycle.rs`（または `src/mir/builder/emission/phi.rs` に内包）

最低限の API（案）:

- `define_provisional_phi(builder, block, dst, tag) -> Result<(), String>`
  - `cf_common::insert_phi_at_head_spanned(..., inputs=[], ...)` を呼ぶ。
  - 目的は **Define** だけ（Populate はしない）。
- `patch_phi_inputs(builder, block, dst, inputs, tag) -> Result<(), String>`
  - `builder.update_phi_instruction(block, dst, inputs)` を呼ぶ。
  - “PHIが無い/ブロックが無い” は fail-fast（タグは SSOT 化する）。

許可/禁止（最重要）:

- 禁止: それ以外のファイルが `cf_common::insert_phi_at_head*` や `builder.update_phi_instruction` を直接呼ぶこと。
- 許可: PHI lifecycle SSOT 入口の内部だけ。

### Step 2（LoopLowerer を SSOT 入口へ寄せる）

対象: `src/mir/builder/control_flow/plan/lowerer/loop_lowering.rs`

- Step 1.5（provisional insert）を “SSOT入口呼び出し” に置換する。
- Step 4（patch）を “SSOT入口呼び出し” に置換する。
- エラーメッセージ（freeze tag）は SSOT の形に合わせ、`ai-handoff-and-debug-contract.md` と同期する。

狙い:

- lowerer が “命令探索して書き換える” 詳細を知らない
- lowerer が “cf_common の low-level API” を知らない

### Step 3（直書き呼び出しの封じ込め）

検索対象（例）:

- `insert_phi_at_head` / `insert_phi_at_head_spanned`
- `update_phi_instruction`

方針:

- 全 callsite を SSOT 入口へ寄せる（挙動不変）。
- “ここだけ特別” を増やさない（例外が必要なら先に SSOT に理由と撤去条件を書く）。

### Step 4（optional: LocalSSA との境界整理）

現状 `local_ssa::ensure()` は pin slot の “PHI redirect” のために `variable_map` を覗いている。
これは責務境界としては強く結合しているので、将来の BoxShape 候補:

- pin slot/slotified 由来の “最新 ValueId への解決” を pinning 側の SSOT に寄せる
- LocalSSA は “純粋に in-block 定義を保証する” だけに縮退（variable_map を直接見ない）

この Step は大きくなりやすいので、別コミット・別指示書に切る（BoxShape と BoxCount を混ぜない）。

## タグ/契約（debug contract SSOT との同期）

本リファクタで追加/統一すべき最低限:

- PHI patch fail-fast（loop lowerer 経由で露出）:
  - `[freeze:contract][lowerer/phi_patch_missing] block=<...> dst=<...> tag=<...> error=<...>`

※ タグ名を `phi/*` へ寄せるかは別 Decision。まずは “SSOTに載せる” を優先する。

## 受け入れ基準（挙動不変）

- `cargo build --release --bin hakorune`
- `SMOKES_ENABLE_SELFHOST=1 HAKO_JOINIR_DEBUG=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - PASS もしくは first_error が "別の既知ブロッカー" に寄っていること（PHI lifecycle 由来に戻らない）

---

## Block Existence Contract (PHI define の前提)

PHI を挿入する前に、対象 block が存在している必要があります。

**Decision**: auto-create しない
- PHI lifecycle SSOT 入口は block を自動作成しない
- 責務: 呼び出し元が `start_new_block(bb_id)` を呼んで block を作成する
- 順序: `start_new_block(bb_id)` → `define_provisional_phi()` / `define_phi_final()` / `patch_phi_inputs()`
- 違反検知: block が存在しない場合、`[freeze:contract][cf_common/phi_block_missing] ... op=... tag=...` で fail-fast
