# Phase 215: ExprResult Exit Contract

## Overview

Phase 215 establishes a unified ExprResult Exit Contract to properly propagate loop computation results from JoinIR lowerers through to the final MIR return statement.

**Problem**: Loop computations (e.g., `sum=9` in if-sum pattern) execute correctly but return RC=0 instead of the expected value because `expr_result` gets discarded at the final return statement.

**Goal**: Make `expr_result` with `ParamRole::ExprResult` propagate consistently from JoinIR lowerers → ExitMeta → Boundary → ExitLine → MIR return statement.

## Current State Analysis

### Investigation Results (Phase 215 Task Agent)

#### Pattern 1 (Simple While)
- **File**: `src/mir/join_ir/lowering/simple_while_minimal.rs`
- **Return Type**: `Option<JoinModule>` (no JoinFragmentMeta)
- **expr_result Support**: ❌ None
- **Status**: Intentionally simple, no value propagation needed

#### Pattern 2 (Loop with Break)
- **File**: `src/mir/join_ir/lowering/loop_with_break_minimal.rs`
- **Return Type**: `(JoinModule, JoinFragmentMeta)`
- **expr_result Creation**: ✅ Line 502
  ```rust
  let fragment_meta = JoinFragmentMeta::with_expr_result(i_exit, exit_meta);
  ```
- **Boundary Passing**: ✅ Line 447
  ```rust
  .with_expr_result(fragment_meta.expr_result)
  ```
- **Final Return**: ❌ Lines 461-468 - **DISCARDS expr_result**
  ```rust
  // Phase 188-Impl-2: Return Void (loops don't produce values)
  // The subsequent 'return i' statement will emit its own Load + Return
  let void_val = crate::mir::builder::emission::constant::emit_void(self);
  trace::trace().debug("pattern2", &format!("Loop complete, returning Void {:?}", void_val));
  Ok(Some(void_val))
  ```
- **Root Cause**: Design assumes loops update `variable_map` and subsequent statements read from it

#### Pattern 3 (If-PHI)
- **File**: `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs`
- **Return Type**: `(JoinModule, JoinFragmentMeta)`
- **expr_result Creation**: ❌ Line 427 - **Uses carrier_only()**
  ```rust
  let fragment_meta = JoinFragmentMeta::carrier_only(exit_meta);
  ```
- **Boundary Passing**: ❌ No `.with_expr_result()` call (lines 176-180)
- **Final Return**: ❌ Lines 192-196 - Returns Void
  ```rust
  let void_val = crate::mir::builder::emission::constant::emit_void(self);
  trace::trace().debug("pattern3/if-sum", &format!("Loop complete, returning Void {:?}", void_val));
  Ok(Some(void_val))
  ```
- **Status**: **Primary target for Phase 215 fix**

#### Pattern 4 (Loop with Continue)
- **File**: `src/mir/join_ir/lowering/loop_with_continue_minimal.rs`
- **Return Type**: `(JoinModule, ExitMeta)` (no JoinFragmentMeta - **inconsistent API**)
- **expr_result Support**: ❌ None
- **Status**: API inconsistency, but not primary target

## Data Flow Diagram

### Current Flow (RC=0 Problem)

```
┌─────────────────────────────────────────────────────────────┐
│ JoinIR Lowerer (Pattern 3)                                  │
│                                                               │
│  lower_loop_with_if_phi_pattern()                           │
│    └─> JoinFragmentMeta::carrier_only(exit_meta)           │
│         ├─> exit_meta: ExitMeta {                           │
│         │     exit_values: {"sum" → ValueId(1008)}         │
│         │   }                                                │
│         └─> expr_result: None  ← ❌ PROBLEM HERE            │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Pattern3 Dispatcher (pattern3_with_if_phi.rs)              │
│                                                               │
│  lower_pattern3_if_sum() / lower_pattern3_legacy()          │
│    └─> JoinInlineBoundaryBuilder::new()                    │
│         ├─> .with_inputs(join_inputs, host_inputs)         │
│         ├─> .with_exit_bindings(exit_bindings)             │
│         └─> ❌ NO .with_expr_result() call                 │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ JoinIRConversionPipeline                                    │
│                                                               │
│  execute(builder, join_module, boundary, ...)               │
│    └─> exit_line_reconnector::reconnect_exit_lines()       │
│         └─> ExitLineReconnector::run()                     │
│              └─> Updates variable_map with exit PHIs       │
│                   {"sum" → ValueId(r456)}  ← Correct!      │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Pattern3 Final Return (pattern3_with_if_phi.rs)            │
│                                                               │
│  let void_val = emit_void(self);                            │
│  Ok(Some(void_val))  ← ❌ DISCARDS expr_result!            │
│                                                               │
│  Result: RC=0 (Void type)                                   │
└─────────────────────────────────────────────────────────────┘
```

### Target Flow (Phase 215 Goal)

```
┌─────────────────────────────────────────────────────────────┐
│ JoinIR Lowerer (Pattern 3)                                  │
│                                                               │
│  lower_loop_with_if_phi_pattern()                           │
│    └─> ✅ JoinFragmentMeta::with_expr_result(              │
│           sum_final_value,  ← "sum" carrier final value     │
│           exit_meta                                          │
│         )                                                    │
│         ├─> exit_meta: ExitMeta {                           │
│         │     exit_values: {"sum" → ValueId(1008)}         │
│         │   }                                                │
│         └─> expr_result: Some(ValueId(1008))  ← ✅ FIX 1   │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Pattern3 Dispatcher (pattern3_with_if_phi.rs)              │
│                                                               │
│  lower_pattern3_if_sum() / lower_pattern3_legacy()          │
│    └─> JoinInlineBoundaryBuilder::new()                    │
│         ├─> .with_inputs(join_inputs, host_inputs)         │
│         ├─> .with_exit_bindings(exit_bindings)             │
│         └─> ✅ .with_expr_result(fragment_meta.expr_result) │
│              ← FIX 2: Pass expr_result to boundary          │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ JoinIRConversionPipeline                                    │
│                                                               │
│  execute(builder, join_module, boundary, ...)               │
│    ├─> Merge blocks (allocates PHI nodes)                  │
│    │    └─> exit_phi_result_id = Some(ValueId(r999))      │
│    │         ↑                                              │
│    │         └─ This is the merged expr_result PHI!        │
│    │                                                         │
│    └─> exit_line_reconnector::reconnect_exit_lines()       │
│         └─> ExitLineReconnector::run()                     │
│              ├─> Updates variable_map with exit PHIs       │
│              │    {"sum" → ValueId(r456)}                  │
│              │                                               │
│              └─> ✅ Returns exit_phi_result_id             │
│                   Some(ValueId(r999))  ← FIX 3             │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Pattern3 Final Return (pattern3_with_if_phi.rs)            │
│                                                               │
│  let loop_result = JoinIRConversionPipeline::execute(...)?; │
│  if let Some(result_id) = loop_result {                    │
│      ✅ Ok(Some(result_id))  ← FIX 4: Return expr_result!  │
│  } else {                                                    │
│      let void_val = emit_void(self);                        │
│      Ok(Some(void_val))                                     │
│  }                                                           │
│                                                               │
│  Result: RC=2 (Integer value)                               │
└─────────────────────────────────────────────────────────────┘
```

## Where expr_result Gets Discarded

### Primary Discard Points

1. **Pattern 3 JoinIR Lowerer** (Line 427)
   - File: `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs`
   - Code: `JoinFragmentMeta::carrier_only(exit_meta)`
   - Effect: Sets `expr_result = None`, losing the loop result ValueId

2. **Pattern 3 Boundary Builder** (Lines 176-180)
   - File: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
   - Missing: `.with_expr_result()` call
   - Effect: Boundary doesn't know about expr_result

3. **Pattern 3 Final Return** (Lines 192-196, 312-316)
   - File: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
   - Code: `let void_val = emit_void(self); Ok(Some(void_val))`
   - Effect: Discards merge result, returns Void

### Secondary Discard Points

4. **Pattern 2 Final Return** (Lines 461-468)
   - File: `src/mir/join_ir/lowering/loop_with_break_minimal.rs`
   - Code: Same Void return pattern
   - Note: Pattern 2 correctly creates expr_result but also discards it

## Implementation Plan

### Task 215-2: Unify expr_result in Lowerers

**Pattern 3 Changes**:
1. Change `JoinFragmentMeta::carrier_only()` to `with_expr_result()` in both:
   - `loop_with_if_phi_minimal.rs` (legacy lowerer)
   - `loop_with_if_phi_if_sum.rs` (if-sum lowerer)
2. Identify which carrier represents the loop result (e.g., "sum" in if-sum pattern)
3. Pass that carrier's ValueId as expr_result

**Pattern 1/2 Audit**:
- Pattern 1: No changes needed (intentionally simple)
- Pattern 2: Already correct, use as reference implementation

### Task 215-3: Unify Boundary/ExitLine Handling

**Boundary Changes** (`pattern3_with_if_phi.rs`):
```rust
let boundary = JoinInlineBoundaryBuilder::new()
    .with_inputs(join_inputs, host_inputs)
    .with_exit_bindings(exit_bindings)
    .with_loop_var_name(Some(ctx.loop_var_name.clone()))
    .with_expr_result(fragment_meta.expr_result)  // ← ADD THIS
    .build();
```

**ExitLine Changes** (if needed):
- Verify `ExitLineReconnector` already handles expr_result correctly
- Current implementation returns `Option<ValueId>` from merge
- Should propagate `exit_phi_result_id` when expr_result exists

**Final Return Changes** (`pattern3_with_if_phi.rs`):
```rust
// Replace Void return with conditional expr_result return
let merge_result = JoinIRConversionPipeline::execute(...)?;

if let Some(result_id) = merge_result {
    // Loop produced a value (expr-position)
    Ok(Some(result_id))
} else {
    // Loop only updates variable_map (statement-position)
    let void_val = emit_void(self);
    Ok(Some(void_val))
}
```

### Task 215-4: Testing

**Primary Target**:
- `phase212_if_sum_min.hako` → Expected: RC=2 (sum value)

**Regression Tests**:
- `loop_if_phi.hako` → Expected: sum=9 in variable_map (statement-position)
- Multi-carrier tests → Expected: All carriers updated correctly
- Pattern 1/2/4 tests → Expected: No behavioral changes

**Verification**:
- MIR dump: Check final return uses correct ValueId
- JoinIRVerifier: Ensure no contract violations
- VM execution: Verify correct RC values

### Task 215-5: Documentation

**Update Files**:
1. `phase212-if-sum-impl.md` - Mark Phase 215 as complete
2. `joinir-architecture-overview.md` - Add ExprResult flow section
3. `CURRENT_TASK.md` - Record Phase 215 completion

## Design Contracts

### JoinFragmentMeta Contract

```rust
pub struct JoinFragmentMeta {
    pub expr_result: Option<ValueId>,  // Loop result for expr-position
    pub exit_meta: ExitMeta,           // Carrier exit values
}
```

**Usage Rules**:
- `carrier_only(exit_meta)`: Statement-position loops (updates variable_map only)
- `with_expr_result(value_id, exit_meta)`: Expr-position loops (returns value)

### ParamRole Contract

```rust
pub enum ParamRole {
    LoopParam,      // Loop variable (e.g., i)
    Condition,      // Loop condition result
    Carrier,        // Carrier value (e.g., sum)
    ExprResult,     // Final expression result ← New role
}
```

**Usage Rules**:
- `Carrier`: Updates `variable_map` via `exit_bindings`
- `ExprResult`: Returns from loop expression via merge result

### ExitLineReconnector Contract

**Input**: `JoinInlineBoundary` with optional `expr_result`
**Output**: `Option<ValueId>` (merge result for expr-position loops)

**Behavior**:
1. Update `variable_map` with all `exit_bindings` (Carrier role)
2. If `expr_result` exists, return `exit_phi_result_id` (ExprResult role)
3. Distinguish "carrier to variable_map" vs "expr_result to return"

## References

- **Phase 213**: AST-based if-sum lowering (dual-mode architecture)
- **Phase 214**: Dynamic join_inputs generation fix
- **Phase 188**: Original JoinIR pipeline design
- **Phase 33-16**: LoopHeaderPhi integration
Status: Active  
Scope: Expr result / exit contract 設計（JoinIR v2）
