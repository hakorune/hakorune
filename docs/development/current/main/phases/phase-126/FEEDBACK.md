# Phase 126: Feedback Report

## 完了状況

✅ **Phase 126 完全完了** (P0-P5 全ステップ PASS)

- P0: docs-only契約固定 ✅
- P1: AvailableInputsCollectorBox 実装 ✅
- P2: dev-only配線 ✅
- P3: Fixture強化 ✅
- P4: 回帰テスト ✅
- P5: docs DONE ✅

## 実装サマリ

### 新規箱: AvailableInputsCollectorBox

**場所**: `src/mir/control_tree/normalized_shadow/available_inputs_collector.rs` (143行)

**責務**: available_inputs を SSOT から収集

**SSOT ソース**:
1. Function params: `scope_ctx.function_param_names` + `variable_ctx.lookup()`
2. CapturedEnv: `captured_env.vars` (pinned/captured)

**設計の良い点**:
- ✅ Box-First: 単一責任（収集のみ）
- ✅ 決定的順序: BTreeMap で結果を返す
- ✅ 優先順位明確: params > CapturedEnv
- ✅ 公開API使用: `variable_ctx.lookup()` で private field 回避
- ✅ Unit tests: 5 tests (empty/params/captured/priority/deterministic)

**改善提案**:
- なし（現時点で十分に SSOT 化されている）

### 配線: lower_function_body → try_lower_if_only

**場所**: `src/mir/builder/calls/lowering.rs` + `builder.rs`

**変更**:
1. `AvailableInputsCollectorBox::collect(self, None)` 呼び出し追加
2. `try_lower_if_only()` シグネチャ拡張（available_inputs を受け取る）
3. `EnvLayout::from_contract()` が実際の available_inputs を使用

**設計の良い点**:
- ✅ dev-only: `joinir_dev_enabled()` でのみ動作
- ✅ 既定挙動不変: 本番経路に影響なし
- ✅ CapturedEnv: None で呼び出し（if-only patterns は CapturedEnv 不要）
- ✅ テスト互換性: 既存テストに空の BTreeMap を渡す

**改善提案**:
- なし（段階的投入として適切）

### Fixture 強化: phase125_if_only_return_readonly_input_min.hako

**変更前**: 単純な `return x`（構造のみ）
**変更後**: if-only pattern で reads-only 変数を return

**設計**:
```hako
local outer_x = 7
if (flag == 0) { /* outer_x を書かない */ }
return outer_x  // reads-only input から解決
```

**検証**:
- ✅ smoke test PASS (exit code 7)
- ✅ NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 で動作
- ✅ outer_x が inputs に入る（writes に入らない）

## 箱化モジュール化の観点

### 単一責任の原則

✅ **完全達成**

- AvailableInputsCollectorBox: 収集のみ（整形・検証はしない）
- EnvLayout::from_contract(): inputs 決定のみ（emit はしない）
- 各箱が1つの関心事のみ処理

### 分離と SSOT

✅ **完全達成**

**ソース SSOT**:
- Function params: `scope_ctx.function_param_names` + `variable_ctx`
- CapturedEnv: `loop_pattern_detection::function_scope_capture`

**禁止事項遵守**:
- ❌ AST からの推測 capture（SSOT 違反）→ 回避済み
- ❌ 直接 private field アクセス → 公開API使用

### テスト容易性

✅ **完全達成**

- Unit tests: 5 tests (AvailableInputsCollectorBox)
- Integration smoke: 1 test (phase125_if_only_return_input_vm.sh)
- Regression: Phase 121-125, 118 全 PASS

### Fail-Fast 原則

⚠️ **部分達成**（Phase 126 では strict mode でエラー検出なし）

**現状**:
- available_inputs に無い変数を reads で参照しても、今は単に inputs に含まれない
- strict mode での fail-fast は Phase 125 P2 の `freeze_with_hint()` に含まれる予定

**改善提案**:
- Phase 127 で "reads にあるのに available_inputs に無い" 変数を strict mode で検出
- エラーフォーマット: `freeze_with_hint("phase126/unknown_read/<var_name>", "...")`

## レガシー発見

### なし

- Phase 126 は新規機能追加のみ
- 既存コードの削除・変更なし

## 回帰テスト結果

### Unit Tests

```
cargo test --lib: 1165 passed; 0 failed
```

**新規追加**: 5 tests (AvailableInputsCollectorBox)

### Integration Smokes

✅ Phase 121: shadow_if_only_vm PASS
✅ Phase 122: normalized_emit_vm PASS
✅ Phase 123: normalized_semantics_vm PASS
✅ Phase 124: return_var_vm PASS
✅ Phase 125/126: return_input_vm PASS (exit code 7)
✅ Phase 118: loop_nested_if_merge_vm PASS

## 総評

### 成功点

1. **SSOT 達成**: available_inputs を function params + CapturedEnv から明確に収集
2. **Box-First**: AvailableInputsCollectorBox が単一責任を持つ
3. **dev-only**: 既定挙動を変えずに段階的投入
4. **テスト完備**: Unit + Integration で検証
5. **決定的順序**: BTreeMap で一貫性保証

### 次のステップ（Phase 127 候補）

1. **Strict fail-fast**: reads にあるが available_inputs に無い変数を検出
   - エラーフォーマット: `freeze_with_hint("phase126/unknown_read/<var_name>", "Hint: variable '<var_name>' is read but not available from outer scope")`

2. **CapturedEnv 統合**: loop patterns で CapturedEnv を available_inputs に含める
   - 現在: if-only patterns は `collect(self, None)` で CapturedEnv なし
   - 将来: Pattern2/3/4 で `collect(self, Some(&captured))` を使う

3. **Return(Variable from inputs) サポート拡張**:
   - Phase 125/126 で inputs 構造は完成
   - Phase 127 で実際の JoinIR → MIR lowering で inputs から値を取得

## コミット一覧

```
b195e8bad docs: Phase 126 DONE (available_inputs wired)
7ae424df3 test(joinir): Phase 126 assert readonly inputs actually wired
72f2c1f64 feat(joinir/dev): Phase 126 wire available_inputs into normalized builder
89c2915fa feat(control_tree): Phase 126 AvailableInputsCollectorBox
b7a16aacd docs: Phase 126 plan (wire available_inputs)
```

## フィードバック

### 箱化モジュール化の観点で気づいた点

**良い点**:
- AvailableInputsCollectorBox が MirBuilder を引数に取ることで、private fields のアクセス問題を回避
- 公開API (`variable_ctx.lookup()`) を使用することで、実装の詳細に依存しない

**改善提案**:
- 特になし（現時点で十分に SSOT 化されている）

### 設計の一貫性

**良い点**:
- Phase 125 の EnvLayout 設計が Phase 126 の available_inputs 収集を自然にサポート
- from_contract() の SSOT 原則が守られている

**改善提案**:
- 特になし（段階的投入として適切）

### レガシー発見

**なし**: Phase 126 は新規機能追加のみ
