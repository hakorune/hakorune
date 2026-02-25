# Phase 242-EX: Pattern3 Legacy Lowerer Analysis

## Summary
**Recommendation: Option A - Delete Legacy Lowerer**

The Pattern3 legacy lowerer (`loop_with_if_phi_minimal.rs`) contains hardcoded carrier names ("sum", "count") that violate the by-name hardcoding prohibition. However, investigation reveals that **the entire lowerer is a Phase 188 PoC** hardcoded for one specific test case, making partial fixes insufficient.

## Problem Analysis

### Hardcoded Carrier Names
**File**: `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs:423-424`

```rust
// Phase 213: Build ExitMeta for dynamic exit binding generation
let mut exit_values = vec![];
exit_values.push(("sum".to_string(), sum_final));      // ← hardcoded "sum"
exit_values.push(("count".to_string(), count_final));  // ← hardcoded "count"
```

### Full Scope of Hardcoding

The legacy lowerer is **completely hardcoded** for `loop_if_phi.hako`:

1. **Loop condition**: `i <= 5` (lines 218-243)
2. **If condition**: `i % 2 == 1` (lines 256-289)
3. **If branches**: `sum + i` vs `sum + 0` (lines 291-335)
4. **Counter updates**: `count + 1` vs `count + 0` (lines 301-345)
5. **Carrier names**: "sum", "count" (lines 423-424)
6. **Carrier count**: Exactly 2 carriers (hardcoded allocation)

### Architecture Issue

**AST-based Path (Pattern 3):**
- Uses `LoopScopeShapeBuilder::empty_body_locals()` → `carriers: BTreeSet::new()`
- `CarrierInfo` is populated from `variable_map`
- But `LoopScopeShape.carriers` remains **empty**

**LoopForm-based Path:**
- Uses `LoopScopeShapeBuilder::from_loopform_intake()` → populates carriers from LoopFormIntake
- This path is for structured loop lowering, not applicable to AST-based MIR building

**Result**: `scope.carriers` is empty → Option B (extract from LoopScopeShape) fails.

## Option Evaluation

### Option A: Delete Legacy Lowerer ✅ **RECOMMENDED**
**Status**: Feasible if if-sum mode can be enhanced.

#### Current if-sum Mode Rejection Logic
File: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs:194-210`

```rust
pub fn is_if_sum_pattern(&self) -> bool {
    // (a) Pattern check: must be SimpleComparison
    let pattern = analyze_condition_pattern(condition);
    if pattern != ConditionPattern::SimpleComparison {
        // Complex condition → legacy mode (PoC lowering)
        return false;  // ← Rejects i % 2 == 1
    }
    // ...
}
```

**Reason for Rejection**: `i % 2 == 1` has `BinaryOp` in LHS → `ConditionPattern::Complex`

File: `src/mir/join_ir/lowering/condition_pattern.rs:79-125`

```rust
pub fn analyze_condition_pattern(cond: &ASTNode) -> ConditionPattern {
    match cond {
        ASTNode::BinaryOp { operator, left, right, .. } => {
            // Check LHS/RHS patterns
            let left_is_var = matches!(left.as_ref(), ASTNode::Variable { .. });
            let right_is_literal = matches!(right.as_ref(), ASTNode::Literal { .. });

            if left_is_var && right_is_literal {
                return ConditionPattern::SimpleComparison;  // e.g., i > 0
            }

            // Complex LHS/RHS (e.g., i % 2 == 1) → ConditionPattern::Complex
            ConditionPattern::Complex  // ← i % 2 is BinaryOp, not Variable
        }
        _ => ConditionPattern::Complex,
    }
}
```

#### Enhancement Strategy

**Approach**: Extend if-sum mode to lower complex if conditions dynamically.

1. **Accept complex conditions**: Remove `SimpleComparison` check
2. **Lower condition AST**: Use existing AST-based lowering infrastructure
3. **Generate Select instruction**: Use condition result to select between branches

**Implementation Changes**:
- `condition_pattern.rs`: Add `ConditionPattern::ComplexComparison` (still a comparison, but with expressions)
- `loop_with_if_phi_if_sum.rs`: Lower complex condition expressions using `lower_expression()`
- `pattern_pipeline.rs`: Accept `ComplexComparison` in `is_if_sum_pattern()`

**Benefits**:
- Removes 438-line PoC file
- Unified lowering path (no legacy/if-sum split)
- No by-name hardcoding
- Supports arbitrary if conditions (not just `i % 2 == 1`)

**Risks**:
- Need to handle arbitrary expressions in if condition
- May require additional ValueId allocation in condition lowering

### Option B: Extract from LoopScopeShape ❌ **NOT VIABLE**
**Status**: Not feasible due to empty carriers.

**Problem**: `LoopScopeShape.carriers` is empty in AST-based path.

**Why it fails**:
```rust
// pattern_pipeline.rs:283-291
let loop_scope = match variant {
    PatternVariant::Pattern1 | PatternVariant::Pattern3 => {
        LoopScopeShapeBuilder::empty_body_locals(
            BasicBlockId(0), BasicBlockId(0), BasicBlockId(0), BasicBlockId(0),
            BTreeSet::new(),  // ← Empty pinned set
        )  // ← Returns LoopScopeShape { carriers: BTreeSet::new(), .. }
    }
    // ...
}
```

**Alternative**: Extract from `ctx.carrier_info.carriers` instead.

**Implementation**:
```rust
// pattern3_with_if_phi.rs:262 (in lower_pattern3_legacy)
let carrier_names: Vec<String> = ctx.carrier_info.carriers
    .iter()
    .map(|c| c.name.clone())
    .collect();

// Pass to legacy lowerer
let (join_module, fragment_meta) = lower_loop_with_if_phi_pattern(
    carrier_names,  // New parameter
    &mut join_value_space
)?;

// loop_with_if_phi_minimal.rs:421-424
let mut exit_values = vec![];
for (i, carrier_name) in carrier_names.iter().enumerate() {
    let value_id = match i {
        0 => sum_final,
        1 => count_final,
        _ => return Err(format!("Legacy lowerer only supports 2 carriers")),
    };
    exit_values.push((carrier_name.clone(), value_id));
}
```

**Why still not recommended**:
- Only fixes carrier names, not loop/if conditions
- Still a PoC implementation (hardcoded logic remains)
- Better to invest in Option A (proper generalization)

### Option C: Pass CarrierInfo ❌ **NOT RECOMMENDED**
**Status**: Same issues as Option B.

**Implementation**: Change legacy lowerer signature to accept `CarrierInfo`.

**Why not recommended**:
- Same fundamental issue: only fixes names, not logic
- Requires signature changes across multiple files
- Still doesn't generalize the PoC lowering

## Test Impact Analysis

### Tests Using Legacy Lowerer
**Only 1 test**: `apps/tests/loop_if_phi.hako`

```nyash
static box Main {
  main(args) {
    local console = new ConsoleBox()
    local i = 1
    local sum = 0
    loop(i <= 5) {
      if (i % 2 == 1) { sum = sum + i } else { sum = sum + 0 }
      i = i + 1
    }
    console.println("sum=" + sum)
    return 0
  }
}
```

**Why it uses legacy mode**: `i % 2 == 1` is a complex condition (BinaryOp in LHS).

**Other Pattern 3 tests**: None found using `rg --type rust 'loop_if_phi\.hako'`.

### Tests Using If-Sum Mode
**Tests using Pattern 3 if-sum mode**: Minimal (mostly `phase212_if_sum_min.hako`).

**Pattern 3 detection**: File `src/mir/loop_pattern_detection/loop_patterns.rs`

```rust
pub fn detect_pattern3_if_phi(body: &[ASTNode]) -> bool {
    // Has if-else with PHI, no break/continue
    has_if_else_phi(body) && !has_break_continue(body)
}
```

## Implementation Plan

### Recommended: Option A - Enhanced If-Sum Mode

#### Phase 242-EX-A1: Extend Condition Pattern Analysis
**File**: `src/mir/join_ir/lowering/condition_pattern.rs`

1. Add `ConditionPattern::ComplexComparison`:
   ```rust
   pub enum ConditionPattern {
       SimpleComparison,      // var CmpOp literal (e.g., i > 0)
       ComplexComparison,     // expr CmpOp expr (e.g., i % 2 == 1)
       Complex,               // Non-comparison (e.g., a && b)
   }
   ```

2. Update `analyze_condition_pattern()`:
   ```rust
   pub fn analyze_condition_pattern(cond: &ASTNode) -> ConditionPattern {
       match cond {
           ASTNode::BinaryOp { operator, .. } => {
               let is_comparison = matches!(operator, /* ... */);
               if !is_comparison {
                   return ConditionPattern::Complex;
               }
               // Complex comparison (expr CmpOp expr)
               ConditionPattern::ComplexComparison
           }
           _ => ConditionPattern::Complex,
       }
   }
   ```

#### Phase 242-EX-A2: Accept Complex Comparisons in If-Sum Mode
**File**: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`

```rust
pub fn is_if_sum_pattern(&self) -> bool {
    // ...
    let pattern = analyze_condition_pattern(condition);
    if pattern == ConditionPattern::Complex {
        // Non-comparison (e.g., a && b) → legacy mode
        return false;
    }
    // Accept both SimpleComparison and ComplexComparison
    // ...
}
```

#### Phase 242-EX-A3: Lower Complex Conditions in If-Sum Lowerer
**File**: `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`

**Current**: Only lowers simple comparisons via `ConditionEnv`.

**Enhanced**: Lower arbitrary condition expressions:

```rust
// For complex conditions (e.g., i % 2 == 1):
// 1. Lower LHS expression (i % 2)
let lhs_vid = lower_expression_to_joinir(if_condition.lhs, env, body)?;
// 2. Lower RHS expression (1)
let rhs_vid = lower_expression_to_joinir(if_condition.rhs, env, body)?;
// 3. Generate Compare instruction
let cond_vid = alloc_value();
body.push(JoinInst::Compute(MirLikeInst::Compare {
    dst: cond_vid,
    op: map_compare_op(if_condition.operator),
    lhs: lhs_vid,
    rhs: rhs_vid,
}));
```

**Infrastructure needed**:
- `lower_expression_to_joinir()` helper (similar to `lower_comparison()`)
- Handle `BinaryOp`, `Variable`, `Literal` recursively
- Allocate temporary ValueIds for intermediate results

#### Phase 242-EX-A4: Delete Legacy Lowerer
**Files to delete**:
- `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs` (entire file, 438 lines)

**Files to modify**:
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`:
  - Remove `lower_pattern3_legacy()` function
  - Remove dual-mode dispatch in `cf_loop_pattern3_with_if_phi()`
  - Always use if-sum mode
- `src/mir/builder/control_flow/joinir/patterns/mod.rs`:
  - Remove `pub mod loop_with_if_phi_minimal;`

#### Phase 242-EX-A5: Verify Tests
```bash
# Build
cargo build --release

# Test original case
./target/release/hakorune apps/tests/loop_if_phi.hako
# Expected output: sum=9

# Run full test suite
cargo test --release
# Expected: 909 tests pass
```

## Success Criteria

1. ✅ `cargo build --release` success
2. ✅ All 909 tests pass
3. ✅ `loop_if_phi.hako` runs correctly (output: `sum=9`)
4. ✅ No hardcoded "sum"/"count" references
5. ✅ Legacy lowerer deleted (438 lines removed)
6. ✅ If-sum mode handles complex conditions

## Timeline Estimate

- **Phase 242-EX-A1**: 30 minutes (condition pattern analysis)
- **Phase 242-EX-A2**: 15 minutes (pattern acceptance)
- **Phase 242-EX-A3**: 2-3 hours (complex condition lowering)
- **Phase 242-EX-A4**: 15 minutes (deletion)
- **Phase 242-EX-A5**: 30 minutes (testing)

**Total**: 3.5-4.5 hours

## References

- **Phase 188**: Original legacy lowerer implementation
- **Phase 195**: Multi-carrier support (hardcoded "sum", "count")
- **Phase 213**: ExitMeta-based exit binding generation
- **Phase 219**: AST-based if-sum mode introduction
- **Phase 222**: Condition normalization
- **Phase 241**: Removed hardcoded 'sum' check from loop body analyzer

## Related Files

### Core Legacy Lowerer
- `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs` (438 lines)

### Callers
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs:248-358` (legacy path)

### Condition Analysis
- `src/mir/join_ir/lowering/condition_pattern.rs` (pattern detection)
- `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs:194-229` (is_if_sum_pattern)

### If-Sum Mode
- `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs` (AST-based lowerer)

### Tests
- `apps/tests/loop_if_phi.hako` (only test using legacy mode)
Status: Historical  
Scope: 旧 lowerer 分析メモ（現役 ExprLowerer ラインとは別枠）
