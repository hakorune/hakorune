# Case C Analysis: loop(true) + break/continue Pattern

**Date**: 2025-12-14
**Phase**: 131-11 (Case C本命タスク)
**Status**: 🔍 Root Cause Analysis Complete

## Executive Summary

**Problem**: `loop(true) { ... break ... continue }` fails JoinIR pattern matching
**Root Cause**: Loop variable extraction expects comparison operators, not boolean literals
**Pattern Gap**: Need a dedicated “infinite loop + early exit” pattern (avoid “Pattern5” naming collision with existing Trim/P5), or a carefully-scoped Pattern2 extension.

---

## Test Case

**File**: `apps/tests/llvm_stage3_loop_only.hako`

```nyash
static box Main {
  main() {
    local counter = 0
    loop (true) {
      counter = counter + 1
      if counter == 3 { break }
      continue
    }
    print("Result: " + counter)
    return 0
  }
}
```

**Error**:
```
❌ MIR compilation error: [joinir/freeze] Loop lowering failed:
   JoinIR does not support this pattern, and LoopBuilder has been removed.
Function: main
Hint: This loop pattern is not supported. All loops must use JoinIR lowering.
```

---

## Root Cause Analysis

### 1. Pattern Detection Flow

**Current Flow** (for `loop(true)`):
```
1. LoopPatternContext::new() extracts features from AST
   ├─ has_break = true (✅ detected)
   ├─ has_continue = true (✅ detected)
   └─ pattern_kind = Pattern4Continue (❌ WRONG - should be Pattern2 or new Pattern5)

2. Pattern 4 (Continue) detect function called
   └─ Tries to extract loop variable from condition
      └─ extract_loop_variable_from_condition(BoolLiteral(true))
         └─ ❌ FAILS - expects BinaryOp comparison, not boolean literal

3. Pattern falls through, no pattern matches
   └─ Returns Ok(None) - "No pattern matched"
      └─ Main router calls freeze() error
```

**Why it fails**:
- **Location**: `src/mir/builder/control_flow/utils.rs:25-52`
- **Function**: `extract_loop_variable_from_condition()`
- **Expected**: Binary comparison like `i < 3`
- **Actual**: Boolean literal `true`
- **Result**: Error "Unsupported loop condition pattern"

### 2. Pattern Classification Issue

**Classification logic** (`src/mir/loop_pattern_detection/mod.rs:276-305`):

```rust
pub fn classify(features: &LoopFeatures) -> LoopPatternKind {
    // Pattern 4: Continue (highest priority)
    if features.has_continue {
        return LoopPatternKind::Pattern4Continue;  // ← Case C goes here
    }

    // Pattern 3: If-PHI (check before Pattern 1)
    if features.has_if && features.carrier_count >= 1
        && !features.has_break && !features.has_continue {
        return LoopPatternKind::Pattern3IfPhi;
    }

    // Pattern 2: Break
    if features.has_break && !features.has_continue {
        return LoopPatternKind::Pattern2Break;
    }

    // Pattern 1: Simple While
    if !features.has_break && !features.has_continue && !features.has_if {
        return LoopPatternKind::Pattern1SimpleWhile;
    }

    LoopPatternKind::Unknown
}
```

**Problem**: Case C has `has_continue = true`, so it routes to Pattern 4, but:
- Pattern 4 expects a loop variable in condition (e.g., `i < 10`)
- `loop(true)` has no loop variable - it's an infinite loop

---

## Pattern Coverage Gap

### Current Patterns (4 total)

| Pattern | Condition Type | Break | Continue | Notes |
|---------|---------------|-------|----------|-------|
| Pattern 1 | Comparison (`i < 3`) | ❌ | ❌ | Simple while loop |
| Pattern 2 | Comparison (`i < 3`) | ✅ | ❌ | Loop with conditional break |
| Pattern 3 | Comparison (`i < 3`) | ❌ | ❌ | Loop with if-else PHI |
| Pattern 4 | Comparison (`i < 3`) | ❌ | ✅ | Loop with continue |

### Missing Pattern: Infinite Loop with Early Exit

**Case C characteristics**:
- **Condition**: Boolean literal (`true`) - infinite loop
- **Break**: ✅ Yes (exit condition inside loop)
- **Continue**: ✅ Yes (skip iteration)
- **Carrier**: Single variable (`counter`)

**Pattern Gap**: None of the 4 patterns handle infinite loops!

---

## Implementation Options

### Option A: Pattern 2 Extension (Break-First Variant)

**Idea**: Extend Pattern 2 to handle both:
- `loop(i < 3) { if cond { break } }` (existing)
- `loop(true) { if cond { break } }` (new)

**Changes**:
1. Modify `extract_loop_variable_from_condition()` to handle boolean literals
   - Return special token like `"__infinite__"` for `loop(true)`
2. Update Pattern 2 to skip loop variable PHI when `loop_var == "__infinite__"`
3. Use break condition as the only exit mechanism

**Pros**:
- Minimal code changes (1 file: `utils.rs`)
- Reuses existing Pattern 2 infrastructure
- Matches semantic similarity (both use `break` for exit)

**Cons**:
- Couples two different loop forms (finite vs infinite)
- Special-case handling (`__infinite__` token) is a code smell
- Pattern 2 assumes loop variable exists in multiple places

---

### Option B: New Pattern 5 (Infinite Loop with Early Exit)

**Idea**: Create dedicated Pattern 5 for infinite loops

**Structure**:
```rust
// src/mir/builder/control_flow/joinir/patterns/pattern5_infinite_break.rs

/// Pattern 5: Infinite Loop with Early Exit
///
/// Handles:
/// - loop(true) { ... break ... }
/// - loop(true) { ... continue ... }
/// - loop(true) { ... break ... continue ... }
///
/// Key differences from Pattern 2:
/// - No loop variable in condition
/// - Break condition is the ONLY exit mechanism
/// - Continue jumps to top (no loop variable increment)
pub fn can_lower(builder: &MirBuilder, ctx: &LoopPatternContext) -> bool {
    // Check 1: Condition must be boolean literal `true`
    matches!(ctx.condition, ASTNode::BoolLiteral { value: true, .. })
    // Check 2: Must have break statement
    && ctx.has_break
}

pub fn lower(builder: &mut MirBuilder, ctx: &LoopPatternContext)
    -> Result<Option<ValueId>, String> {
    // Similar to Pattern 2, but:
    // - No loop variable PHI
    // - Break condition becomes the only exit test
    // - Continue jumps directly to loop header
}
```

**Changes**:
1. Add `pattern5_infinite_break.rs` (new file, ~200 lines)
2. Register in `patterns/mod.rs` and `patterns/router.rs`
3. Update `classify()` to add Pattern 5 before Pattern 4:
   ```rust
   // Pattern 5: Infinite loop (highest priority after continue)
   if matches!(condition, ASTNode::BoolLiteral { value: true, .. })
       && (features.has_break || features.has_continue) {
       return LoopPatternKind::Pattern5InfiniteLoop;
   }
   ```

**Pros**:
- **Fail-Fast principle**: Clear separation of concerns
- Independent testability (Pattern 5 doesn't affect Pattern 2)
- Easy to extend for `loop(true)` without break (future Pattern 6?)
- Matches Box Theory modularization philosophy

**Cons**:
- More code (~200 lines new file)
- Duplicates some logic from Pattern 2 (break condition extraction)

---

### Option C: Pattern Classification Fix (Recommended)

**Idea**: Fix the classification logic to route `loop(true) + break` to Pattern 2, and add infinite loop support there

**Changes**:

1. **Step 1**: Add `is_infinite_loop` feature to `LoopFeatures`
   ```rust
   // src/mir/loop_pattern_detection/mod.rs
   pub struct LoopFeatures {
       pub has_break: bool,
       pub has_continue: bool,
       pub has_if: bool,
       pub has_if_else_phi: bool,
       pub carrier_count: usize,
       pub break_count: usize,
       pub continue_count: usize,
       pub is_infinite_loop: bool,  // NEW: true for loop(true)
       pub update_summary: Option<LoopUpdateSummary>,
   }
   ```

2. **Step 2**: Detect infinite loop in `ast_feature_extractor.rs`
   ```rust
   // src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs
   pub(crate) fn extract_features(
       condition: &ASTNode,  // NEW: need condition for infinite loop detection
       body: &[ASTNode],
       has_continue: bool,
       has_break: bool
   ) -> LoopFeatures {
       let is_infinite_loop = matches!(condition, ASTNode::BoolLiteral { value: true, .. });
       // ... rest of extraction
       LoopFeatures {
           has_break,
           has_continue,
           // ... other fields
           is_infinite_loop,
           // ...
       }
   }
   ```

3. **Step 3**: Update classification priority
   ```rust
   // src/mir/loop_pattern_detection/mod.rs:classify()
   pub fn classify(features: &LoopFeatures) -> LoopPatternKind {
       // PRIORITY FIX: Infinite loop with break -> Pattern 2
       // (check BEFORE Pattern 4 Continue)
       if features.is_infinite_loop && features.has_break && !features.has_continue {
           return LoopPatternKind::Pattern2Break;
       }

       // Pattern 4: Continue
       if features.has_continue {
           // Infinite loop with continue -> needs special handling
           if features.is_infinite_loop {
               // Option: Create Pattern6InfiniteLoopContinue
               // For now: return Unknown to fail fast
               return LoopPatternKind::Unknown;
           }
           return LoopPatternKind::Pattern4Continue;
       }

       // ... rest of classification
   }
   ```

4. **Step 4**: Make Pattern 2 handle infinite loops
   ```rust
   // src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs

   // In can_lower():
   pub fn can_lower(builder: &MirBuilder, ctx: &LoopPatternContext) -> bool {
       // Check if classified as Pattern 2
       ctx.pattern_kind == LoopPatternKind::Pattern2Break
   }

   // In lower():
   pub fn lower(builder: &mut MirBuilder, ctx: &LoopPatternContext)
       -> Result<Option<ValueId>, String> {

       // Check if infinite loop
       let is_infinite = matches!(ctx.condition, ASTNode::BoolLiteral { value: true, .. });

       if is_infinite {
           // Infinite loop path: no loop variable extraction
           // Use break condition as the only exit
           return lower_infinite_loop_with_break(builder, ctx);
       } else {
           // Existing finite loop path
           return lower_finite_loop_with_break(builder, ctx);
       }
   }

   fn lower_infinite_loop_with_break(...) -> Result<Option<ValueId>, String> {
       // Similar to existing Pattern 2, but:
       // - Skip loop variable extraction
       // - Skip loop variable PHI
       // - Generate infinite loop header (always jump to body)
       // - Break condition becomes the only exit test
   }
   ```

**Pros**:
- **Reuses Pattern 2 infrastructure** (break condition extraction, exit routing)
- **Clear separation via helper function** (`lower_infinite_loop_with_break`)
- **Fail-Fast for unsupported cases** (`loop(true) + continue` returns Unknown)
- **Incremental implementation** (can add Pattern 6 for continue later)

**Cons**:
- Pattern 2 becomes more complex (2 lowering paths)
- Need to update 3+ files (features, classifier, pattern2)

---

## Recommended Approach: Option C

**Rationale**:
1. **Semantic similarity**: `loop(true) { break }` and `loop(i < 3) { break }` both use break as the primary exit mechanism
2. **Code reuse**: Break condition extraction, exit routing, boundary application all the same
3. **Fail-Fast**: Explicitly returns Unknown for `loop(true) + continue` (Case C variant)
4. **Incremental**: Can add Pattern 6 for `loop(true) + continue` in future phase

**Implementation Steps** (next section)

---

## Implementation Plan (Option C)

### Phase 131-11-A: Feature Detection

**Files Modified**:
1. `src/mir/loop_pattern_detection/mod.rs` - Add `is_infinite_loop` field
2. `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs` - Detect `loop(true)`
3. `src/mir/builder/control_flow/joinir/patterns/router.rs` - Pass condition to extract_features

**Changes**:
```diff
// LoopFeatures struct
pub struct LoopFeatures {
    pub has_break: bool,
    pub has_continue: bool,
    pub has_if: bool,
    pub has_if_else_phi: bool,
    pub carrier_count: usize,
    pub break_count: usize,
    pub continue_count: usize,
+   pub is_infinite_loop: bool,
    pub update_summary: Option<LoopUpdateSummary>,
}

// extract_features signature
- pub(crate) fn extract_features(body: &[ASTNode], has_continue: bool, has_break: bool) -> LoopFeatures
+ pub(crate) fn extract_features(condition: &ASTNode, body: &[ASTNode], has_continue: bool, has_break: bool) -> LoopFeatures

// In LoopPatternContext::new()
- let features = ast_features::extract_features(body, has_continue, has_break);
+ let features = ast_features::extract_features(condition, body, has_continue, has_break);
```

### Phase 131-11-B: Classification Priority Fix

**File Modified**: `src/mir/loop_pattern_detection/mod.rs`

**Change**:
```rust
pub fn classify(features: &LoopFeatures) -> LoopPatternKind {
    // NEW: Infinite loop with break -> Pattern 2 (BEFORE Pattern 4 check!)
    if features.is_infinite_loop && features.has_break && !features.has_continue {
        return LoopPatternKind::Pattern2Break;
    }

    // Pattern 4: Continue (existing)
    if features.has_continue {
        if features.is_infinite_loop {
            // Fail-Fast: loop(true) + continue not supported yet
            return LoopPatternKind::Unknown;
        }
        return LoopPatternKind::Pattern4Continue;
    }

    // ... rest of classification unchanged
}
```

### Phase 131-11-C: Pattern 2 Infinite Loop Lowering

**File Modified**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

**Changes**:
1. Add `is_infinite_loop()` helper
2. Split `lower()` into two paths
3. Implement `lower_infinite_loop_with_break()`

**Pseudo-code**:
```rust
pub fn lower(builder: &mut MirBuilder, ctx: &LoopPatternContext)
    -> Result<Option<ValueId>, String> {

    if is_infinite_loop(ctx.condition) {
        lower_infinite_loop_with_break(builder, ctx)
    } else {
        lower_finite_loop_with_break(builder, ctx)  // existing code
    }
}

fn is_infinite_loop(condition: &ASTNode) -> bool {
    matches!(condition, ASTNode::BoolLiteral { value: true, .. })
}

fn lower_infinite_loop_with_break(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext
) -> Result<Option<ValueId>, String> {
    // Similar to existing Pattern 2, but:
    // 1. Skip loop variable extraction (no loop_var_name)
    // 2. Skip loop variable PHI (no counter increment)
    // 3. Loop header unconditionally jumps to body (no condition check)
    // 4. Break condition becomes the only exit test
    // 5. Carriers are still tracked and merged at exit
}
```

---

## Test Strategy

### Unit Tests

**File**: `src/mir/loop_pattern_detection/mod.rs` (tests module)

```rust
#[test]
fn test_classify_infinite_loop_with_break() {
    let features = LoopFeatures {
        has_break: true,
        has_continue: false,
        is_infinite_loop: true,
        // ... other fields
    };
    assert_eq!(classify(&features), LoopPatternKind::Pattern2Break);
}

#[test]
fn test_classify_infinite_loop_with_continue_unsupported() {
    let features = LoopFeatures {
        has_break: false,
        has_continue: true,
        is_infinite_loop: true,
        // ... other fields
    };
    assert_eq!(classify(&features), LoopPatternKind::Unknown);
}
```

### Integration Tests

**Case C Variants**:

1. **Minimal** (`/tmp/case_c_minimal.hako`):
   ```nyash
   static box Main {
     main() {
       local i = 0
       loop (true) {
         i = i + 1
         if i == 3 { break }
       }
       print(i)
       return 0
     }
   }
   ```
   Expected: Prints `3`

2. **With Continue** (`apps/tests/llvm_stage3_loop_only.hako`):
   ```nyash
   static box Main {
     main() {
       local counter = 0
       loop (true) {
         counter = counter + 1
         if counter == 3 { break }
         continue
       }
       print("Result: " + counter)
       return 0
     }
   }
   ```
   Expected: MIR compile error (Fail-Fast - not supported yet)

3. **Multi-Carrier** (future test):
   ```nyash
   loop (true) {
     i = i + 1
     sum = sum + i
     if sum > 10 { break }
   }
   ```

---

## Migration Path

### Phase 131-11: Infinite Loop with Break (Priority 1)

**Goal**: Make Case C (minimal) compile and run

**Tasks**:
1. ✅ Phase 131-11-A: Feature Detection (is_infinite_loop)
2. ✅ Phase 131-11-B: Classification Priority Fix
3. ✅ Phase 131-11-C: Pattern 2 Infinite Loop Lowering
4. ✅ Unit tests for classification
5. ✅ Integration test: `/tmp/case_c_minimal.hako`
6. ✅ LLVM end-to-end test (EMIT + LINK + RUN)

**Success Criteria**:
- Case C (minimal) passes VM ✅
- Case C (minimal) passes LLVM AOT ✅
- Case C (with continue) fails with clear error ✅

### Phase 131-12: Infinite Loop with Continue (Priority 2)

**Goal**: Support `loop(true) + continue` (Case C full variant)

**Approach**: Create Pattern 6 or extend Pattern 4

**Not started yet** - pending Phase 131-11 completion

---

## SSOT Update

**File**: `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`

**Section to Add**:

```markdown
### 4. TAG-EMIT: JoinIR Pattern Coverage Gap (Case C)

**File**: `apps/tests/llvm_stage3_loop_only.hako`

**Code**:
```nyash
static box Main {
  main() {
    local counter = 0
    loop (true) {
      counter = counter + 1
      if counter == 3 { break }
      continue
    }
    print("Result: " + counter)
    return 0
  }
}
```

**MIR Compilation**: FAILURE
```
❌ MIR compilation error: [joinir/freeze] Loop lowering failed:
   JoinIR does not support this pattern, and LoopBuilder has been removed.
```

**Root Cause**:
- **Pattern Gap**: `loop(true)` (infinite loop) is not recognized by any of Patterns 1-4
- **Loop Variable Extraction Fails**: `extract_loop_variable_from_condition()` expects binary comparison (`i < 3`), not boolean literal (`true`)
- **Classification Priority Bug**: `has_continue = true` routes to Pattern 4, but Pattern 4 expects a loop variable

**Solution** (Phase 131-11):
- Add `is_infinite_loop` feature to `LoopFeatures`
- Update classification priority: infinite loop + break → Pattern 2
- Extend Pattern 2 to handle infinite loops (no loop variable PHI)

**Location**:
- Feature detection: `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs`
- Classification: `src/mir/loop_pattern_detection/mod.rs:classify()`
- Lowering: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

**Analysis**: [docs/development/current/main/case-c-infinite-loop-analysis.md](./case-c-infinite-loop-analysis.md)
```

---

## Box Theory Alignment

### Fail-Fast Principle ✅

- **Unsupported patterns return Unknown** (not fallback to broken code)
- **Clear error messages** ("JoinIR does not support this pattern")
- **No silent degradation** (LoopBuilder removed, no hidden fallback)

### Modular Boundaries ✅

- **Feature extraction**: Pure function, no MirBuilder dependency
- **Classification**: Centralized SSOT (`classify()` function)
- **Pattern lowering**: Isolated modules (pattern2_with_break.rs)

### Incremental Extension ✅

- **Phase 131-11**: Add infinite loop with break (Pattern 2 extension)
- **Phase 131-12**: Add infinite loop with continue (Pattern 6 new)
- **No regression risk**: Existing patterns unchanged

---

## References

### Related Files

**Pattern Detection**:
- `src/mir/loop_pattern_detection/mod.rs` - Classification logic
- `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs` - Feature extraction
- `src/mir/builder/control_flow/joinir/patterns/router.rs` - Pattern routing

**Pattern 2 Implementation**:
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs` - Break pattern lowering
- `src/mir/builder/control_flow/utils.rs` - Loop variable extraction

**Testing**:
- `apps/tests/llvm_stage3_loop_only.hako` - Case C test file
- `apps/tests/loop_min_while.hako` - Case B (working reference)

### Documentation

- **Phase 131-3 Inventory**: `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **Pattern Design**: `docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md`

---

## Appendix: AST Structure Comparison

### Case B: `loop(i < 3)` (Working)

```
Loop {
  condition: BinaryOp {
    operator: Less,
    left: Variable { name: "i" },
    right: IntLiteral { value: 3 }
  },
  body: [...]
}
```

**extract_loop_variable_from_condition()**: ✅ Returns `"i"`

### Case C: `loop(true)` (Failing)

```
Loop {
  condition: BoolLiteral { value: true },
  body: [...]
}
```

**extract_loop_variable_from_condition()**: ❌ Error "Unsupported loop condition pattern"

---

## Timeline Estimate

**Phase 131-11-A (Feature Detection)**: 30 minutes
- Add `is_infinite_loop` field to LoopFeatures
- Update extract_features signature
- Update callers (LoopPatternContext)

**Phase 131-11-B (Classification Fix)**: 15 minutes
- Update classify() priority order
- Add unit tests

**Phase 131-11-C (Pattern 2 Extension)**: 2-3 hours
- Implement `is_infinite_loop()` helper
- Implement `lower_infinite_loop_with_break()`
- Handle carriers without loop variable PHI
- Integration testing

**Total**: 3-4 hours to complete Phase 131-11

---

**Last Updated**: 2025-12-14
**Author**: Claude (Analysis Phase)
**Next Step**: Get user confirmation on Option C approach
