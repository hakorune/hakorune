# Phase 170: ValueId Boundary Mapping Analysis

**Date**: 2025-12-07
**Status**: Root Cause Identified
**Impact**: CRITICAL - Blocks all JsonParserBox complex condition tests

## Problem Summary

JoinIR loop patterns with complex conditions (e.g., `start < end` in `_trim`) compile successfully but fail silently at runtime because condition variable ValueIds are not properly mapped between HOST and JoinIR contexts.

## Symptoms

```
[ssa-undef-debug] fn=TrimTest.trim/1 bb=BasicBlockId(12) inst_idx=0 used=ValueId(33) inst=Compare { dst: ValueId(26), op: Lt, lhs: ValueId(33), rhs: ValueId(34) }
[ssa-undef-debug] fn=TrimTest.trim/1 bb=BasicBlockId(12) inst_idx=0 used=ValueId(34) inst=Compare { dst: ValueId(26), op: Lt, lhs: ValueId(33), rhs: ValueId(34) }
```

- Condition uses undefined ValueIds (33, 34) for variables `start` and `end`
- Program compiles but produces no output (silent runtime failure)
- PHI nodes also reference undefined carrier values

## Root Cause

### Architecture Overview

The JoinIR merge process uses two separate ValueId "namespaces":

1. **HOST context**: Main MirBuilder's ValueId space (e.g., `start = ValueId(33)`)
2. **JoinIR context**: Fresh ValueId allocator starting from 0 (e.g., `ValueId(0), ValueId(1), ...`)

The `JoinInlineBoundary` mechanism is supposed to bridge these two spaces by injecting Copy instructions at the entry block.

### The Bug

**Location**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs` (and other patterns)

**Current boundary creation**:
```rust
let boundary = JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0)],      // JoinIR's main() parameter (loop variable init)
    vec![loop_var_id],     // Host's loop variable
);
```

**What's missing**: Condition variables (`start`, `end`) are NOT in the boundary!

**What happens**:
1. `condition_to_joinir.rs` looks up variables in `builder.variable_map`:
   ```rust
   builder.variable_map.get("start") // Returns ValueId(33) from HOST
   builder.variable_map.get("end")   // Returns ValueId(34) from HOST
   ```

2. JoinIR instructions are generated with these HOST ValueIds:
   ```rust
   JoinInst::Compute(MirLikeInst::Compare {
       dst: ValueId(26),
       op: Lt,
       lhs: ValueId(33),  // HOST ValueId, not in JoinIR space!
       rhs: ValueId(34),  // HOST ValueId, not in JoinIR space!
   })
   ```

3. During merge, `remap_values()` only remaps ValueIds that are in `used_values`:
   - ValueIds 0, 1, 2, ... from JoinIR → new ValueIds from builder
   - **But ValueId(33) and ValueId(34) are not in JoinIR's used_values set!**
   - They're HOST ValueIds that leaked into JoinIR space

4. Result: Compare instruction references undefined ValueIds

## Architectural Issue

The current design has a conceptual mismatch:

- **`condition_to_joinir.rs`** assumes it can directly reference HOST ValueIds from `builder.variable_map`
- **JoinIR merge** assumes all ValueIds come from JoinIR's fresh allocator
- **Boundary mechanism** only maps explicitly listed inputs/outputs

This works for simple patterns where:
- Condition is hardcoded (e.g., `i < 3`)
- All condition values are constants or loop variables already in the boundary

This breaks when:
- Condition references variables from HOST context (e.g., `start < end`)
- Those variables are not in the boundary inputs

## Affected Code Paths

### Phase 169 Integration: `condition_to_joinir.rs`

Lines 183-189:
```rust
ASTNode::Variable { name, .. } => {
    builder
        .variable_map
        .get(name)
        .copied()
        .ok_or_else(|| format!("Variable '{}' not found in variable_map", name))
}
```

This returns HOST ValueIds directly without checking if they need boundary mapping.

### Pattern Lowerers: `pattern2_with_break.rs`, etc.

Pattern lowerers create minimal boundaries that only include:
- Loop variable (e.g., `i`)
- Accumulator (if present)

But NOT:
- Variables referenced in loop condition (e.g., `start`, `end` in `start < end`)
- Variables referenced in loop body expressions

### Merge Infrastructure: `merge/mod.rs`

The merge process has no way to detect that HOST ValueIds have leaked into JoinIR instructions.

## Test Case: `TrimTest.trim/1`

**Code**:
```nyash
local start = 0
local end = s.length()

loop(start < end) {  // Condition references start, end
    // ...
    break
}
```

**Expected boundary**:
```rust
JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0), ValueId(1), ValueId(2)],  // loop var, start, end
    vec![loop_var_id, start_id, end_id],        // HOST ValueIds
)
```

**Actual boundary**:
```rust
JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0)],      // Only loop var
    vec![loop_var_id],     // Only loop var
)
```

**Result**: `start` and `end` are undefined in JoinIR space

## Solutions

### Option A: Extract Condition Variables into Boundary (Recommended)

**Where**: Pattern lowerers (pattern1/2/3/4)

**Steps**:
1. Before calling `lower_condition_to_joinir()`, analyze AST to find all variables
2. For each variable, get HOST ValueId from `builder.variable_map`
3. Allocate JoinIR-side ValueIds (e.g., ValueId(1), ValueId(2))
4. Create boundary with all condition variables:
   ```rust
   let cond_vars = extract_condition_variables(condition_ast, builder);
   let boundary = JoinInlineBoundary::new_inputs_only(
       vec![ValueId(0), ValueId(1), ValueId(2)],  // loop var + cond vars
       vec![loop_var_id, cond_vars[0], cond_vars[1]],
   );
   ```

**Pros**:
- Minimal change to existing architecture
- Clear separation: boundary handles HOST↔JoinIR mapping
- Works for all condition complexity

**Cons**:
- Need to implement variable extraction from AST
- Each pattern needs updating

### Option B: Delay Variable Resolution Until Merge

**Where**: `condition_to_joinir.rs`

**Idea**: Instead of resolving variables immediately, emit placeholder instructions and resolve during merge.

**Pros**:
- Cleaner separation: JoinIR doesn't touch HOST ValueIds

**Cons**:
- Major refactoring required
- Need new placeholder instruction type
- Complicates merge logic

### Option C: Use Variable Names Instead of ValueIds in JoinIR

**Where**: JoinIR instruction format

**Idea**: JoinIR uses variable names (strings) instead of ValueIds, resolve during merge.

**Pros**:
- Most robust solution
- Eliminates ValueId namespace confusion

**Cons**:
- Breaks current JoinIR design (uses MirLikeInst which has ValueIds)
- Major architectural change

## Recommendation

**Option A** - Extract condition variables and add to boundary.

**Implementation Plan**:

1. **Create AST variable extractor** (30 minutes)
   - File: `src/mir/builder/control_flow/joinir/patterns/cond_var_extractor.rs`
   - Function: `extract_condition_variables(ast: &ASTNode, builder: &MirBuilder) -> Vec<(String, ValueId)>`
   - Recursively walk AST, collect all Variable nodes

2. **Update Pattern2** (1 hour)
   - Extract condition variables before calling pattern lowerer
   - Create boundary with extracted variables
   - Test with `TrimTest.trim/1`

3. **Update Pattern1, Pattern3, Pattern4** (1 hour each)
   - Apply same pattern

4. **Validation** (30 minutes)
   - Re-run `TrimTest.trim/1` → should output correctly
   - Re-run JsonParserBox tests → should work

**Total Estimate**: 4.5 hours

## Files Affected

**New**:
- `src/mir/builder/control_flow/joinir/patterns/cond_var_extractor.rs` (new utility)

**Modified**:
- `src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

## Related Issues

- **Phase 169**: BoolExprLowerer integration exposed this issue
- **Phase 166**: JsonParserBox validation blocked by this bug
- **Phase 188-189**: Boundary mechanism exists but incomplete

## Next Steps

1. Implement Option A (condition variable extraction)
2. Update all 4 patterns
3. Re-run Phase 170 validation tests
4. Document the "always include condition variables in boundary" pattern
Status: Historical
