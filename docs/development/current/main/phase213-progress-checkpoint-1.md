# Phase 213: Progress Checkpoint 1

**Date**: 2025-12-09
**Status**: ✅ Foundation Complete, Ready for Lowerer Refactoring
**Commit**: d7805e59

---

## 🎯 Completed Work

### ✅ Task 213-2-2: PatternPipelineContext Extension

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`

**Changes**:
1. Added new fields for Pattern 3:
   - `loop_condition: Option<ASTNode>` - Loop condition AST
   - `loop_body: Option<Vec<ASTNode>>` - Loop body AST
   - `loop_update_summary: Option<LoopUpdateSummary>` - Update expressions

2. Updated `build_pattern_context()` for Pattern3:
   - Stores `condition.clone()` in `loop_condition`
   - Stores `body.to_vec()` in `loop_body`
   - Placeholder `None` for `loop_update_summary` (TODO: Task 213-2-4)

3. Updated test cases with new fields

### ✅ Task 213-2-3: CarrierUpdateInfo Extension

**File**: `src/mir/join_ir/lowering/loop_update_summary.rs`

**Changes**:
1. Extended `CarrierUpdateInfo` struct:
   - `then_expr: Option<ASTNode>` - Then branch update expression
   - `else_expr: Option<ASTNode>` - Else branch update expression

2. Updated `analyze_loop_updates()`:
   - Default `None` for `then_expr`/`else_expr`
   - Comment: "Will be populated by Pattern 3 analyzer"

---

## 📊 Current State Analysis

### MIR Analysis for `phase212_if_sum_min.hako`

**Expected vs Actual**:

| Element | Expected | Actual (Hardcoded) |
|---------|----------|-------------------|
| Loop limit | `i < 3` | `i <= 5` |
| Loop cond | `icmp Lt %8, %18` | `icmp Le %8, %18` |
| If cond | `i > 0` | `i % 2 == 1` |
| If cond code | `icmp Gt %8, %zero` | `%8 Mod %2; icmp Eq ... %1` |
| Then update | `sum + 1` | `sum + i` |
| Then code | `%9 Add %const_1` | `%9 Add %8` |

**Root Cause**: `loop_with_if_phi_minimal.rs` lines 217-385 are completely hardcoded for `loop_if_phi.hako` test pattern.

---

## 🔄 Next Steps (3 Approaches)

### Approach A: Full AST-Based Generalization (Phase 213 Original Plan)

**Tasks**:
1. Create `Pattern3IfAnalyzer` module
2. Extract if statement from loop body
3. Parse then/else branches for carrier updates
4. Populate `LoopUpdateSummary` with AST expressions
5. Replace hardcoded conditions in lowerer
6. Replace hardcoded updates in lowerer
7. Return `ExitMeta` instead of hardcoded ValueIds

**Pros**: Complete generalization, supports all if-sum patterns
**Cons**: Large scope, ~500-800 lines of new code

### Approach B: Minimal Incremental (80/20 Rule)

**Tasks**:
1. Just replace the 3 hardcoded constants:
   - Loop limit: 5 → extract from AST condition
   - If condition: `i % 2 == 1` → extract from AST if statement
   - Update value: `i` → extract from AST assignment
2. Keep existing structure, minimal changes

**Pros**: Small scope, fast to implement, gets `phase212_if_sum_min.hako` working
**Cons**: Still not fully generic, will need refactoring later

### Approach C: Hybrid - BoolExprLowerer First

**Tasks**:
1. Focus on condition lowering only (loop + if)
2. Keep update expressions hardcoded for now
3. Use existing `condition_to_joinir` infrastructure
4. Defer update generalization to Phase 214

**Pros**: Leverages existing infrastructure, cleaner architecture
**Cons**: `phase212_if_sum_min.hako` still won't work fully

---

## 💭 Analysis & Recommendation

### Key Insight from Phase 212.5

Phase 212.5 discovered that:
1. Pattern routing works correctly (✅ Pattern 3 detected)
2. MIR PHI generation works (✅ `%31 = phi` created)
3. **Only the lowerer's hardcoded values are wrong**

This means the JoinIR → MIR pipeline is solid. We just need to feed it the right JoinIR.

### Recommendation: Approach B (Minimal Incremental)

**Rationale**:
1. **User's 80/20 philosophy**: "完璧より進捗" (progress over perfection)
2. **Fail-Fast principle**: Get `phase212_if_sum_min.hako` working first
3. **Box Theory**: Make minimal, reversible change
4. **Evidence-based**: Phase 212.5 proved the pipeline works

**Implementation Plan**:
1. Extract loop condition from `ctx.loop_condition` AST
2. Extract if condition from `ctx.loop_body` if statement
3. Extract update expression from if-then assignment
4. Replace 3 hardcoded sections in `loop_with_if_phi_minimal.rs`
5. Test with `phase212_if_sum_min.hako` → RC=2 ✅
6. Keep existing tests working (backward compatibility)

**Estimated effort**: 2-3 hours (vs 8-10 hours for Approach A)

---

## 🔍 Technical Details for Approach B

### What Needs to Change

**File**: `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs`

**Section 1: Loop Condition (lines 217-233)**
```rust
// Before (hardcoded):
loop_step_func.body.push(JoinInst::Compute(MirLikeInst::Const {
    dst: const_5,
    value: ConstValue::Integer(5),  // ← Hardcoded!
}));
loop_step_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
    dst: cmp_le,
    op: CompareOp::Le,  // ← Hardcoded!
    lhs: i_param,
    rhs: const_5,
}));

// After (AST-based):
// Extract from ctx.loop_condition: "i < 3"
// Lower to JoinIR using existing infrastructure
```

**Section 2: If Condition (lines 254-288)**
```rust
// Before (hardcoded):
// const 2, mod, const 1, eq → (i % 2 == 1)

// After (AST-based):
// Extract from loop_body if statement: "i > 0"
// Lower to JoinIR: const 0, cmp Gt
```

**Section 3: Update Expression (lines 290-298)**
```rust
// Before (hardcoded):
loop_step_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
    dst: sum_then,
    op: BinOpKind::Add,
    lhs: sum_param,
    rhs: i_param,  // ← Hardcoded! Should be const 1
}));

// After (AST-based):
// Extract from then-branch assignment: "sum = sum + 1"
// Lower to JoinIR: sum_param Add const_1
```

### Helper Functions Needed

```rust
// Extract loop condition details
fn extract_loop_condition(condition: &ASTNode)
    -> Result<(CompareOp, i64), String>

// Extract if statement from loop body
fn extract_if_statement(body: &[ASTNode])
    -> Result<&ASTNode, String>

// Extract if condition details
fn extract_if_condition(if_node: &ASTNode)
    -> Result<(CompareOp, i64), String>

// Extract update value from assignment
fn extract_update_value(assignment: &ASTNode)
    -> Result<i64, String>
```

---

## 📋 Decision Point

**Question for user**: Which approach should we take?

A. Full AST-based generalization (Approach A)
B. Minimal incremental replacement (Approach B) ← **Recommended**
C. Hybrid BoolExprLowerer-first (Approach C)
D. Different approach?

**Current blockers**: None - foundation is complete
**Current branch**: main (`d7805e59`)
**Build status**: ✅ Passing
Status: Active  
Scope: If-sum 進捗チェックポイント（JoinIR v2）
