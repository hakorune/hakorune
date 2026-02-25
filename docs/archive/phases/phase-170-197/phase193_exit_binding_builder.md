# Phase 193-4: ExitBindingBuilder Design & Implementation

**Phase**: 193-4
**Status**: Design Phase
**Date**: 2025-12-06
**Goal**: Fully boxify loop exit binding generation for Pattern 3 & 4, eliminating hardcoded variable names and ValueId assumptions

---

## Overview

The ExitBindingBuilder box connects JoinIR exit values (from loop lowering) back to the host function's variable_map. This eliminates:
- Hardcoded variable names like `"sum"`, `"printed"`
- Assumptions about single-carrier patterns
- Complex ValueId plumbing scattered across Pattern 3/4 lowerers

### Architecture Diagram

```
Pattern Lowerer
    ↓
    ├─ CarrierInfo (loop_var, carriers[], host_ids)
    ├─ ExitMeta (join_exit_values[])
    └─ variable_map
    ↓
ExitBindingBuilder
    ↓
    ├─ LoopExitBinding[] (carrier → host mapping)
    └─ JoinInlineBoundary update (host_outputs, join_outputs)
    ↓
Host function variable_map (updated with new ValueIds)
```

---

## Data Structures

### Input: CarrierInfo

```rust
pub struct CarrierInfo {
    pub loop_var_name: String,        // e.g., "i"
    pub loop_var_id: ValueId,         // Host-side ValueId for loop var
    pub carriers: Vec<CarrierVar>,    // [{ name: "sum", host_id: ValueId(10) }, ...]
}

pub struct CarrierVar {
    pub name: String,                 // Variable name (e.g., "sum")
    pub host_id: ValueId,             // Host-side ValueId (initial value)
}
```

### Input: ExitMeta

```rust
pub struct ExitMeta {
    pub exit_values: Vec<(String, ValueId)>,
    // Example: [("sum", ValueId(15)), ("printed", ValueId(16))]
    // where ValueId(15/16) are in JoinIR-local space (parameters/results)
}
```

### Output: LoopExitBinding (New)

```rust
pub struct LoopExitBinding {
    /// Carrier variable name (e.g., "sum", "printed")
    pub carrier_name: String,

    /// Host-side ValueId for this carrier
    pub host_id: ValueId,

    /// Join-side exit ValueId (from ExitMeta)
    pub join_exit_id: ValueId,
}
```

### JoinInlineBoundary Updates

```rust
pub struct JoinInlineBoundary {
    // ... existing fields ...

    /// Host-side output ValueIds (one per carrier + loop_var)
    pub host_outputs: Vec<ValueId>,

    /// Join-side output ValueIds (one per carrier + loop_var, in JoinIR space)
    pub join_outputs: Vec<ValueId>,
}
```

---

## API Design

### ExitBindingBuilder

```rust
pub struct ExitBindingBuilder<'a> {
    carrier_info: &'a CarrierInfo,
    exit_meta: &'a ExitMeta,
    variable_map: &'a mut HashMap<String, ValueId>,
}

impl<'a> ExitBindingBuilder<'a> {
    /// Create a new builder from metadata
    pub fn new(
        carrier_info: &'a CarrierInfo,
        exit_meta: &'a ExitMeta,
        variable_map: &'a mut HashMap<String, ValueId>,
    ) -> Result<Self, String>;

    /// Generate loop exit bindings
    ///
    /// Returns one LoopExitBinding per carrier, in sorted order.
    /// Updates variable_map with new post-loop ValueIds.
    pub fn build_loop_exit_bindings(&mut self) -> Result<Vec<LoopExitBinding>, String>;

    /// Apply bindings to JoinInlineBoundary
    ///
    /// Sets host_outputs and join_outputs based on loop_var + carriers.
    /// Must be called after build_loop_exit_bindings().
    pub fn apply_to_boundary(&self, boundary: &mut JoinInlineBoundary) -> Result<(), String>;

    /// Get the updated loop_var exit binding (always first)
    pub fn loop_var_exit_binding(&self) -> LoopExitBinding;
}
```

---

## Validation Rules

### Single Carrier Case

**Input Example**:
```
CarrierInfo {
    loop_var_name: "i",
    loop_var_id: ValueId(5),
    carriers: [{ name: "sum", host_id: ValueId(10) }]
}

ExitMeta {
    exit_values: [("sum", ValueId(15))]
}

variable_map: {"i": ValueId(5), "sum": ValueId(10)}
```

**Output**:
```
LoopExitBinding {
    carrier_name: "sum",
    host_id: ValueId(10),
    join_exit_id: ValueId(15)
}

variable_map (updated): {"i": ValueId(5), "sum": ValueId(???)}  // NEW ValueId for post-loop sum
```

### Multiple Carrier Case

**Input Example**:
```
CarrierInfo {
    loop_var_name: "i",
    loop_var_id: ValueId(5),
    carriers: [
        { name: "printed", host_id: ValueId(11) },
        { name: "sum", host_id: ValueId(10) }
    ]
}

ExitMeta {
    exit_values: [
        ("printed", ValueId(14)),
        ("sum", ValueId(15))
    ]
}

variable_map: {"i": ValueId(5), "sum": ValueId(10), "printed": ValueId(11)}
```

**Output**:
```
LoopExitBinding[
    { carrier_name: "printed", host_id: ValueId(11), join_exit_id: ValueId(14) },
    { carrier_name: "sum", host_id: ValueId(10), join_exit_id: ValueId(15) }
]

variable_map (updated):
{
    "i": ValueId(5),
    "sum": ValueId(???),      // NEW post-loop ValueId
    "printed": ValueId(???)   // NEW post-loop ValueId
}
```

### Error Cases

1. **Carrier name mismatch**: ExitMeta contains carrier name not in CarrierInfo
   - Error: `"Exit carrier 'foo' not found in CarrierInfo"`

2. **Missing carrier in ExitMeta**: CarrierInfo has carrier not in ExitMeta
   - Error: `"Carrier 'sum' missing in ExitMeta"`

3. **Loop variable in ExitMeta**: ExitMeta erroneously maps loop_var
   - Error: `"Loop variable 'i' should not be in exit_values"`

---

## Implementation Strategy

### File Structure

**New file**: `src/mir/builder/control_flow/joinir/exit_binding.rs`

```rust
use crate::mir::ValueId;
use crate::mir::join_ir::JoinInlineBoundary;
use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, ExitMeta};
use std::collections::HashMap;

pub struct LoopExitBinding { ... }

pub struct ExitBindingBuilder<'a> { ... }

impl<'a> ExitBindingBuilder<'a> {
    pub fn new(...) -> Result<Self, String> { ... }
    pub fn build_loop_exit_bindings(&mut self) -> Result<Vec<LoopExitBinding>, String> { ... }
    pub fn apply_to_boundary(&self, boundary: &mut JoinInlineBoundary) -> Result<(), String> { ... }
}
```

### Module Declaration

Update `src/mir/builder/control_flow/joinir/mod.rs`:

```rust
pub mod exit_binding;
```

### Integration Points

**Pattern 3 & 4 Lowerers**:

```rust
// OLD: Direct boundary manipulation
boundary.host_outputs.push(sum_value_id);
boundary.join_outputs.push(join_sum_exit);
variable_map.insert("sum".to_string(), new_sum_id);

// NEW: Via ExitBindingBuilder
let mut builder = ExitBindingBuilder::new(&carrier_info, &exit_meta, variable_map)?;
let _bindings = builder.build_loop_exit_bindings()?;
builder.apply_to_boundary(&mut boundary)?;
```

---

## Testing Strategy

### Unit Tests (exit_binding.rs)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_carrier_binding() { ... }

    #[test]
    fn test_multi_carrier_binding() { ... }

    #[test]
    fn test_carrier_name_mismatch_error() { ... }

    #[test]
    fn test_variable_map_update() { ... }
}
```

### Integration Tests

**File**: `apps/tests/loop_continue_multi_carrier.hako`

```hako
static box Main {
    main() {
        local sum = 0
        local printed = 0

        loop(i = 0; i < 5; i = i + 1) {
            if (i > 2) {
                printed = printed + 1
                continue
            }
            sum = sum + i
        }

        // Expected: sum = 0+1+2 = 3, printed = 2 (i=3,4)
        print(sum)
        print(printed)
    }
}
```

**Test command**:
```bash
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/loop_continue_multi_carrier.hako
# Expected output:
# 3
# 2
```

---

## Tracking Variable Updates

### variable_map lifecycle

1. **Before loop lowering**: `{"i": ValueId(5), "sum": ValueId(10), "printed": ValueId(11)}`

2. **After JoinModule creation**: (unchanged)

3. **ExitBindingBuilder::build_loop_exit_bindings()**:
   - Allocates new ValueIds for post-loop carrier values
   - Updates variable_map: `{"i": ValueId(5), "sum": ValueId(??), "printed": ValueId(??)}`

4. **After loop lowering**: variable_map reflects post-loop state

### Debugging support

Optional environment variable: `NYASH_TRACE_EXIT_BINDING=1`

Output example:
```
[exit_binding] Carrier "sum": host_id=ValueId(10) → join_exit=ValueId(15) → post_loop=ValueId(23)
[exit_binding] Carrier "printed": host_id=ValueId(11) → join_exit=ValueId(14) → post_loop=ValueId(24)
[exit_binding] JoinInlineBoundary: host_outputs=[ValueId(5), ValueId(23), ValueId(24)]
```

---

## Related Phases

- **Phase 188**: JoinInlineBoundary initial design
- **Phase 190**: CarrierInfo (Phase 193-2 enhancement)
- **Phase 193-3**: Pattern classification helpers
- **Phase 193-4**: ExitBindingBuilder (THIS PHASE)
- **Phase 193-5**: Multi-carrier testing and validation

---

## Success Criteria

- [ ] ExitBindingBuilder compiles and passes unit tests
- [ ] Pattern 3 & 4 lowerers refactored to use ExitBindingBuilder
- [ ] No hardcoded variable names or ValueId assumptions remain in lowering
- [ ] loop_continue_multi_carrier.hako test passes with correct output
- [ ] Variable map correctly reflects post-loop carrier state
- [ ] Debugging environment variable works as expected
Status: Historical
