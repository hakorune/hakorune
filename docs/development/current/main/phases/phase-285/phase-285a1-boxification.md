# Phase 285A1: Weak Field Validator Boxification

**Status**: ✅ Complete (2025-12-24)

## Goal

Phase 285A1 の weak field 処理を専用 Box に箱化し、以下を実現：
- **単一責任**: weak field 契約検証のみを担当
- **テスト容易性**: 独立してテスト可能
- **再利用性**: 他の箇所でも使える

## Implementation

### 新規ファイル

#### `src/mir/builder/weak_field_validator.rs`

```rust
/// Phase 285A1: Weak Field Contract Validator Box
///
/// 単一責任: weak field の契約検証のみ
/// - 読み込み時: WeakRef 型注釈
/// - 書き込み時: 型契約検証（WeakRef/Void のみ許可）
pub(super) struct WeakFieldValidatorBox;

impl WeakFieldValidatorBox {
    /// Weak field 読み込み時の型注釈を追加
    pub(super) fn annotate_read_result(
        type_ctx: &mut TypeContext,
        dst: ValueId,
    ) {
        // Phase 285A1: Mark the result as WeakRef type
        type_ctx.value_types.insert(dst, MirType::WeakRef);
    }

    /// Weak field への代入を検証（3つの許可ケース）
    ///
    /// Phase 285A1 Fail-Fast 契約:
    /// - **許可**: WeakRef (weak() または weak field 読み込み)
    /// - **許可**: Void (クリア操作)
    /// - **禁止**: BoxRef (weak() なしの Box)
    /// - **禁止**: プリミティブ型
    /// - **禁止**: 型追跡されていない値
    pub(super) fn validate_assignment(
        value_type: Option<&MirType>,
        box_name: &str,
        field_name: &str,
    ) -> Result<(), String> {
        // 3つの許可ケースを検証
        // Fail-Fast: エラーは即座に返す
    }
}
```

**特徴**:
- Phase 33 の箱化モジュール化と同じ思想（単一責任、明確な境界）
- 充実したコメント（目的、契約、例）
- 単体テスト完備（5つのテストケース）

### 変更ファイル

#### `src/mir/builder/fields.rs`

**Before** (277行):
- `check_weak_field_assignment()` メソッド（232-277行）
- 読み込み時の型注釈（inline）
- 書き込み時の型検証（inline）

**After** (237行):
- `WeakFieldValidatorBox::annotate_read_result()` 呼び出し
- `WeakFieldValidatorBox::validate_assignment()` 呼び出し
- **40行削減**（-14.4%）

#### `src/mir/builder.rs`

モジュール宣言追加:
```rust
mod weak_field_validator; // Phase 285A1: Weak field contract validator
```

## Contract Enforcement

### 読み込み契約

```rust
// Phase 285A1: weak field 読み込みは WeakRef 型を返す（自動昇格なし）
WeakFieldValidatorBox::annotate_read_result(&mut self.type_ctx, dst);
```

### 書き込み契約

**3つの許可ケース**:
1. **WeakRef**: `weak()` または weak field 読み込みの結果
2. **Void**: クリア操作（`me.next = Void`）
3. ~~BoxRef~~: **禁止** - エラーメッセージで `weak()` 使用を提案

**エラーメッセージ例**:
```
Cannot assign Box (StringBox) to weak field 'Node.next'.
Use weak(...) to create weak reference: me.next = weak(value)
```

## Tests

### 単体テスト（5つ）

1. `test_validate_weakref_allowed`: WeakRef 許可
2. `test_validate_void_allowed`: Void 許可
3. `test_validate_boxref_forbidden`: BoxRef 禁止
4. `test_validate_untracked_forbidden`: 型追跡なし禁止
5. `test_validate_primitive_forbidden`: プリミティブ禁止

### 動作確認

```bash
# ビルド成功
cargo build --release
# Finished `release` profile [optimized] target(s) in 1m 23s

# 基本機能確認
echo 'static box Main { main() { print("Hello"); return 0 } }' > test.hako
./target/release/hakorune test.hako
# Output: Hello
# RC: 0

# フィールドアクセス確認
# Box with fields works correctly
```

## Architecture Benefits

### 単一責任（SRP）

- **Before**: `fields.rs` が weak field 処理を含む（複数責任）
- **After**: `WeakFieldValidatorBox` が検証のみ担当

### テスト容易性

- **Before**: `fields.rs` 全体のテストが必要
- **After**: `WeakFieldValidatorBox` 単独でテスト可能

### 再利用性

- **Before**: `check_weak_field_assignment()` は `MirBuilder` メソッド
- **After**: `WeakFieldValidatorBox` はどこからでも使える

### 明確な境界

```
fields.rs
  ├─ フィールド読み込み/書き込みロジック
  └─ WeakFieldValidatorBox::validate_assignment() 呼び出し
      ↓
weak_field_validator.rs
  └─ weak field 契約検証のみ
```

## Code Quality

### Phase 33 思想の継承

- **単一責任**: 1つの Box = 1つの関心事
- **明確な境界**: 入力/出力が明確
- **充実したコメント**: 目的、契約、例
- **Fail-Fast**: エラーは即座に返す（フォールバック禁止）

### メトリクス

- **行数削減**: 277行 → 237行（-40行、-14.4%）
- **モジュール化**: 1ファイル → 2ファイル（責任分離）
- **テストカバレッジ**: 0 → 5単体テスト

## Phase 285 Context

Phase 285A1 は Phase 285 の一部として、weak field 処理の基盤を整備：

- **Phase 285 Goal**: Box lifecycle / weakref / finalization / GC conformance
- **Phase 285A1 Scope**: weak field 契約検証の箱化
- **Status**: P0 (docs-only) → 実装進行中

**Note**: weak field 構文（`weak next: Node`）は未実装。Phase 285 P1 で実装予定。

---

## A1.5: Parser Hang Fix - Parameter Type Annotations ✅

**Status**: ✅ Complete (2025-12-24)

### Problem

During Phase 285A1.4 implementation, discovered critical parser bug:
- User writes: `setParent(p: Node) { ... }`
- Parser hangs infinitely at COLON token (no advance on unexpected token)
- Workaround required removing type annotation → `setParent(p) { ... }` → Works

### Root Cause

**6 identical vulnerable parameter parsing loops** across the parser codebase:
1. `src/parser/declarations/box_def/members/methods.rs:21-30`
2. `src/parser/declarations/box_def/members/constructors.rs:27-34` (init)
3. `src/parser/declarations/box_def/members/constructors.rs:101-108` (pack)
4. `src/parser/declarations/box_def/members/constructors.rs:138-145` (birth)
5. `src/parser/items/functions.rs:34-51`
6. `src/parser/items/static_items.rs:72-87`

**Vulnerable Code Pattern**:
```rust
while !p.match_token(&TokenType::RPAREN) && !p.is_at_end() {
    must_advance!(p, _unused, "method parameter parsing");
    if let TokenType::IDENTIFIER(param) = &p.current_token().token_type {
        params.push(param.clone());
        p.advance();  // ← Only advances on IDENTIFIER
    }
    // ⚠️ COLON token: not IDENTIFIER, not COMMA, not RPAREN → NO ADVANCE → INFINITE LOOP
    if p.match_token(&TokenType::COMMA) {
        p.advance();
    }
}
```

**EBNF Spec Finding**: `params` grammar undefined in EBNF.md, `:` TYPE is for return type only

### Solution: Shared Helper Function (DRY Principle)

Created `src/parser/common/params.rs` with common parameter parsing logic:

```rust
/// Parse parameter name list with Fail-Fast on unexpected tokens
///
/// Parses: IDENT (',' IDENT)*
/// Rejects: Type annotations, unexpected tokens, malformed comma sequences
pub(crate) fn parse_param_name_list(
    p: &mut NyashParser,
    context: &str,  // "method", "constructor", "function" for error messages
) -> Result<Vec<String>, ParseError>
```

**Key Features**:
- **Progress-zero detection**: Tracks token position, errors if stuck (prevents infinite loops)
- **Explicit token handling**: All token types (IDENTIFIER, COMMA, other) explicitly matched
- **Fail-Fast**: Either advances or errors (no infinite loop possible)
- **Unified error messages**: Context parameter customizes messages per call site

### Files Modified

**New Files** (2):
- `src/parser/common/params.rs` (~90 lines) - Helper function
- `tools/smokes/v2/profiles/quick/parser/phase285_param_type_annotation_nohang.sh` - Timeout smoke test

**Modified Files** (8):
1. `src/parser/common/mod.rs` - Module declaration (moved common.rs → common/mod.rs)
2. `src/parser/declarations/box_def/members/methods.rs` - Replaced 12 lines with 1 call
3. `src/parser/declarations/box_def/members/constructors.rs` - Replaced 3 loops (init/pack/birth)
4. `src/parser/items/functions.rs` - Replaced 20 lines with 2 lines
5. `src/parser/items/static_items.rs` - Replaced 22 lines with 2 lines
6. `apps/tests/phase285_parser_param_type_annot_should_not_hang.hako` - Regression test

**Net Change**: +90 new - 72 removed + 6 calls = **+24 lines** (with better error handling!)

### Tests

**Regression Test**: `apps/tests/phase285_parser_param_type_annot_should_not_hang.hako`
```hako
box TestNode {
    value: IntegerBox
    // ❌ Should error immediately (type annotation not supported)
    setParent(p: Node) {
        return 0
    }
}
```

**Smoke Test**: `tools/smokes/v2/profiles/quick/parser/phase285_param_type_annotation_nohang.sh`
- Timeout: 3 seconds (detects hang)
- Expected: Parse error within 1 second
- Validation: Error message mentions "Unexpected token COLON" and "Parameter type annotations not supported"

**Result**: ✅ PASS
```
✅ PASS: Parser correctly rejects param type annotations without hanging
```

### Error Message Examples

**Before** (infinite hang):
```
🚨 PARSER INFINITE LOOP DETECTED at method parameter parsing
```

**After** (clear error):
```
❌ Parse error: Unexpected token COLON, expected ',' or ')' in method parameter list.
Note: Parameter type annotations are not supported.
```

### Architecture Benefits

**DRY Principle**:
- **Before**: 6 identical vulnerable loops (72 lines total)
- **After**: 1 helper function (~90 lines) + 6 one-line calls

**Maintainability**:
- Future fixes only need 1 location
- Unified error messages (single source of truth)
- Progress-zero detection guaranteed in one place

**Safety**:
- No infinite loop possible (Fail-Fast guaranteed)
- All token types explicitly handled (no silent fallthrough)
- Context-aware error messages (better UX)

## References

- Phase 33: Box Theory Modularization ([phase-33-modularization.md](../../../architecture/phase-33-modularization.md))
- Phase 285: Box lifecycle ([phase-285/README.md](README.md))
- `src/mir/builder/weak_field_validator.rs`: A1.1 実装本体
- `src/mir/builder/fields.rs`: A1.1 呼び出し側
- `src/parser/common/params.rs`: A1.5 実装本体
- `src/parser/declarations/box_def/members/fields.rs`: A1.2-A1.4 実装本体
