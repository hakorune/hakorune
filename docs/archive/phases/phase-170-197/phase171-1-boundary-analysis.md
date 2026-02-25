# Phase 171-1: Boundary Coverage Analysis

**Date**: 2025-12-07
**Status**: Analysis Complete

## Current Boundary Coverage Table

| Component | Currently in Boundary? | How Mapped? | File Location |
|-----------|----------------------|-------------|---------------|
| Loop variable (`i`) | ✅ Yes | `join_inputs[0]` → `host_inputs[0]` | `inline_boundary.rs:115-124` |
| Carriers (`sum`, `count`) | ✅ Yes | `exit_bindings` (Phase 190+) | `inline_boundary.rs:150-167` |
| Exit values | ✅ Yes | `exit_bindings.join_exit_value` | `exit_binding.rs:87-116` |
| **Condition inputs (`start`, `end`, `len`)** | ❌ **NO** | **MISSING** | **N/A** |

---

## Detailed Analysis

### 1. Loop Variable (`i`) - ✅ Properly Mapped

**Mapping Flow**:
```
HOST: variable_map["i"] = ValueId(5)
  ↓ join_inputs = [ValueId(0)]  (JoinIR local ID)
  ↓ host_inputs = [ValueId(5)]  (HOST ID)
JoinIR: main() parameter = ValueId(0)
  ↓ merge_joinir_mir_blocks() injects:
MIR: ValueId(100) = Copy ValueId(5)  // Boundary Copy instruction
```

**Evidence**:
- `inline_boundary.rs:175-189` - `new_inputs_only()` constructor
- `merge/mod.rs:104-106` - Boundary reconnection call
- **Works correctly** in all existing JoinIR tests

---

### 2. Carriers (`sum`, `count`) - ✅ Properly Mapped (Phase 190+)

**Mapping Flow**:
```
HOST: variable_map["sum"] = ValueId(10)
  ↓ exit_bindings = [
      LoopExitBinding {
        carrier_name: "sum",
        join_exit_value: ValueId(18),  // k_exit param in JoinIR
        host_slot: ValueId(10)          // HOST variable
      }
    ]
JoinIR: k_exit(sum_exit) - parameter = ValueId(18)
  ↓ merge_joinir_mir_blocks() remaps:
  ↓ remapper.set_value(ValueId(18), ValueId(200))
MIR: variable_map["sum"] = ValueId(200)  // Reconnected
```

**Evidence**:
- `inline_boundary.rs:259-311` - `new_with_exit_bindings()` constructor
- `exit_binding.rs:87-116` - Exit binding builder
- `merge/mod.rs:188-267` - `reconnect_boundary()` implementation
- **Works correctly** in Pattern 3/4 tests

---

### 3. **Condition Inputs (`start`, `end`, `len`) - ❌ NOT MAPPED**

**Current Broken Flow**:
```
HOST: variable_map["start"] = ValueId(33)
  ↓ condition_to_joinir() reads from variable_map DIRECTLY
  ↓ lower_value_expression() returns ValueId(33)
JoinIR: Uses ValueId(33) in Compare instruction
  ↓ NO BOUNDARY REGISTRATION
  ↓ NO REMAPPING
MIR: ValueId(33) undefined → RUNTIME ERROR
```

**Evidence** (from Phase 170):
```
[ssa-undef-debug] fn=TrimTest.trim/1 bb=BasicBlockId(12)
                 inst_idx=0 used=ValueId(33)
```

**Root Cause**:
- `condition_to_joinir.rs:183-189` - Reads from `builder.variable_map` directly
- `condition_to_joinir.rs:236-239` - Returns HOST ValueId unchanged
- **NO registration** in `JoinInlineBoundary`
- **NO remapping** in `merge_joinir_mir_blocks()`

---

## Why This is a Problem

### Example: `loop(start < end)`

**What SHOULD happen**:
```rust
// HOST preparation
let start_host = ValueId(33);
let end_host = ValueId(34);

// JoinIR lowerer
let start_joinir = ValueId(0);  // Local param
let end_joinir = ValueId(1);    // Local param

// Boundary
JoinInlineBoundary {
  join_inputs: [ValueId(0), ValueId(1)],      // start, end in JoinIR
  host_inputs: [ValueId(33), ValueId(34)],    // start, end in HOST
  // ...
}

// Merge
// Injects: ValueId(100) = Copy ValueId(33)  // start
// Injects: ValueId(101) = Copy ValueId(34)  // end
// Remaps all JoinIR ValueId(0) → ValueId(100), ValueId(1) → ValueId(101)
```

**What CURRENTLY happens**:
```rust
// JoinIR lowerer
let start = builder.variable_map.get("start");  // Returns ValueId(33) - HOST ID!
let end = builder.variable_map.get("end");      // Returns ValueId(34) - HOST ID!

// JoinIR uses HOST ValueIds directly
Compare { lhs: ValueId(33), rhs: ValueId(34) }  // WRONG - uses HOST IDs

// No boundary registration → No remapping → UNDEFINED VALUE ERROR
```

---

## Comparison: Loop Variable vs Condition Inputs

| Aspect | Loop Variable (`i`) | Condition Inputs (`start`, `end`) |
|--------|--------------------|------------------------------------|
| **Who allocates ValueId?** | JoinIR lowerer (`alloc_value()`) | HOST (`builder.variable_map`) |
| **Boundary registration?** | ✅ Yes (`join_inputs[0]`) | ❌ NO |
| **Remapping?** | ✅ Yes (via boundary Copy) | ❌ NO |
| **Result?** | ✅ Works | ❌ Undefined ValueId error |

---

## Root Cause Summary

The core problem is **two-faced ValueId resolution** in `condition_to_joinir()`:

1. **Loop variable** (`i`):
   - Allocated by JoinIR lowerer: `i_param = alloc_value()` → `ValueId(0)`
   - Used in condition: `i < end`
   - Properly registered in boundary ✅

2. **Condition variables** (`start`, `end`):
   - Read from HOST: `builder.variable_map.get("start")` → `ValueId(33)`
   - Used in condition: `start < end`
   - **NOT registered in boundary** ❌

---

## Files Involved

### Boundary Definition
- `src/mir/join_ir/lowering/inline_boundary.rs` (340 lines)
  - `JoinInlineBoundary` struct
  - `join_inputs`, `host_inputs` fields
  - **Missing**: Condition inputs field

### Boundary Builder
- `src/mir/builder/control_flow/joinir/patterns/exit_binding.rs` (401 lines)
  - `LoopExitBinding` struct
  - `ExitBindingBuilder` - builds exit bindings
  - **Missing**: Condition input builder

### Merge Implementation
- `src/mir/builder/control_flow/joinir/merge/mod.rs` (268 lines)
  - `merge_joinir_mir_blocks()` - main merge coordinator
  - `reconnect_boundary()` - updates variable_map with exit values
  - **Missing**: Condition input Copy injection

### Condition Lowering
- `src/mir/join_ir/lowering/condition_to_joinir.rs` (443 lines)
  - `lower_condition_to_joinir()` - AST → JoinIR conversion
  - `lower_value_expression()` - reads from `builder.variable_map`
  - **Problem**: Returns HOST ValueIds directly

### Loop Lowerers
- `src/mir/join_ir/lowering/loop_with_break_minimal.rs` (295 lines)
  - `lower_loop_with_break_minimal()` - Pattern 2 lowerer
  - Calls `lower_condition_to_joinir()` at line 138-144
  - **Missing**: Extract condition variables, register in boundary

---

## Next Steps (Phase 171-2)

We need to design a "box" for condition inputs. Three options:

**Option A**: Extend `JoinInlineBoundary` with `condition_inputs` field
**Option B**: Create new `LoopInputBinding` structure
**Option C**: Extend `LoopExitBinding` to include condition inputs

Proceed to Phase 171-2 for design decision.

---

## References

- Phase 170 Analysis: `phase170-valueid-boundary-analysis.md`
- Phase 170 Completion: `phase170-completion-report.md`
- JoinIR Design: `docs/development/current/main/phase33-10-if-joinir-design.md`
Status: Historical
