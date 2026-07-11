# Phase 134 P0: Normalization Entry Point Consolidation

**Date**: 2025-12-18
**Status**: ✅ Complete
**Scope**: Unified entry point for Normalized shadow detection and execution

---

## Background

**Problem (Before)**:
- **Dual entry points** with duplicated responsibility:
  - `try_normalized_shadow()` in routing.rs (loop-only patterns)
  - `suffix_router_box` in patterns/policies/ (loop + post statements)
- **Decision logic scattered**: "What to lower" was decided in two different places
- **Maintenance burden**: Changes to pattern detection required updates in multiple locations

---

## Solution (Phase 134 P0)

**Unified decision point** using Box-First architecture:
1. **NormalizationPlanBox**: SSOT for pattern detection ("what to normalize")
2. **NormalizationExecuteBox**: SSOT for execution ("how to execute")
3. **Both entry points** use the same PlanBox for consistent decisions

---

## Implementation

### New Module Structure

`src/mir/builder/control_flow/normalization/` (5 files created):
- **README.md**: Design documentation and contract (SSOT)
- **plan.rs**: NormalizationPlan data structure
- **plan_box.rs**: NormalizationPlanBox (pattern detection)
- **execute_box.rs**: NormalizationExecuteBox (execution logic)
- **mod.rs**: Module integration

### NormalizationPlan Structure

```rust
pub struct NormalizationPlan {
    pub consumed: usize,          // Statements to consume from remaining
    pub kind: PlanKind,           // What to lower
    pub requires_return: bool,    // Whether pattern includes return
}

pub enum PlanKind {
    LoopOnly,                     // Phase 131: loop(true) alone
    LoopWithPost {                // Phase 132-133: loop + assigns + return
        post_assign_count: usize,
    },
}
```

### NormalizationPlanBox API

```rust
pub fn plan_block_suffix(
    builder: &MirBuilder,
    remaining: &[ASTNode],
    func_name: &str,
    debug: bool,
) -> Result<Option<NormalizationPlan>, String>
```

**Returns**:
- `Ok(Some(plan))`: Pattern detected, proceed with normalization
- `Ok(None)`: Not a normalized pattern, use legacy fallback
- `Err(_)`: Internal error

### NormalizationExecuteBox API

```rust
pub fn execute(
    builder: &mut MirBuilder,
    plan: &NormalizationPlan,
    remaining: &[ASTNode],
    func_name: &str,
    debug: bool,
) -> Result<ValueId, String>
```

**Side Effects**:
- Modifies builder state (adds blocks, instructions)
- Updates variable_map with exit values (DirectValue mode)
- May emit return statement (for suffix patterns)

---

## Code Changes

### 1. normalized_shadow_suffix_router_box.rs

**Before** (258 lines):
- Pattern detection logic (50+ lines)
- Merge logic (50+ lines)
- StepTree building and lowering

**After** (116 lines, -142 lines):
- Delegates to NormalizationPlanBox for detection
- Delegates to NormalizationExecuteBox for execution
- Only handles suffix-specific logic (return emission)

### 2. routing.rs try_normalized_shadow()

**Before** (165 lines):
- StepTree building
- AvailableInputsCollector
- Boundary creation
- Merge logic
- Exit reconnection

**After** (87 lines, -78 lines):
- Delegates to NormalizationPlanBox for detection
- Delegates to NormalizationExecuteBox for execution
- Pattern kind validation (loop-only vs loop+post)

---

## Pattern Detection Logic

### Phase 131: Loop-Only
- **Pattern**: `loop(true) { ... break }`
- **Consumed**: 1 statement
- **Example**: `loop(true) { x = 1; break }; return x`

### Phase 132: Loop + Single Post
- **Pattern**: `loop(true) { ... break }; <assign>; return <expr>`
- **Consumed**: 3 statements
- **Example**: `loop(true) { x = 1; break }; x = x + 2; return x`

### Phase 133: Loop + Multiple Post
- **Pattern**: `loop(true) { ... break }; <assign>+; return <expr>`
- **Consumed**: 2 + N statements
- **Example**: `loop(true) { x = 1; break }; x = x + 2; x = x + 3; return x`

---

## Testing

### Unit Tests (7 tests)
Location: `src/mir/builder/control_flow/normalization/plan_box.rs`

✅ test_plan_block_suffix_phase131_loop_only
✅ test_plan_block_suffix_phase132_loop_post_single
✅ test_plan_block_suffix_phase133_loop_post_multi
✅ test_plan_block_suffix_return_boundary
✅ test_plan_block_suffix_no_match_empty
✅ test_plan_block_suffix_no_match_not_loop
✅ test_plan_block_suffix_no_match_no_return

### Lib Tests
```bash
cargo test --lib --package nyash-rust
```
Result: **1186 passed; 0 failed; 56 ignored**

### Regression Smokes

✅ **Phase 133 VM**: `phase133_loop_true_break_once_post_multi_add_vm.sh` - PASS
✅ **Phase 133 LLVM**: `phase133_loop_true_break_once_post_multi_add_llvm_exe.sh` - PASS (exit code 6)
✅ **Phase 132 LLVM**: `phase132_loop_true_break_once_post_add_llvm_exe.sh` - PASS (exit code 3)
✅ **Phase 131 LLVM**: `phase131_loop_true_break_once_llvm_exe.sh` - PASS (exit code 1)
✅ **Phase 131 VM**: `phase131_loop_true_break_once_vm.sh` - PASS

---

## Benefits

### Code Quality
- **-220 lines** of duplicated code eliminated
- **Single responsibility**: Each Box has one clear purpose
- **Testability**: Plan detection can be tested independently

### Maintainability
- **SSOT**: Pattern detection in one place (NormalizationPlanBox)
- **Documentation**: Comprehensive README.md with contract
- **Clear separation**: Detection vs execution

### Future-Proofing
- **Extensible**: Easy to add new pattern kinds
- **Flexible**: ExecuteBox can be swapped for different implementations
- **Traceable**: Debug logging at each decision point

---

## Design Principles Applied

### Box-First
- **Plan Box**: Single responsibility for detection
- **Execute Box**: Single responsibility for execution
- Clear boundaries enable independent evolution

### SSOT (Single Source of Truth)
- **Entry**: NormalizationPlanBox is the only place for normalization decisions
- **Contract**: Documented in normalization/README.md
- **No duplication**: Both entry points use the same logic

### Fail-Fast
- Invalid patterns return `Err` (not silent fallback)
- STRICT mode treats contract violations as panics
- Clear error messages with hints

### Legacy Preservation
- Existing behavior unchanged (dev-only guard)
- Non-normalized patterns return `Ok(None)` → legacy fallback
- No breaking changes to existing smokes

---

## Files Modified

- `src/mir/builder/control_flow/normalization/` (NEW module, 5 files)
- `src/mir/builder/control_flow/mod.rs` (added normalization module)
- `src/mir/builder/control_flow/joinir/patterns/policies/normalized_shadow_suffix_router_box.rs` (refactored, -142 lines)
- `src/mir/builder/control_flow/joinir/routing.rs` (refactored, -78 lines)

**Net Change**: +~300 lines (new module), -220 lines (refactoring) = +80 lines overall

---

## Acceptance Criteria

- ✅ **Entry SSOT**: normalization/README.md documents the contract
- ✅ **By-name avoidance**: Uses boundary contract SSOT
- ✅ **cargo test --lib**: 1186/1186 PASS
- ✅ **Phase 133 smokes**: 2/2 PASS (VM + LLVM EXE)
- ✅ **Phase 131/132 regression**: PASS
- ✅ **Default behavior unchanged**: Dev-only guard maintained

---

## Next Steps

### P1: Additional Patterns (Future)
- **Phase 135+**: Support for more complex post-loop patterns
- **Conditional post**: `if (cond) { assign }; return`
- **Nested structures**: Multiple loops with post statements

### P2: Performance Optimization (Future)
- **Caching**: Pattern detection results
- **Lazy evaluation**: Only build StepTree when needed
- **Parallel detection**: Check multiple patterns simultaneously

### P3: Enhanced Debugging (Future)
- **Structured tracing**: JSON-formatted trace output
- **Visualization**: DOT graph of normalization decisions
- **Metrics**: Track pattern match rates

---

## References

- **Module**: `src/mir/builder/control_flow/normalization/`
- **Contract**: `src/mir/builder/control_flow/normalization/README.md`
- **Tests**: `src/mir/builder/control_flow/normalization/plan_box.rs::tests`
- **Phase 131**: loop(true) break-once Normalized
- **Phase 132**: loop(true) + post-loop minimal
- **Phase 133**: loop(true) + multiple post-loop assigns
