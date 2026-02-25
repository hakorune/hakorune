# Phase 185: Body-local Pattern2/4 Integration (int loops priority)

**Date**: 2025-12-09
**Status**: In Progress
**Phase Goal**: Integrate Phase 184 infrastructure into Pattern2/4 for integer loop support

---

## Overview

Phase 184 completed the **body-local MIR lowering infrastructure** with three boxes:
- `LoopBodyLocalEnv`: Storage for body-local variable mappings
- `UpdateEnv`: Unified resolution (ConditionEnv + LoopBodyLocalEnv)
- `CarrierUpdateEmitter`: Extended with `emit_carrier_update_with_env()`

Phase 185 **integrates this infrastructure** into Pattern2/4 lowerers to enable integer loops with body-local variables.

### Target Loops

**JsonParser integer loops**:
- `_parse_number`: Parses numeric strings with `local digit_pos` calculations
- `_atoi`: Converts string to integer with `local digit` temporary

**Test cases**:
- `phase184_body_local_update.hako`: Pattern1 test (already works)
- `phase184_body_local_with_break.hako`: Pattern2 test (needs integration)

### What We Do

**Integrate body-local variables into update expressions**:
```nyash
loop(pos < len) {
    local digit_pos = pos - start  // Body-local variable
    sum = sum * 10                 // Update using body-local
    sum = sum + digit_pos
    pos = pos + 1
    if (sum > 1000) break
}
```

**Enable**: `digit_pos` in `sum = sum + digit_pos` update expression

### What We DON'T Do

**String concatenation** (Phase 178 Fail-Fast maintained):
```nyash
loop(pos < len) {
    local ch = s.substring(pos, pos+1)
    num_str = num_str + ch  // ❌ Still rejected (string concat)
}
```

**Reason**: String UpdateKind support is Phase 186+ work.

---

## Architecture Integration

### Current Flow (Phase 184 - Infrastructure Only)

```
┌──────────────────────┐
│ ConditionEnvBuilder  │ → ConditionEnv (loop params)
└──────────────────────┘
          ↓
┌──────────────────────┐
│ LoopBodyLocalEnv     │ ← NEW (Phase 184)
│  from_locals()       │    Body-local variables
└──────────────────────┘
          ↓
┌──────────────────────┐
│ UpdateEnv            │ ← NEW (Phase 184)
│  resolve(name)       │    Unified resolution
└──────────────────────┘
          ↓
┌──────────────────────┐
│ CarrierUpdateEmitter │ ← EXTENDED (Phase 184)
│ emit_carrier_update_ │    UpdateEnv version
│  with_env()          │
└──────────────────────┘
```

**Status**: Infrastructure complete, but Pattern2/4 still use old `ConditionEnv` path.

### Phase 185 Flow (Integration)

**Pattern2 changes**:
```rust
// 1. Collect body-local variables
let body_locals = collect_body_local_variables(_body);
let body_local_env = LoopBodyLocalEnv::from_locals(body_locals);

// 2. Create UpdateEnv
let update_env = UpdateEnv::new(&condition_env, &body_local_env);

// 3. Use UpdateEnv in carrier update
let update_value = emit_carrier_update_with_env(
    &carrier,
    &update_expr,
    &mut alloc_value,
    &update_env,  // ✅ Now has body-local support
    &mut instructions,
)?;
```

**Pattern4**: Same pattern (minimal changes, copy from Pattern2 approach).

---

## Task Breakdown

### Task 185-1: Design Document ✅

**This document** - Architecture, scope, constraints, validation strategy.

### Task 185-2: Pattern2 Integration

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

**Changes**:

1. **Add helper function** (before cf_loop_pattern2_with_break):
```rust
/// Collect body-local variable declarations from loop body
///
/// Returns Vec<(name, ValueId)> for variables declared with `local` in loop body.
fn collect_body_local_variables(
    body: &[ASTNode],
    alloc_join_value: &mut dyn FnMut() -> ValueId,
) -> Vec<(String, ValueId)> {
    let mut locals = Vec::new();
    for node in body {
        if let ASTNode::LocalDecl { name, .. } = node {
            let value_id = alloc_join_value();
            locals.push((name.clone(), value_id));
        }
    }
    locals
}
```

2. **Modify cf_loop_pattern2_with_break** (after ConditionEnvBuilder):
```rust
// Phase 185: Collect body-local variables
let body_locals = collect_body_local_variables(_body, &mut alloc_join_value);
let body_local_env = LoopBodyLocalEnv::from_locals(body_locals);

eprintln!("[pattern2/body-local] Collected {} body-local variables", body_local_env.len());
for (name, vid) in body_local_env.iter() {
    eprintln!("  {} → {:?}", name, vid);
}

// Phase 185: Create UpdateEnv for unified resolution
let update_env = UpdateEnv::new(&env, &body_local_env);
```

3. **Update carrier update calls** (search for `emit_carrier_update`):
```rust
// OLD (Phase 184):
// let update_value = emit_carrier_update(&carrier, &update_expr, &mut alloc_join_value, &env, &mut instructions)?;

// NEW (Phase 185):
use crate::mir::join_ir::lowering::carrier_update_emitter::emit_carrier_update_with_env;
let update_value = emit_carrier_update_with_env(
    &carrier,
    &update_expr,
    &mut alloc_join_value,
    &update_env,  // ✅ UpdateEnv instead of ConditionEnv
    &mut instructions,
)?;
```

4. **Add imports**:
```rust
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::update_env::UpdateEnv;
use crate::mir::join_ir::lowering::carrier_update_emitter::emit_carrier_update_with_env;
```

**Estimate**: 1 hour (straightforward, follow Phase 184 design)

### Task 185-3: Pattern4 Integration (Minimal)

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

**Changes**: Same pattern as Pattern2 (copy-paste approach):
1. Add `collect_body_local_variables()` helper
2. Create `LoopBodyLocalEnv` after ConditionEnvBuilder
3. Create `UpdateEnv`
4. Replace `emit_carrier_update()` with `emit_carrier_update_with_env()`

**Constraint**: Only int carriers (string filter from Phase 178 remains active)

**Estimate**: 45 minutes (copy from Pattern2, minimal changes)

### Task 185-4: Test Cases

#### Existing Tests (Reuse)

1. **phase184_body_local_update.hako** (Pattern1)
   - Already passing (Pattern1 uses UpdateEnv)
   - Verification: `NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase184_body_local_update.hako`

2. **phase184_body_local_with_break.hako** (Pattern2)
   - Currently blocked (Pattern2 not integrated yet)
   - **Will pass after Task 185-2**

#### New Test: JsonParser Mini Pattern

**File**: `apps/tests/phase185_p2_body_local_int_min.hako`

```nyash
// Minimal JsonParser-style loop with body-local integer calculation
static box Main {
    main() {
        local sum = 0
        local pos = 0
        local start = 0
        local end = 5

        // Pattern2: break loop with body-local digit_pos
        loop(pos < end) {
            local digit_pos = pos - start  // Body-local calculation
            sum = sum * 10
            sum = sum + digit_pos          // Use body-local in update
            pos = pos + 1

            if (sum > 50) break  // Break condition
        }

        print(sum)  // Expected: 0*10+0 → 0*10+1 → 1*10+2 → 12*10+3 → 123 → break
                    // Output: 123 (breaks before digit_pos=4)
    }
}
```

**Expected behavior**:
- Pattern2 detection: ✅ (has break, no continue)
- Body-local collection: `digit_pos → ValueId(X)`
- UpdateEnv resolution: `digit_pos` found in LoopBodyLocalEnv
- Update emission: `sum = sum + digit_pos` → BinOp instruction
- Execution: Output `123`

**Validation commands**:
```bash
# Build
cargo build --release

# Structure trace
NYASH_JOINIR_STRUCTURE_ONLY=1 ./target/release/hakorune apps/tests/phase185_p2_body_local_int_min.hako

# Full execution
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase185_p2_body_local_int_min.hako
```

#### String Concatenation Test (Fail-Fast Verification)

**File**: `apps/tests/phase185_p2_string_concat_rejected.hako`

```nyash
// Verify Phase 178 Fail-Fast is maintained (string concat still rejected)
static box Main {
    main() {
        local result = ""
        local i = 0

        loop(i < 3) {
            local ch = "a"
            result = result + ch  // ❌ Should be rejected (string concat)
            i = i + 1
        }

        print(result)
    }
}
```

**Expected behavior**:
- Pattern2 can_lower: ❌ Rejected (string/complex update detected)
- Error message: `[pattern2/can_lower] Phase 178: String/complex update detected, rejecting Pattern 2 (unsupported)`
- Build: ✅ Succeeds (compilation)
- Runtime: ❌ Falls back to error (no legacy LoopBuilder)

### Task 185-5: Documentation Updates

**Files to update**:

1. **joinir-architecture-overview.md** (Section 2.2):
```markdown
### 2.2 条件式ライン（式の箱）

...

- **LoopBodyLocalEnv / UpdateEnv / CarrierUpdateEmitter（Phase 184-185）**
  - **Phase 184**: Infrastructure implementation
  - **Phase 185**: Integration into Pattern2/4
  - Pattern2/4 now use UpdateEnv for body-local variable support
  - String concat still rejected (Phase 178 Fail-Fast maintained)
```

2. **CURRENT_TASK.md** (Update Phase 185 entry):
```markdown
  - [x] **Phase 185: Body-local Pattern2/4 Integration** ✅ (2025-12-09)
        - Task 185-1: Design document (phase185-body-local-integration.md)
        - Task 185-2: Pattern2 integration (body-local collection + UpdateEnv)
        - Task 185-3: Pattern4 integration (minimal, copy from Pattern2)
        - Task 185-4: Test cases (phase185_p2_body_local_int_min.hako)
        - Task 185-5: Documentation updates
        - **成果**: Pattern2/4 now support body-local variables in integer update expressions
        - **制約**: String concat still rejected (Phase 178 Fail-Fast)
        - **次ステップ**: Phase 186 (String UpdateKind support)
```

3. **This document** (phase185-body-local-integration.md):
   - Add "Implementation Complete" section
   - Record test results
   - Document any issues found

---

## Scope and Constraints

### In Scope

1. **Integer carrier updates with body-local variables** ✅
   - `sum = sum + digit_pos` where `digit_pos` is body-local
   - Pattern2 (break) and Pattern4 (continue)

2. **Phase 184 infrastructure integration** ✅
   - LoopBodyLocalEnv collection
   - UpdateEnv usage
   - emit_carrier_update_with_env() calls

3. **Backward compatibility** ✅
   - Existing tests must still pass
   - No changes to Pattern1/Pattern3
   - No changes to Trim patterns (Pattern5)

### Out of Scope

1. **String concatenation** ❌
   - Phase 178 Fail-Fast is maintained
   - `result = result + ch` still rejected
   - Will be Phase 186+ work

2. **Complex expressions in body-locals** ❌
   - Method calls: `local ch = s.substring(pos, pos+1)` (limited by JoinIrBuilder)
   - Will be addressed in Phase 186+

3. **Condition variable usage of body-locals** ❌
   - `if (temp > 6) break` where `temp` is body-local (already handled by Phase 183 rejection)

---

## Validation Strategy

### Success Criteria

1. **Unit tests pass**: All existing carrier_update tests still green ✅
2. **Pattern2 integration**: phase184_body_local_with_break.hako executes correctly ✅
3. **Pattern4 integration**: Pattern4 tests with body-locals work (if exist) ✅
4. **Representative test**: phase185_p2_body_local_int_min.hako outputs `123` ✅
5. **Fail-Fast maintained**: phase185_p2_string_concat_rejected.hako rejects correctly ✅
6. **No regression**: Trim patterns (phase172_trim_while.hako) still work ✅

### Test Commands

```bash
# 1. Unit tests
cargo test --release --lib pattern2_with_break
cargo test --release --lib pattern4_with_continue
cargo test --release --lib carrier_update

# 2. Integration tests
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase184_body_local_update.hako
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase184_body_local_with_break.hako
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase185_p2_body_local_int_min.hako

# 3. Fail-Fast verification
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase185_p2_string_concat_rejected.hako 2>&1 | grep "String/complex update detected"

# 4. Regression check
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase172_trim_while.hako
```

---

## Design Principles

### Box Theory Compliance

1. **Single Responsibility**:
   - LoopBodyLocalEnv: Storage only
   - UpdateEnv: Resolution only
   - CarrierUpdateEmitter: Emission only

2. **Clear Boundaries**:
   - ConditionEnv vs LoopBodyLocalEnv (distinct scopes)
   - UpdateEnv composition (no ownership, just references)

3. **Deterministic**:
   - BTreeMap in LoopBodyLocalEnv (consistent ordering)
   - Priority order in UpdateEnv (condition → body-local)

4. **Conservative**:
   - No changes to Trim/Pattern5 logic
   - String concat still rejected (Phase 178 Fail-Fast)

### Fail-Fast Principle

**From Phase 178**: Reject unsupported patterns explicitly, not silently.

**Maintained in Phase 185**:
- String concat → Explicit error in can_lower()
- Complex expressions → Error from JoinIrBuilder
- Shadowing → Error from UpdateEnv priority logic

**No fallback to LoopBuilder** (deleted in Phase 187).

---

## Known Limitations

### Not Supported (By Design)

1. **String concatenation**:
```nyash
result = result + ch  // ❌ Still rejected (Phase 178)
```

2. **Body-local in conditions**:
```nyash
loop(i < 5) {
    local temp = i * 2
    if (temp > 6) break  // ❌ Already rejected (Phase 183)
}
```

3. **Complex init expressions**:
```nyash
local temp = s.substring(pos, pos+1)  // ⚠️ Limited by JoinIrBuilder
```

### Will Be Addressed

- **Phase 186**: String UpdateKind support (careful, gradual)
- **Phase 187**: Method call support in body-local init
- **Phase 188**: Full JsonParser loop coverage

---

## Implementation Notes

### collect_body_local_variables() Helper

**Design decision**: Keep it simple, only collect `LocalDecl` nodes.

**Why not more complex?**:
- Body-local variables are explicitly declared with `local` keyword
- No need to track assignments (that's carrier analysis)
- No need to track scopes (loop body is single scope)

**Alternative approaches considered**:
1. Reuse LoopScopeShapeBuilder logic ❌ (too heavyweight, circular dependency)
2. Scan all variable references ❌ (over-complex, not needed)
3. Simple LocalDecl scan ✅ (chosen - sufficient, clean)

### UpdateEnv vs ConditionEnv

**Why not extend ConditionEnv?**:
- Separation of concerns (condition variables vs body-locals are conceptually different)
- Composition over inheritance (UpdateEnv composes two environments)
- Backward compatibility (ConditionEnv unchanged, existing code still works)

### emit_carrier_update_with_env vs emit_carrier_update

**Why two functions?**:
- Backward compatibility (old code uses ConditionEnv directly)
- Clear API contract (with_env = supports body-locals, without = condition only)
- Gradual migration (Pattern1/3 can stay with old API, Pattern2/4 migrate)

---

## References

- **Phase 184**: LoopBodyLocalEnv/UpdateEnv/CarrierUpdateEmitter infrastructure
- **Phase 183**: LoopBodyLocal role separation (condition vs body-only)
- **Phase 178**: String carrier rejection (Fail-Fast principle)
- **Phase 171-C**: LoopBodyCarrierPromoter (Trim pattern handling)
- **pattern2_with_break.rs**: Current Pattern2 implementation
- **pattern4_with_continue.rs**: Current Pattern4 implementation
- **carrier_update_emitter.rs**: Update emission logic

---

## Implementation Status (2025-12-09)

### ✅ Completed

1. **Task 185-1**: Design document created ✅
2. **Task 185-2**: Pattern2 integration skeleton completed ✅
   - `collect_body_local_variables()` helper added
   - `body_local_env` parameter added to `lower_loop_with_break_minimal`
   - `emit_carrier_update_with_env()` integration
   - Build succeeds (no compilation errors)

3. **Task 185-3**: Pattern4 deferred ✅ (different architecture, inline lowering)

### ❌ Blocked

**Task 185-4**: Test execution BLOCKED by missing body-local init lowering

**Error**: `use of undefined value ValueId(11)` for body-local variable `digit_pos`

**Root cause**: Phase 184 implemented storage/resolution infrastructure but left **initialization lowering** unimplemented.

**What's missing**:
1. Body-local init expression lowering (`local digit_pos = pos - start`)
2. JoinIR instruction generation for init expressions
3. Insertion of init instructions in loop body

**Current behavior**:
- ✅ Variables are collected (name → ValueId mapping)
- ✅ UpdateEnv can resolve body-local variable names
- ❌ Init expressions are NOT lowered to JoinIR
- ❌ ValueIds are allocated but never defined

**Evidence**:
```
[pattern2/body-local] Collected local 'digit_pos' → ValueId(2)  ✅ Name mapping OK
[pattern2/body-local] Phase 185-2: Collected 1 body-local variables  ✅ Collection OK
[ERROR] use of undefined value ValueId(11)  ❌ Init not lowered
```

### Scope Clarification

**Phase 184** scope:
- LoopBodyLocalEnv (storage) ✅
- UpdateEnv (resolution) ✅
- emit_carrier_update_with_env() (emission) ✅
- **Body-local init lowering**: ⚠️ NOT IMPLEMENTED

**Phase 185** intended scope:
- Pattern2/4 integration ✅ (Pattern2 skeleton done)
- **Assumed** init lowering was in Phase 184 ❌ (incorrect assumption)

**Actual blocker**: Init lowering is **Phase 186 work**, not Phase 185.

---

## Next Phase: Phase 186 - Body-local Init Lowering

### Goal

Implement body-local variable initialization lowering to make Phase 185 integration functional.

### Required Changes

#### 1. Modify collect_body_local_variables()

**Current** (Phase 185):
```rust
fn collect_body_local_variables(body: &[ASTNode], alloc: &mut dyn FnMut() -> ValueId) -> Vec<(String, ValueId)> {
    // Only allocates ValueIds, doesn't lower init expressions
    for node in body {
        if let ASTNode::Local { variables, .. } = node {
            for name in variables {
                let value_id = alloc();  // Allocated but never defined!
                locals.push((name.clone(), value_id));
            }
        }
    }
}
```

**Needed** (Phase 186):
```rust
fn collect_and_lower_body_locals(
    body: &[ASTNode],
    env: &ConditionEnv,
    alloc: &mut dyn FnMut() -> ValueId,
    instructions: &mut Vec<JoinInst>,  // Need to emit init instructions!
) -> Result<Vec<(String, ValueId)>, String> {
    for node in body {
        if let ASTNode::Local { variables, initial_values, .. } = node {
            for (name, init_expr_opt) in variables.iter().zip(initial_values.iter()) {
                if let Some(init_expr) = init_expr_opt {
                    // Lower init expression to JoinIR
                    let init_value_id = lower_expr_to_joinir(init_expr, env, alloc, instructions)?;
                    locals.push((name.clone(), init_value_id));
                } else {
                    // No init: allocate but leave undefined (or use Void constant)
                    let value_id = alloc();
                    locals.push((name.clone(), value_id));
                }
            }
        }
    }
}
```

#### 2. Add Expression Lowerer

Need a helper function to lower AST expressions to JoinIR:
```rust
fn lower_expr_to_joinir(
    expr: &ASTNode,
    env: &ConditionEnv,
    alloc: &mut dyn FnMut() -> ValueId,
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    match expr {
        ASTNode::BinOp { op, left, right, .. } => {
            let lhs = lower_expr_to_joinir(left, env, alloc, instructions)?;
            let rhs = lower_expr_to_joinir(right, env, alloc, instructions)?;
            let result = alloc();
            instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: result,
                op: map_binop(op),
                lhs,
                rhs,
            }));
            Ok(result)
        }
        ASTNode::Variable { name, .. } => {
            env.get(name).ok_or_else(|| format!("Variable '{}' not in scope", name))
        }
        // ... handle other expression types
    }
}
```

#### 3. Update lower_loop_with_break_minimal

Insert body-local init instructions at the start of loop_step function:
```rust
// After allocating loop_step parameters, before break condition:
if let Some(body_env) = body_local_env {
    // Emit body-local init instructions
    for (name, value_id) in body_env.iter() {
        // Init instructions already emitted by collect_and_lower_body_locals
        // Just log for debugging
        eprintln!("[loop_step] Body-local '{}' initialized as {:?}", name, value_id);
    }
}
```

### Estimate

- Helper function (lower_expr_to_joinir): 2-3 hours (complex, many AST variants)
- collect_and_lower_body_locals refactor: 1 hour
- Integration into lower_loop_with_break_minimal: 1 hour
- Testing and debugging: 2 hours

**Total**: 6-7 hours for Phase 186

### Alternative: Simplified Scope

If full expression lowering is too complex, **Phase 186-simple** could:
1. Only support **variable references** in body-local init (no binops)
   - `local temp = i` ✅
   - `local temp = i + 1` ❌ (Phase 187)
2. Implement just variable copying
3. Get tests passing with simple cases
4. Defer complex expressions to Phase 187

**Estimate for Phase 186-simple**: 2-3 hours

---

## Lessons Learned

1. **Phase 184 scope was incomplete**: Infrastructure without lowering is not functional
2. **Testing earlier would have caught this**: Phase 184 should have had E2E test
3. **Phase 185 assumption was wrong**: Assumed init lowering was done, it wasn't
4. **Clear scope boundaries needed**: "Infrastructure" vs "Full implementation"
Status: Historical
