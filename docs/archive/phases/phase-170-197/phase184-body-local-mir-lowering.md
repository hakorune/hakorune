# Phase 184: Body-local MIR Lowering Design

## Status: In Progress (2025-12-08)

## Overview

Phase 183 completed **LoopBodyLocal role separation** (condition vs body-only).
Phase 184 implements **body-local MIR lowering** - the ability to use body-only local variables in update expressions safely.

### Problem Statement

**Current limitation**:
```nyash
local sum = 0
local i = 0
loop(i < 5) {
    local temp = i * 2  // body-local variable
    sum = sum + temp    // ❌ ERROR: temp not found in ConditionEnv
    i = i + 1
}
```

**Root cause**:
- `CarrierUpdateEmitter` only has access to `ConditionEnv`
- `ConditionEnv` only contains condition variables (loop parameters)
- Body-local variables (`temp`) are not in `ConditionEnv`
- Update expression `sum = sum + temp` fails to resolve `temp`

**Goal**: Enable body-local variables in update expressions while maintaining architectural clarity.

## Design Solution

### Architecture: Two-Environment System

```
┌─────────────────────┐
│   ConditionEnv      │  Condition variables (loop parameters)
│  - i → ValueId(0)   │  Priority: HIGH
│  - end → ValueId(1) │
└─────────────────────┘
         ↓
┌─────────────────────┐
│ LoopBodyLocalEnv    │  Body-local variables
│  - temp → ValueId(5)│  Priority: LOW (only if not in ConditionEnv)
│  - digit → ValueId(6)│
└─────────────────────┘
         ↓
┌─────────────────────┐
│    UpdateEnv        │  Unified resolution layer
│  resolve(name):     │  1. Try ConditionEnv first
│    cond.get(name)   │  2. Fallback to LoopBodyLocalEnv
│    .or(body.get())  │
└─────────────────────┘
```

### Box-First Design

Following **箱理論 (Box Theory)** principles:

#### Box 1: LoopBodyLocalEnv (Storage Box)

**Single Responsibility**: Collect and manage body-local variable mappings (name → ValueId).

```rust
pub struct LoopBodyLocalEnv {
    locals: BTreeMap<String, ValueId>, // Deterministic ordering
}

impl LoopBodyLocalEnv {
    /// Scan loop body AST and collect local definitions
    pub fn from_loop_body(
        body: &[ASTNode],
        builder: &mut JoinIrBuilder
    ) -> Self;

    /// Resolve a body-local variable name to JoinIR ValueId
    pub fn get(&self, name: &str) -> Option<ValueId>;
}
```

**Design rationale**:
- **BTreeMap**: Ensures deterministic iteration (PHI ordering consistency)
- **from_loop_body()**: Separates collection logic from lowering logic
- **get()**: Simple lookup, no side effects

#### Box 2: UpdateEnv (Composition Box)

**Single Responsibility**: Unified variable resolution for update expressions.

```rust
pub struct UpdateEnv<'a> {
    condition_env: &'a ConditionEnv,      // Priority 1: Condition vars
    body_local_env: &'a LoopBodyLocalEnv, // Priority 2: Body-local vars
}

impl<'a> UpdateEnv<'a> {
    /// Resolve variable name with priority order
    pub fn resolve(&self, name: &str) -> Option<ValueId> {
        self.condition_env.get(name)
            .or_else(|| self.body_local_env.get(name))
    }
}
```

**Design rationale**:
- **Composition**: Combines two environments without owning them
- **Priority order**: Condition variables take precedence (shadowing prevention)
- **Lightweight**: No allocation, just references

### Integration Points

#### Current Flow (Pattern2 Example)

```rust
// Pattern2Lowerer::lower()
let condition_env = /* ... */;

// Emit carrier update
let update_value = emit_carrier_update(
    &carrier,
    &update_expr,
    &mut alloc_value,
    &condition_env,  // ❌ Only has condition variables
    &mut instructions,
)?;
```

#### Phase 184 Flow

```rust
// Pattern2Lowerer::lower()
let condition_env = /* ... */;
let body_local_env = LoopBodyLocalEnv::from_loop_body(&body_nodes, builder);

// Create unified environment
let update_env = UpdateEnv {
    condition_env: &condition_env,
    body_local_env: &body_local_env,
};

// Emit carrier update (now with body-local support)
let update_value = emit_carrier_update_with_env(
    &carrier,
    &update_expr,
    &mut alloc_value,
    &update_env,  // ✅ Has both condition and body-local variables
    &mut instructions,
)?;
```

## Scope and Constraints

### In Scope (Phase 184)

1. **Body-local variable collection**: `LoopBodyLocalEnv::from_loop_body()`
2. **Unified resolution**: `UpdateEnv::resolve()`
3. **CarrierUpdateEmitter integration**: Use `UpdateEnv` instead of `ConditionEnv`
4. **Pattern2/4 integration**: Pass `body_local_env` to update lowering

### Out of Scope

1. **Condition variable usage**: Body-locals still cannot be used in conditions
2. **Pattern5 (Trim) integration**: Defer to Phase 185
3. **Complex expressions**: Only simple variable references in updates
4. **Type checking**: Assume type correctness (existing type inference handles this)

### Design Constraints

Following **Phase 178 Fail-Fast** and **箱理論** principles:

1. **Single Responsibility**: Each Box has one clear purpose
2. **Deterministic**: BTreeMap for consistent ordering
3. **Conservative**: No changes to Trim/Pattern5 logic
4. **Explicit errors**: If body-local used in condition → Fail loudly

## Implementation Tasks

### Task 184-1: Design Document ✅ (This document)

### Task 184-2: LoopBodyLocalEnv Implementation

**File**: `src/mir/join_ir/lowering/loop_body_local_env.rs` (new)

**Core logic**:
```rust
impl LoopBodyLocalEnv {
    pub fn from_loop_body(body: &[ASTNode], builder: &mut JoinIrBuilder) -> Self {
        let mut locals = BTreeMap::new();

        for node in body {
            if let ASTNode::LocalDecl { name, init_value } = node {
                // Lower init_value to JoinIR
                let value_id = builder.lower_expr(init_value)?;
                locals.insert(name.clone(), value_id);
            }
        }

        Self { locals }
    }
}
```

**Unit tests** (3-5 tests):
- `test_empty_body`: No locals → empty env
- `test_single_local`: One local → one mapping
- `test_multiple_locals`: Multiple locals → sorted keys
- `test_get_existing`: Lookup succeeds
- `test_get_nonexistent`: Lookup returns None

### Task 184-3: UpdateEnv Implementation

**File**: `src/mir/join_ir/lowering/update_env.rs` (new)

**Core logic**:
```rust
pub struct UpdateEnv<'a> {
    condition_env: &'a ConditionEnv,
    body_local_env: &'a LoopBodyLocalEnv,
}

impl<'a> UpdateEnv<'a> {
    pub fn new(
        condition_env: &'a ConditionEnv,
        body_local_env: &'a LoopBodyLocalEnv,
    ) -> Self {
        Self { condition_env, body_local_env }
    }

    pub fn resolve(&self, name: &str) -> Option<ValueId> {
        self.condition_env.get(name)
            .or_else(|| self.body_local_env.get(name))
    }
}
```

**Unit tests** (2-3 tests):
- `test_resolve_condition_priority`: Condition var found first
- `test_resolve_body_local_fallback`: Body-local found when condition absent
- `test_resolve_not_found`: Neither env has variable → None

### Task 184-4: CarrierUpdateEmitter Integration

**File**: `src/mir/join_ir/lowering/carrier_update_emitter.rs` (modify)

**Changes**:
1. Add new function variant accepting `UpdateEnv`:
```rust
pub fn emit_carrier_update_with_env(
    carrier: &CarrierVar,
    update: &UpdateExpr,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &UpdateEnv,  // New: UpdateEnv instead of ConditionEnv
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    // Use env.resolve() instead of env.get()
}
```

2. Keep existing `emit_carrier_update()` for backward compatibility
3. Update Pattern2/4 callers to use new variant

**Validation**: Existing tests must pass (backward compatibility).

### Task 184-5: Representative Tests

**File**: `apps/tests/phase184_body_local_update.hako` (new)

```nyash
// Body-local used in update expression
static box Main {
    main() {
        local sum = 0
        local i = 0
        loop(i < 5) {
            local temp = i * 2  // Body-local variable
            sum = sum + temp    // Use in update expression
            i = i + 1
        }
        print(sum)  // Expected: 0+2+4+6+8 = 20
    }
}
```

**Test commands**:
```bash
# Structure trace
NYASH_JOINIR_STRUCTURE_ONLY=1 ./target/release/hakorune apps/tests/phase184_body_local_update.hako

# Full execution
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase184_body_local_update.hako
```

### Task 184-6: Documentation Updates

1. **Update**: `docs/development/current/main/joinir-architecture-overview.md`
   - Add LoopBodyLocalEnv section
   - Update UpdateEnv integration diagram

2. **Update**: `CURRENT_TASK.md`
   - Mark Phase 184 complete
   - Add Phase 185 preview

## Validation Strategy

### Success Criteria

1. **Unit tests pass**: LoopBodyLocalEnv and UpdateEnv tests green
2. **Backward compatibility**: Existing Pattern2/4 tests still pass
3. **Representative test**: phase184_body_local_update.hako executes correctly (output: 20)
4. **No regression**: Trim patterns (Pattern5) unaffected

### Test Commands

```bash
# Unit tests
cargo test --release --lib loop_body_local_env
cargo test --release --lib update_env
cargo test --release --lib carrier_update

# Integration test
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase184_body_local_update.hako

# Regression check (Trim pattern)
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase172_trim_while.hako
```

## Known Limitations

### Not Supported (Explicit Design Decision)

1. **Body-local in conditions**:
```nyash
loop(i < 5) {
    local temp = i * 2
    if (temp > 6) break  // ❌ ERROR: Body-local in condition not allowed
}
```
**Reason**: Condition variables must be loop parameters (JoinIR entry signature constraint).

2. **Shadowing**:
```nyash
loop(i < 5) {
    local i = 10  // ❌ ERROR: Shadows condition variable
}
```
**Reason**: `UpdateEnv` prioritizes condition variables - shadowing forbidden.

3. **Complex expressions**:
```nyash
loop(i < 5) {
    local temp = obj.method()  // ⚠️ May not work yet
}
```
**Reason**: Limited to expressions `JoinIrBuilder::lower_expr()` supports.

## Future Work (Phase 185+)

### Phase 185: Trim Pattern Integration

- Extend LoopBodyLocalEnv to handle Trim carrier variables
- Update TrimLoopLowerer to use UpdateEnv

### Phase 186: Condition Expression Support

- Allow body-local variables in break/continue conditions
- Requires inline expression evaluation in condition lowering

## References

- **Phase 183**: LoopBodyLocal role separation (condition vs body-only)
- **Phase 178**: Fail-Fast error handling principles
- **Phase 171-C**: LoopBodyCarrierPromoter original design
- **carrier_update_emitter.rs**: Current update emission logic
- **condition_env.rs**: Condition variable environment design
Status: Historical
