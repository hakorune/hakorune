# Phase 125 P2-P5 Implementation Feedback

## 完了状況

### P0: docs-only ✅
- README.md 作成
- 方針固定（reads-only inputs を env に追加）

### P1: StepTreeContract に read_sources を追加しない（方針） ✅
- StepTree は "何を読むか（reads）" だけ
- "それがどこから来るか" は builder 側で解決（ScopeManager の SSOT）
- 実装変更なし、方針のみ確認

### P2: Normalized builder の env を 2 レーン化 ✅
- EnvLayout 導入（writes + inputs）
- from_contract: reads ∩ available_inputs で inputs を決定
- env_map: writes は ValueId 生成、inputs は参照
- 実装: `src/mir/control_tree/normalized_shadow/builder.rs` (186行追加)

### P3: dev-only 配線：available_inputs を渡す ⚠️ **未実装**
- 配線点: routing.rs / lowering.rs
- SSOT: function params + CapturedEnv (pinned/captured)
- 現状: available_inputs は空の BTreeMap（stub）

### P4: Return(Variable) の解決を拡張 ✅
- lower_return_value: env (writes + inputs) から解決
- Fail-Fast with hint: env に無い変数は構造化エラー
- Error tags: [phase125/return/var_not_in_env]
- Graceful degradation: Phase 125 errors → Ok(None)

### P5: fixture + smoke（VM） ✅
- fixture: `apps/tests/phase125_if_only_return_readonly_input_min.hako`
- smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase125_if_only_return_input_vm.sh`
- 結果: PASS (exit code 7)
- Note: 構造のみ（P3 実装後に完全動作）

### P6: docs 完了 ✅
- 10-Now.md 更新
- Phase 125 P2-P5 完了記録

## テスト結果

### Unit Tests
- 全 18 tests PASS (normalized_shadow module)
- 全 1160 tests PASS (lib全体)
- 新規テスト:
  - `test_return_variable_from_inputs_stub`: PASS
  - `test_return_variable_out_of_scope`: PASS (Phase 125 対応更新)

### Integration Smoke
- Phase 125: PASS (phase125_if_only_return_input_vm.sh)
- Regression: 全 PASS
  - phase124_if_only_return_var_vm: PASS
  - phase123_if_only_normalized_semantics_vm: PASS
  - phase121_shadow_if_only_vm: PASS (3/3)
  - phase118_loop_nested_if_merge_vm: PASS

## 箱化モジュール化の観点でのフィードバック

### 良い点

1. **単一責任の原則**
   - EnvLayout: env レイアウト決定のみ
   - from_contract: 入力から決定的に導出（idempotent）
   - lower_return_value: return 値の lowering のみ

2. **SSOT 化**
   - EnvLayout.from_contract: reads ∩ available_inputs が SSOT
   - AST から推測しない（Phase 100 の pinned と混同しない）
   - BTreeMap で決定的順序保証

3. **Fail-Fast 原則**
   - env に無い変数は即座にエラー
   - 構造化エラー tags: [phase125/return/var_not_in_env]
   - Hint 必須（"Pass as param, add to pinned capture, or define before if"）

4. **段階的投入**
   - P2-P5: 構造のみ実装（available_inputs は空）
   - P3: 配線（別 Phase として分離可能）
   - 既定挙動不変（dev-only）

### 改善提案

1. **P3 配線の明確化**
   - 現状: available_inputs は空 BTreeMap（stub）
   - 提案: P3 で明示的に配線点を文書化
   - SSOT: function params + CapturedEnv (pinned/captured)
   - Phase 121/122 の配線と同じ場所

2. **EnvLayout の責務拡大の懸念**
   - 現状: writes + inputs の決定のみ
   - 将来: outputs, side_effects なども追加される可能性
   - 提案: EnvLayout を "環境レイアウト決定" の SSOT として明確化
   - 別の Box（EnvLayoutBuilder など）への分離も検討

3. **Error hint の改善可能性**
   - 現状: 固定文字列 hint
   - 提案: 文脈に応じた hint（例: "x is in reads but not in function params"）
   - Phase 109 の error_tags hints SSOT に統合

4. **fixture の完全性**
   - 現状: P3 未実装のため、fixture は構造のみ
   - 提案: P3 完了後、"真の reads-only input" fixture を追加
   - 例: `local x=7; if flag==0 { /* no writes */ } return x`

## レガシー発見

なし（Phase 125 は新規機能のため）

## 次のステップ（Phase 125 P3 以降）

### P3: available_inputs wiring
- 配線点: `src/mir/builder/control_flow/joinir/routing.rs` または
  `src/mir/builder/calls/lowering.rs`
- SSOT: ScopeManager / CapturedEnv / function params
- inputs が実際に env に載るようになる

### 将来の拡張
- 複雑な条件式（BinOp, MethodCall など）の reads 抽出
- loop との統合（loop 内の reads を継続的に追跡）
- if-else merge で reads-only 変数の扱い

## 総評

Phase 125 P2-P5 は、箱化モジュール化の観点で良好な実装。

- **単一責任**: EnvLayout / lower_return_value
- **SSOT**: reads ∩ available_inputs
- **Fail-Fast**: 構造化エラー + hint
- **段階的投入**: 構造のみ実装（P3 で完成）

P3 の配線が完了すれば、Phase 125 の完全な機能が動作する見込み。
