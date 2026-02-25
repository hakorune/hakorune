# Modularization Implementation Plan for Large Source Files

## Executive Summary

This plan details the modularization of three oversized source files in the Nyash MIR builder:
- **control_flow.rs** (1,632 lines) - **HIGHEST PRIORITY**
- **generic_case_a.rs** (1,056 lines) - **MEDIUM PRIORITY**
- **loopform_builder.rs** (1,166 lines) - **LOWER PRIORITY**

The strategy prioritizes **control_flow.rs** first due to its critical role in JoinIR integration and ongoing Pattern 4+ development. Each modularization is broken into incremental phases that maintain backward compatibility, with clear verification points and rollback procedures.

**Total Estimated Effort**: 15-20 hours across 2-3 weeks
**Key Principle**: Zero breaking changes, backward compatible at every phase

---

## Why control_flow.rs First?

**control_flow.rs** is the integration point for JoinIR lowering patterns, actively being extended for Pattern 4+. Modularizing now:

1. **Prevents future pain** - Pattern 4/5/6 would add another 500+ lines to an already massive file
2. **Sets the pattern** - Establishes the modularization template for other files
3. **Reduces merge conflicts** - Isolates pattern-specific changes to dedicated files
4. **Improves debuggability** - `NYASH_OPTION_C_DEBUG` traces become easier to locate
5. **Currently blocking** - The 714-line `merge_joinir_mir_blocks()` function is a code smell that makes maintenance difficult

---

# 1. control_flow.rs Modularization (HIGHEST PRIORITY)

## Current State

**File**: `src/mir/builder/control_flow.rs` (1,632 lines)

**Functions** (13 total):
- `trace_varmap()` (8 lines) - Debug utility
- `cf_block()` (4 lines) - Block entry point
- `cf_if()` (10 lines) - If entry point
- `cf_loop()` (80 lines) - Loop entry point
- `try_cf_loop_joinir()` (91 lines) - JoinIR routing logic
- `cf_loop_joinir_impl()` (247 lines) - JoinIR pattern dispatcher
- `cf_loop_pattern1_minimal()` (143 lines) - Pattern 1 lowering
- `cf_loop_pattern2_with_break()` (120 lines) - Pattern 2 lowering
- `cf_loop_pattern3_with_if_phi()` (168 lines) - Pattern 3 lowering
- **`merge_joinir_mir_blocks()` (714 lines) - LARGEST FUNCTION** ⚠️
- `cf_try_catch()` (138 lines) - Exception handling
- `extract_loop_variable_from_condition()` (31 lines) - Utility
- `cf_throw()` (23 lines) - Throw entry point

**Key Issues**:
- `merge_joinir_mir_blocks()` is 714 lines (44% of the file!)
- Pattern lowerers (1/2/3) are isolated but scattered
- JoinIR integration logic mixed with entry points
- No clear separation between routing and implementation

---

## Proposed New Structure

```
src/mir/builder/control_flow/
├── mod.rs                          (~150 lines) - Public API, entry points
├── debug.rs                        (~50 lines)  - Debug utilities (trace_varmap)
├── joinir/
│   ├── mod.rs                      (~100 lines) - JoinIR integration coordinator
│   ├── routing.rs                  (~150 lines) - try_cf_loop_joinir, dispatcher
│   ├── merge/
│   │   ├── mod.rs                  (~100 lines) - merge_joinir_mir_blocks entry point
│   │   ├── id_remapper.rs          (~150 lines) - ValueId/BlockId remapping
│   │   ├── block_allocator.rs      (~100 lines) - Block ID allocation
│   │   ├── value_collector.rs      (~100 lines) - Value collection phase
│   │   ├── instruction_rewriter.rs (~150 lines) - Instruction rewriting
│   │   └── exit_phi_builder.rs     (~100 lines) - Exit PHI construction
│   └── patterns/
│       ├── mod.rs                  (~50 lines)  - Pattern dispatcher
│       ├── pattern1_minimal.rs     (~150 lines) - Pattern 1 lowering
│       ├── pattern2_with_break.rs  (~130 lines) - Pattern 2 lowering
│       └── pattern3_with_if_phi.rs (~180 lines) - Pattern 3 lowering
├── exception/
│   ├── mod.rs                      (~50 lines)  - Exception handling API
│   ├── try_catch.rs                (~150 lines) - try/catch implementation
│   └── throw.rs                    (~30 lines)  - throw implementation
└── utils.rs                        (~50 lines)  - extract_loop_variable, etc.

Total: ~1,850 lines (13% increase for clarity, but distributed across 19 files)
Average file size: ~97 lines (vs 1,632 lines monolith)
```

---

## Phase-by-Phase Migration Plan

### Phase 1: Extract Debug Utilities (30 min)

**Goal**: Move `trace_varmap()` to a dedicated debug module.

**Steps**:
1. Create `src/mir/builder/control_flow/debug.rs`
2. Move `trace_varmap()` implementation
3. Update `mod.rs` to re-export `pub(super) use debug::*;`
4. Run verification

**Files Created**:
- `src/mir/builder/control_flow/debug.rs` (~50 lines)

**Files Modified**:
- `src/mir/builder/control_flow.rs` → `.../control_flow/mod.rs`

**Verification**:
```bash
cargo build --release
cargo test --lib
tools/smokes/v2/run.sh --profile quick --filter "loop_*"
```

**Rollback**: Delete `debug.rs`, revert imports in `mod.rs`

**Estimated Effort**: 30 minutes

---

### Phase 2: Extract Pattern Lowerers (2 hours)

**Goal**: Move Pattern 1/2/3 lowering functions to dedicated files.

**Steps**:
1. Create `src/mir/builder/control_flow/joinir/patterns/` directory
2. Create `patterns/mod.rs` with dispatcher
3. Move `cf_loop_pattern1_minimal()` to `pattern1_minimal.rs`
4. Move `cf_loop_pattern2_with_break()` to `pattern2_with_break.rs`
5. Move `cf_loop_pattern3_with_if_phi()` to `pattern3_with_if_phi.rs`
6. Update imports in `mod.rs`
7. Run verification

**Files Created**:
- `control_flow/joinir/patterns/mod.rs` (~50 lines)
- `control_flow/joinir/patterns/pattern1_minimal.rs` (~150 lines)
- `control_flow/joinir/patterns/pattern2_with_break.rs` (~130 lines)
- `control_flow/joinir/patterns/pattern3_with_if_phi.rs` (~180 lines)

**Public API Changes**: None (all functions are already `fn`, not `pub(super) fn`)

**Verification**:
```bash
cargo build --release
cargo test --lib -- --include-ignored
NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/tests/loop_min_while.hako
tools/smokes/v2/run.sh --profile quick --filter "loop_*"
```

**Rollback**: Delete `patterns/` directory, revert imports

**Estimated Effort**: 2 hours

---

### Phase 3: Extract JoinIR Routing Logic (1.5 hours)

**Goal**: Move `try_cf_loop_joinir()` and `cf_loop_joinir_impl()` to routing module.

**Steps**:
1. Create `control_flow/joinir/routing.rs`
2. Move `try_cf_loop_joinir()` implementation
3. Move `cf_loop_joinir_impl()` implementation
4. Update imports in `mod.rs`
5. Run verification

**Files Created**:
- `control_flow/joinir/routing.rs` (~150 lines)

**Files Modified**:
- `control_flow/mod.rs` (update imports)

**Verification**:
```bash
cargo build --release
cargo test --release
HAKO_JOINIR_PRINT_TOKENS_MAIN=1 ./target/release/nyash test_program.hako
```

**Rollback**: Delete `routing.rs`, revert imports

**Estimated Effort**: 1.5 hours

---

### Phase 4: Break Down merge_joinir_mir_blocks (6 hours) ⚠️ **CRITICAL**

**Goal**: Split the 714-line monster function into 6 logical modules.

**Background**: `merge_joinir_mir_blocks()` performs 6 distinct phases:
1. **Block ID allocation** (lines 864-923)
2. **Value collection** (lines 931-971)
3. **Block merging** (lines 973-1100)
4. **Instruction rewriting** (lines 1102-1400)
5. **Exit PHI construction** (lines 1402-1500)
6. **Boundary reconnection** (lines 1502-1578)

**Steps**:
1. Create `control_flow/joinir/merge/` directory structure
2. Extract `id_remapper.rs` - ID remapping utilities
3. Extract `block_allocator.rs` - Block ID allocation logic
4. Extract `value_collector.rs` - Value collection phase
5. Extract `instruction_rewriter.rs` - Instruction transformation
6. Extract `exit_phi_builder.rs` - Exit PHI construction
7. Create `merge/mod.rs` as the coordinator
8. Update imports
9. Run comprehensive verification

**Files Created**:
- `control_flow/joinir/merge/mod.rs` (~100 lines) - Coordinator
- `control_flow/joinir/merge/id_remapper.rs` (~150 lines)
- `control_flow/joinir/merge/block_allocator.rs` (~100 lines)
- `control_flow/joinir/merge/value_collector.rs` (~100 lines)
- `control_flow/joinir/merge/instruction_rewriter.rs` (~150 lines)
- `control_flow/joinir/merge/exit_phi_builder.rs` (~100 lines)

**Public API**:
```rust
// control_flow/joinir/merge/mod.rs
pub(in crate::mir::builder) fn merge_joinir_mir_blocks(
    builder: &mut MirBuilder,
    mir_module: &MirModule,
    boundary: Option<&JoinInlineBoundary>,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    // Coordinator that calls the 6 sub-modules
    let remapper = id_remapper::create_remapper(builder, mir_module, debug)?;
    let values = value_collector::collect_values(mir_module, &remapper, debug)?;
    // ... etc
}
```

**Verification** (CRITICAL - run ALL tests):
```bash
# Step 1: Build verification
cargo build --release
cargo test --lib

# Step 2: Smoke tests (ALL patterns)
tools/smokes/v2/run.sh --profile quick

# Step 3: Debug trace verification
NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/tests/loop_min_while.hako 2>&1 | grep "merge_joinir"

# Step 4: Regression check (run 3 times for determinism)
for i in 1 2 3; do
  echo "=== Run $i ==="
  cargo test --release test_loop_patterns 2>&1 | grep "test result"
done

# Step 5: Full integration test
cargo test --release --all-features
```

**Rollback**:
```bash
# If anything breaks, immediately:
rm -rf src/mir/builder/control_flow/joinir/merge
git checkout src/mir/builder/control_flow.rs
cargo build --release && cargo test
```

**Risk Mitigation**:
- Keep the original `merge_joinir_mir_blocks()` as a comment at the top of `merge/mod.rs`
- Add `#[cfg(test)]` unit tests for each sub-module
- Use feature flag `NYASH_USE_LEGACY_MERGE=1` for emergency fallback (optional)

**Estimated Effort**: 6 hours (most complex phase)

---

### Phase 5: Extract Exception Handling (1 hour)

**Goal**: Move `cf_try_catch()` and `cf_throw()` to exception module.

**Steps**:
1. Create `control_flow/exception/` directory
2. Move `cf_try_catch()` to `try_catch.rs`
3. Move `cf_throw()` to `throw.rs`
4. Create `exception/mod.rs` as coordinator
5. Update imports
6. Run verification

**Files Created**:
- `control_flow/exception/mod.rs` (~50 lines)
- `control_flow/exception/try_catch.rs` (~150 lines)
- `control_flow/exception/throw.rs` (~30 lines)

**Verification**:
```bash
cargo build --release
cargo test --lib -- exception
```

**Rollback**: Delete `exception/` directory, revert imports

**Estimated Effort**: 1 hour

---

### Phase 6: Extract Utilities (30 min)

**Goal**: Move `extract_loop_variable_from_condition()` to utils module.

**Steps**:
1. Create `control_flow/utils.rs`
2. Move utility functions
3. Update imports
4. Run verification

**Files Created**:
- `control_flow/utils.rs` (~50 lines)

**Verification**:
```bash
cargo build --release
cargo test --lib
```

**Rollback**: Delete `utils.rs`, revert imports

**Estimated Effort**: 30 minutes

---

### Phase 7: Final Cleanup & Documentation (1 hour)

**Goal**: Clean up `mod.rs`, add module documentation, verify all imports.

**Steps**:
1. Review `control_flow/mod.rs` for clarity
2. Add module-level documentation to each file
3. Ensure all `pub(super)` visibility is correct
4. Run final comprehensive verification
5. Update CLAUDE.md with new structure

**Documentation Template**:
```rust
//! Pattern 1 Minimal Loop Lowering
//!
//! This module implements the simplest JoinIR loop lowering pattern:
//! - Single loop variable
//! - No break statements
//! - Simple condition (i < N)
//!
//! Used by: minimal_ssa_skip_ws, simple while loops
//! Phase: 188 (Pattern 1 implementation)
```

**Verification**:
```bash
cargo build --release --all-features
cargo test --release
cargo clippy --all-targets
tools/smokes/v2/run.sh --profile integration
```

**Estimated Effort**: 1 hour

---

## Public API Changes

### Before (control_flow.rs)
```rust
impl MirBuilder {
    pub(super) fn cf_block(...) -> Result<ValueId, String>
    pub(super) fn cf_if(...) -> Result<ValueId, String>
    pub(super) fn cf_loop(...) -> Result<ValueId, String>
    pub(super) fn cf_try_catch(...) -> Result<ValueId, String>
    pub(super) fn cf_throw(...) -> Result<ValueId, String>
}
```

### After (control_flow/mod.rs)
```rust
// Re-export all entry points (NO CHANGE to public API)
pub(super) use entry_points::{cf_block, cf_if, cf_loop, cf_try_catch, cf_throw};

// Internal modules (not exposed outside control_flow)
mod debug;
mod utils;
mod joinir;
mod exception;
mod entry_points;
```

**Guarantee**: Zero breaking changes - all `pub(super)` functions remain accessible from `MirBuilder`.

---

## Build Verification Strategy

### After Each Phase:

```bash
# 1. Compilation check
cargo build --release
echo "Build: $?" >> /tmp/modularization_log.txt

# 2. Unit tests
cargo test --lib
echo "Unit tests: $?" >> /tmp/modularization_log.txt

# 3. Integration tests
cargo test --release
echo "Integration tests: $?" >> /tmp/modularization_log.txt

# 4. Smoke tests (only for Phases 2-4)
tools/smokes/v2/run.sh --profile quick --filter "loop_*"
echo "Smoke tests: $?" >> /tmp/modularization_log.txt

# 5. Debug trace verification (for Phase 4)
NYASH_OPTION_C_DEBUG=1 ./target/release/nyash apps/tests/loop_min_while.hako 2>&1 | tee /tmp/debug_trace.txt
grep -q "merge_joinir" /tmp/debug_trace.txt && echo "Debug trace: OK"
```

### Failure Handling:
```bash
# If any step fails, STOP immediately
if [ $? -ne 0 ]; then
    echo "FAILURE detected at Phase $PHASE"
    git status
    echo "Run rollback procedure? (y/n)"
    # ... rollback steps
fi
```

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation | Detection |
|------|------------|--------|------------|-----------|
| **Breaking imports** | Medium | High | Incremental phases, test after each | `cargo build` fails |
| **Merge function breakage** | Low | Critical | Keep original as comment, feature flag | Smoke tests fail |
| **Performance regression** | Very Low | Medium | No algorithmic changes | Benchmark before/after |
| **Debug trace changes** | Low | Low | Verify `NYASH_OPTION_C_DEBUG` output | Manual inspection |
| **HashMap non-determinism** | Very Low | Low | Already using BTreeMap in critical paths | Run tests 3x |

### Critical Mitigations:
1. **Phase 4 (merge function)**: Keep original function as comment
2. **All phases**: Commit after each successful phase
3. **Rollback plan**: Document exact `git checkout` commands for each file

---

## Implementation Effort Breakdown

| Phase | Description | Effort | Risk |
|-------|-------------|--------|------|
| Phase 1 | Debug utilities | 30 min | Low |
| Phase 2 | Pattern lowerers | 2 hours | Low |
| Phase 3 | JoinIR routing | 1.5 hours | Low |
| **Phase 4** | **merge_joinir_mir_blocks** | **6 hours** | **Medium** |
| Phase 5 | Exception handling | 1 hour | Low |
| Phase 6 | Utilities | 30 min | Low |
| Phase 7 | Cleanup & docs | 1 hour | Low |
| **Total** | **control_flow.rs** | **12.5 hours** | - |

**Recommended Schedule**:
- **Week 1**: Phases 1-3 (4 hours total) - Low risk warmup
- **Week 2**: Phase 4 (6 hours) - Dedicated time for merge function
- **Week 3**: Phases 5-7 (2.5 hours) - Final cleanup

---

## Success Criteria

- ✅ All 267 tests pass (no regressions)
- ✅ Build time ≤ current (no increase)
- ✅ `control_flow/mod.rs` is < 200 lines (88% reduction)
- ✅ Largest single file is < 200 lines (vs 714 lines before)
- ✅ Debug traces still work (`NYASH_OPTION_C_DEBUG=1`)
- ✅ Smoke tests pass for all patterns (1/2/3)
- ✅ No HashMap non-determinism introduced
- ✅ Code is easier to navigate (measured by developer feedback)

---

# 2. generic_case_a.rs Modularization (MEDIUM PRIORITY)

## Current State

**File**: `src/mir/join_ir/lowering/generic_case_a.rs` (1,056 lines)

**Functions** (4 public lowerers):
- `lower_case_a_skip_ws_with_scope()` + `lower_case_a_skip_ws_core()` (~203 lines)
- `lower_case_a_trim_with_scope()` + `lower_case_a_trim_core()` (~479 lines)
- `lower_case_a_append_defs_with_scope()` + `lower_case_a_append_defs_core()` (~156 lines)
- `lower_case_a_stage1_usingresolver_with_scope()` + `lower_case_a_stage1_usingresolver_core()` (~167 lines)

**Key Observation**: This file already has companion files:
- `generic_case_a_entry_builder.rs` (4,828 bytes)
- `generic_case_a_whitespace_check.rs` (4,552 bytes)

**Strategy**: Complete the modularization pattern by splitting the 4 lowerers into separate files.

---

## Proposed New Structure

```
src/mir/join_ir/lowering/generic_case_a/
├── mod.rs                        (~100 lines) - Public API
├── skip_ws.rs                    (~220 lines) - skip_ws lowerer
├── trim.rs                       (~500 lines) - trim lowerer
├── append_defs.rs                (~170 lines) - append_defs lowerer
├── stage1_using_resolver.rs      (~180 lines) - stage1 using resolver
├── entry_builder.rs              (~150 lines) - (moved from parent)
└── whitespace_check.rs           (~150 lines) - (moved from parent)

Total: ~1,470 lines (39% increase for clarity, distributed across 7 files)
Average: ~210 lines per file
```

---

## Phase-by-Phase Migration Plan

### Phase 1: Create Directory Structure (15 min)

**Steps**:
1. Create `src/mir/join_ir/lowering/generic_case_a/` directory
2. Create `mod.rs` with public API exports
3. Move existing companion files into directory
4. Update parent `mod.rs` imports
5. Run verification

**Verification**:
```bash
cargo build --release
```

**Rollback**: Delete directory, revert parent `mod.rs`

**Estimated Effort**: 15 minutes

---

### Phase 2: Extract skip_ws Lowerer (45 min)

**Steps**:
1. Create `generic_case_a/skip_ws.rs`
2. Move `lower_case_a_skip_ws_with_scope()` and `_core()`
3. Add module documentation
4. Update `mod.rs` imports
5. Run verification

**Verification**:
```bash
cargo build --release
cargo test --lib -- skip_ws
```

**Estimated Effort**: 45 minutes

---

### Phase 3: Extract trim Lowerer (1 hour)

**Steps**:
1. Create `generic_case_a/trim.rs`
2. Move `lower_case_a_trim_with_scope()` and `_core()`
3. Add documentation
4. Update imports
5. Run verification

**Verification**:
```bash
cargo build --release
cargo test --lib -- trim
```

**Estimated Effort**: 1 hour

---

### Phase 4: Extract append_defs & stage1 Lowerers (1 hour)

**Steps**:
1. Create `generic_case_a/append_defs.rs`
2. Create `generic_case_a/stage1_using_resolver.rs`
3. Move respective functions
4. Add documentation
5. Update imports
6. Run verification

**Verification**:
```bash
cargo build --release
cargo test --release
tools/smokes/v2/run.sh --profile quick --filter "funcscanner_*"
```

**Estimated Effort**: 1 hour

---

### Phase 5: Final Cleanup (30 min)

**Steps**:
1. Add module-level documentation
2. Verify all imports are clean
3. Run comprehensive tests
4. Update documentation

**Verification**:
```bash
cargo build --release --all-features
cargo test --release
```

**Estimated Effort**: 30 minutes

---

## Public API Changes

### Before
```rust
// src/mir/join_ir/lowering/generic_case_a.rs
pub(crate) fn lower_case_a_skip_ws_with_scope(...) -> Option<JoinModule>
pub(crate) fn lower_case_a_trim_with_scope(...) -> Option<JoinModule>
// ... etc
```

### After
```rust
// src/mir/join_ir/lowering/generic_case_a/mod.rs
pub(crate) use skip_ws::lower_case_a_skip_ws_with_scope;
pub(crate) use trim::lower_case_a_trim_with_scope;
pub(crate) use append_defs::lower_case_a_append_defs_with_scope;
pub(crate) use stage1_using_resolver::lower_case_a_stage1_usingresolver_with_scope;
```

**Guarantee**: Zero breaking changes - all `pub(crate)` functions remain accessible.

---

## Implementation Effort Breakdown

| Phase | Description | Effort | Risk |
|-------|-------------|--------|------|
| Phase 1 | Directory setup | 15 min | Low |
| Phase 2 | skip_ws lowerer | 45 min | Low |
| Phase 3 | trim lowerer | 1 hour | Low |
| Phase 4 | append_defs & stage1 | 1 hour | Low |
| Phase 5 | Cleanup | 30 min | Low |
| **Total** | **generic_case_a.rs** | **3.5 hours** | - |

---

## Success Criteria

- ✅ All tests pass
- ✅ `generic_case_a/mod.rs` is < 150 lines
- ✅ Each lowerer is in a dedicated file
- ✅ Companion files integrated into directory
- ✅ Documentation added to all modules

---

# 3. loopform_builder.rs Modularization (LOWER PRIORITY)

## Current State

**File**: `src/mir/phi_core/loopform_builder.rs` (1,166 lines)

**Status**: Already partially modularized in Phase 191!

**Existing Structure** (Phase 191):
```
src/mir/phi_core/
├── loopform_builder.rs           (1,166 lines) - Main coordinator
├── loopform_context.rs            - ValueId management
├── loopform_variable_models.rs    - CarrierVariable, PinnedVariable
├── loopform_utils.rs              - Debug and bypass utilities
├── loopform_exit_phi.rs           - Exit PHI builder
```

**Remaining Work**: The main `loopform_builder.rs` still contains implementation logic that should be moved to dedicated modules.

---

## Proposed New Structure

```
src/mir/phi_core/loopform/
├── mod.rs                        (~100 lines) - Public API
├── context.rs                    (~150 lines) - (existing loopform_context.rs)
├── variable_models.rs            (~150 lines) - (existing loopform_variable_models.rs)
├── utils.rs                      (~100 lines) - (existing loopform_utils.rs)
├── exit_phi.rs                   (~150 lines) - (existing loopform_exit_phi.rs)
├── passes/
│   ├── mod.rs                    (~50 lines)  - 4-pass architecture coordinator
│   ├── pass1_discovery.rs        (~150 lines) - Variable discovery
│   ├── pass2_header_phi.rs       (~150 lines) - Header PHI construction
│   ├── pass3_latch.rs            (~100 lines) - Latch block processing
│   └── pass4_exit_phi.rs         (~150 lines) - Exit PHI construction
└── builder_core.rs               (~200 lines) - Core builder logic

Total: ~1,450 lines (24% increase for clarity, distributed across 11 files)
Average: ~132 lines per file
```

---

## Phase-by-Phase Migration Plan

### Phase 1: Directory Structure (30 min)

**Steps**:
1. Create `src/mir/phi_core/loopform/` directory
2. Move existing modular files into directory
3. Create `mod.rs` with re-exports
4. Update parent `mod.rs` imports
5. Run verification

**Verification**:
```bash
cargo build --release
cargo test --lib -- loopform
```

**Estimated Effort**: 30 minutes

---

### Phase 2: Extract 4-Pass Architecture (2 hours)

**Steps**:
1. Create `loopform/passes/` directory
2. Identify the 4 passes in `loopform_builder.rs`
3. Extract each pass to dedicated file
4. Create `passes/mod.rs` as coordinator
5. Update imports
6. Run verification

**Verification**:
```bash
cargo build --release
cargo test --release -- loopform
NYASH_LOOPFORM_DEBUG=1 ./target/release/nyash test_loop.hako
```

**Estimated Effort**: 2 hours

---

### Phase 3: Extract Core Builder Logic (1 hour)

**Steps**:
1. Create `loopform/builder_core.rs`
2. Move remaining builder logic
3. Update imports
4. Run verification

**Verification**:
```bash
cargo build --release
cargo test --release
```

**Estimated Effort**: 1 hour

---

### Phase 4: Final Cleanup (30 min)

**Steps**:
1. Add module documentation
2. Verify all re-exports
3. Run comprehensive tests

**Verification**:
```bash
cargo build --release --all-features
cargo test --release
tools/smokes/v2/run.sh --profile quick --filter "phi_*"
```

**Estimated Effort**: 30 minutes

---

## Public API Changes

### Before
```rust
// src/mir/phi_core/loopform_builder.rs
pub use loopform_context::LoopFormContext;
pub use loopform_variable_models::{CarrierVariable, PinnedVariable};
pub fn build_exit_phis_for_control<O: LoopFormOps>(...)
```

### After
```rust
// src/mir/phi_core/loopform/mod.rs
pub use context::LoopFormContext;
pub use variable_models::{CarrierVariable, PinnedVariable};
pub use exit_phi::build_exit_phis_for_control;
```

**Guarantee**: Zero breaking changes.

---

## Implementation Effort Breakdown

| Phase | Description | Effort | Risk |
|-------|-------------|--------|------|
| Phase 1 | Directory setup | 30 min | Low |
| Phase 2 | 4-pass extraction | 2 hours | Medium |
| Phase 3 | Core builder | 1 hour | Low |
| Phase 4 | Cleanup | 30 min | Low |
| **Total** | **loopform_builder.rs** | **4 hours** | - |

---

## Success Criteria

- ✅ All tests pass
- ✅ `loopform/mod.rs` is < 150 lines
- ✅ Each pass is in a dedicated file
- ✅ Existing modular files integrated
- ✅ Documentation added

---

# Implementation Order & Timeline

## Recommended Schedule (3 weeks)

### Week 1: control_flow.rs Phases 1-3 (Low Risk)
- **Monday**: Phase 1 (Debug utilities) - 30 min
- **Tuesday**: Phase 2 (Pattern lowerers) - 2 hours
- **Wednesday**: Phase 3 (JoinIR routing) - 1.5 hours
- **Verification**: Run full smoke tests at end of week

### Week 2: control_flow.rs Phase 4 (High Risk)
- **Monday-Tuesday**: Phase 4 (merge_joinir_mir_blocks) - 6 hours
- **Wednesday**: Buffer day for fixing any issues
- **Thursday-Friday**: Phases 5-7 (Exception, utilities, cleanup) - 2.5 hours

### Week 3: generic_case_a.rs (Optional)
- **Monday-Tuesday**: generic_case_a.rs Phases 1-5 - 3.5 hours
- **Wednesday**: Buffer
- **Thursday-Friday**: Documentation & final verification

### Future (After Pattern 4+): loopform_builder.rs
- **Timing**: After Pattern 4/5/6 development stabilizes
- **Effort**: 4 hours
- **Priority**: Lower (already partially modularized)

---

## Effort Summary

| File | Total Effort | Priority | Complexity | Blocking? |
|------|--------------|----------|------------|-----------|
| **control_flow.rs** | **12.5 hours** | **HIGHEST** | **High** | **Yes** (Pattern 4+) |
| **generic_case_a.rs** | **3.5 hours** | **MEDIUM** | **Low** | No |
| **loopform_builder.rs** | **4 hours** | **LOWER** | **Medium** | No |
| **TOTAL** | **20 hours** | - | - | - |

---

## Risk/Mitigation Matrix

| Risk | Likelihood | Impact | Mitigation | Detection Method |
|------|------------|--------|------------|------------------|
| **Breaking imports** | Medium | High | Incremental phases, test after each | `cargo build` fails |
| **Merge function breakage** | Low | Critical | Keep original as comment, feature flag | Smoke tests fail |
| **Pattern lowerer breakage** | Low | High | Test each pattern independently | Integration tests |
| **Performance regression** | Very Low | Medium | No algorithmic changes | Benchmark suite |
| **Debug trace changes** | Low | Low | Verify `NYASH_OPTION_C_DEBUG` output | Manual inspection |
| **Test failures** | Low | Medium | Run tests after every phase | CI/CD pipeline |
| **Merge conflicts** | Medium | Low | Work on dedicated branch | Git status |

---

## Success Criteria (Global)

### Quantitative:
- ✅ All 267+ tests pass (no regressions)
- ✅ Build time ≤ current (no increase)
- ✅ Largest single file in modularized areas is < 250 lines
- ✅ Average file size in modularized areas is < 150 lines

### Qualitative:
- ✅ Code is easier to navigate (developer feedback)
- ✅ New patterns can be added without modifying 1,600-line files
- ✅ Debug traces remain functional
- ✅ Documentation is clear and helpful

### Process:
- ✅ Zero breaking changes at any phase
- ✅ Each phase can be rolled back independently
- ✅ Commits are small and focused
- ✅ CI/CD passes after every commit

---

## Appendix: Emergency Rollback Procedure

If anything goes wrong during modularization:

```bash
# 1. Identify the problematic phase
echo "ROLLBACK: Phase $PHASE failed"

# 2. Check git status
git status

# 3. Rollback created files
rm -rf src/mir/builder/control_flow/  # (example)

# 4. Restore original file
git checkout src/mir/builder/control_flow.rs

# 5. Verify build
cargo build --release
cargo test --lib

# 6. Document the issue
echo "$(date): Phase $PHASE rollback due to: $REASON" >> docs/development/refactoring/rollback_log.txt

# 7. Review and adjust plan
# - Was the failure expected?
# - Do we need to adjust the approach?
# - Should we skip this phase?
```

---

## Conclusion

This modularization plan provides a **safe, incremental path** to breaking down 3 large files into maintainable modules. The approach prioritizes:

1. **Zero breaking changes** - Backward compatible at every step
2. **Clear verification** - Test after each phase
3. **Easy rollback** - Can undo any phase if issues arise
4. **Pattern setting** - control_flow.rs establishes the template for others

**Next Steps**:
1. Review this plan with the team
2. Get approval for Week 1 (control_flow.rs Phases 1-3)
3. Create a dedicated branch: `refactor/modularize-control-flow`
4. Begin Phase 1!

**Questions? Concerns?** Open an issue or discuss in the team channel.

---

**Document Version**: 1.0
**Created**: 2025-12-05
**Author**: Claude Code (AI-assisted planning)
**Status**: Draft - Awaiting Review
