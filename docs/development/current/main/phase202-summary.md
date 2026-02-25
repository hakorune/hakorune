# Phase 202: Pattern 1-4 JoinValueSpace Unification

**Status**: ✅ Complete
**Date**: 2025-12-09
**Commits**:
- `6e778948` Phase 202-A (Pattern 1)
- `98e81b26` Phase 202-B (Pattern 3)
- `ae741d97` Phase 202-C (Pattern 4)

## Overview

Phase 202 unified all JoinIR loop patterns (Pattern 1-4) to use the **JoinValueSpace** system introduced in Phase 201, eliminating manual ValueId allocation counters and ensuring complete ValueId collision safety through region separation.

## Motivation

Phase 201 successfully migrated Pattern 2 to JoinValueSpace, revealing the benefits of unified ValueId allocation:
- **Safety**: Disjoint regions (PHI/Param/Local) prevent collisions
- **Consistency**: Single allocation mechanism across all patterns
- **Maintainability**: No manual counter management
- **Debuggability**: Clear region boundaries (100, 1000)

Phase 202 extended this to the remaining patterns (1, 3, 4) to achieve complete architectural consistency.

## Implementation Summary

### Phase 202-A: Pattern 1 (Simple While)

**File Changes**:
- `simple_while_minimal.rs`: Replace `value_counter` with `JoinValueSpace`
- `pattern1_minimal.rs`: Create and pass `JoinValueSpace` to lowerer
- `loop_view_builder.rs`: Create `JoinValueSpace` in router

**ValueId Usage**:
| Region | Usage |
|--------|-------|
| PHI Reserved (0-99) | ❌ Not used |
| Param (100-999) | ❌ Not used (no ConditionEnv) |
| Local (1000+) | ✅ All temps (Const, Compare, UnaryOp) |

**Why Pattern 1 is simpler**: No break conditions → No ConditionEnv → No Param region needed

**Test Results**:
```bash
$ cargo test --release --lib pattern
✅ 119 passed

$ ./target/release/hakorune apps/tests/loop_min_while.hako
✅ Output: "0 1 2"
```

### Phase 202-B: Pattern 3 (If-Else PHI)

**File Changes**:
- `loop_with_if_phi_minimal.rs`: Replace `value_counter` with `JoinValueSpace`
- `pattern3_with_if_phi.rs`: Create and pass `JoinValueSpace` to lowerer
- `loop_patterns/with_if_phi.rs`: Update legacy wrapper

**ValueId Usage**:
| Region | Usage |
|--------|-------|
| PHI Reserved (0-99) | ❌ Not used |
| Param (100-999) | ❌ Not used (PHI values from ConditionEnv) |
| Local (1000+) | ✅ All temps (PHI, Select, BinOp) |

**Test Results**:
```bash
$ cargo test --release --lib if_phi
✅ 5/5 passed

$ ./target/release/hakorune apps/tests/loop_if_phi.hako
✅ Output: "9" (sum)

$ ./target/release/hakorune apps/tests/loop_if_phi_continue.hako
✅ Output: "9" (sum with continue)
```

### Phase 202-C: Pattern 4 (Continue)

**File Changes**:
- `loop_with_continue_minimal.rs`: Replace dual counters with `JoinValueSpace`
- `pattern4_with_continue.rs`: Create and pass `JoinValueSpace` to lowerer

**Dual Counter Problem (Before)**:
```rust
// Two separate counters - collision risk!
let mut value_counter = 0u32;
let mut join_value_counter = 0u32; // Manually incremented
```

**Unified Allocation (After)**:
```rust
// Single source of truth - no collision possible
let mut join_value_space = JoinValueSpace::new();
let mut alloc_value = || join_value_space.alloc_local();
let mut alloc_param = || join_value_space.alloc_param();
```

**ValueId Usage**:
| Region | Usage |
|--------|-------|
| PHI Reserved (0-99) | ❌ Not used |
| Param (100-999) | ✅ ConditionEnv variables |
| Local (1000+) | ✅ All temps (Select, BinOp, Const) |

**Test Results**:
```bash
$ cargo test --release --lib continue
✅ 11 passed (3 ignored)

$ ./target/release/hakorune apps/tests/loop_continue_pattern4.hako
✅ Output: "25" (single carrier)

$ ./target/release/hakorune apps/tests/loop_continue_multi_carrier.hako
✅ Output: "100\n10" (multi carrier)
```

## Comparison Table: Before vs After

### Before Phase 202 (Manual Counters)

| Pattern | Allocation Method | Collision Risk |
|---------|------------------|----------------|
| Pattern 1 | `value_counter = 0u32` | ⚠️ Yes (overlaps with ConditionEnv if added) |
| Pattern 2 | `value_counter = 0u32` | ⚠️ Yes (collided with Param region) |
| Pattern 3 | `value_counter = 0u32` | ⚠️ Yes (overlaps with ConditionEnv if added) |
| Pattern 4 | `value_counter + join_value_counter` | ⚠️ Yes (dual counter management) |

### After Phase 202 (JoinValueSpace)

| Pattern | Allocation Method | Collision Risk |
|---------|------------------|----------------|
| Pattern 1 | `JoinValueSpace.alloc_local()` | ✅ No (Local region isolated) |
| Pattern 2 | `JoinValueSpace.alloc_param() + alloc_local()` | ✅ No (Param/Local disjoint) |
| Pattern 3 | `JoinValueSpace.alloc_local()` | ✅ No (Local region isolated) |
| Pattern 4 | `JoinValueSpace.alloc_param() + alloc_local()` | ✅ No (Param/Local disjoint) |

## Region Usage Matrix

| Pattern | PHI (0-99) | Param (100-999) | Local (1000+) |
|---------|------------|-----------------|---------------|
| Pattern 1 | ❌ | ❌ | ✅ Const, Compare, UnaryOp |
| Pattern 2 | ❌ | ✅ ConditionEnv, CarrierInfo | ✅ Const, BinOp, temps |
| Pattern 3 | ❌ | ❌ | ✅ PHI, Select, BinOp |
| Pattern 4 | ❌ | ✅ ConditionEnv (continue vars) | ✅ Select, BinOp, temps |

**Key Insight**: Patterns 1 and 3 only need Local region (simple structure), while Patterns 2 and 4 need both Param and Local regions (complex condition analysis).

## Benefits Achieved

### 1. Safety
- **ValueId collision impossible**: Disjoint regions (PHI/Param/Local) guarantee no overlap
- **Contract enforcement**: Debug-mode assertions catch violations early
- **Future-proof**: Easy to add new allocation patterns within regions

### 2. Consistency
- **Single allocation mechanism**: All patterns use JoinValueSpace
- **Unified API**: `alloc_param()` / `alloc_local()` across all patterns
- **Maintainable**: No pattern-specific counter logic

### 3. Debuggability
- **Clear region boundaries**: ValueId ranges reveal allocation source
  - 100-999 → "This is a Param (ConditionEnv/CarrierInfo)"
  - 1000+ → "This is a Local (intermediate value)"
- **Traceable**: JoinValueSpace provides single point of debug logging

### 4. Code Quality
- **75 lines removed** (Phase 203-A dead code cleanup)
- **No manual counter management**: Eliminated error-prone increment logic
- **Pattern 4 dual counter eliminated**: Simplified from 2 counters to 1 allocator

## Test Coverage

### Build Status
```bash
$ cargo build --release --lib
✅ Success (0 errors, 4 warnings)
```

### Unit Tests
```bash
$ cargo test --release --lib
✅ 821 passed; 0 failed; 64 ignored
```

### E2E Tests (Representative Cases)

| Test File | Pattern | Expected Output | Result |
|-----------|---------|----------------|--------|
| loop_min_while.hako | P1 | "0 1 2" | ✅ PASS |
| minimal_ssa_bug_loop.hako | P2 | RC: 0 | ✅ PASS |
| loop_if_phi.hako | P3 | "9" | ✅ PASS |
| loop_continue_pattern4.hako | P4 | "25" | ✅ PASS |
| loop_continue_multi_carrier.hako | P4 | "100\n10" | ✅ PASS |
| phase200d_capture_minimal.hako | P2 | "30" | ✅ PASS |

**No regressions**: All existing tests continue to pass.

## Architecture Impact

### Invariant Strengthening

**Added to Section 1.9 of joinir-architecture-overview.md**:
- ValueId space diagram (PHI/Param/Local regions)
- Component-to-region mapping table
- Design principles (fixed boundaries, reserve_phi vs alloc_phi)
- Relationship with value_id_ranges.rs (module-level vs intra-lowering)

### Component Updates

**All pattern lowerers now follow the same structure**:
```rust
pub fn lower_pattern_X(
    // ... pattern-specific params
    join_value_space: &mut JoinValueSpace,
) -> Result<JoinModule, ...> {
    let mut alloc_local = || join_value_space.alloc_local();
    let mut alloc_param = || join_value_space.alloc_param();
    // ... lowering logic
}
```

**Callers create JoinValueSpace before calling lowerer**:
```rust
let mut join_value_space = JoinValueSpace::new();
let result = lower_pattern_X(..., &mut join_value_space)?;
```

## Related Work

### Phase 201 (Foundation)
- Introduced JoinValueSpace box
- Migrated Pattern 2 as reference implementation
- Established Param/Local region separation
- Validated with 821 unit tests

### Phase 203-A (Cleanup)
- Removed obsolete v1 API (70 lines)
- Converted 3 unit tests to v2 API
- Removed 2 unused imports
- Documented stub functions

### Future Phases (Phase 204+)
- Pattern 5 (Trim/JsonParser) integration
- Advanced carrier analysis with JoinValueSpace
- Potential PHI reservation usage (currently unused)

## Commit History

```
6e778948 feat(joinir): Phase 202-A Pattern 1 uses JoinValueSpace
98e81b26 feat(joinir): Phase 202-B Pattern 3 uses JoinValueSpace
ae741d97 feat(joinir): Phase 202-C Pattern 4 uses JoinValueSpace, unify dual counters
```

## References

- **Phase 201**: JoinValueSpace design and Pattern 2 migration
- **JoinValueSpace Implementation**: `src/mir/join_ir/lowering/join_value_space.rs`
- **Pattern Lowerers**:
  - `simple_while_minimal.rs` (Pattern 1)
  - `loop_with_break_minimal.rs` (Pattern 2)
  - `loop_with_if_phi_minimal.rs` (Pattern 3)
  - `loop_with_continue_minimal.rs` (Pattern 4)
- **Architecture Overview**: `joinir-architecture-overview.md` Section 1.9
- **Design Document**: `phase201-join-value-space-design.md`
- **Pattern 1 Details**: `phase202-a-pattern1-joinvaluespace.md`

## Conclusion

Phase 202 achieved complete architectural consistency across all JoinIR loop patterns by migrating Pattern 1, 3, and 4 to the unified JoinValueSpace system. This eliminates all ValueId collision risks, simplifies maintenance, and establishes a solid foundation for future JoinIR enhancements.

**Key Achievement**: 4/4 patterns (100%) now use JoinValueSpace, with 0 manual counter systems remaining in the codebase.
Status: Active  
Scope: Join Value Space 適用サマリー（JoinIR v2）
