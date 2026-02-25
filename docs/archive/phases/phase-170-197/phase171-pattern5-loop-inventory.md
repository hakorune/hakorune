# Phase 171-A: Blocked Loop Inventory

**Date**: 2025-12-07
**Status**: Initial inventory complete
**Purpose**: Identify loops blocked by LoopBodyLocal variables in break conditions

---

## Overview

This document catalogs loops that cannot be lowered by Pattern 2/4 because they use **LoopBodyLocal** variables in their break conditions. These are candidates for Pattern 5 carrier promotion.

---

## Blocked Loops Found

### 1. TrimTest.trim/1 - Leading Whitespace Trim

**File**: `local_tests/test_trim_main_pattern.hako`
**Lines**: 20-27

```hako
loop(start < end) {
    local ch = s.substring(start, start+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        start = start + 1
    } else {
        break
    }
}
```

**Blocking Variable**: `ch` (LoopBodyLocal)

**Analysis**:
- Loop parameter: `start` (LoopParam)
- Outer variable: `end` (OuterLocal)
- Break condition uses: `ch` (LoopBodyLocal)
- `ch` is defined inside loop body as `s.substring(start, start+1)`

**Error Message**:
```
[trace:debug] pattern2: Pattern 2 lowerer failed: Variable 'ch' not bound in ConditionEnv
```

**Why Blocked**:
Pattern 2 expects break conditions to only use:
- LoopParam (`start`)
- OuterLocal (`end`, `s`)

But the break condition `ch == " " || ...` uses `ch`, which is defined inside the loop body.

---

### 2. TrimTest.trim/1 - Trailing Whitespace Trim

**File**: `local_tests/test_trim_main_pattern.hako`
**Lines**: 30-37

```hako
loop(end > start) {
    local ch = s.substring(end-1, end)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        end = end - 1
    } else {
        break
    }
}
```

**Blocking Variable**: `ch` (LoopBodyLocal)

**Analysis**:
- Loop parameter: `end` (LoopParam)
- Outer variable: `start` (OuterLocal)
- Break condition uses: `ch` (LoopBodyLocal)
- `ch` is defined inside loop body as `s.substring(end-1, end)`

**Same blocking reason as Loop 1.**

---

### 3. JsonParserBox - MethodCall in Condition

**File**: `tools/hako_shared/json_parser.hako`

```hako
loop(i < s.length()) {
    // ...
}
```

**Blocking Issue**: `s.length()` is a MethodCall in the condition expression.

**Error Message**:
```
[ERROR] ❌ MIR compilation error: [cf_loop/pattern4] Lowering failed:
Unsupported expression in value context: MethodCall {
    object: Variable { name: "s", ... },
    method: "length",
    arguments: [],
    ...
}
```

**Why Blocked**:
Pattern 4's value context lowering doesn't support MethodCall expressions yet.

**Note**: This is not a LoopBodyLocal issue, but a MethodCall limitation. May be addressed in Phase 171-D (Optional).

---

## Pattern5-A Target Decision

### Selected Target: TrimTest Loop 1 (Leading Whitespace)

We select the **first loop** from TrimTest as Pattern5-A target for the following reasons:

1. **Clear structure**: Simple substring + equality checks
2. **Representative**: Same pattern as many real-world parsers
3. **Self-contained**: Doesn't depend on complex outer state
4. **Testable**: Easy to write unit tests

### Pattern5-A Specification

**Loop Structure**:
```hako
loop(start < end) {
    local ch = s.substring(start, start+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        start = start + 1
    } else {
        break
    }
}
```

**Variables**:
- `start`: LoopParam (carrier, mutated)
- `end`: OuterLocal (condition-only)
- `s`: OuterLocal (used in body)
- `ch`: LoopBodyLocal (blocking variable)

**Break Condition**:
```
!(ch == " " || ch == "\t" || ch == "\n" || ch == "\r")
```

---

## Promotion Strategy: Design D (Evaluated Bool Carrier)

### Rationale

We choose **Design D** (Evaluated Bool Carrier) over other options:

**Why not carry `ch` directly?**
- `ch` is a StringBox, not a primitive value
- Would require complex carrier type system
- Would break existing Pattern 2/4 assumptions

**Design D approach**:
- Introduce a new carrier: `is_whitespace` (bool)
- Evaluate `ch == " " || ...` in loop body
- Store result in `is_whitespace` carrier
- Use `is_whitespace` in break condition

### Transformed Structure

**Before (Pattern5-A)**:
```hako
loop(start < end) {
    local ch = s.substring(start, start+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        start = start + 1
    } else {
        break
    }
}
```

**After (Pattern2 compatible)**:
```hako
// Initialization (before loop)
local is_whitespace = true  // Initial assumption

loop(start < end && is_whitespace) {
    local ch = s.substring(start, start+1)
    is_whitespace = (ch == " " || ch == "\t" || ch == "\n" || ch == "\r")

    if is_whitespace {
        start = start + 1
    } else {
        break  // Now redundant, but kept for clarity
    }
}
```

**Key transformations**:
1. Add `is_whitespace` carrier initialization
2. Update loop condition to include `is_whitespace`
3. Compute `is_whitespace` in loop body
4. Original if-else becomes simpler (could be optimized away)

---

## Next Steps (Phase 171-C)

### Phase 171-C-1: Skeleton Implementation ✅
- Create `LoopBodyCarrierPromoter` box
- Define `PromotionRequest` / `PromotionResult` types
- Implement skeleton `try_promote()` method
- Add `find_definition_in_body()` helper

### Phase 171-C-2: Trim Pattern Promotion Logic
- Detect substring + equality pattern
- Generate `is_whitespace` carrier
- Generate initialization statement
- Generate update statement

### Phase 171-C-3: Integration with Pattern 2/4
- Call `LoopBodyCarrierPromoter::try_promote()` in routing
- If promotion succeeds, route to Pattern 2
- If promotion fails, return UnsupportedPattern

### Phase 171-D: MethodCall Support (Optional)
- Handle `s.length()` in loop conditions
- May require carrier promotion for method results
- Lower priority than Trim pattern

---

## Summary

**Blocked Loops**:
- 2 loops in TrimTest (LoopBodyLocal `ch`)
- 1+ loops in JsonParser (MethodCall in condition)

**Pattern5-A Target**:
- TrimTest leading whitespace trim loop
- Clear, representative, testable

**Promotion Strategy**:
- Design D: Evaluated Bool Carrier
- Transform `ch` checks → `is_whitespace` carrier
- Make compatible with Pattern 2

**Implementation Status**:
- Phase 171-A: ✅ Inventory complete
- Phase 171-B: ✅ Target selected
- Phase 171-C-1: ✅ Skeleton implementation complete
- Phase 171-C-2: ✅ Trim pattern detection implemented
  - `find_definition_in_body()`: AST traversal for variable definitions
  - `is_substring_method_call()`: Detects `substring()` method calls
  - `extract_equality_literals()`: Extracts string literals from OR chains
  - `TrimPatternInfo`: Captures pattern details for carrier promotion
- Phase 171-C-3: ✅ Integration with Pattern 2/4 routing complete
- Phase 171-C-4: ✅ CarrierInfo integration complete (2025-12-07)
  - `CarrierInfo::merge_from()`: Deduplicated carrier merging with deterministic sorting
  - `TrimPatternInfo::to_carrier_info()`: Conversion to CarrierInfo with TrimLoopHelper
  - Pattern 2/4 lowerers: Promoted carrier merging in `Promoted` branch
  - 7 unit tests: Merge success/failure/duplication/determinism validation
- Phase 171-C-5: ✅ TrimLoopHelper design complete (2025-12-07)
  - `TrimLoopHelper` struct: Encapsulates Trim pattern lowering logic
  - `CarrierInfo::trim_helper()`: Accessor for pattern-specific helper
  - Module export: `mod.rs` updated with `pub use TrimLoopHelper`
  - 4 unit tests: Helper creation and accessor validation

---

## Phase 171-C-3/4/5: Responsibility Positions and Data Flow

### Responsibility Separation Principle

The promotion system follows Box Theory's single responsibility principle:

1. **router.rs**: Pattern table + `can_lower()`/`lower()` call abstraction (no Scope/condition logic)
2. **Pattern 2/4 lowerer**: Holds LoopScope / ConditionScope / CarrierInfo / Promoter
3. **LoopBodyCarrierPromoter**: LoopBodyLocal handling specialist box
4. **TrimLoopHelper**: Trim pattern-specific helper (future extensibility)

### Data Flow Diagram

```
LoopConditionScopeBox::analyze()
        ↓
    has_loop_body_local()?
        ↓ true
LoopBodyCarrierPromoter::try_promote()
        ↓ Promoted { trim_info }
    TrimPatternInfo::to_carrier_info()
        ↓
    CarrierInfo::merge_from()
        ↓
    TrimLoopHelper (attached to CarrierInfo)
        ↓
    Pattern 2/4 lowerer (JoinIR generation)
```

### Implementation Locations

**Phase 171-C-4 Changes**:
- `src/mir/join_ir/lowering/carrier_info.rs`: Added `merge_from()`, `trim_helper()`, `trim_helper` field
- `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`: Updated `to_carrier_info()` to attach TrimLoopHelper
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`: Promoted branch now merges carriers
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`: Promoted branch now merges carriers

**Phase 171-C-5 Changes**:
- `src/mir/loop_pattern_detection/trim_loop_helper.rs`: NEW - TrimLoopHelper struct with 4 unit tests
- `src/mir/loop_pattern_detection/mod.rs`: Export TrimLoopHelper module

---

## Phase 171-impl-Trim: Trim 特例の実戦投入

**Date**: 2025-12-08
**Status**: ✅ Validation complete
**Purpose**: Safely integrate Trim pattern detection into JoinIR pipeline with validation-only implementation

### 設計原則

> **「LoopBodyLocal を全面解禁」ではなく、Trim パターンだけを箱経由でホワイトリストに載せる**

### 安全機構

1. `TrimLoopHelper::is_safe_trim()` - 構造的に安全か判定
2. `TrimLoopHelper::is_trim_like()` - Trim パターンに合致するか判定
3. `TrimLoopHelper::has_valid_structure()` - 構造チェック

### データフロー（Trim 特例）

```
LoopConditionScopeBox::analyze()
        ↓
    has_loop_body_local() == true
        ↓
LoopBodyCarrierPromoter::try_promote()
        ↓ Promoted { trim_info }
    TrimPatternInfo::to_carrier_info()
        ↓
    CarrierInfo::merge_from()
        ↓
    carrier_info.trim_helper()?.is_safe_trim()
        ↓ true
    ✅ Validation Success (TODO: JoinIR lowering in Phase 172)
```

### 実装状況

- [x] Phase 171-impl-Trim-1: 受け入れ条件を 1 箇所に ✅
  - `TrimLoopHelper::is_safe_trim()` implemented
  - `Pattern2/4` で Trim 特例ルート実装
  - Fail-Fast on unsafe patterns
- [x] Phase 171-impl-Trim-2: TrimLoopHelper 判定メソッド ✅
  - `is_trim_like()`, `has_valid_structure()` implemented
  - 4+ ユニットテスト追加 (9 tests total, all passing)
- [x] Phase 171-impl-Trim-3: E2E テスト ✅
  - `local_tests/test_trim_main_pattern.hako` validated
  - 出力: `[pattern2/trim] Safe Trim pattern detected`
  - 出力: `✅ Trim pattern validation successful!`
  - Status: Validation-only (JoinIR lowering deferred to Phase 172)
- [x] Phase 171-impl-Trim-4: ドキュメント更新 ✅
  - `phase171-pattern5-loop-inventory.md` updated
  - `CURRENT_TASK.md` status tracking

### 実装詳細

**ファイル変更**:
1. `src/mir/loop_pattern_detection/trim_loop_helper.rs`
   - `is_safe_trim()`, `is_trim_like()`, `has_valid_structure()` methods
   - 4 new unit tests for safety validation

2. `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
   - Trim exception route with safety check
   - `body_locals` extraction from loop body AST
   - Validation message for successful detection

3. `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`
   - Same Trim exception logic as Pattern2
   - `body_locals` extraction from normalized loop body

4. `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`
   - `ASTNode::Local { variables, initial_values, .. }` handler
   - Support for `local ch = expr` pattern recognition

### テスト結果

**ユニットテスト**: ✅ 9/9 passing
- `test_is_safe_trim`
- `test_is_safe_trim_empty_carrier`
- `test_is_safe_trim_no_whitespace`
- `test_has_valid_structure`
- (+ 5 existing tests)

**E2E テスト**: ✅ Validation successful
```
[pattern2/check] Analyzing condition scope: 3 variables
[pattern2/check]   'ch': LoopBodyLocal
[pattern2/promoter] LoopBodyLocal 'ch' promoted to carrier 'is_ch_match'
[pattern2/trim] Safe Trim pattern detected, bypassing LoopBodyLocal restriction
[pattern2/trim] Carrier: 'is_ch_match', original var: 'ch', whitespace chars: ["\r", "\n", "\t", " "]
✅ Trim pattern validation successful! Carrier 'is_ch_match' ready for Phase 172 implementation.
(Pattern detection: PASS, Safety check: PASS, JoinIR lowering: TODO)
```

**ビルド結果**: ✅ Success
- `cargo build --release`: Success
- `cargo test --lib trim_loop_helper`: 9/9 passing

### 重要な発見

1. **AST構造の理解**: `local ch = expr` は `ASTNode::Local { variables, initial_values, .. }` として表現される
2. **body_locals 抽出**: Pattern2/4 で `LoopScopeShape.body_locals` を AST から抽出する必要があった
3. **段階的実装**: Validation-only approach で安全性を先に確立し、JoinIR lowering は Phase 172 に分離

### 次のステップ (Phase 172)

- [ ] Phase 172-1: Trim pattern JoinIR generation
  - Carrier initialization code
  - Carrier update logic (substring + OR chain → bool)
  - Exit PHI mapping
- [ ] Phase 172-2: JsonParser loops への展開
  - Similar pattern recognition
  - Generalized carrier promotion
Status: Historical
