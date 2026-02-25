# Phase 192: Complex Addend Design (Doc-Only)

**Status**: Design Phase
**Date**: 2025-12-09
**Prerequisite**: Phase 191 complete (body-local init integrated)

---

## 目的

`result = result * 10 + digits.indexOf(ch)` のような
「加算側がメソッド呼び出し」のパターンを、
既存の NumberAccumulation ラインの前処理として安全に扱えるようにする。

新パターンは増やさず、Analyzer/Lowerer の箱を拡張するだけに留める。

---

## Section 1: 対象 RHS パターン一覧

### 1.1 JsonParser._parse_number (lines 106-142)

**ループパターン**:
```nyash
local num_str = ""
local digits = "0123456789"
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)

    if digit_pos < 0 {
        break
    }

    num_str = num_str + ch    // ← StringAppendChar (already supported)
    p = p + 1
}
```

**Note**: このループは string accumulation なので Phase 192 の対象外（既にサポート済み）。

### 1.2 JsonParser._atoi (lines 436-467)

**ループパターン**:
```nyash
local v = 0
local digits = "0123456789"
loop(i < n) {
    local ch = s.substring(i, i+1)
    if ch < "0" || ch > "9" { break }
    local pos = digits.indexOf(ch)
    if pos < 0 { break }
    v = v * 10 + pos    // ← NumberAccumulation with body-local (Phase 191 supported)
    i = i + 1
}
```

**Current Status**: Phase 191 で body-local `pos` が使えるようになったので対応可能。

### 1.3 Complex Addend Pattern (Target for Phase 192)

**パターン**:
```nyash
local v = 0
local digits = "0123456789"
loop(i < n) {
    local ch = s.substring(i, i+1)
    v = v * 10 + digits.indexOf(ch)    // ← Complex addend (method call)
    i = i + 1
}
```

**AST Form**:
```
Assign(
  lhs = "v",
  rhs = BinaryOp(
    op = Add,
    left = BinaryOp(
      op = Mul,
      left = Variable("v"),
      right = Literal(Integer(10))
    ),
    right = MethodCall(
      receiver = Variable("digits"),
      method = "indexOf",
      args = [Variable("ch")]
    )
  )
)
```

**Characteristics**:
- LHS appears exactly once in RHS (in left-most multiplication)
- Base: 10 (constant)
- Addend: **MethodCall** (complex expression)
- Current behavior: Rejected as `UpdateKind::Complex`

---

## Section 2: LHS 出現回数とMethod Call位置の整理

### 2.1 パターンマトリクス

| Pattern | LHS Count | Base | Addend | Current | Phase 192 |
|---------|-----------|------|--------|---------|-----------|
| `v = v * 10 + pos` | 1 | Const(10) | Variable | NumberAccumulation | No change |
| `v = v * 10 + 5` | 1 | Const(10) | Const(5) | NumberAccumulation | No change |
| `v = v * 10 + digits.indexOf(ch)` | 1 | Const(10) | MethodCall | **Complex** | **Normalize** |
| `v = v * base + x` | 1 | Variable | Variable | Complex | Fail-Fast |
| `v = v * 10 + (a + b)` | 1 | Const(10) | BinaryOp | Complex | Future |
| `v = v * 10 + v` | 2 | Const(10) | Variable | Complex | Fail-Fast |

### 2.2 Method Call の位置分類

**Type A: Addend に Method Call（Phase 192 対象）**:
```nyash
v = v * 10 + digits.indexOf(ch)
```
→ Normalize: `temp = digits.indexOf(ch); v = v * 10 + temp`

**Type B: Base に Method Call（対象外）**:
```nyash
v = v * getBase() + x
```
→ Fail-Fast（base は constant のみ許可）

**Type C: LHS 側に Method Call（対象外）**:
```nyash
v = obj.getValue() * 10 + x
```
→ Fail-Fast（LHS は simple variable のみ）

### 2.3 Nested Method Call（将来拡張）

```nyash
v = v * 10 + parser.parse(s.substring(i, i+1))
```
→ Phase 192 では Fail-Fast、Phase 193+ で対応検討

---

## Section 3: Temp 分解戦略

### 3.1 正規化アプローチ

**Before (Complex)**:
```nyash
result = result * 10 + digits.indexOf(ch)
```

**After (Normalized)**:
```nyash
local temp_digit = digits.indexOf(ch)
result = result * 10 + temp_digit
```

### 3.2 正規化の責務

新しい箱: **ComplexAddendNormalizer**

**入力**:
- `Assign(lhs, complex_rhs)` where `complex_rhs` has MethodCall in addend
- ループ本体 AST（temp 変数を挿入する位置情報）

**出力**:
- `temp` 定義: `local temp = methodCall(...)`
- 正規化された Assign: `lhs = lhs * base + temp`

**配置**:
- Pattern2 lowering の前処理ライン（`can_lower` の前）

---

## Section 4: ComplexAddendNormalizer 擬似コード

### 4.1 検出ロジック

```rust
fn is_complex_addend_pattern(update_expr: &UpdateExpr) -> bool {
    // Check structure: lhs = lhs * base + addend
    let UpdateExpr::BinOp { op: BinOpKind::Add, left, right } = update_expr else {
        return false;
    };

    // Left side: lhs * base (multiplication)
    let UpdateRhs::BinOp { op: BinOpKind::Mul, .. } = left else {
        return false;
    };

    // Right side (addend): MethodCall or complex expression
    matches!(right, UpdateRhs::MethodCall(_) | UpdateRhs::BinaryOp(_))
}
```

### 4.2 正規化ロジック

```rust
fn normalize_complex_addend(
    lhs: &str,
    update_expr: &UpdateExpr,
    body_ast: &mut Vec<ASTNode>,
) -> Result<(String, UpdateExpr), String> {
    // Extract addend (method call or complex expression)
    let addend = extract_addend(update_expr)?;

    // Generate temp variable name
    let temp_name = format!("temp_{}_addend", lhs);

    // Insert temp assignment at current position
    // local temp = digits.indexOf(ch)
    let temp_init = ASTNode::LocalDeclaration {
        name: temp_name.clone(),
        init: Some(Box::new(addend.to_ast())),
    };
    body_ast.insert(0, temp_init);  // Insert at loop body start

    // Create normalized update expression
    // lhs = lhs * 10 + temp
    let normalized_expr = UpdateExpr::BinOp {
        op: BinOpKind::Add,
        left: Box::new(UpdateRhs::BinOp {
            op: BinOpKind::Mul,
            left: Box::new(UpdateRhs::Variable(lhs.to_string())),
            right: Box::new(UpdateRhs::Const(extract_base(update_expr)?)),
        }),
        right: Box::new(UpdateRhs::Variable(temp_name.clone())),
    };

    Ok((temp_name, normalized_expr))
}
```

### 4.3 Phase 191 との統合

Phase 191 の `LoopBodyLocalInitLowerer` が `temp` の初期化を処理:
- `local temp_digit = digits.indexOf(ch)` → JoinIR に emit
- `LoopBodyLocalEnv` に `temp_digit -> join_id` を登録
- `UpdateEnv` が `temp_digit` を解決して NumberAccumulation に渡す

---

## Section 5: LoopUpdateAnalyzer との責務分け

### 5.1 現在のフロー

```
AST → LoopUpdateAnalyzer → UpdateKind::Complex (Fail-Fast)
```

### 5.2 Phase 192 フロー

```
AST → LoopUpdateAnalyzer → UpdateKind::Complex
  ↓ (Pattern2 内 can_lower 前)
ComplexAddendNormalizer → 前処理 (temp 生成)
  ↓
再度 LoopUpdateAnalyzer → UpdateKind::NumberAccumulation { base: 10 }
  ↓
Pattern2 lowering → JoinIR emission
```

### 5.3 can_lower の変更点

**Before (Phase 191)**:
```rust
fn can_lower_carrier_updates(updates: &HashMap<String, UpdateExpr>) -> bool {
    for (name, update_expr) in updates {
        let kind = classify_update_kind(update_expr);
        match kind {
            UpdateKind::Complex => {
                eprintln!("[joinir/freeze] Complex carrier update");
                return false;
            }
            _ => { /* OK */ }
        }
    }
    true
}
```

**After (Phase 192)**:
```rust
fn can_lower_carrier_updates_with_normalization(
    updates: &HashMap<String, UpdateExpr>,
    body_ast: &mut Vec<ASTNode>,
) -> Result<HashMap<String, UpdateExpr>, String> {
    let mut normalized_updates = HashMap::new();

    for (name, update_expr) in updates {
        let kind = classify_update_kind(update_expr);
        match kind {
            UpdateKind::Complex => {
                // Try normalization
                if is_complex_addend_pattern(update_expr) {
                    let (temp_name, normalized_expr) =
                        ComplexAddendNormalizer::normalize(name, update_expr, body_ast)?;

                    // Re-classify
                    let normalized_kind = classify_update_kind(&normalized_expr);
                    if matches!(normalized_kind, UpdateKind::NumberAccumulation { .. }) {
                        normalized_updates.insert(name.clone(), normalized_expr);
                    } else {
                        return Err("[joinir/freeze] Normalization failed".to_string());
                    }
                } else {
                    return Err("[joinir/freeze] Complex pattern not supported".to_string());
                }
            }
            _ => {
                normalized_updates.insert(name.clone(), update_expr.clone());
            }
        }
    }

    Ok(normalized_updates)
}
```

---

## Section 6: Emission 側への影響

### 6.1 JoinIR Emission（変更なし）

ComplexAddendNormalizer で前処理するため、既存の emission ラインは変更不要:

1. **LoopBodyLocalInitLowerer** (Phase 191):
   - `local temp_digit = digits.indexOf(ch)` を JoinIR に emit
   - MethodCall の lowering は既存の式 lowerer に委譲

2. **CarrierUpdateLowerer** (Phase 190):
   - `result = result * 10 + temp_digit` を NumberAccumulation として emission
   - `temp_digit` は UpdateEnv 経由で解決

3. **NumberAccumulation emission**:
   - Phase 190 の 2-instruction emission そのまま:
     ```
     tmp = result * 10
     result = tmp + temp_digit
     ```

### 6.2 設計原則

- **Separation of Concerns**: 正規化 (ComplexAddendNormalizer) と emission (CarrierUpdateLowerer) を分離
- **Reusability**: 既存の body-local init / NumberAccumulation emission を再利用
- **Fail-Fast**: 対応できないパターンは明示的エラー

---

## Section 7: Implementation Phases (TBD)

### Phase 192-impl-A: ComplexAddendNormalizer 実装

1. `is_complex_addend_pattern()` 検出ロジック
2. `normalize_complex_addend()` 正規化ロジック
3. temp 変数生成とAST挿入

### Phase 192-impl-B: Pattern2 統合

1. `can_lower` を `can_lower_with_normalization` に拡張
2. 正規化後の UpdateExpr で再解析
3. Unit tests (5+ cases)

### Phase 192-impl-C: E2E テスト

1. `phase192_complex_addend_atoi.hako` 作成
2. `result = result * 10 + digits.indexOf(ch)` パターンで期待値確認
3. 退行テスト（phase191_*.hako）

---

## Section 8: 制約と Non-Goals

### 8.1 対応パターン

**Phase 192 で対応**:
- Addend に simple MethodCall: `v = v * 10 + digits.indexOf(ch)`
- Addend が Variable の場合は Phase 191 で対応済み

**Phase 192 で非対応（将来拡張）**:
- Nested method call: `v = v * 10 + parser.parse(s.substring(...))`
- Complex binary expression: `v = v * 10 + (a + b * c)`
- Multiple method calls in same update

### 8.2 Fail-Fast ケース

- Base が variable: `v = v * base + f(x)`
- LHS が複数回出現: `v = v * 10 + v`
- Method call が base 側: `v = v * getBase() + x`

---

## Appendix A: AST Examples

### A.1 Before Normalization

**Source**:
```nyash
result = result * 10 + digits.indexOf(ch)
```

**AST**:
```
Assignment {
    target: Variable { name: "result" },
    value: BinaryOp {
        operator: Add,
        left: BinaryOp {
            operator: Mul,
            left: Variable { name: "result" },
            right: Literal { value: Integer(10) },
        },
        right: MethodCall {
            receiver: Variable { name: "digits" },
            method: "indexOf",
            args: [Variable { name: "ch" }],
        },
    },
}
```

### A.2 After Normalization

**Source**:
```nyash
local temp_result_addend = digits.indexOf(ch)
result = result * 10 + temp_result_addend
```

**AST**:
```
[
  LocalDeclaration {
      name: "temp_result_addend",
      init: Some(MethodCall {
          receiver: Variable { name: "digits" },
          method: "indexOf",
          args: [Variable { name: "ch" }],
      }),
  },
  Assignment {
      target: Variable { name: "result" },
      value: BinaryOp {
          operator: Add,
          left: BinaryOp {
              operator: Mul,
              left: Variable { name: "result" },
              right: Literal { value: Integer(10) },
          },
          right: Variable { name: "temp_result_addend" },
      },
  },
]
```

---

---

## Section 9: Implementation Complete (2025-12-09)

### 9.1 Implementation Summary

**Status**: ✅ Phase 192-impl Complete

**Deliverables**:
1. ✅ `ComplexAddendNormalizer` module implemented (`src/mir/join_ir/lowering/complex_addend_normalizer.rs`)
2. ✅ 5 unit tests all passing (method call, simple variable, wrong LHS, no multiplication, subtraction)
3. ✅ Pattern2 integration complete (preprocessing before carrier update analysis)
4. ✅ Existing tests verified (phase190/191 tests still pass)
5. ✅ Documentation updated

### 9.2 Actual Implementation

**File**: `src/mir/join_ir/lowering/complex_addend_normalizer.rs`

**API**:
```rust
pub enum NormalizationResult {
    Unchanged,
    Normalized { temp_def: ASTNode, new_assign: ASTNode, temp_name: String },
}

impl ComplexAddendNormalizer {
    pub fn normalize_assign(assign: &ASTNode) -> NormalizationResult;
}
```

**Integration Point** (Pattern2 - line 243-279):
```rust
// Phase 192: Normalize complex addend patterns in loop body
let mut normalized_body = Vec::new();
let mut has_normalization = false;

for node in _body {
    match ComplexAddendNormalizer::normalize_assign(node) {
        NormalizationResult::Normalized { temp_def, new_assign, temp_name } => {
            normalized_body.push(temp_def);      // local temp = <complex_expr>
            normalized_body.push(new_assign);    // lhs = lhs * base + temp
            has_normalization = true;
        }
        NormalizationResult::Unchanged => {
            normalized_body.push(node.clone());
        }
    }
}

let analysis_body = if has_normalization { &normalized_body } else { _body };
// Pass analysis_body to LoopUpdateAnalyzer and lower_loop_with_break_minimal
```

### 9.3 AST Transformation Example

**Before**:
```nyash
result = result * 10 + digits.indexOf(ch)
```

**After** (Normalized AST):
```nyash
local temp_result_addend = digits.indexOf(ch)
result = result * 10 + temp_result_addend
```

**AST Structure**:
```
[
  Local { variables: ["temp_result_addend"], initial_values: [MethodCall(...)] },
  Assignment { target: "result", value: BinOp(Add, BinOp(Mul, "result", 10), "temp_result_addend") }
]
```

### 9.4 Test Results

**Unit Tests** (5/5 passing):
- ✅ `test_normalize_complex_addend_method_call` - Core normalization pattern
- ✅ `test_normalize_simple_variable_unchanged` - No-op for simple patterns
- ✅ `test_normalize_wrong_lhs_unchanged` - Reject invalid patterns
- ✅ `test_normalize_no_multiplication_unchanged` - Reject non-accumulation patterns
- ✅ `test_normalize_subtraction_complex_addend` - Subtraction variant

**Integration Tests** (regression verified):
- ✅ `phase190_atoi_impl.hako` → 12 (no regression)
- ✅ `phase190_parse_number_impl.hako` → 123 (no regression)
- ✅ `phase191_body_local_atoi.hako` → 123 (no regression)
- ✅ `phase192_normalization_demo.hako` → 123 (new demo test)

### 9.5 Current Limitations (Phase 193+ Work)

**Limitation**: Full E2E flow with MethodCall in temp variables requires extending `LoopBodyLocalInitLowerer` (Phase 186).

**Current Behavior**:
```nyash
local temp = digits.indexOf(ch)  // ❌ Phase 186 error: "Unsupported init expression: method call"
```

**Phase 186 Scope**: Only supports int/arithmetic operations (`+`, `-`, `*`, `/`, constants, variables)

**Future Work** (Phase 193+):
- Extend `LoopBodyLocalInitLowerer::lower_init_expr()` to handle:
  - `ASTNode::MethodCall` (e.g., `digits.indexOf(ch)`)
  - `ASTNode::Call` (e.g., `parseInt(s)`)
- Add emission logic for method call results in JoinIR
- Add UpdateEnv resolution for method call temps

**Workaround**: For now, complex addend normalization works at the AST level, but lowering requires manual temp extraction outside the loop.

### 9.6 Design Principles Validated

✅ **箱化 (Box-First)**:
- ComplexAddendNormalizer is a pure AST transformer (single responsibility)
- No emission logic mixed in (delegation to existing Phase 191/190 infrastructure)

✅ **Fail-Fast**:
- Unsupported patterns return `NormalizationResult::Unchanged`
- Phase 186 limitation explicitly documented

✅ **Reusability**:
- Normalizer works with any BinaryOp pattern (Add/Subtract)
- Integration point is clean (10 lines in Pattern2)

✅ **Minimal Changes**:
- Zero changes to emission layers (CarrierUpdateEmitter, LoopBodyLocalInitLowerer)
- Only preprocessing added to Pattern2 (before carrier analysis)

### 9.7 Trace Output

When running a test with complex addend pattern:
```
[pattern2/phase192] Normalized complex addend: temp='temp_result_addend' inserted before update
[cf_loop/pattern2] Phase 176-3: Analyzed 1 carrier updates
```

---

## Revision History

- **2025-12-09**: Initial design document (Section 1-8)
- **2025-12-09**: Implementation complete (Section 9)
Status: Historical
