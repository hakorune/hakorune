# Phase 205: ValueId Region Boundaries - Design Document

**Author**: Claude Sonnet 4.5
**Date**: 2025-12-09
**Status**: In Progress

## Overview

Phase 205 establishes strict ValueId region contracts for JoinIR lowering, completing the Box-First architecture started in Phase 201. This phase ensures that ValueId allocation is:

1. **Predictable**: Each ValueId belongs to a clearly defined region
2. **Verifiable**: Region violations are detected in debug mode
3. **Maintainable**: All allocation goes through JoinValueSpace Box

## ValueId Region Architecture

### Region Layout

```text
 0          100        1000                     u32::MAX
 ├──────────┼──────────┼──────────────────────────┤
 │  PHI     │  Param   │       Local             │
 │  Reserved│  Region  │       Region            │
 └──────────┴──────────┴──────────────────────────┘
```

### Region Definitions

| Region | Range | Purpose | Examples |
|--------|-------|---------|----------|
| **PHI Reserved** | 0-99 | LoopHeader PHI destinations | `phi_dst: ValueId(0)` |
| **Param Region** | 100-999 | Loop arguments & environment | `Condition.bool_id`, `Carrier.join_id`, `CapturedEnv` |
| **Local Region** | 1000+ | JoinIR-internal values | Const, BinOp, Load, etc. |

### Constants (Phase 205)

```rust
// Explicit region boundaries
pub const PHI_RESERVED_MIN: u32 = 0;
pub const PHI_RESERVED_MAX: u32 = 99;
pub const PARAM_MIN: u32 = 100;
pub const PARAM_MAX: u32 = 999;
pub const LOCAL_MIN: u32 = 1000;
pub const LOCAL_MAX: u32 = 100000;
```

## Box-First Design

### ValueIdAllocator Box (JoinValueSpace)

**Responsibility**: Single Source of Truth for ValueId allocation

**API**:
```rust
impl JoinValueSpace {
    // Primary allocation methods
    pub fn alloc_param(&mut self) -> ValueId;  // Returns 100+
    pub fn alloc_local(&mut self) -> ValueId;  // Returns 1000+
    pub fn reserve_phi(&mut self, id: ValueId); // Marks PHI dst

    // Phase 205: Enhanced verification
    pub fn verify_region(&self, id: ValueId, expected: Region) -> Result<(), String>;
    pub fn check_collision(&self, id: ValueId, role: &str); // debug-only
}
```

**Invariants**:
1. `alloc_param()` never returns id >= 1000
2. `alloc_local()` never returns id < 1000
3. No ValueId is allocated twice
4. PHI dst always in range 0-99

### RegionVerifier Box

**Responsibility**: Verify region contracts at merge boundaries

**Location**: `src/mir/builder/control_flow/joinir/merge/mod.rs`

**API**:
```rust
#[cfg(debug_assertions)]
fn verify_valueid_regions(
    boundary: &JoinInlineBoundary,
    loop_info: &LoopHeaderPhiInfo,
    join_value_space: &JoinValueSpace,
);
```

**Checks**:
1. All `boundary.join_inputs` are in Param region
2. All `carrier_phis[].phi_dst` are in valid range (<= LOCAL_MAX)
3. No overlap between Param and Local regions
4. PHI reservations are in PHI Reserved region

## ValueId Role Mapping

### Param Region (100-999)

| Role | Allocated By | Example |
|------|-------------|---------|
| **Condition.bool_id** | `condition_env_builder.rs` | `ValueId(100)` |
| **Carrier.join_id** | Pattern frontend (P1/P2/P3/P4) | `ValueId(101)`, `ValueId(102)` |
| **CapturedEnv vars** | Pattern frontend | `ValueId(103+)` |
| **Boundary inputs** | `common_init.rs` | `ValueId(104+)` |

### Local Region (1000+)

| Role | Allocated By | Example |
|------|-------------|---------|
| **Const values** | Lowerers (pattern1-4, trim) | `ValueId(1000)` |
| **BinOp results** | Lowerers | `ValueId(1001)` |
| **Load results** | Lowerers | `ValueId(1002)` |
| **Intermediate values** | Lowerers | `ValueId(1003+)` |

### PHI Reserved (0-99)

| Role | Allocated By | Example |
|------|-------------|---------|
| **PHI dst** | MirBuilder (host side) | `ValueId(0)`, `ValueId(1)` |

**Note**: PHI dst comes from host MirBuilder, NOT JoinValueSpace. `reserve_phi()` is for verification only.

## Current State Inventory (Task 205-2)

### Pattern 1 (Minimal)

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs`

**Status**: ✅ Fully integrated with JoinValueSpace

**Allocation Sites**:
- ConditionEnv: Uses `alloc_param()` via `condition_env_builder.rs`
- Carrier (i): Uses `alloc_param()` in frontend
- Lowerer: Uses `alloc_local()` for all JoinIR values

**Raw ValueId Usage**: None detected

### Pattern 2 (With Break)

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

**Status**: ✅ Fully integrated with JoinValueSpace

**Allocation Sites**:
- ConditionEnv: Uses `alloc_param()` via `condition_env_builder.rs`
- Carrier (v): Uses `alloc_param()` in frontend
- Lowerer: Uses `alloc_local()` for all JoinIR values

**Raw ValueId Usage**: None detected

**Historical Note**: Pattern 2 was the original motivation for Phase 201 - previously had collision between `alloc_join_value()` (param) and `alloc_value()` (local starting from 0).

### Pattern 3 (With If-PHI)

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`

**Status**: ⚠️ Needs verification

**Allocation Sites**:
- ConditionEnv: Uses `alloc_param()` via `condition_env_builder.rs`
- Carriers (sum, count): Uses `alloc_param()` in frontend
- Lowerer: Uses `alloc_local()` for all JoinIR values

**Potential Issues**:
- If-PHI lowering: Need to verify all temporary values use `alloc_local()`
- ExitLine reconnection: Verify no raw `ValueId(..)` usage

**Action Required**: Task 205-5 will audit

### Pattern 4 (With Continue)

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

**Status**: ⚠️ Needs verification

**Allocation Sites**:
- ConditionEnv: Uses `alloc_param()` via `condition_env_builder.rs`
- Carriers: Uses `alloc_param()` in frontend
- Lowerer: Uses `alloc_local()` for all JoinIR values

**Potential Issues**:
- Continue-pattern has more complex control flow
- UpdateSummary handling: Verify all intermediate values use `alloc_local()`

**Action Required**: Task 205-5 will audit

### Trim Pattern Lowerer

**File**: `src/mir/builder/control_flow/joinir/patterns/trim_pattern_lowerer.rs`

**Status**: ⚠️ Needs verification

**Allocation Sites**:
- Uses `alloc_fn: &mut dyn FnMut() -> ValueId` pattern
- Should receive `space.local_allocator()` closure

**Potential Issues**:
- Multiple lowerer sites (JsonParser, other Trim use cases)
- Need to ensure all call sites pass `space.local_allocator()`

**Action Required**: Task 205-5 will audit

### ConditionEnv Builder

**File**: `src/mir/builder/control_flow/joinir/patterns/condition_env_builder.rs`

**Status**: ✅ Already uses `alloc_param()`

**Implementation**:
```rust
pub fn build_condition_env(
    condition_ast: &AstNode,
    join_value_space: &mut JoinValueSpace,
    // ...
) -> Result<ConditionEnv, String> {
    let bool_id = join_value_space.alloc_param(); // ✅ Correct
    // ...
}
```

### Exit Binding & Common Init

**Files**:
- `src/mir/builder/control_flow/joinir/patterns/exit_binding.rs`
- `src/mir/builder/control_flow/joinir/patterns/common_init.rs`

**Status**: ⚠️ Needs verification

**Potential Issues**:
- Exit binding may create temporary ValueIds
- Common init should use `alloc_param()` for boundary inputs

**Action Required**: Task 205-5 will audit

## Implementation Plan

### Task 205-3: ValueIdAllocator Box Enhancement

**Changes to** `src/mir/join_ir/lowering/join_value_space.rs`:

```rust
// Add explicit max constants
pub const LOCAL_MAX: u32 = 100000;

// Add collision detection (debug-only)
#[cfg(debug_assertions)]
fn check_collision(&self, id: ValueId, role: &str) {
    if self.allocated_ids.contains(&id) {
        panic!(
            "[JoinValueSpace] ValueId collision: {:?} already allocated (role: {})",
            id, role
        );
    }
}

// Add region verification
#[cfg(debug_assertions)]
pub fn verify_region(&self, id: ValueId, expected_region: Region) -> Result<(), String> {
    let actual = self.region_of(id);
    if actual != expected_region {
        return Err(format!(
            "ValueId {:?} is in {:?} region, expected {:?}",
            id, actual, expected_region
        ));
    }
    Ok(())
}

// Track allocated IDs (debug-only)
#[cfg(debug_assertions)]
allocated_ids: HashSet<u32>,

// Update alloc_param/alloc_local to track allocations
#[cfg(debug_assertions)]
pub fn alloc_param(&mut self) -> ValueId {
    let id = self.next_param;
    debug_assert!(id < LOCAL_BASE, "Param region overflow");
    self.check_collision(ValueId(id), "param");
    self.allocated_ids.insert(id);
    self.next_param += 1;
    ValueId(id)
}
```

### Task 205-4: RegionVerifier Box Implementation

**Location**: `src/mir/builder/control_flow/joinir/merge/mod.rs`

**Integration Point**: Add to existing `verify_joinir_contracts()` function

```rust
#[cfg(debug_assertions)]
fn verify_joinir_contracts(
    func: &JoinIRFunction,
    boundary: &JoinInlineBoundary,
    loop_info: &LoopHeaderPhiInfo,
    join_value_space: &JoinValueSpace,
) {
    // Existing PHI contract verification
    verify_phi_contracts(func, loop_info);

    // Phase 205: Add region verification
    verify_valueid_regions(boundary, loop_info, join_value_space);
}

#[cfg(debug_assertions)]
fn verify_valueid_regions(
    boundary: &JoinInlineBoundary,
    loop_info: &LoopHeaderPhiInfo,
    join_value_space: &JoinValueSpace,
) {
    // 1. Verify boundary inputs are in Param region
    for join_id in &boundary.join_inputs {
        let region = join_value_space.region_of(*join_id);
        if region != Region::Param {
            panic!(
                "[RegionVerifier] Boundary input {:?} is in {:?} region, expected Param",
                join_id, region
            );
        }
    }

    // 2. Verify PHI dst are in valid range
    for (carrier_name, entry) in &loop_info.carrier_phis {
        let region = join_value_space.region_of(entry.phi_dst);
        // PHI dst may be in PHI Reserved or early Param range (depending on MirBuilder)
        if entry.phi_dst.0 > LOCAL_MAX {
            panic!(
                "[RegionVerifier] Carrier '{}' PHI dst {:?} exceeds LOCAL_MAX",
                carrier_name, entry.phi_dst
            );
        }
    }

    // 3. Verify JoinValueSpace internal consistency
    if let Err(e) = join_value_space.verify_no_overlap() {
        panic!("[RegionVerifier] JoinValueSpace overlap detected: {}", e);
    }
}
```

### Task 205-5: Pattern Integration Audit

**Files to Audit**:
1. `pattern1_minimal.rs` - ✅ Already correct
2. `pattern2_with_break.rs` - ✅ Already correct
3. `pattern3_with_if_phi.rs` - ⚠️ Verify If-PHI lowering
4. `pattern4_with_continue.rs` - ⚠️ Verify UpdateSummary handling
5. `trim_pattern_lowerer.rs` - ⚠️ Verify all call sites
6. `exit_binding.rs` - ⚠️ Verify no raw ValueId usage
7. `common_init.rs` - ⚠️ Verify boundary input allocation

**Audit Checklist**:
- [ ] No raw `ValueId(..)` construction in lowerers
- [ ] All Carrier `join_id` use `alloc_param()`
- [ ] All lowerer intermediate values use `alloc_local()`
- [ ] All `alloc_fn` closures receive `space.local_allocator()`

**Fix Strategy**:
```rust
// ❌ Before (if found):
let temp = ValueId(next_id);
next_id += 1;

// ✅ After:
let temp = join_value_space.alloc_local();
```

### Task 205-6: Testing & Documentation

**Test Cases**:
1. `loop_min_while.hako` (Pattern 1)
2. `loop_with_break.hako` (Pattern 2)
3. `loop_if_phi.hako` (Pattern 3)
4. `loop_continue_pattern4.hako` (Pattern 4)
5. Trim/JsonParser representative case

**Expected Outcome**:
- All 821 tests pass
- No regression
- Debug assertions detect region violations (if any)

**Documentation Updates**:
1. `joinir-architecture-overview.md`:
   - Add "ValueId Region Contract" section
   - Update Box boundary diagram
   - Link to this design doc
2. `CURRENT_TASK.md`:
   - Mark Phase 205 complete
   - Add handoff notes for Phase 206

## Fail-Fast Principles

### Region Violations

**Principle**: Detect region violations immediately, fail fast with clear error messages.

**Implementation**:
```rust
#[cfg(debug_assertions)]
fn verify_region(&self, id: ValueId, expected: Region) -> Result<(), String> {
    let actual = self.region_of(id);
    if actual != expected {
        // ✅ Clear, actionable error message
        return Err(format!(
            "ValueId {:?} is in {:?} region, expected {:?}\n\
             Hint: Use alloc_param() for loop arguments, alloc_local() for JoinIR values",
            id, actual, expected
        ));
    }
    Ok(())
}
```

**No Fallback**: If a region violation occurs, panic immediately. Do not:
- Silently remap ValueIds
- Use fallback allocation
- Continue with corrupted state

### Collision Detection

**Principle**: Each ValueId allocated exactly once.

**Implementation**:
```rust
#[cfg(debug_assertions)]
fn check_collision(&self, id: ValueId, role: &str) {
    if self.allocated_ids.contains(&id.0) {
        panic!(
            "[JoinValueSpace] ValueId collision detected!\n\
             ID: {:?}\n\
             Role: {}\n\
             This indicates a bug in JoinIR lowering - contact maintainer",
            id, role
        );
    }
}
```

## Box Boundaries

### SSOT (Single Source of Truth)

**JoinValueSpace is the SSOT for JoinIR ValueId allocation.**

**Boundary Rules**:
1. ✅ **Inside JoinIR lowering**: All ValueIds come from JoinValueSpace
2. ❌ **Outside JoinIR lowering**: MirBuilder allocates PHI dst independently
3. ⚠️ **Bridge**: `reserve_phi()` synchronizes PHI dst for verification

**Example**:
```rust
// ✅ Correct: JoinIR lowering
let mut join_value_space = JoinValueSpace::new();
let carrier_id = join_value_space.alloc_param(); // Inside SSOT boundary

// ✅ Correct: MirBuilder allocates PHI dst
let phi_dst = mir_builder.alloc_value(); // Outside SSOT boundary

// ⚠️ Bridge: Sync for verification
join_value_space.reserve_phi(phi_dst); // Tell JoinValueSpace about external PHI
```

### Allocator Closures

**Pattern**: Pass allocation function to lowerers

```rust
// ✅ Correct pattern:
fn lower_pattern3(
    alloc_local: &mut dyn FnMut() -> ValueId, // Receives closure
    // ...
) {
    let const_id = alloc_local(); // ✅ Uses closure
}

// Call site:
lower_pattern3(
    &mut join_value_space.local_allocator(), // ✅ Passes JoinValueSpace closure
    // ...
);
```

**Benefits**:
- Lowerer doesn't need direct JoinValueSpace reference
- Maintains Box boundary
- Easy to test with mock allocators

## Success Criteria

Phase 205 is complete when:

1. ✅ Design document created (this file)
2. ✅ JoinValueSpace has collision detection & region verification (debug-only)
3. ✅ RegionVerifier integrated into merge verification
4. ✅ All patterns (P1/P2/P3/P4) audited for raw ValueId usage
5. ✅ All tests pass (821 tests, 0 regression)
6. ✅ Documentation updated (overview + CURRENT_TASK)

## Future Work (Phase 206+)

### Potential Enhancements

1. **Runtime Region Tracking** (if needed):
   - Track ValueId → Role mapping for better error messages
   - Example: "ValueId(105) is carrier 'sum', expected local region"

2. **Region Statistics**:
   - Report param/local/PHI usage per pattern
   - Detect potential region exhaustion early

3. **Contract Testing**:
   - Generate test cases that deliberately violate regions
   - Verify debug assertions trigger correctly

4. **Allocator Modes**:
   - Dense allocation (minimize gaps)
   - Sparse allocation (easier debugging)
   - Deterministic allocation (reproducible builds)

## References

- **Phase 201**: JoinValueSpace initial implementation
- **Phase 204**: PHI contract verification (dst overwrite, inputs sanity)
- **Box-First Principle**: CLAUDE.md Section "箱理論（Box-First）"

## Appendix: Region Math

### Current Capacity

| Region | Range | Capacity | Typical Usage |
|--------|-------|----------|---------------|
| PHI Reserved | 0-99 | 100 IDs | 1-5 PHIs per loop |
| Param | 100-999 | 900 IDs | 3-10 params per loop |
| Local | 1000-99999 | 99000 IDs | 10-1000 values per loop |

### Overflow Scenarios

**Param Overflow** (highly unlikely):
- Would require 900+ loop parameters
- Current max observed: ~10 params (Pattern 3)
- Debug assertion will catch at param #900

**Local Overflow** (theoretical):
- Would require 99000+ JoinIR instructions
- Current max observed: ~100 instructions (JsonParser)
- Would indicate pathological code generation

**PHI Overflow** (impossible):
- PHI dst allocated by MirBuilder, not JoinValueSpace
- JoinValueSpace only verifies PHI dst <= 99
- If violated, indicates bug in MirBuilder

## Version History

- **2025-12-09**: Initial design document (Claude Sonnet 4.5)
- **Phase 205-1**: Created as part of ValueId region boundary task
Status: Active  
Scope: ValueId Regions 設計（JoinIR/ValueId ライン）
