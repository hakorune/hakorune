# Phase 224-E: DigitPos Condition Normalizer

Status: Active（DigitPos 条件正規化ラインの設計メモ / 実装ガイド）  
Scope: digit_pos → is_digit_pos Carrier / ConditionEnv / ExprLowerer の正規化経路の SSOT ドキュメントだよ。Phase 26‑H / 34 系の JsonParser `_parse_number` / `_atoi` でもこの設計を前提にしている。

## Problem Statement

### Background
Phase 228-8 successfully implemented the DigitPos pattern promotion:
- `digit_pos: i32` → `is_digit_pos: bool` carrier
- CarrierRole::ConditionOnly + CarrierInit::BoolConst(false)
- ConditionEnv alias: `digit_pos → is_digit_pos`

### Current Error
```
Type error: unsupported compare Lt on Bool(false) and Integer(0)
```

**Root Cause**: The break condition AST still contains `digit_pos < 0`, which after alias resolution becomes `Bool(is_digit_pos) < Integer(0)`, causing a type mismatch.

**Why alias isn't enough**:
- Alias only renames the variable reference
- The comparison operator and integer literal remain unchanged
- Need to transform the entire expression structure

## Solution

### Transformation
Transform the break condition AST from integer comparison to boolean negation:

**Before**: `digit_pos < 0`
```
BinaryOp {
    operator: Lt,
    left: Var("digit_pos"),
    right: Const(0)
}
```

**After**: `!is_digit_pos`
```
UnaryOp {
    operator: Not,
    expr: Var("is_digit_pos")
}
```

### Semantic Equivalence
- `digit_pos < 0` means "indexOf() didn't find the character" (returns -1)
- `!is_digit_pos` means "character is not a digit" (bool carrier is false)
- Both express the same condition: "break when character not found/matched"

## Design

### Box: DigitPosConditionNormalizer

**Responsibility**: Transform digit_pos comparison patterns to boolean carrier expressions

**API**:
```rust
pub struct DigitPosConditionNormalizer;

impl DigitPosConditionNormalizer {
    /// Normalize digit_pos condition AST
    ///
    /// Transforms: `digit_pos < 0` → `!is_digit_pos`
    ///
    /// # Arguments
    /// * `cond` - Break/continue condition AST
    /// * `promoted_var` - Original variable name (e.g., "digit_pos")
    /// * `carrier_name` - Promoted carrier name (e.g., "is_digit_pos")
    ///
    /// # Returns
    /// Normalized AST (or original if pattern doesn't match)
    pub fn normalize(
        cond: &ASTNode,
        promoted_var: &str,
        carrier_name: &str,
    ) -> ASTNode;
}
```

### Pattern Matching Logic

**Match Pattern**: `<var> < 0` where:
1. Operator is `Lt` (Less than)
2. Left operand is `Var(promoted_var)`
3. Right operand is `Const(0)`

**Transformation**: → `UnaryOp { op: Not, expr: Var(carrier_name) }`

**Non-Match Behavior**: Return original AST unchanged (Fail-Safe)

### Integration Point

**Location**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

**Integration Steps**:
1. After `LoopBodyCondPromoter::try_promote_for_condition()` succeeds
2. Extract `promoted_var` and `carrier_name` from promotion result
3. Apply `DigitPosConditionNormalizer::normalize()` to break condition AST
4. Use normalized AST in subsequent condition lowering

**Code Position** (around line 331):
```rust
ConditionPromotionResult::Promoted {
    carrier_info: promoted_carrier,
    promoted_var,
    carrier_name,
} => {
    // ... existing merge logic ...

    // Phase 224-E: Normalize digit_pos condition before lowering
    let normalized_break_condition = DigitPosConditionNormalizer::normalize(
        &break_condition_node,
        &promoted_var,
        &carrier_name,
    );

    // Use normalized_break_condition in subsequent processing
}
```

## Testing Strategy

### Unit Tests (3-4 tests)
1. **Happy path**: `digit_pos < 0` → `!is_digit_pos`
2. **Wrong operator**: `digit_pos >= 0` → No change
3. **Wrong variable**: `other_var < 0` → No change
4. **Wrong constant**: `digit_pos < 10` → No change

### E2E Test
**Test File**: `apps/tests/phase2235_p2_digit_pos_min.hako`
このテストは digit_pos 昇格と型整合性・SSA 安定性を確認するインフラ用途で、数値としての戻り値の意味論は今後の JsonParser 本体フェーズで定義する予定だよ。

**Success Criteria**:
- No type error ("unsupported compare Lt on Bool and Integer")
- Break condition correctly evaluates
- Loop exits when digit not found

**Debug Command**:
```bash
NYASH_JOINIR_DEBUG=1 ./target/release/hakorune apps/tests/phase2235_p2_digit_pos_min.hako
```

### Regression Tests
- Verify existing Trim tests still pass
- Verify skip_whitespace tests still pass
- Check that 877/884 test count is maintained

## Success Criteria

1. ✅ `cargo build --release` succeeds
2. ✅ Unit tests (3-4) pass
3. ✅ Type error eliminated from phase2235_p2_digit_pos_min.hako
4. ✅ Existing 877/884 tests remain passing
5. ✅ No regressions in Trim/skip_whitespace patterns

## Box-First Principles

### Single Responsibility
- DigitPosConditionNormalizer only handles AST transformation
- No side effects, no state mutation
- Pure pattern matching function

### Fail-Safe Design
- Non-matching patterns returned unchanged
- No panics, no errors
- Conservative transformation strategy

### Boundary Clarity
- Input: AST + promoted_var + carrier_name
- Output: Transformed or original AST
- Clear interface contract

## Future Extensions

**Pattern 4 Support**: When Pattern 4 (continue) needs similar normalization, the same box can be reused with continue condition AST.

**Other Comparison Operators**: Currently handles `< 0`. Could extend to:
- `>= 0` → `is_digit_pos` (no NOT)
- `!= -1` → `is_digit_pos`
- `== -1` → `!is_digit_pos`

**Multiple Conditions**: For complex boolean expressions with multiple promoted variables, apply normalization recursively.

## References

- **Phase 228-8**: DigitPos promotion implementation
- **Phase 229**: Dynamic condition variable resolution
- **CarrierInfo**: `src/mir/join_ir/lowering/carrier_info.rs`
- **ConditionEnv**: `src/mir/join_ir/lowering/condition_env.rs`
Status: Active  
Scope: digitpos condition 正規化（ExprLowerer ライン）
