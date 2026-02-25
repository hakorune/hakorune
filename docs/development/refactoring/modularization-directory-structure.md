# Modularization Directory Structure Diagrams

Visual reference for the proposed directory structures after modularization.

---

## 1. control_flow.rs Modularization

### Before (Current State)
```
src/mir/builder/
├── control_flow.rs                    (1,632 lines) ⚠️ MONOLITH
├── if_form.rs
├── loops.rs
└── ... (other files)
```

### After (Proposed Structure)
```
src/mir/builder/
├── control_flow/
│   ├── mod.rs                         (~150 lines) ✅ Entry points
│   │   ├── pub(super) fn cf_block()
│   │   ├── pub(super) fn cf_if()
│   │   ├── pub(super) fn cf_loop()
│   │   ├── pub(super) fn cf_try_catch()
│   │   └── pub(super) fn cf_throw()
│   │
│   ├── debug.rs                       (~50 lines) ✅ Debug utilities
│   │   └── fn trace_varmap()
│   │
│   ├── utils.rs                       (~50 lines) ✅ Utility functions
│   │   └── fn extract_loop_variable_from_condition()
│   │
│   ├── joinir/
│   │   ├── mod.rs                     (~100 lines) ✅ JoinIR coordinator
│   │   │
│   │   ├── routing.rs                 (~150 lines) ✅ Routing logic
│   │   │   ├── fn try_cf_loop_joinir()
│   │   │   └── fn cf_loop_joinir_impl()
│   │   │
│   │   ├── merge/
│   │   │   ├── mod.rs                 (~100 lines) ✅ Merge coordinator
│   │   │   │   └── pub fn merge_joinir_mir_blocks()
│   │   │   │
│   │   │   ├── id_remapper.rs         (~150 lines) ✅ ID remapping
│   │   │   │   ├── struct JoinIrIdRemapper
│   │   │   │   ├── fn create_remapper()
│   │   │   │   └── fn remap_ids()
│   │   │   │
│   │   │   ├── block_allocator.rs     (~100 lines) ✅ Block allocation
│   │   │   │   └── fn allocate_blocks()
│   │   │   │
│   │   │   ├── value_collector.rs     (~100 lines) ✅ Value collection
│   │   │   │   └── fn collect_values()
│   │   │   │
│   │   │   ├── instruction_rewriter.rs (~150 lines) ✅ Instruction rewriting
│   │   │   │   ├── fn rewrite_instructions()
│   │   │   │   └── fn convert_call_to_jump()
│   │   │   │
│   │   │   └── exit_phi_builder.rs    (~100 lines) ✅ Exit PHI construction
│   │   │       └── fn build_exit_phi()
│   │   │
│   │   └── patterns/
│   │       ├── mod.rs                 (~50 lines) ✅ Pattern dispatcher
│   │       │   └── pub fn dispatch_pattern()
│   │       │
│   │       ├── pattern1_minimal.rs    (~150 lines) ✅ Pattern 1 lowering
│   │       │   └── pub fn cf_loop_pattern1_minimal()
│   │       │
│   │       ├── pattern2_with_break.rs (~130 lines) ✅ Pattern 2 lowering
│   │       │   └── pub fn cf_loop_pattern2_with_break()
│   │       │
│   │       └── pattern3_with_if_phi.rs (~180 lines) ✅ Pattern 3 lowering
│   │           └── pub fn cf_loop_pattern3_with_if_phi()
│   │
│   └── exception/
│       ├── mod.rs                     (~50 lines) ✅ Exception API
│       │
│       ├── try_catch.rs               (~150 lines) ✅ try/catch impl
│       │   └── pub fn cf_try_catch()
│       │
│       └── throw.rs                   (~30 lines) ✅ throw impl
│           └── pub fn cf_throw()
│
├── if_form.rs
├── loops.rs
└── ... (other files - unchanged)
```

### Metrics Comparison

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| **Total lines** | 1,632 | ~1,850 | +13% (for clarity) |
| **Number of files** | 1 | 19 | +1800% |
| **Largest file** | 1,632 | ~180 | -89% |
| **Average file size** | 1,632 | ~97 | -94% |
| **Functions per file** | 13 | 1-2 | Much clearer |

### Benefits
- ✅ **714-line merge function → 6 focused modules** (100-150 lines each)
- ✅ **Pattern lowerers isolated** → Easy to add Pattern 4/5/6
- ✅ **Debug traces easier to locate** → NYASH_OPTION_C_DEBUG output clearer
- ✅ **Merge conflicts reduced** → Changes isolated to specific files
- ✅ **Code navigation improved** → Jump to definition works better

---

## 2. generic_case_a.rs Modularization

### Before (Current State)
```
src/mir/join_ir/lowering/
├── generic_case_a.rs                      (1,056 lines) ⚠️ LARGE
├── generic_case_a_entry_builder.rs        (4,828 bytes)
├── generic_case_a_whitespace_check.rs     (4,552 bytes)
└── ... (other files)
```

### After (Proposed Structure)
```
src/mir/join_ir/lowering/
├── generic_case_a/
│   ├── mod.rs                             (~100 lines) ✅ Public API
│   │   ├── pub use skip_ws::lower_case_a_skip_ws_with_scope
│   │   ├── pub use trim::lower_case_a_trim_with_scope
│   │   ├── pub use append_defs::lower_case_a_append_defs_with_scope
│   │   └── pub use stage1_using_resolver::...
│   │
│   ├── skip_ws.rs                         (~220 lines) ✅ skip_ws lowerer
│   │   ├── pub fn lower_case_a_skip_ws_with_scope()
│   │   └── fn lower_case_a_skip_ws_core()
│   │
│   ├── trim.rs                            (~500 lines) ✅ trim lowerer
│   │   ├── pub fn lower_case_a_trim_with_scope()
│   │   └── fn lower_case_a_trim_core()
│   │
│   ├── append_defs.rs                     (~170 lines) ✅ append_defs lowerer
│   │   ├── pub fn lower_case_a_append_defs_with_scope()
│   │   └── fn lower_case_a_append_defs_core()
│   │
│   ├── stage1_using_resolver.rs          (~180 lines) ✅ stage1 lowerer
│   │   ├── pub fn lower_case_a_stage1_usingresolver_with_scope()
│   │   └── fn lower_case_a_stage1_usingresolver_core()
│   │
│   ├── entry_builder.rs                   (~150 lines) ✅ (moved from parent)
│   │   └── struct EntryFunctionBuilder
│   │
│   └── whitespace_check.rs                (~150 lines) ✅ (moved from parent)
│       └── fn check_whitespace()
│
└── ... (other files - unchanged)
```

### Metrics Comparison

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| **Total lines** | 1,056 | ~1,470 | +39% (for clarity) |
| **Number of files** | 3 (scattered) | 7 (organized) | +133% |
| **Largest file** | 1,056 | ~500 | -53% |
| **Average file size** | 352 | ~210 | -40% |
| **Functions per file** | 4 | 1-2 | Much clearer |

### Benefits
- ✅ **Each lowerer in its own file** → Easy to maintain
- ✅ **Companion files integrated** → All Case A logic in one directory
- ✅ **trim.rs still large (500 lines)** → Could be further split if needed
- ✅ **Clear public API** → mod.rs shows what's exported

---

## 3. loopform_builder.rs Modularization

### Before (Current State - After Phase 191)
```
src/mir/phi_core/
├── loopform_builder.rs                    (1,166 lines) ⚠️ LARGE
├── loopform_context.rs                    (✅ already modularized)
├── loopform_variable_models.rs            (✅ already modularized)
├── loopform_utils.rs                      (✅ already modularized)
├── loopform_exit_phi.rs                   (✅ already modularized)
└── ... (other files)
```

### After (Proposed Structure)
```
src/mir/phi_core/
├── loopform/
│   ├── mod.rs                             (~100 lines) ✅ Public API
│   │   ├── pub use context::LoopFormContext
│   │   ├── pub use variable_models::{CarrierVariable, PinnedVariable}
│   │   ├── pub use exit_phi::build_exit_phis_for_control
│   │   └── pub use builder_core::LoopFormBuilder
│   │
│   ├── context.rs                         (~150 lines) ✅ (existing)
│   │   └── pub struct LoopFormContext
│   │
│   ├── variable_models.rs                 (~150 lines) ✅ (existing)
│   │   ├── pub struct CarrierVariable
│   │   ├── pub struct PinnedVariable
│   │   └── pub struct LoopBypassFlags
│   │
│   ├── utils.rs                           (~100 lines) ✅ (existing)
│   │   ├── pub fn is_loopform_debug_enabled()
│   │   └── pub fn get_loop_bypass_flags()
│   │
│   ├── exit_phi.rs                        (~150 lines) ✅ (existing)
│   │   └── pub fn build_exit_phis_for_control()
│   │
│   ├── passes/
│   │   ├── mod.rs                         (~50 lines) ✅ 4-pass coordinator
│   │   │   └── pub fn run_4_pass_architecture()
│   │   │
│   │   ├── pass1_discovery.rs             (~150 lines) ✅ Variable discovery
│   │   │   └── pub fn discover_variables()
│   │   │
│   │   ├── pass2_header_phi.rs            (~150 lines) ✅ Header PHI
│   │   │   └── pub fn build_header_phi()
│   │   │
│   │   ├── pass3_latch.rs                 (~100 lines) ✅ Latch processing
│   │   │   └── pub fn process_latch()
│   │   │
│   │   └── pass4_exit_phi.rs              (~150 lines) ✅ Exit PHI
│   │       └── pub fn build_exit_phi()
│   │
│   └── builder_core.rs                    (~200 lines) ✅ Core builder
│       └── pub struct LoopFormBuilder
│
└── ... (other files - unchanged)
```

### Metrics Comparison

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| **Total lines** | 1,166 (main file) | ~1,450 | +24% (for clarity) |
| **Number of files** | 5 (partially modularized) | 11 (fully modularized) | +120% |
| **Largest file** | 1,166 | ~200 | -83% |
| **Average file size** | 233 | ~132 | -43% |

### Benefits
- ✅ **Completes Phase 191 modularization** → Finishes what was started
- ✅ **4-pass architecture explicit** → Each pass in its own file
- ✅ **Already partially modularized** → Lower risk than control_flow.rs
- ✅ **Clear separation of concerns** → Context, models, passes, builder

---

## File Size Distribution Comparison

### control_flow.rs
```
Before:
█████████████████████████████████████████████████████████████████ 1,632 lines

After:
merge/instruction_rewriter.rs: ███████ 150 lines
pattern1_minimal.rs:           ███████ 150 lines
routing.rs:                    ███████ 150 lines
pattern3_with_if_phi.rs:       ████████ 180 lines
try_catch.rs:                  ███████ 150 lines
... (14 more files < 150 lines each)
```

### generic_case_a.rs
```
Before:
███████████████████████████████████████████████████ 1,056 lines

After:
trim.rs:                       ████████████████████████ 500 lines
skip_ws.rs:                    ██████████ 220 lines
stage1_using_resolver.rs:      ████████ 180 lines
append_defs.rs:                ████████ 170 lines
... (3 more files < 150 lines each)
```

### loopform_builder.rs
```
Before:
████████████████████████████████████████████████████████ 1,166 lines

After:
builder_core.rs:               █████████ 200 lines
context.rs:                    ███████ 150 lines
variable_models.rs:            ███████ 150 lines
exit_phi.rs:                   ███████ 150 lines
pass1_discovery.rs:            ███████ 150 lines
... (6 more files < 150 lines each)
```

---

## Import Path Changes

### control_flow.rs

#### Before
```rust
use crate::mir::builder::MirBuilder;

impl MirBuilder {
    pub(super) fn cf_loop(...) -> Result<ValueId, String> {
        // 1,632 lines of code
    }
}
```

#### After
```rust
// src/mir/builder/control_flow/mod.rs
use crate::mir::builder::MirBuilder;

impl MirBuilder {
    pub(super) fn cf_loop(...) -> Result<ValueId, String> {
        // Delegates to routing module
        joinir::routing::try_cf_loop_joinir(self, ...)
    }
}

// src/mir/builder/control_flow/joinir/routing.rs
pub(in crate::mir::builder::control_flow) fn try_cf_loop_joinir(
    builder: &mut MirBuilder,
    ...
) -> Result<Option<ValueId>, String> {
    // 150 lines of focused code
}
```

### generic_case_a.rs

#### Before
```rust
// src/mir/join_ir/lowering/generic_case_a.rs
pub(crate) fn lower_case_a_skip_ws_with_scope(...) -> Option<JoinModule> {
    // 200+ lines
}
```

#### After
```rust
// src/mir/join_ir/lowering/generic_case_a/mod.rs
pub(crate) use skip_ws::lower_case_a_skip_ws_with_scope;

// src/mir/join_ir/lowering/generic_case_a/skip_ws.rs
pub(crate) fn lower_case_a_skip_ws_with_scope(...) -> Option<JoinModule> {
    // 220 lines
}
```

### loopform_builder.rs

#### Before
```rust
// src/mir/phi_core/loopform_builder.rs
pub use loopform_context::LoopFormContext;

pub fn build_exit_phis_for_control<O: LoopFormOps>(...) {
    // 1,166 lines
}
```

#### After
```rust
// src/mir/phi_core/loopform/mod.rs
pub use context::LoopFormContext;
pub use exit_phi::build_exit_phis_for_control;

// src/mir/phi_core/loopform/exit_phi.rs
pub fn build_exit_phis_for_control<O: LoopFormOps>(...) {
    // 150 lines
}
```

---

## Navigation Improvements

### Before (Monolith Files)
```
Developer: "Where is the merge function?"
Answer: "control_flow.rs, line 864-1578 (search through 1,632 lines)"

Developer: "Where is Pattern 3 lowering?"
Answer: "control_flow.rs, line 696-863 (search through 1,632 lines)"

Developer: "What does merge_joinir_mir_blocks do?"
Answer: "Read 714 lines to understand"
```

### After (Modularized)
```
Developer: "Where is the merge function?"
Answer: "control_flow/joinir/merge/mod.rs (100 lines coordinator)"

Developer: "Where is Pattern 3 lowering?"
Answer: "control_flow/joinir/patterns/pattern3_with_if_phi.rs (180 lines)"

Developer: "What does merge_joinir_mir_blocks do?"
Answer: "Read merge/mod.rs (100 lines) → delegates to 6 sub-modules"
```

---

## Conclusion

The modularization dramatically improves code organization:

- **control_flow.rs**: 1,632 lines → 19 files (avg 97 lines)
- **generic_case_a.rs**: 1,056 lines → 7 files (avg 210 lines)
- **loopform_builder.rs**: 1,166 lines → 11 files (avg 132 lines)

**Total Impact**: 3,854 lines → 37 focused modules

**Developer Experience**:
- ✅ Easier navigation (jump to definition works better)
- ✅ Clearer separation of concerns
- ✅ Less merge conflicts
- ✅ Easier to add new patterns/lowerers
- ✅ Better code review experience (smaller diffs)

---

**Last Updated**: 2025-12-05
