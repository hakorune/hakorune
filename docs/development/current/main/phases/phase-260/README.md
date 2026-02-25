# Phase 260: Block-Parameterized CFG（edge-args）段階導入

Status: P2 完了 ✅（EdgeCFG SSOT確立）
Last updated: 2025-12-21

## 進捗（セーブポイント）

- P0（併存導入の芯）: 完了（読む側 SSOT を `out_edges()`/`edge_args_to()` に寄せた）
  - Commit: `4dfe3349b`
- P0.1（hardening）: 完了（legacy layout 無し禁止 + DCE/verify の参照点整理）
  - Commit: `1fe5be347`
- P0.2（instruction_rewriter モジュール化）: 完了
  - Commits: `cbed040a7`, `aa3fdf3c1`, `c2e8099ff`, `84cd653ae`, `875bfee1b`, `666de9d3e`
  - 抽出モジュール: exit_collection, block_allocator, merge_variable_handler, call_generator, handlers/（8ファイル）
- P0.3（joinir_block_converter モジュール化）: 完了 ✅
  - Commits: `e7f9adcfe`, `2c01a7335`
  - Phase 1: terminator_builder, block_finalizer
  - Phase 2: handlers/（call, jump, conditional_method_call, if_merge, nested_if_merge）
  - **合計抽出**: 15モジュール、約3941行、53単体テスト全てpass
  - **統合テスト**: 45/46 pass（既知のPattern2 LoopBodyLocal問題1件のみ）
- **P2（削除）: 完了 ✅**
  - **BasicBlock.jump_args 完全削除**、edge-args SSOT を terminator operand 側に一本化
  - フィールド変更: `jump_args` → `return_env`（Return専用metadata）
  - API簡略化: legacy helper 8個削除、terminator更新API保持（successors同期維持）
  - Verification簡素化: dual-source検証削除（cfg.rs 62行削除）
  - **テスト結果**: cargo test --lib 1368 PASS、quick smoke 45/46 PASS、phase258 tail call PASS
  - **検証**: `rg 'jump_args' src/` = 0件（コメント除く）

## P0.2/P0.3 モジュール分割の責務一覧

### Utilities（7モジュール）
- **block_allocator.rs**: ブロックID割り当て統一（allocate_one/two/three/n）
- **merge_variable_handler.rs**: マージコピー生成（emit_merge_copies, MergeBranch）
- **call_generator.rs**: Call命令生成統一（emit_call_pair, emit_call_pair_with_spans）
- **terminator_builder.rs**: Terminator生成統一（create_branch/jump/return_terminator, emit_branch_and_finalize）
- **block_finalizer.rs**: PHI保持ブロック確定（finalize_block, finalize_remaining_instructions）
- **exit_collection.rs**: Exit値収集（collect_exit_values, ExitCollectionResult）
- **convert.rs**: MirLikeInst変換（convert_mir_like_inst）

### Handlers（8モジュール）
- **handlers/ret.rs**: Return処理
- **handlers/method_call.rs**: MethodCall → BoxCall変換
- **handlers/field_access.rs**: FieldAccess → BoxCall（getter pattern）
- **handlers/new_box.rs**: NewBox命令処理
- **handlers/select.rs**: Select命令（直接、展開なし）
- **handlers/call.rs**: Call + tail call処理（Phase 131 P2: 安定ValueId、legacy jump_args）
- **handlers/jump.rs**: Jump → tail call変換（Phase 256 P1.9: continuation、Phase 246-EX: legacy jump_args）
- **handlers/conditional_method_call.rs**: ConditionalMethodCall → if/phi展開（単一変数）
- **handlers/if_merge.rs**: IfMerge → if/phi（複数変数）
- **handlers/nested_if_merge.rs**: NestedIfMerge → 多段分岐（N+3ブロック、最複雑）

### SSOT維持の確認項目（Phase 260の契約）

✅ **jump_args直参照ゼロ維持**: 全モジュールで`BasicBlock.jump_args`への直接参照なし（`out_edges()`/`edge_args_to()`経由のみ）
✅ **out_edges()/edge_args_to() SSOT維持**: 読む側APIが単一参照点に統一（Branchを含む複数edge前提）
✅ **terminator operand優先**: 新規コードはterminator operandを優先的に使用（legacy jump_argsは併存のみ）
✅ **PHI preservation**: block_finalizer.rsでPhase 189 FIX（PHI命令をブロック先頭保持）を維持
✅ **Phase metadata保持**: Phase 131 P2（安定ValueId）、Phase 246-EX（legacy jump_args）、Phase 256 P1.9（continuation名前解決）の契約を全て保持

### EdgeCFG SSOT確立（P2完了後）

**Edge-args の SSOT定義**:
- **Jump/Branch**: terminator operand `edge_args` が SSOT ✅
- **Return**: metadata `return_env` が SSOT（例外、terminator に operand フィールドなし）⚠️

**統一API**:
- **読み出し**: `out_edges()`, `edge_args_to()`, `edge_args_from_terminator()`
- **書き込み**: `set_jump_with_edge_args()`, `set_branch_with_edge_args()`, `set_return_env()`
  - 直代入禁止（successors 同期漏れ防止、Phase 260/261の核心成果）

**妥協点の明記**:
- Returnは「CFG successorを持たない」semantic上の特殊ケースのため、metadata許容
- 理想形（Option A）はReturn terminatorにenvフィールド追加だが、P2では工数削減のためOption B'採用

## 目的（P0）

JoinIR→MIR の暗黙 ABI（jump_args / carriers / expr_result slot / successors）を減らし、将来的な **block-parameterized CFG**（edge-args を第一級に持つCFG）へ収束するための「大工事パート」を開始する。

このフェーズは “一括置換” ではなく、**併存導入（Strangler）**で可逆に進める。

## 背景（Phase 256-259 で露出した型）

- `jump_args` が IR 外メタとして存在すると、DCE/verify/CFG 更新が「忘れると壊れる」になる
- spans が並行 Vec だと、最適化や変換で同期漏れが起きやすい
- continuation / entry / exit の識別と args 順序が散在すると、推測・補正が増殖する

North Star: `docs/development/current/main/design/join-explicit-cfg-construction.md`

## 方針（2段正規化）

- **Semantic Normalization（意味SSOT）**: terminator 語彙の固定（例: cond付きJumpを正規形から禁止しBranchへ）
- **Plumbing Normalization（配線SSOT）**: edge-args / CFG successor / spans を IR 構造に閉じ込め、写像に縮退

## スコープ（Phase 260）

### In scope

- MIR に「edge-args を持つ terminator 表現」を **併存導入**する（旧 `BasicBlock.jump_args` は残す）
- “読む側” を単一APIに寄せる（`Branch` を含むので “複数 edge” 前提で一本化する）
  - 例: `block.out_edges()` / `block.edge_args_to(target)`
- 互換期間は **一致検証を Fail-Fast**（両方ある場合は矛盾で即死）

### Out of scope（P0ではやらない）

- `BasicBlock.jump_args` の削除（削除は Phase 261+）
- spans の内部表現を `Vec<Spanned<_>>` に一気に切替（Phase 261+ で段階導入）
- JoinIR を削除する（builder DSL 降格は長期）

## 実装タスク（P0）

1. `MirTerminator`（または既存 terminator に edge-args を持てる variant）を追加（併存導入）
   - `Jump` だけでなく `Branch` を含むため、API は “複数 edge” を前提にする
2. bridge が `Jump/Branch` の edge-args を terminator operand としてもセット（旧jump_argsも併記してよい）
3. merge/ExitLine/DCE/verify/printer が参照する入口を一本化（読む側の Strangler）
   - 推奨: `block.out_edges()` / `block.edge_args_to(target)` のような API（`edge_args()` 単発は Branch で曖昧）
4. Fail-Fast 契約チェック（`--verify` 時に必須）
   - “両方ある場合は一致” を verify で保証
   - 追加: “terminator から計算した successors” と “block.successors キャッシュ” の一致も verify で保証（同期漏れを即死）

## 受け入れ基準（P0）

- `cargo build --release` が通る
- `./tools/smokes/v2/run.sh --profile quick` が少なくとも悪化しない（same first FAIL 以上）
- `--verify` の既存テストが壊れない（PHI/CFG検証が健全）
- legacy 依存の “推測” が増えていない（新規の env var 追加なし）
- `rg "jump_args"` を走らせて、移行コードと API 以外に参照が増えていない（読む側の寄せ漏れを検出）
- DCE 回帰が 1 本以上あり、「edge-args だけで使われる値」が消されないことを固定できている

## ロードマップ（P0→P3）

### P0（併存導入の芯）

- **単一参照API**を作る（Branch を含むので “複数 edge” 前提）
  - 例: `BasicBlock::out_edges()` / `BasicBlock::edge_args_to(target)`
- MIR terminator に edge-args を持てる表現を追加（旧 `jump_args` と **併存**）
  - 推奨: `EdgeArgs { layout: JumpArgsLayout, values: Vec<ValueId> }` のように “意味（layout）” も同梱する
- bridge が edge-args を terminator operand に必ず埋める（旧jump_argsも同内容でセットしてよい）
- merge/ExitLine/DCE/verify/printer は参照点を `out_edges()`/`edge_args_to(...)` に寄せる
- 両方ある場合の **一致検証を Fail-Fast**（`--verify` で必須）

### P1（切替）

- `jump_args` を読む経路を段階的に減らす（参照点は `out_edges()`/`edge_args_to(...)` のみ）
- terminator 更新の **API一本化**（successors/preds の同期漏れを構造で潰す）
  - 読む側だけでなく、書く側（terminator 設定/edge-args 設定）も API 経由に寄せる
- DCE/verify が terminator operand から自然に use/pred を追えることを固定する

### P2（削除）

- `BasicBlock.jump_args` を削除（併存チェックも撤去）
- `jump_args` 特例の DCE/verify コードを削除（terminator operand が SSOT）

### P3（spans 収束）

- `instructions` + `instruction_spans` の並行 Vec を段階導入で廃止
  - 先に編集APIを一本化 → 最終的に `Vec<Spanned<MirInstruction>>` へ

## メモ（設計SSOT）

- 相談パケット: `docs/development/current/main/investigations/phase-259-block-parameterized-cfg-consult.md`
- decisions: `docs/development/current/main/20-Decisions.md`
