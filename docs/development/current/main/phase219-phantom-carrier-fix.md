# Phase 219: Phantom Carrier Bug Fix

**Status**: ✅ Complete (2025-12-10)

## Problem

Name-based heuristic in `loop_update_summary.rs` created phantom carriers, blocking AST-based if-sum lowerer activation.

### Root Cause

```rust
// BAD: All variables from variable_map treated as carriers
let carrier_info = CarrierInfo::from_variable_map(loop_var, &variable_map)?;
// Result: variable_map includes non-assigned vars (e.g., function name "count")

// BAD: Name-based classification
if name.contains("count") {  // Phantom "count" → CounterLike
    UpdateKind::CounterLike
}
```

**Result**: Phantom "count" carrier detected → `counter_count() == 2` → `is_simple_if_sum_pattern() == false`

### Example

**phase212_if_sum_min.hako**:
```nyash
loop(i < len) {
    if i > 0 {
        sum = sum + 1  // Only 2 carriers: i, sum
    }
    i = i + 1
}
```

**Bug**: `variable_map` contains `{"i", "sum", "defs", "len"}` (4 vars)
→ Phantom carriers detected → AST lowerer never activates → RC=0 (fail)

## Solution

### Phase 219-1: Assignment-Based Detection

Only variables **actually assigned** in loop body are carriers:

```rust
fn extract_assigned_variables(loop_body: &[ASTNode]) -> HashSet<String> {
    // Walk AST, find all LHS of assignments
    // Handles nested if/loop statements
}
```

### Phase 219-2: RHS Structure Classification

Classify update kind by **RHS expression structure**, NOT name:

```rust
fn classify_update_kind_from_rhs(rhs: &ASTNode) -> UpdateKind {
    match rhs {
        ASTNode::BinaryOp { operator: Add, left, right, .. } => {
            if let ASTNode::Literal { value: Integer(1), .. } = right {
                UpdateKind::CounterLike  // x = x + 1
            } else {
                UpdateKind::AccumulationLike  // x = x + expr
            }
        }
        _ => UpdateKind::Other
    }
}
```

### Phase 219-3: Hybrid Name+Structure Heuristic

**Problem**: Both `i = i + 1` and `sum = sum + 1` match `x = x + 1` pattern.

**Solution**: Use variable name to distinguish loop index from accumulator:

```rust
if is_likely_loop_index(name) {  // i, j, k, idx, etc.
    UpdateKind::CounterLike
} else if rhs matches `x = x + 1` {
    UpdateKind::AccumulationLike  // sum, count, etc.
}
```

## Implementation

### Modified Files

**Primary**:
- `src/mir/join_ir/lowering/loop_update_summary.rs`:
  - Added `extract_assigned_variables()` - AST walker for LHS detection
  - Added `find_assignment_rhs()` - RHS extraction for classification
  - Added `classify_update_kind_from_rhs()` - Structure-based classification
  - Added `analyze_loop_updates_from_ast()` - New assignment-based API
  - Deprecated `analyze_loop_updates()` - Legacy name-based API

**Updated**:
- `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`:
  - Updated `is_if_sum_pattern()` to use `analyze_loop_updates_from_ast()`

## Test Results

### Phase 212: Basic If-Sum (SUCCESS ✅)

**Test**: `apps/tests/phase212_if_sum_min.hako`

**Before Phase 219**:
```
[Phase 219 DEBUG] carrier_names: ["i", "defs", "len", "sum"]
[Phase 219 DEBUG] counter_count = 2  (i + phantom "count")
[Phase 219 DEBUG] is_simple_if_sum_pattern = false  ❌
→ Falls back to legacy lowerer
→ RC=0 (FAIL)
```

**After Phase 219**:
```
[Phase 219 DEBUG] carrier_names: ["i", "defs", "len", "sum"]
[Phase 219 DEBUG] assigned_vars: {"i", "sum"}
[Phase 219 DEBUG] Final carriers: [("i", CounterLike), ("sum", AccumulationLike)]
[Phase 219 DEBUG] counter_count = 1
[Phase 219 DEBUG] accumulation_count = 1
[Phase 219 DEBUG] is_simple_if_sum_pattern = true  ✅
→ AST lowerer activates
→ (New error: Phase 214+ - variable 'len' in condition not yet supported)
```

**Key Improvement**:
- ✅ Phantom carriers eliminated
- ✅ AST lowerer activates correctly
- ✅ No regression in pattern detection

### Phase 218: JsonParser If-Sum (Blocked by Phase 214)

**Test**: `apps/tests/phase218_json_if_sum_min.hako`

**Status**: ⏳ Blocked by Phase 214+ (condition variable support)

Same phantom carrier fix applied, but hits Phase 214 limitation.

### Phase 217: Multi-Carrier (Not Tested - Out of Scope)

**Test**: `apps/tests/phase217_if_sum_multi_min.hako`

**Status**: ⏸️ Deferred (Phase 219 scope is phantom carrier fix only)

## Design Principles

### Box Responsibility: LoopUpdateSummary

**LoopUpdateSummary analyzes structure, not names.**

**Invariants**:
1. **No Phantom Carriers**: Only variables with actual assignments in loop body
2. **Assignment-Based Detection**: LHS variables from AST assignments only
3. **Structure-Based Classification**: RHS expression patterns (with name assist for disambiguation)

### API Contract

**New API** (Phase 219):
```rust
pub fn analyze_loop_updates_from_ast(
    carrier_names: &[String],  // Candidate carriers from scope
    loop_body: &[ASTNode],      // Loop body AST for assignment detection
) -> LoopUpdateSummary
```

**Legacy API** (Deprecated):
```rust
#[deprecated(since = "Phase 219", note = "Use analyze_loop_updates_from_ast() instead")]
pub fn analyze_loop_updates(carrier_names: &[String]) -> LoopUpdateSummary
```

## Known Limitations

### 1. Name Heuristic Still Used

**Issue**: `is_likely_loop_index()` uses name patterns (`i`, `j`, `k`, etc.)

**Rationale**: Both `i = i + 1` and `sum = sum + 1` match the same RHS pattern. Variable name is the only reliable distinguisher without full control flow analysis.

**Future**: Phase 220+ could analyze conditional vs. unconditional updates.

### 2. First Assignment Only

**Issue**: `find_assignment_rhs()` returns the first assignment RHS found.

**Rationale**: Sufficient for if-sum patterns where carriers have uniform update patterns.

**Future**: Phase 221+ could analyze multiple assignments for complex patterns.

### 3. Legacy Call Sites

**Issue**: 3 call sites still use deprecated `analyze_loop_updates()`:
- `case_a_lowering_shape.rs:302`
- `loop_view_builder.rs:79`
- `loop_pattern_detection/mod.rs:222`

**Status**: Deprecation warnings emitted. Migration deferred to avoid scope creep.

## Migration Guide

### For New Code

```rust
// ✅ Use new AST-based API
use crate::mir::join_ir::lowering::loop_update_summary::analyze_loop_updates_from_ast;

let summary = analyze_loop_updates_from_ast(&carrier_names, loop_body);
if summary.is_simple_if_sum_pattern() {
    // AST-based lowering
}
```

### For Legacy Code

```rust
// ⚠️ Deprecated - will be removed in future
use crate::mir::join_ir::lowering::loop_update_summary::analyze_loop_updates;

let summary = analyze_loop_updates(&carrier_names);  // No phantom detection
```

## Related Phases

- **Phase 213**: AST-based if-sum lowerer (dual-mode dispatch)
- **Phase 214**: Condition variable support (blocked by)
- **Phase 218**: JsonParser if-sum (beneficiary)
- **Phase 220**: Conditional update analysis (future enhancement)

## References

- **Architecture**: [joinir-architecture-overview.md](joinir-architecture-overview.md)
- **LoopUpdateSummary**: `src/mir/join_ir/lowering/loop_update_summary.rs`
- **Pattern Pipeline**: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`
Status: Active  
Scope: Phantom carrier 修正（JoinIR v2）
