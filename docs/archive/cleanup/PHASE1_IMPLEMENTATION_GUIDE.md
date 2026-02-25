Status: Historical

# Phase 1実装ガイド - 低リスク・高効果ヘルパー関数

## 概要

Phase 1では以下3つのヘルパー関数を実装し、約270-380行のコード削減を目指します。

1. Destination書き込みヘルパー（150-200行削減）
2. 引数検証ヘルパー（100-150行削減）
3. Receiver変換ヘルパー（20-30行削減）

**実装時間見込み**: 5-8時間
**リスク**: 低
**優先度**: 最高

---

## 実装手順

### Step 1: ユーティリティモジュール作成

```bash
# ディレクトリ構造作成
mkdir -p src/backend/mir_interpreter/utils
touch src/backend/mir_interpreter/utils/mod.rs
touch src/backend/mir_interpreter/utils/register_ops.rs
touch src/backend/mir_interpreter/utils/validation.rs
touch src/backend/mir_interpreter/utils/conversions.rs
```

---

### Step 2: register_ops.rs 実装

**ファイル**: `src/backend/mir_interpreter/utils/register_ops.rs`

```rust
use crate::backend::mir_interpreter::MirInterpreter;
use crate::box_trait::NyashBox;
use crate::mir::ValueId;
use crate::backend::mir_interpreter::VMValue;

impl MirInterpreter {
    /// Write a VMValue result to destination register if present.
    ///
    /// # Example
    /// ```rust
    /// self.write_result(dst, VMValue::Integer(42));
    /// ```
    pub(crate) fn write_result(&mut self, dst: Option<ValueId>, value: VMValue) {
        if let Some(d) = dst {
            self.regs.insert(d, value);
        }
    }

    /// Write a NyashBox result to destination register (converted to VMValue::BoxRef).
    ///
    /// # Example
    /// ```rust
    /// let result_box = string_box.trim();
    /// self.write_box_result(dst, result_box);
    /// ```
    pub(crate) fn write_box_result(&mut self, dst: Option<ValueId>, boxed: Box<dyn NyashBox>) {
        self.write_result(dst, VMValue::from_nyash_box(boxed));
    }

    /// Write VMValue::Void to destination register.
    ///
    /// # Example
    /// ```rust
    /// arr.push(item);
    /// self.write_void(dst);  // push returns void
    /// ```
    pub(crate) fn write_void(&mut self, dst: Option<ValueId>) {
        self.write_result(dst, VMValue::Void);
    }

    /// Write integer value to destination register.
    pub(crate) fn write_integer(&mut self, dst: Option<ValueId>, value: i64) {
        self.write_result(dst, VMValue::Integer(value));
    }

    /// Write boolean value to destination register.
    pub(crate) fn write_bool(&mut self, dst: Option<ValueId>, value: bool) {
        self.write_result(dst, VMValue::Bool(value));
    }

    /// Write string value to destination register.
    pub(crate) fn write_string(&mut self, dst: Option<ValueId>, value: String) {
        self.write_result(dst, VMValue::String(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_write_result_with_dst() {
        let mut interp = MirInterpreter {
            regs: HashMap::new(),
            // ... other fields
        };
        let dst = Some(ValueId(42));
        interp.write_result(dst, VMValue::Integer(100));
        assert_eq!(interp.regs.get(&ValueId(42)), Some(&VMValue::Integer(100)));
    }

    #[test]
    fn test_write_result_without_dst() {
        let mut interp = MirInterpreter {
            regs: HashMap::new(),
            // ... other fields
        };
        interp.write_result(None, VMValue::Integer(100));
        assert!(interp.regs.is_empty());
    }

    #[test]
    fn test_write_void() {
        let mut interp = MirInterpreter {
            regs: HashMap::new(),
            // ... other fields
        };
        let dst = Some(ValueId(10));
        interp.write_void(dst);
        assert_eq!(interp.regs.get(&ValueId(10)), Some(&VMValue::Void));
    }
}
```

---

### Step 3: validation.rs 実装

**ファイル**: `src/backend/mir_interpreter/utils/validation.rs`

```rust
use crate::backend::mir_interpreter::{MirInterpreter, VMError};
use crate::mir::ValueId;

impl MirInterpreter {
    /// Validate that the method call has exactly the expected number of arguments.
    ///
    /// # Example
    /// ```rust
    /// self.validate_args_exact("push", args, 1)?;
    /// let val = self.reg_load(args[0])?;
    /// ```
    ///
    /// # Errors
    /// Returns `VMError::InvalidInstruction` if argument count doesn't match.
    pub(crate) fn validate_args_exact(
        &self,
        method: &str,
        args: &[ValueId],
        expected: usize,
    ) -> Result<(), VMError> {
        let actual = args.len();
        if actual != expected {
            return Err(VMError::InvalidInstruction(format!(
                "{} expects exactly {} argument(s), but got {}",
                method, expected, actual
            )));
        }
        Ok(())
    }

    /// Validate that the method call has arguments within the expected range (inclusive).
    ///
    /// # Example
    /// ```rust
    /// // substring accepts 1 or 2 arguments
    /// self.validate_args_range("substring", args, 1, 2)?;
    /// ```
    ///
    /// # Errors
    /// Returns `VMError::InvalidInstruction` if argument count is out of range.
    pub(crate) fn validate_args_range(
        &self,
        method: &str,
        args: &[ValueId],
        min: usize,
        max: usize,
    ) -> Result<(), VMError> {
        let actual = args.len();
        if actual < min || actual > max {
            return Err(VMError::InvalidInstruction(format!(
                "{} expects {}-{} argument(s), but got {}",
                method, min, max, actual
            )));
        }
        Ok(())
    }

    /// Validate that the method call has at least the minimum number of arguments.
    ///
    /// # Example
    /// ```rust
    /// self.validate_args_min("varargs_method", args, 2)?;
    /// ```
    pub(crate) fn validate_args_min(
        &self,
        method: &str,
        args: &[ValueId],
        min: usize,
    ) -> Result<(), VMError> {
        let actual = args.len();
        if actual < min {
            return Err(VMError::InvalidInstruction(format!(
                "{} expects at least {} argument(s), but got {}",
                method, min, actual
            )));
        }
        Ok(())
    }

    /// Validate that the method call has no arguments.
    ///
    /// # Example
    /// ```rust
    /// self.validate_args_empty("pop", args)?;
    /// ```
    pub(crate) fn validate_args_empty(&self, method: &str, args: &[ValueId]) -> Result<(), VMError> {
        self.validate_args_exact(method, args, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_interp() -> MirInterpreter {
        // Create minimal interpreter for testing
        MirInterpreter {
            // ... minimal fields
        }
    }

    #[test]
    fn test_validate_args_exact_ok() {
        let interp = dummy_interp();
        let args = vec![ValueId(1)];
        assert!(interp.validate_args_exact("push", &args, 1).is_ok());
    }

    #[test]
    fn test_validate_args_exact_too_many() {
        let interp = dummy_interp();
        let args = vec![ValueId(1), ValueId(2)];
        let result = interp.validate_args_exact("push", &args, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expects exactly 1"));
    }

    #[test]
    fn test_validate_args_range_ok() {
        let interp = dummy_interp();
        let args = vec![ValueId(1), ValueId(2)];
        assert!(interp.validate_args_range("substring", &args, 1, 2).is_ok());
    }

    #[test]
    fn test_validate_args_range_too_few() {
        let interp = dummy_interp();
        let args = vec![];
        let result = interp.validate_args_range("substring", &args, 1, 2);
        assert!(result.is_err());
    }
}
```

---

### Step 4: conversions.rs 実装

**ファイル**: `src/backend/mir_interpreter/utils/conversions.rs`

```rust
use crate::backend::mir_interpreter::{MirInterpreter, VMValue};
use crate::box_trait::NyashBox;

impl MirInterpreter {
    /// Convert a VMValue to Box<dyn NyashBox>, handling both BoxRef and primitive types.
    ///
    /// # Example
    /// ```rust
    /// let recv = self.reg_load(box_val)?;
    /// let recv_box = self.convert_to_box(&recv);
    /// ```
    pub(crate) fn convert_to_box(&self, value: &VMValue) -> Box<dyn NyashBox> {
        match value.clone() {
            VMValue::BoxRef(b) => b.share_box(),
            other => other.to_nyash_box(),
        }
    }

    /// Convert and downcast to a specific box type.
    ///
    /// # Example
    /// ```rust
    /// if let Some(arr) = self.downcast_box::<ArrayBox>(&recv)? {
    ///     // Handle ArrayBox methods
    /// }
    /// ```
    pub(crate) fn downcast_box<T: NyashBox + 'static>(
        &self,
        value: &VMValue,
    ) -> Option<&T> {
        let boxed = self.convert_to_box(value);
        boxed.as_any().downcast_ref::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boxes::array::ArrayBox;

    #[test]
    fn test_convert_to_box_from_primitive() {
        let interp = dummy_interp();
        let value = VMValue::Integer(42);
        let boxed = interp.convert_to_box(&value);
        // Should convert to IntegerBox
        assert!(boxed.as_any().is::<crate::boxes::integer::IntegerBox>());
    }

    #[test]
    fn test_convert_to_box_from_boxref() {
        let interp = dummy_interp();
        let arr = Box::new(ArrayBox::new());
        let value = VMValue::BoxRef(arr.into());
        let boxed = interp.convert_to_box(&value);
        assert!(boxed.as_any().is::<ArrayBox>());
    }
}
```

---

### Step 5: mod.rs 設定

**ファイル**: `src/backend/mir_interpreter/utils/mod.rs`

```rust
//! Utility functions for MIR interpreter handlers.
//!
//! This module provides common patterns used across handler implementations:
//! - Register operations (writing results)
//! - Argument validation
//! - Type conversions

mod conversions;
mod register_ops;
mod validation;

// Re-export for convenience (optional)
pub(crate) use conversions::*;
pub(crate) use register_ops::*;
pub(crate) use validation::*;
```

---

### Step 6: ハンドラー更新 - 段階的移行

#### 例: boxes_array.rs の更新

**Before**:
```rust
"push" => {
    if args.len() != 1 {
        return Err(VMError::InvalidInstruction("push expects 1 arg".into()));
    }
    let val = this.reg_load(args[0])?.to_nyash_box();
    let _ = ab.push(val);
    if let Some(d) = dst {
        this.regs.insert(d, VMValue::Void);
    }
    return Ok(true);
}
```

**After**:
```rust
"push" => {
    this.validate_args_exact("push", args, 1)?;
    let val = this.reg_load(args[0])?.to_nyash_box();
    let _ = ab.push(val);
    this.write_void(dst);
    return Ok(true);
}
```

**削減**: 6行 → 4行（2行削減）

---

## 移行チェックリスト

### Phase 1-A: インフラ構築（1-2時間）
- [ ] `utils/` ディレクトリ作成
- [ ] `mod.rs`, `register_ops.rs`, `validation.rs`, `conversions.rs` 実装
- [ ] ユニットテスト追加
- [ ] 全テストが通ることを確認

### Phase 1-B: Handler更新（3-5時間）
ファイル単位で更新し、都度テストを実行：

1. [ ] `boxes_array.rs` 更新（最も小さいファイルから開始）
   - [ ] 引数検証を`validate_args_*`に置き換え
   - [ ] Destination書き込みを`write_*`に置き換え
   - [ ] Receiver変換を`convert_to_box`に置き換え
   - [ ] テスト実行

2. [ ] `boxes_map.rs` 更新
3. [ ] `boxes_string.rs` 更新
4. [ ] `boxes_plugin.rs` 更新
5. [ ] `boxes_instance.rs` 更新
6. [ ] `boxes_object_fields.rs` 更新
7. [ ] `boxes.rs` 更新
8. [ ] `calls.rs` 更新（大きいファイルは最後に）

### Phase 1-C: 検証（1時間）
- [ ] 全ハンドラーファイルでパターン検索
  ```bash
  # 残存する古いパターンを確認
  grep -rn "if let Some(d) = dst { this.regs.insert" src/backend/mir_interpreter/handlers/
  grep -rn "args.len() !=" src/backend/mir_interpreter/handlers/
  grep -rn "match recv.clone()" src/backend/mir_interpreter/handlers/
  ```
- [ ] スモークテスト実行
  ```bash
  ./tools/jit_smoke.sh
  ```
- [ ] ドキュメント更新

---

## テスト戦略

### 単体テスト
各ユーティリティ関数に対して：
- 正常系（期待通りの動作）
- 異常系（エラーケース）
- 境界値（0引数、大量引数など）

### 回帰テスト
既存のスモークテストを活用：
```bash
# 変更前にベースライン取得
./tools/jit_smoke.sh > /tmp/before.log 2>&1

# 変更適用

# 変更後に結果比較
./tools/jit_smoke.sh > /tmp/after.log 2>&1
diff /tmp/before.log /tmp/after.log
```

---

## トラブルシューティング

### 問題: コンパイルエラー「method not found」
**原因**: `utils/mod.rs` のインポートが不足
**解決**: `src/backend/mir_interpreter/mod.rs` に以下を追加
```rust
mod utils;
```

### 問題: テスト実行時のパニック
**原因**: `MirInterpreter` の最小構成が不完全
**解決**: テスト用のビルダー関数を用意
```rust
#[cfg(test)]
fn test_interpreter() -> MirInterpreter {
    MirInterpreter::new_for_test()
}
```

### 問題: 既存テストが失敗
**原因**: エラーメッセージの形式変更
**解決**: テストの期待値を更新（より良いメッセージに改善された）

---

## 期待される結果

### コード削減
- **boxes_array.rs**: 63行 → 約50行（13行削減、21%減）
- **boxes_map.rs**: 134行 → 約110行（24行削減、18%減）
- **boxes_string.rs**: 208行 → 約170行（38行削減、18%減）
- **全体**: 約270-380行削減（8-11%減）

### 品質向上
- エラーメッセージの統一
- テストカバレッジの向上
- 新機能追加の容易性

---

## 次のステップ（Phase 2）

Phase 1完了後、以下を実施：
1. Phase 2計画（エラー生成、PHIヘルパー）
2. 追加の共通化機会の調査
3. Phase 3（Box Handler統合）の詳細設計

---

**実装者**: （名前）
**開始日**: （日付）
**完了予定**: （日付）
**実績**: （完了日）
