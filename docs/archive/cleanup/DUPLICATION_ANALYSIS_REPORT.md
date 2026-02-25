Status: Historical

# Hakorune Rustコードベース 重複コード・共通化調査レポート

## 概要

このレポートは、Hakorune Rustコードベースにおける重複コード、共通化可能なパターンを特定し、DRY原則違反を解消するための調査結果をまとめたものです。

**調査日**: 2025-11-06
**調査対象**: `/home/tomoaki/git/hakorune-selfhost/src/`

---

## エグゼクティブサマリー

### 主要な発見

- **重複パターン総数**: 5つの主要カテゴリで約260インスタンス
- **最大削減見込み**: 推定500-800行（全体の約2-3%）
- **優先度最高**: Handler内のBox操作パターン統一（49+55+95=199インスタンス）

### クイックメトリクス

| カテゴリ | 重複数 | 削減見込み行数 | 優先度 |
|---------|--------|--------------|--------|
| Receiver変換パターン | 5箇所 | 20-30行 | 高 |
| Destination書き込み | 49箇所 | 150-200行 | 最高 |
| 引数検証 | 55箇所 | 100-150行 | 高 |
| エラー生成 | 95箇所 | 200-300行 | 最高 |
| PHI挿入 | 13箇所 | 50-100行 | 中 |

---

## 1. MIR Interpreter Handlers の重複パターン

### 1.1 Receiver変換パターン（5箇所の完全重複）

**場所**:
- `src/backend/mir_interpreter/handlers/boxes.rs:49-52`
- `src/backend/mir_interpreter/handlers/boxes_array.rs:12-15`
- `src/backend/mir_interpreter/handlers/boxes_map.rs:12-15`
- `src/backend/mir_interpreter/handlers/boxes_plugin.rs:12-15`
- `src/backend/mir_interpreter/handlers/boxes_instance.rs:14-17`

**重複コード**:
```rust
let recv_box: Box<dyn NyashBox> = match recv.clone() {
    VMValue::BoxRef(b) => b.share_box(),
    other => other.to_nyash_box(),
};
```

**共通化提案**:
```rust
// src/backend/mir_interpreter/utils/conversions.rs
impl MirInterpreter {
    fn convert_to_box(&self, recv: &VMValue) -> Box<dyn NyashBox> {
        match recv.clone() {
            VMValue::BoxRef(b) => b.share_box(),
            other => other.to_nyash_box(),
        }
    }
}
```

**期待効果**:
- 削減行数: 約20行（4行 × 5箇所）
- 保守性向上: 変換ロジックの一元管理
- テスト容易性: 1箇所のテストで全体をカバー

**リスク**: 低 - 単純な抽出で破壊的変更なし

---

### 1.2 Destination Register書き込みパターン（49箇所）

**典型例**:
```rust
// パターン1: 単純な書き込み（最頻出）
if let Some(d) = dst {
    this.regs.insert(d, VMValue::from_nyash_box(ret));
}

// パターン2: 直接値
if let Some(d) = dst {
    this.regs.insert(d, VMValue::Void);
}

// パターン3: 変換後
if let Some(d) = dst {
    this.regs.insert(d, VMValue::Integer(idx));
}
```

**場所**: 全handlerファイルに散在（特に`boxes_*.rs`に集中）

**共通化提案**:
```rust
// src/backend/mir_interpreter/utils/register_ops.rs
impl MirInterpreter {
    /// Write result to destination register if present
    fn write_result(&mut self, dst: Option<ValueId>, value: VMValue) {
        if let Some(d) = dst {
            self.regs.insert(d, value);
        }
    }

    /// Write NyashBox result to destination register
    fn write_box_result(&mut self, dst: Option<ValueId>, boxed: Box<dyn NyashBox>) {
        self.write_result(dst, VMValue::from_nyash_box(boxed));
    }

    /// Write void to destination register
    fn write_void(&mut self, dst: Option<ValueId>) {
        self.write_result(dst, VMValue::Void);
    }
}
```

**期待効果**:
- 削減行数: 約150-200行（3-4行 × 49箇所 → 1行呼び出し）
- 可読性向上: 意図が明確になる（`write_box_result`の方が意味が伝わる）
- エラー削減: 書き込み忘れを防止

**リスク**: 低 - 単純なヘルパー関数化

---

### 1.3 引数検証パターン（55箇所）

**典型例**:
```rust
// パターン1: 固定数検証
if args.len() != 1 {
    return Err(VMError::InvalidInstruction("push expects 1 arg".into()));
}

// パターン2: 範囲検証
if args.len() != 2 {
    return Err(VMError::InvalidInstruction("set expects 2 args".into()));
}

// パターン3: 複数許容
if args.len() < 1 || args.len() > 2 {
    return Err(VMError::InvalidInstruction("substring expects 1 or 2 args".into()));
}
```

**共通化提案**:
```rust
// src/backend/mir_interpreter/utils/validation.rs
impl MirInterpreter {
    /// Validate exact argument count
    fn validate_args_exact(&self, method: &str, args: &[ValueId], expected: usize) -> Result<(), VMError> {
        if args.len() != expected {
            return Err(VMError::InvalidInstruction(
                format!("{} expects {} arg(s), got {}", method, expected, args.len())
            ));
        }
        Ok(())
    }

    /// Validate argument count range
    fn validate_args_range(&self, method: &str, args: &[ValueId], min: usize, max: usize) -> Result<(), VMError> {
        let len = args.len();
        if len < min || len > max {
            return Err(VMError::InvalidInstruction(
                format!("{} expects {}-{} arg(s), got {}", method, min, max, len)
            ));
        }
        Ok(())
    }
}
```

**使用例**:
```rust
// Before
if args.len() != 1 {
    return Err(VMError::InvalidInstruction("push expects 1 arg".into()));
}

// After
self.validate_args_exact("push", args, 1)?;
```

**期待効果**:
- 削減行数: 約100-150行（2-3行 × 55箇所 → 1行呼び出し）
- エラーメッセージ統一: 一貫した形式
- 保守性向上: 検証ロジックの一元管理

**リスク**: 低 - 純粋な抽出、既存動作を変更しない

---

### 1.4 エラー生成パターン（95箇所）

**典型例**:
```rust
return Err(VMError::InvalidInstruction("some message".into()));
return Err(VMError::InvalidInstruction(format!("message with {}", var)));
```

**共通化提案**:
```rust
// src/backend/mir_interpreter/utils/errors.rs
impl MirInterpreter {
    fn invalid_instruction<S: Into<String>>(&self, msg: S) -> VMError {
        VMError::InvalidInstruction(msg.into())
    }

    fn method_not_found(&self, box_type: &str, method: &str) -> VMError {
        VMError::InvalidInstruction(
            format!("Method {} not found on {}", method, box_type)
        )
    }

    fn arg_count_error(&self, method: &str, expected: usize, got: usize) -> VMError {
        VMError::InvalidInstruction(
            format!("{} expects {} arg(s), got {}", method, expected, got)
        )
    }
}
```

**期待効果**:
- 削減行数: 約200-300行（複雑な`format!`マクロが1行呼び出しに）
- エラーメッセージ統一: 一貫した形式とタイポ防止
- i18n対応準備: 将来の多言語化が容易

**リスク**: 低 - エラー生成ロジックの抽出のみ

---

## 2. MIR Builder の重複パターン

### 2.1 PHI挿入パターン（13箇所）

**場所**: `src/mir/builder/` 配下の複数ファイル
- `exprs_peek.rs`: 2箇所
- `if_form.rs`: 2箇所
- `ops.rs`: 7箇所
- `phi.rs`: 2箇所

**典型例**:
```rust
if let (Some(func), Some(cur_bb)) = (self.current_function.as_mut(), self.current_block) {
    crate::mir::ssot::cf_common::insert_phi_at_head(func, cur_bb, result_val, phi_inputs);
} else {
    self.emit_instruction(MirInstruction::Phi { dst: result_val, inputs: phi_inputs })?;
}
```

**共通化提案**:
```rust
// src/mir/builder/utils/phi_helpers.rs
impl MirBuilder {
    /// Insert PHI instruction at current block head or emit legacy format
    fn insert_phi(&mut self, dst: ValueId, inputs: Vec<(BasicBlockId, ValueId)>) -> Result<(), String> {
        if let (Some(func), Some(cur_bb)) = (self.current_function.as_mut(), self.current_block) {
            crate::mir::ssot::cf_common::insert_phi_at_head(func, cur_bb, dst, inputs);
            Ok(())
        } else {
            self.emit_instruction(MirInstruction::Phi { dst, inputs })
        }
    }
}
```

**期待効果**:
- 削減行数: 約50-100行（4-5行 × 13箇所 → 1行呼び出し）
- 可読性向上: PHI挿入の意図が明確
- 保守性向上: PHI挿入ロジックの一元管理

**リスク**: 低 - 既存パターンの単純な抽出

---

## 3. 統合可能な似たモジュール

### 3.1 Box Handler群の統合機会

**現状**:
- `boxes_array.rs` (63行)
- `boxes_map.rs` (134行)
- `boxes_string.rs` (208行)
- `boxes_plugin.rs` (217行)
- `boxes_instance.rs` (153行)

**共通構造**:
1. Receiver変換（全て同一）
2. Box型ダウンキャスト
3. Method名によるdispatch
4. 引数検証
5. Destination書き込み

**統合提案**:
```rust
// src/backend/mir_interpreter/handlers/box_dispatch.rs
pub trait BoxMethodHandler {
    fn handle_method(&self, method: &str, args: &[ValueId], interp: &mut MirInterpreter)
        -> Result<Option<VMValue>, VMError>;
}

impl MirInterpreter {
    fn dispatch_box_method<T: NyashBox + BoxMethodHandler>(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, VMError> {
        let recv = self.reg_load(box_val)?;
        let recv_box = self.convert_to_box(&recv);

        if let Some(handler) = recv_box.as_any().downcast_ref::<T>() {
            if let Some(result) = handler.handle_method(method, args, self)? {
                self.write_result(dst, result);
            } else {
                self.write_void(dst);
            }
            return Ok(true);
        }
        Ok(false)
    }
}
```

**期待効果**:
- 削減行数: 約300-400行（共通部分の統一化）
- 新Box型の追加が容易: Traitを実装するだけ
- テスト容易性: Handler単位でのテストが可能

**リスク**: 中 - 大規模なリファクタリングが必要

---

## 4. 優先度付きアクションプラン

### Phase 1: 低リスク・高効果（即効性）

1. **Destination書き込みヘルパー** （優先度: 最高）
   - 実装時間: 2-3時間
   - 削減見込み: 150-200行
   - リスク: 低

2. **引数検証ヘルパー** （優先度: 高）
   - 実装時間: 2-3時間
   - 削減見込み: 100-150行
   - リスク: 低

3. **Receiver変換ヘルパー** （優先度: 高）
   - 実装時間: 1-2時間
   - 削減見込み: 20-30行
   - リスク: 低

### Phase 2: 中リスク・中効果（基盤整備）

4. **エラー生成ヘルパー** （優先度: 中）
   - 実装時間: 3-4時間
   - 削減見込み: 200-300行
   - リスク: 低

5. **PHI挿入ヘルパー** （優先度: 中）
   - 実装時間: 2-3時間
   - 削減見込み: 50-100行
   - リスク: 低

### Phase 3: 高リスク・高効果（抜本改革）

6. **Box Handler統合** （優先度: 低-中、将来課題）
   - 実装時間: 1-2週間
   - 削減見込み: 300-400行
   - リスク: 中-高
   - 備考: Phase 1-2完了後に検討

---

## 5. 実装ガイドライン

### 5.1 新規ユーティリティモジュール構成

```
src/backend/mir_interpreter/
├── handlers/
│   ├── ... (existing files)
│   └── mod.rs
└── utils/  (新規)
    ├── mod.rs
    ├── conversions.rs  (Phase 1: Receiver変換)
    ├── register_ops.rs (Phase 1: Destination書き込み)
    ├── validation.rs   (Phase 1: 引数検証)
    ├── errors.rs       (Phase 2: エラー生成)
    └── phi_helpers.rs  (Phase 2: PHI挿入, MIR Builder用)
```

### 5.2 段階的移行戦略

1. **ユーティリティ関数実装**: 既存コードに影響を与えずに新機能を追加
2. **並行期間**: 新旧両方のコードが共存
3. **1ファイルずつ移行**: テストを都度実行して確認
4. **完全移行後**: 旧パターンの削除

### 5.3 テスト戦略

```rust
// 各ユーティリティ関数に対応するテストを追加
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_args_exact() {
        // Exact match: OK
        // Too many: Error
        // Too few: Error
    }

    #[test]
    fn test_write_box_result() {
        // With destination: writes
        // Without destination: no-op
    }
}
```

---

## 6. 期待される全体効果

### 6.1 定量的効果

| 項目 | 現状 | Phase 1完了後 | Phase 2完了後 |
|------|------|--------------|--------------|
| Handler総行数 | 3,335行 | 2,965行 (-11%) | 2,715行 (-19%) |
| 重複パターン数 | 260箇所 | 109箇所 (-58%) | 46箇所 (-82%) |
| ユーティリティ関数数 | 0 | 3 | 5 |

### 6.2 定性的効果

- **保守性向上**: 変更が1箇所で完結
- **可読性向上**: 意図が明確な関数名
- **バグ削減**: 共通ロジックのバグ修正が全体に波及
- **新機能追加の容易性**: 統一されたパターン
- **テスト容易性**: ユーティリティ関数の単体テストで広範囲をカバー

---

## 7. リスク評価とミティゲーション

### 7.1 技術的リスク

| リスク | 確率 | 影響度 | 対策 |
|-------|------|--------|------|
| ユーティリティ関数のバグ | 低 | 高 | 単体テスト、段階的移行 |
| パフォーマンス劣化 | 極低 | 低 | ベンチマーク測定 |
| 既存動作の変更 | 低 | 高 | 回帰テスト、1ファイルずつ |

### 7.2 プロジェクト管理リスク

| リスク | 確率 | 影響度 | 対策 |
|-------|------|--------|------|
| 実装時間超過 | 中 | 中 | Phaseごとに区切る |
| レビュー負荷 | 中 | 低 | 小さなPRに分割 |

---

## 8. 追加調査項目（将来の改善機会）

### 8.1 MIR Builder内の重複

- `variable_map.insert`パターン（21箇所）
- `current_block`アクセスパターン
- エラーハンドリングの統一

### 8.2 型変換パターン

- `to_nyash_box()`（31箇所）
- `from_nyash_box()`（33箇所）
- 型変換ヘルパーの統一化可能性

### 8.3 Host Providers

- 現状3ファイルのみで大きな重複なし
- 将来の拡張時に再評価

---

## 9. 結論と推奨事項

### 主要な推奨事項

1. **Phase 1を優先実施**: 低リスク・高効果で即効性がある
   - Destination書き込みヘルパー
   - 引数検証ヘルパー
   - Receiver変換ヘルパー

2. **段階的移行**: 一度に全てを変更せず、1ファイルずつ確実に

3. **テストファースト**: ユーティリティ関数のテストを先に書く

4. **Phase 3は慎重に**: Box Handler統合は効果が大きいが、Phase 1-2の完了を待つ

### 期待される最終成果

- **コード削減**: 500-800行（約15-20%削減）
- **保守性向上**: 共通ロジックの一元管理
- **バグ削減**: 統一されたパターンによるエラー低減
- **開発速度向上**: 新機能追加時のボイラープレート削減

---

## 付録A: ファイルサイズ一覧

### Handler Files
```
   907 src/backend/mir_interpreter/handlers/calls.rs
   399 src/backend/mir_interpreter/handlers/boxes_object_fields.rs
   307 src/backend/mir_interpreter/handlers/boxes.rs
   298 src/backend/mir_interpreter/handlers/extern_provider.rs
   218 src/backend/mir_interpreter/handlers/externals.rs
   217 src/backend/mir_interpreter/handlers/boxes_plugin.rs
   208 src/backend/mir_interpreter/handlers/boxes_string.rs
   153 src/backend/mir_interpreter/handlers/boxes_instance.rs
   136 src/backend/mir_interpreter/handlers/arithmetic.rs
   134 src/backend/mir_interpreter/handlers/boxes_map.rs
   107 src/backend/mir_interpreter/handlers/mod.rs
    89 src/backend/mir_interpreter/handlers/call_resolution.rs
    63 src/backend/mir_interpreter/handlers/boxes_array.rs
    47 src/backend/mir_interpreter/handlers/memory.rs
    31 src/backend/mir_interpreter/handlers/misc.rs
    21 src/backend/mir_interpreter/handlers/boxes_void_guards.rs
 3,335 合計
```

### MIR Builder Files
```
 6,885 行（54ファイル）
```

---

**レポート作成者**: Claude Code Agent
**最終更新**: 2025-11-06
**次回レビュー**: Phase 1実装完了後
