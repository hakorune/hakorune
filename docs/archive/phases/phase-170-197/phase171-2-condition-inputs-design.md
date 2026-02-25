# Phase 171-2: Condition Inputs Metadata Design

**Date**: 2025-12-07
**Status**: Design Complete
**Decision**: **Option A - Extend JoinInlineBoundary**

---

## Design Decision: Option A

### Rationale

**Option A: Extend JoinInlineBoundary** (CHOSEN ✅)
```rust
pub struct JoinInlineBoundary {
    pub join_inputs: Vec<ValueId>,
    pub host_inputs: Vec<ValueId>,
    pub join_outputs: Vec<ValueId>,
    pub host_outputs: Vec<ValueId>,
    pub exit_bindings: Vec<LoopExitBinding>,

    // NEW: Condition-only inputs
    pub condition_inputs: Vec<(String, ValueId)>,  // [(var_name, host_value_id)]
}
```

**Why this is best**:
1. **Minimal invasiveness**: Single structure change
2. **Clear semantics**: "Condition inputs" are distinct from "loop parameters"
3. **Reuses existing infrastructure**: Same Copy injection mechanism
4. **Future-proof**: Easy to extend for condition-only outputs (if needed)
5. **Symmetric design**: Mirrors how `exit_bindings` handle exit values

**Rejected alternatives**:

**Option B: Create new LoopInputBinding**
```rust
pub struct LoopInputBinding {
    pub condition_vars: HashMap<String, ValueId>,
}
```
❌ **Rejected**: Introduces another structure; harder to coordinate with boundary

**Option C: Extend LoopExitBinding**
```rust
pub struct LoopExitBinding {
    pub condition_inputs: Vec<String>,  // NEW
    // ...
}
```
❌ **Rejected**: Semantic mismatch (exit bindings are for outputs, not inputs)

---

## Detailed Design

### 1. Extended Structure Definition

**File**: `src/mir/join_ir/lowering/inline_boundary.rs`

```rust
#[derive(Debug, Clone)]
pub struct JoinInlineBoundary {
    /// JoinIR-local ValueIds that act as "input slots"
    ///
    /// These are the ValueIds used **inside** the JoinIR fragment to refer
    /// to values that come from the host. They should be small sequential
    /// IDs (0, 1, 2, ...) since JoinIR lowerers allocate locally.
    ///
    /// Example: For a loop variable `i`, JoinIR uses ValueId(0) as the parameter.
    pub join_inputs: Vec<ValueId>,

    /// Host-function ValueIds that provide the input values
    ///
    /// These are the ValueIds from the **host function** that correspond to
    /// the join_inputs. The merger will inject Copy instructions to connect
    /// host_inputs[i] → join_inputs[i].
    ///
    /// Example: If host has `i` as ValueId(4), then host_inputs = [ValueId(4)].
    pub host_inputs: Vec<ValueId>,

    /// JoinIR-local ValueIds that represent outputs (if any)
    pub join_outputs: Vec<ValueId>,

    /// Host-function ValueIds that receive the outputs (DEPRECATED)
    #[deprecated(since = "Phase 190", note = "Use exit_bindings instead")]
    pub host_outputs: Vec<ValueId>,

    /// Explicit exit bindings for loop carriers (Phase 190+)
    pub exit_bindings: Vec<LoopExitBinding>,

    /// Condition-only input variables (Phase 171+)
    ///
    /// These are variables used ONLY in the loop condition, NOT as loop parameters.
    /// They need to be available in JoinIR scope but are not modified by the loop.
    ///
    /// # Example
    ///
    /// For `loop(start < end) { i = i + 1 }`:
    /// - Loop parameter: `i` → goes in `join_inputs`/`host_inputs`
    /// - Condition-only: `start`, `end` → go in `condition_inputs`
    ///
    /// # Format
    ///
    /// Each entry is `(variable_name, host_value_id)`:
    /// ```
    /// condition_inputs: vec![
    ///     ("start".to_string(), ValueId(33)),  // HOST ID for "start"
    ///     ("end".to_string(), ValueId(34)),    // HOST ID for "end"
    /// ]
    /// ```
    ///
    /// The merger will:
    /// 1. Extract unique variable names from condition AST
    /// 2. Look up HOST ValueIds from `builder.variable_map`
    /// 3. Inject Copy instructions for each condition input
    /// 4. Remap JoinIR references to use the copied values
    pub condition_inputs: Vec<(String, ValueId)>,
}
```

---

### 2. Constructor Updates

**Add new constructor**:

```rust
impl JoinInlineBoundary {
    /// Create a new boundary with condition inputs (Phase 171+)
    ///
    /// # Arguments
    ///
    /// * `join_inputs` - JoinIR-local ValueIds for loop parameters
    /// * `host_inputs` - HOST ValueIds for loop parameters
    /// * `condition_inputs` - Condition-only variables [(name, host_value_id)]
    ///
    /// # Example
    ///
    /// ```ignore
    /// let boundary = JoinInlineBoundary::new_with_condition_inputs(
    ///     vec![ValueId(0)],           // join_inputs (i)
    ///     vec![ValueId(5)],            // host_inputs (i)
    ///     vec![
    ///         ("start".to_string(), ValueId(33)),
    ///         ("end".to_string(), ValueId(34)),
    ///     ],
    /// );
    /// ```
    pub fn new_with_condition_inputs(
        join_inputs: Vec<ValueId>,
        host_inputs: Vec<ValueId>,
        condition_inputs: Vec<(String, ValueId)>,
    ) -> Self {
        assert_eq!(
            join_inputs.len(),
            host_inputs.len(),
            "join_inputs and host_inputs must have same length"
        );
        Self {
            join_inputs,
            host_inputs,
            join_outputs: vec![],
            #[allow(deprecated)]
            host_outputs: vec![],
            exit_bindings: vec![],
            condition_inputs,
        }
    }

    /// Create boundary with inputs, exit bindings, AND condition inputs (Phase 171+)
    pub fn new_with_exit_and_condition_inputs(
        join_inputs: Vec<ValueId>,
        host_inputs: Vec<ValueId>,
        exit_bindings: Vec<LoopExitBinding>,
        condition_inputs: Vec<(String, ValueId)>,
    ) -> Self {
        assert_eq!(
            join_inputs.len(),
            host_inputs.len(),
            "join_inputs and host_inputs must have same length"
        );
        Self {
            join_inputs,
            host_inputs,
            join_outputs: vec![],
            #[allow(deprecated)]
            host_outputs: vec![],
            exit_bindings,
            condition_inputs,
        }
    }
}
```

**Update existing constructors** to set `condition_inputs: vec![]`:

```rust
pub fn new_inputs_only(join_inputs: Vec<ValueId>, host_inputs: Vec<ValueId>) -> Self {
    // ... existing assertions
    Self {
        join_inputs,
        host_inputs,
        join_outputs: vec![],
        #[allow(deprecated)]
        host_outputs: vec![],
        exit_bindings: vec![],
        condition_inputs: vec![],  // NEW: Default to empty
    }
}

pub fn new_with_exit_bindings(
    join_inputs: Vec<ValueId>,
    host_inputs: Vec<ValueId>,
    exit_bindings: Vec<LoopExitBinding>,
) -> Self {
    // ... existing assertions
    Self {
        join_inputs,
        host_inputs,
        join_outputs: vec![],
        #[allow(deprecated)]
        host_outputs: vec![],
        exit_bindings,
        condition_inputs: vec![],  // NEW: Default to empty
    }
}
```

---

### 3. Value Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        HOST MIR Builder                         │
│                                                                 │
│  variable_map:                                                  │
│    "i"     → ValueId(5)    (loop variable - becomes parameter) │
│    "start" → ValueId(33)   (condition input - read-only)       │
│    "end"   → ValueId(34)   (condition input - read-only)       │
│    "sum"   → ValueId(10)   (carrier - exit binding)            │
└─────────────────────────────────────────────────────────────────┘
                            ↓
          ┌─────────────────┴─────────────────┐
          │                                   │
          ↓                                   ↓
┌──────────────────────┐          ┌──────────────────────┐
│  JoinIR Lowerer      │          │  Condition Extractor │
│                      │          │                      │
│  Allocates:          │          │  Extracts variables: │
│    i_param = Val(0)  │          │    ["start", "end"]  │
│    sum_init = Val(1) │          │                      │
└──────────────────────┘          └──────────────────────┘
          ↓                                   ↓
          └─────────────────┬─────────────────┘
                            ↓
        ┌────────────────────────────────────────────┐
        │        JoinInlineBoundary                  │
        │                                            │
        │  join_inputs:  [Val(0), Val(1)]            │
        │  host_inputs:  [Val(5), Val(10)]           │
        │                                            │
        │  condition_inputs: [                       │
        │    ("start", Val(33)),                     │
        │    ("end", Val(34))                        │
        │  ]                                         │
        │                                            │
        │  exit_bindings: [                          │
        │    { carrier: "sum", join_exit: Val(18),   │
        │      host_slot: Val(10) }                  │
        │  ]                                         │
        └────────────────────────────────────────────┘
                            ↓
        ┌────────────────────────────────────────────┐
        │   merge_joinir_mir_blocks()                │
        │                                            │
        │   Phase 1: Inject Copy instructions       │
        │     Val(100) = Copy Val(5)    // i         │
        │     Val(101) = Copy Val(10)   // sum       │
        │     Val(102) = Copy Val(33)   // start ← NEW │
        │     Val(103) = Copy Val(34)   // end   ← NEW │
        │                                            │
        │   Phase 2: Remap JoinIR ValueIds           │
        │     Val(0) → Val(100)  // i param          │
        │     Val(1) → Val(101)  // sum init         │
        │                                            │
        │   Phase 3: Remap condition refs            │
        │     Compare { lhs: Val(33), ... }          │
        │       ↓ NO CHANGE (uses HOST ID directly)  │
        │     Compare { lhs: Val(102), ... } ← FIXED │
        │                                            │
        │   Phase 4: Reconnect exit bindings         │
        │     variable_map["sum"] = Val(200)         │
        └────────────────────────────────────────────┘
                            ↓
                    ✅ All ValueIds valid
```

---

### 4. Key Insight: Two Types of Inputs

This design recognizes **two distinct categories** of JoinIR inputs:

| Category | Examples | Boundary Field | Mutability | Treatment |
|----------|----------|----------------|-----------|-----------|
| **Loop Parameters** | `i` (loop var), `sum` (carrier) | `join_inputs`/`host_inputs` | Mutable | Passed as function params |
| **Condition Inputs** | `start`, `end`, `len` | `condition_inputs` | Read-only | Captured from HOST scope |

**Why separate?**

1. **Semantic clarity**: Loop parameters can be modified; condition inputs are immutable
2. **Implementation simplicity**: Condition inputs don't need JoinIR parameters - just Copy + remap
3. **Future extensibility**: May want condition-only outputs (e.g., for side-effectful conditions)

---

### 5. Implementation Strategy

**Step 1**: Modify `inline_boundary.rs`
- Add `condition_inputs` field
- Update all constructors to initialize it
- Add new constructors for condition input support

**Step 2**: Implement condition variable extraction
- Create `extract_condition_variables()` function
- Recursively traverse condition AST
- Collect unique variable names

**Step 3**: Update loop lowerers
- Call `extract_condition_variables()` on condition AST
- Look up HOST ValueIds from `builder.variable_map`
- Pass to boundary constructor

**Step 4**: Update merge logic
- Inject Copy instructions for condition inputs
- Create temporary mapping: var_name → copied_value_id
- Rewrite condition instructions to use copied ValueIds

**Step 5**: Test with trim pattern
- Should resolve ValueId(33) undefined error
- Verify condition evaluation uses correct values

---

## Remaining Questions

### Q1: Should condition inputs be remapped globally or locally?

**Answer**: **Locally** - only within JoinIR condition instructions

**Rationale**: Condition inputs are used in:
1. Loop condition evaluation (in `loop_step` function)
2. Nowhere else (by definition - they're condition-only)

So we only need to remap ValueIds in the condition instructions, not globally across all JoinIR.

### Q2: What if a condition input is ALSO a loop parameter?

**Example**: `loop(i < 10) { i = i + 1 }`
- `i` is both a loop parameter (mutated in body) AND used in condition

**Answer**: **Loop parameter takes precedence** - it's already in `join_inputs`/`host_inputs`

**Implementation**: When extracting condition variables, SKIP any that are already in `join_inputs`

```rust
fn extract_condition_variables(
    condition_ast: &ASTNode,
    join_inputs_names: &[String],  // Already-registered parameters
) -> Vec<String> {
    let all_vars = collect_variable_names_recursive(condition_ast);
    all_vars.into_iter()
        .filter(|name| !join_inputs_names.contains(name))  // Skip loop params
        .collect()
}
```

### Q3: How to handle condition variables that don't exist in variable_map?

**Answer**: **Fail-fast with clear error**

```rust
let host_value_id = builder.variable_map.get(var_name)
    .ok_or_else(|| {
        format!(
            "Condition variable '{}' not found in variable_map. \
             Loop condition references undefined variable.",
            var_name
        )
    })?;
```

This follows the "Fail-Fast" principle from CLAUDE.md.

---

## Summary

**Design Choice**: Option A - Extend `JoinInlineBoundary` with `condition_inputs` field

**Key Properties**:
- ✅ Minimal invasiveness (single structure change)
- ✅ Clear semantics (condition-only inputs)
- ✅ Reuses existing Copy injection mechanism
- ✅ Symmetric with `exit_bindings` design
- ✅ Handles all edge cases (overlap with loop params, missing variables)

**Next Steps**: Phase 171-3 - Implement condition variable extraction in loop lowerers

---

## References

- Phase 171-1 Analysis: `phase171-1-boundary-analysis.md`
- JoinInlineBoundary: `src/mir/join_ir/lowering/inline_boundary.rs`
- Merge Logic: `src/mir/builder/control_flow/joinir/merge/mod.rs`
Status: Historical
