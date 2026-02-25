# Phase 84-3: PhiTypeResolver 実装完了サマリー

## 🎉 成果

**Case D 削減実績**: 9件 → 4件（**56%削減達成！**）

### Phase 82-84 の累積削減

| Phase | 実装内容 | Case D 件数 | 削減率 | 累積削減率 |
|-------|---------|------------|--------|-----------|
| Phase 82 | フォールバック検出実装 | 12件 | - | - |
| Phase 84-2 | CopyTypePropagator 実装 | 9件 | 25% | 25% |
| **Phase 84-3** | **PhiTypeResolver 実装** | **4件** | **56%** | **67%** |

## Phase 84-3 で解決された 5件

### GroupA: Loop 制御フロー PHI（完全解決）

PhiTypeResolver の DFS 探索により、以下のパターンが解決されました:

1. ✅ `loop_with_continue_and_break_edge_copy_merge` - ValueId(56)
2. ✅ `nested_loop_with_multi_continue_break_edge_copy_merge` - ValueId(135)
3. ✅ `loop_inner_if_multilevel_edge_copy` - ValueId(74)
4. ✅ `loop_break_and_early_return_edge_copy` - ValueId(40)
5. ✅ `vm_exec_break_inside_if` - ValueId(27)

**解決メカニズム**:
```
Copy → PHI → Copy → PHI → ... → base 型定義
 ^                                    ^
 |                                    |
 DFS 探索                       value_types から型取得
```

**技術的ポイント**:
- 循環検出（visited セット）により無限ループ防止
- 探索上限（max_visits）によるタイムアウト防止
- base_types 収集と型一致性検証による安全な型推論

## 残り 4件の分類

| # | テスト名 | ValueId | パターン分類 |
|---|---------|---------|------------|
| 1 | `test_lowering_await_expression` | ValueId(2) | GroupC: await 特殊構文 |
| 2 | `mir_lowering_of_qmark_propagate` | ValueId(7) | **GroupD: QMark 特殊構文（新規）** |
| 3 | `mir_stage1_cli_emit_program_min_compiles_and_verifies` | ValueId(7) | GroupB: Stage1Cli 複雑型推論 |
| 4 | `mir_stage1_cli_emit_program_min_exec_hits_type_error` | ValueId(7) | GroupB: Stage1Cli 複雑型推論 |

### 残存理由の統一

**全て同一問題**: 「base 定義（BoxCall/Await）の戻り値型が value_types に未登録」

PhiTypeResolver の設計上、base 定義の型が未登録の場合は None を返す（正しい動作）。
これは PhiTypeResolver の責務外であり、**BoxCall/Await 命令の lowering 時に型情報を登録すべき**。

## Phase 84-4 への推奨

### 優先度1: BoxCall 型情報登録（3件解決）

**対象**:
- GroupB（2件）: Stage1Cli テスト
- GroupD（1件）: QMark テスト（新規出現）

**実装箇所**: `src/mir/builder/builder_calls.rs`

**実装内容**:
```rust
pub fn emit_box_call(
    &mut self,
    box_val: ValueId,
    method: &str,
    args: Vec<ValueId>,
) -> Result<ValueId, String> {
    let dst = self.next_value_id();

    // 既存の BoxCall 命令生成
    self.emit_instruction(MirInstruction::BoxCall { ... })?;

    // **新機能**: メソッド戻り値型を推論して登録
    if let Some(ret_ty) = self.infer_boxcall_return_type(box_val, method, &args) {
        self.value_types.insert(dst, ret_ty);
    }

    Ok(dst)
}
```

**期待効果**: 4件 → 1件（await のみ残存）

### 優先度2: Await 型情報特殊処理（1件解決）

**対象**: GroupC（1件）- await 特殊構文

**実装箇所**: `src/mir/builder/stmts.rs`

**実装内容**:
```rust
pub(super) fn build_await_expression(
    &mut self,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let future_value = self.build_expression(expression)?;
    self.emit_instruction(MirInstruction::Safepoint)?;

    let result_id = self.next_value_id();

    // **新機能**: Future の型から戻り値型を推論（暫定: Unknown）
    self.value_types.insert(result_id, MirType::Unknown);

    self.emit_instruction(MirInstruction::Await { ... })?;
    self.emit_instruction(MirInstruction::Safepoint)?;
    Ok(result_id)
}
```

**期待効果**: 1件 → 0件（全件解決）

### 優先度3: dev フォールバック（即座のアンブロック）

**実装箇所**: `src/mir/builder/lifecycle.rs`

**環境変数**: `NYASH_PHI_DEV_FALLBACK=1`

**用途**: 開発環境での即座のアンブロック（production 環境は依然として厳格）

## Phase 84-5 への準備

### 完了条件

```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0
```

### if_phi.rs 削除準備完了

Phase 84-4 完了後、以下が可能になります:

1. ✅ `src/mir/join_ir/lowering/if_phi.rs` 完全削除（約 300行）
2. ✅ `GenericTypeResolver` の if_phi 呼び出し削除
3. ✅ `lifecycle.rs` の Case D 処理を全て削除
4. ✅ レガシーフォールバック根絶

## 技術的洞察

### PhiTypeResolver の設計原則（箱理論）

1. **単一責務**: PHI + Copy グラフ追跡のみ
2. **探索限定**: Copy / Phi / base 定義 の 3 種類だけ
3. **安全条件**: 1 種類の型に収束する場合のみ Some を返す

### なぜ base 定義の型推論は PhiTypeResolver の責務外か

**設計上の分離**:
- **PhiTypeResolver**: 既に登録された型を「伝播」するレイヤー
- **emit_box_call/emit_await**: 型を「生成」するレイヤー

**箱理論の実現**:
```
[型生成レイヤー]
  ├─ emit_const()          → MirType::Integer 等を登録
  ├─ emit_box_call()       → メソッド戻り値型を登録（Phase 84-4-B で実装）
  └─ build_await_expression() → Future 戻り値型を登録（Phase 84-4-C で実装）

[型伝播レイヤー]
  ├─ CopyTypePropagator    → Copy 命令で型伝播
  └─ PhiTypeResolver       → PHI + Copy グラフで型伝播

[統合レイヤー]
  └─ GenericTypeResolver   → 全ての型推論箱を調整
```

### GroupD（QMark）が新規出現した理由

**仮説**: 以前は別の型推論経路（if_phi.rs のレガシーフォールバック）で偶然解決していた

**検証方法**:
```bash
# Phase 84-2 の状態に戻して確認
git checkout <phase-84-2-commit>
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib mir_lowering_of_qmark_propagate
# 結果: Case D で失敗するはず
```

**意義**: PhiTypeResolver 実装により、以前は隠蔽されていた型推論の欠陥が顕在化
→ 根本解決（BoxCall 型登録）の必要性が明確化

## まとめ

**Phase 84-3 の成果**:
- ✅ PhiTypeResolver 実装完了
- ✅ 9件 → 4件（56%削減）
- ✅ GroupA（Loop 制御フロー）完全解決
- ✅ 箱理論に基づく型推論システムの明確化

**Phase 84-4 への道筋**:
- 🎯 BoxCall 型情報登録（3件解決）
- 🎯 Await 型情報特殊処理（1件解決）
- 🎯 dev フォールバック（即座のアンブロック）

**Phase 84-5 の目標**:
- 🎯 if_phi.rs レガシー削除
- 🎯 型推論システムの完全箱化達成
- 🎯 Phase 82-84 完全達成宣言

**総削減見込み**:
- 12件（初期）→ 0件（Phase 84-5 完了時）
- **100%削減達成へ！**
Status: Historical
