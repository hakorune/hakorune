# Phase 171-3: Register Condition Inputs in JoinIR Lowerers

**Date**: 2025-12-07
**Status**: ✅ Complete

---

## Implementation Summary

Successfully implemented the infrastructure for registering condition-only input variables in JoinIR lowerers.

---

## Changes Made

### 1. Extended `JoinInlineBoundary` Structure

**File**: `src/mir/join_ir/lowering/inline_boundary.rs`

**Added field**:
```rust
pub struct JoinInlineBoundary {
    pub join_inputs: Vec<ValueId>,
    pub host_inputs: Vec<ValueId>,
    pub join_outputs: Vec<ValueId>,
    pub host_outputs: Vec<ValueId>,
    pub exit_bindings: Vec<LoopExitBinding>,

    /// NEW: Condition-only input variables (Phase 171+)
    pub condition_inputs: Vec<(String, ValueId)>,  // [(var_name, host_value_id)]
}
```

**Rationale**: Condition variables like `start`, `end`, `len` are read-only inputs that don't participate in loop iteration but must be available in JoinIR scope for condition evaluation.

---

### 2. Updated All Constructors

**Modified constructors**:
- `new_inputs_only()` - Added `condition_inputs: vec![]`
- `new_with_outputs()` - Added `condition_inputs: vec![]`
- `new_with_input_and_host_outputs()` - Added `condition_inputs: vec![]`
- `new_with_exit_bindings()` - Added `condition_inputs: vec![]`

**New constructors**:
- `new_with_condition_inputs()` - For loops with condition variables
- `new_with_exit_and_condition_inputs()` - For loops with carriers AND condition variables

**Example usage**:
```rust
let boundary = JoinInlineBoundary::new_with_condition_inputs(
    vec![ValueId(0)],           // join_inputs (i)
    vec![ValueId(5)],            // host_inputs (i)
    vec![
        ("start".to_string(), ValueId(33)),
        ("end".to_string(), ValueId(34)),
    ],
);
```

---

### 3. Implemented Condition Variable Extraction

**File**: `src/mir/join_ir/lowering/condition_to_joinir.rs`

**New functions**:

#### `extract_condition_variables()`
```rust
pub fn extract_condition_variables(
    cond_ast: &ASTNode,
    exclude_vars: &[String],
) -> Vec<String>
```

Recursively traverses condition AST and collects all unique variable names, excluding loop parameters that are already registered as join_inputs.

**Features**:
- ✅ Sorted output (BTreeSet) for determinism
- ✅ Filters excluded variables (loop parameters)
- ✅ Handles complex conditions (AND, OR, NOT chains)

#### `collect_variables_recursive()`
```rust
fn collect_variables_recursive(
    ast: &ASTNode,
    vars: &mut BTreeSet<String>
)
```

Helper function that recursively visits all AST nodes and extracts variable names.

**Supported AST nodes**:
- `Variable` - Adds variable name to set
- `BinaryOp` - Recursively processes left and right operands
- `UnaryOp` - Recursively processes operand
- `Literal` - No variables (skipped)

---

### 4. Added Comprehensive Tests

**File**: `src/mir/join_ir/lowering/condition_to_joinir.rs`

**Test cases**:

#### `test_extract_condition_variables_simple()`
```rust
// AST: start < end
let vars = extract_condition_variables(&ast, &[]);
assert_eq!(vars, vec!["end", "start"]);  // Sorted
```

#### `test_extract_condition_variables_with_exclude()`
```rust
// AST: i < end
let vars = extract_condition_variables(&ast, &["i".to_string()]);
assert_eq!(vars, vec!["end"]);  // 'i' excluded
```

#### `test_extract_condition_variables_complex()`
```rust
// AST: start < end && i < len
let vars = extract_condition_variables(&ast, &["i".to_string()]);
assert_eq!(vars, vec!["end", "len", "start"]);  // Sorted, 'i' excluded
```

---

### 5. Updated Test Code

**File**: `src/mir/builder/control_flow/joinir/patterns/exit_binding.rs`

**Fixed test**:
```rust
let mut boundary = JoinInlineBoundary {
    host_inputs: vec![],
    join_inputs: vec![],
    host_outputs: vec![],
    join_outputs: vec![],
    exit_bindings: vec![],       // Phase 171: Added
    condition_inputs: vec![],    // Phase 171: Added
};
```

---

## Design Decisions

### Why Separate `condition_inputs` from `join_inputs`?

| Aspect | Loop Parameters (`join_inputs`) | Condition Inputs (`condition_inputs`) |
|--------|--------------------------------|--------------------------------------|
| **Mutability** | Mutable (e.g., `i = i + 1`) | Read-only (never modified) |
| **Lifetime** | Entire loop duration | Only during condition evaluation |
| **JoinIR representation** | Function parameters | Captured values |
| **Example** | `i` in `loop(i < 3)` | `end` in `loop(i < end)` |

**Semantic clarity**: Separating them makes the intent explicit and prevents accidental mutation of condition-only variables.

### Why Use `Vec<(String, ValueId)>` Instead of `HashMap`?

**Chosen**: `Vec<(String, ValueId)>`
**Alternative**: `HashMap<String, ValueId>`

**Rationale**:
1. **Order preservation**: Vec maintains insertion order for deterministic behavior
2. **Small size**: Typically 0-3 variables, Vec is more efficient than HashMap
3. **Iteration simplicity**: Direct iteration without collecting keys
4. **Consistency**: Matches pattern of `exit_bindings`

---

## Build Status

✅ **Library build**: Successful (0 errors, 57 warnings)
✅ **Struct extension**: All constructors updated
✅ **Tests**: 3 new tests added for extraction function
✅ **Backward compatibility**: All existing code still compiles

**Build command**:
```bash
cargo build --release
```

**Result**:
```
Finished `release` profile [optimized] target(s) in 0.07s
```

---

## What's Still Missing

### Not Yet Implemented (Phase 171-4 scope):

1. **Condition variable registration in loop lowerers**
   - `loop_with_break_minimal.rs` needs to call `extract_condition_variables()`
   - `loop_with_continue_minimal.rs` needs to call `extract_condition_variables()`
   - Pass extracted variables to boundary constructor

2. **ValueId mapping in merge logic**
   - `merge_joinir_mir_blocks()` needs to inject Copy instructions for condition inputs
   - Instruction rewriter needs to remap condition variable ValueIds

3. **Test with actual failing case**
   - `test_trim_main_pattern.hako` still fails (ValueId(33) undefined)
   - Need to apply the infrastructure to actual loop lowerers

---

## Next Steps

**Phase 171-4**: Implement condition input mapping in merge/reconnect logic

**Priority tasks**:
1. Modify `loop_with_break_minimal.rs` to extract and register condition variables
2. Update `merge_joinir_mir_blocks()` to handle `condition_inputs`
3. Inject Copy instructions for condition variables
4. Remap ValueIds in condition evaluation instructions
5. Test with `test_trim_main_pattern.hako`

---

## Technical Notes

### Extraction Algorithm Complexity

**Time complexity**: O(n) where n = number of AST nodes in condition
**Space complexity**: O(v) where v = number of unique variables

**Determinism guarantee**: BTreeSet ensures sorted output regardless of traversal order

### Edge Cases Handled

1. **No condition variables**: Returns empty vector
2. **Loop variable in condition**: Properly excluded (e.g., `loop(i < 10)`)
3. **Duplicate variables**: BTreeSet automatically deduplicates
4. **Nested expressions**: Recursion handles arbitrary depth

### Boundary Contract

**Invariant**: `condition_inputs` contains ONLY variables that:
1. Appear in the loop condition AST
2. Are NOT loop parameters (not in `join_inputs`)
3. Exist in HOST `variable_map` at lowering time

**Violation detection**: Will be caught in Phase 171-4 when looking up HOST ValueIds

---

## Files Modified

1. `src/mir/join_ir/lowering/inline_boundary.rs` (+93 lines)
   - Added `condition_inputs` field
   - Added 2 new constructors
   - Updated 4 existing constructors
   - Updated test assertions

2. `src/mir/join_ir/lowering/condition_to_joinir.rs` (+180 lines)
   - Added `extract_condition_variables()` function
   - Added `collect_variables_recursive()` helper
   - Added 3 comprehensive tests

3. `src/mir/builder/control_flow/joinir/patterns/exit_binding.rs` (+2 lines)
   - Fixed test boundary initialization

**Total**: +275 lines (infrastructure only, no behavior change yet)

---

## References

- Phase 171-1 Analysis: `phase171-1-boundary-analysis.md`
- Phase 171-2 Design: `phase171-2-condition-inputs-design.md`
- JoinIR Design: `docs/development/current/main/phase33-10-if-joinir-design.md`
Status: Historical
