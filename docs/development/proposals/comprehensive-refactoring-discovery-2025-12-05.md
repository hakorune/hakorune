# Comprehensive Rust Code Refactoring Discovery

Status: Active  
Scope: Rust コード全体のリファクタリング候補を洗い出した調査結果の現行まとめ（2025-12-05 時点）。

**Date**: 2025-12-05
**Scope**: Entire `/home/tomoaki/git/hakorune-selfhost/src` directory
**Methodology**: Line count + complexity analysis + activity analysis + Phase 188 context

---

## Executive Summary

### Key Findings

- **Total problem code**: 72,936 lines in 110 files > 400 lines
- **Critical files**: 8 files requiring immediate refactoring (blocking Phase 188+)
- **Total refactoring effort**: ~65-80 hours (full scope)
- **Quick wins available**: 5 files < 3 hours each (~12 hours total, high impact)
- **Top priority**: `control_flow.rs` (1,632 lines, 168 control flow branches)

### ROI Analysis

**High-Impact Refactorings** (will unblock Phase 188+ development):
- `control_flow.rs` → 6 modules (12.5h) - **CRITICAL** for Pattern 4/5
- `loopform_builder.rs` → 4-6 modules (8h) - Needed for LoopForm evolution
- `strip.rs` → 3 modules (6h) - Required for Stage-2 namespace improvements

**Code Health Metrics**:
- Files > 1000 lines: 6 files (potential for 40-50% reduction)
- Files with 100+ control flow branches: 2 files (cognitive overload)
- Most modified in 2025: `mir/builder.rs` (322 commits) - needs attention

---

## Critical Files (Must Refactor for Phase 188+)

### File 1: `src/mir/builder/control_flow.rs` (1,632 lines)

**Priority**: 🔴 **CRITICAL #1** - BLOCKING Pattern 4/5 Implementation

**Purpose**: Control-flow entrypoints (if/loop/try/throw) centralized entry point

**Complexity**: 5/5
- Control flow branches: 168 (15 match, 128 if, 25 loop)
- Functions: 8 public functions, 1 impl block
- Cognitive load: EXTREME

**Activity Level**: HIGH (modified in Phase 186/187, will be modified in Phase 188+)

**Maintainability Issues**:
- **714-line `try_cf_loop_joinir()` function** (43% of file!)
  - Multiple responsibilities: pattern detection, JoinIR routing, variable mapping, bypass checking
  - 6-level nesting in some branches
  - 50+ environment variable checks scattered throughout
- **Mixed concerns**: JoinIR routing + LoopForm binding + Phase detection + bypass logic
- **Scattered pattern detection**: Pattern 1/2/3/4/5 detection logic mixed with routing
- **Hard to test**: Monolithic function makes unit testing nearly impossible
- **Blocks Phase 188 Pattern 4/5**: Adding new patterns requires navigating 714-line function

**Refactoring Plan**: "Control Flow Orchestration → 6 Modules"

**Strategy**:
1. **Pattern Detection Module** (`pattern_detector.rs`) - 150 lines
   - Extract all pattern detection logic (Pattern 1/2/3/4/5)
   - Enum-based pattern identification
   - Unit testable pattern matchers

2. **Routing Policy Module** (`routing_policy.rs`) - 120 lines
   - JoinIR Core enabled/disabled logic
   - Bypass flag checking
   - Phase-specific routing decisions

3. **Variable Mapping Module** (`variable_mapper.rs`) - 180 lines
   - JoinIR variable name extraction
   - LoopForm context building
   - Scope shape construction

4. **Orchestrator Module** (`orchestrator.rs`) - 200 lines
   - High-level control flow entry points (if/loop/try)
   - Delegates to pattern detector + router
   - Clean public API

5. **Legacy Compatibility Module** (`legacy_compat.rs`) - 100 lines
   - LoopBuilder fallback logic (Phase 186/187)
   - Environment variable warnings
   - Migration helpers

6. **Tests Module** (`tests.rs`) - 300 lines
   - Unit tests for each pattern detector
   - Integration tests for routing policies
   - Regression tests for Phase 186/187

**Before**:
```rust
// control_flow.rs (1,632 lines, 714-line function)
fn try_cf_loop_joinir(&mut self, ...) -> Result<...> {
    // 714 lines of pattern detection + routing + variable mapping
    if phase_49_enabled() { ... }
    if phase_80_enabled() { ... }
    if pattern_1_detected() { ... }
    // ... 700 more lines
}
```

**After**:
```rust
// orchestrator.rs (200 lines)
fn try_cf_loop_joinir(&mut self, ...) -> Result<...> {
    let pattern = PatternDetector::detect(&condition, &body)?;
    let policy = RoutingPolicy::from_env();
    match policy.route(pattern) {
        Route::JoinIR(pattern) => self.lower_pattern(pattern),
        Route::Legacy => self.legacy_fallback(),
    }
}

// pattern_detector.rs (150 lines)
enum LoopPattern {
    MinimalSSA,          // Pattern 1
    WhileBreak,          // Pattern 2
    IfElseMerge,         // Pattern 3
    NestedLoopAccumulator, // Pattern 4 (NEW)
    MultiCarrier,        // Pattern 5 (NEW)
}
```

**Effort**: 12-15 hours
- Pattern extraction: 4h
- Routing policy separation: 3h
- Variable mapping cleanup: 3h
- Orchestrator refactoring: 2h
- Tests + documentation: 3h

**Impact**:
- ✅ Enables Pattern 4/5 implementation in < 1 hour each
- ✅ Reduces cognitive load from 5/5 → 2/5
- ✅ Unit testing becomes possible
- ✅ Future patterns can be added in 30min each

---

### File 2: `src/mir/phi_core/loopform_builder.rs` (1,166 lines)

**Priority**: 🟠 **HIGH** - Will Need Refactoring for LoopForm Evolution

**Purpose**: LoopForm Meta-Box approach to PHI construction

**Complexity**: 4/5
- Control flow branches: 67 (0 match, 54 if, 13 loop)
- Functions: 52 functions, 1 impl block
- Cognitive load: HIGH

**Activity Level**: MEDIUM (stable since Phase 191 modularization)

**Maintainability Issues**:
- **Already partially modularized** (Phase 191):
  - Separated: `loopform_context.rs`, `loopform_variable_models.rs`, `loopform_utils.rs`
  - BUT: Core builder still 1,166 lines
- **4-pass architecture** not clearly separated:
  - Pass 1: prepare_structure() (allocate ValueIds)
  - Pass 2: emit_header_phis() (header PHI nodes)
  - Pass 3: emit_body() (loop body lowering)
  - Pass 4: seal_phis() (finalize PHI incoming edges)
- **Mixed concerns**: ValueId allocation + PHI emission + snapshot merging
- **Will need refactoring** for JoinIR Pattern 4/5 integration

**Refactoring Plan**: "LoopForm 4-Pass Architecture → 4-6 Modules"

**Strategy**:
1. **Pass 1 Module** (`loopform_pass1_structure.rs`) - 200 lines
   - ValueId allocation logic
   - Preheader snapshot capture
   - Carrier/Pinned variable setup

2. **Pass 2 Module** (`loopform_pass2_header_phis.rs`) - 150 lines
   - Header PHI emission
   - Variable classification (carrier vs pinned)

3. **Pass 3 Module** (`loopform_pass3_body.rs`) - 250 lines
   - Loop body lowering delegation
   - Body-local variable tracking

4. **Pass 4 Module** (`loopform_pass4_seal.rs`) - 200 lines
   - PHI finalization
   - Snapshot merge integration
   - Exit PHI construction

5. **Core Orchestrator** (keep in `loopform_builder.rs`) - 300 lines
   - High-level LoopFormBuilder API
   - 4-pass orchestration
   - Public interface

6. **Integration Tests** (`loopform_tests.rs`) - 150 lines
   - Per-pass unit tests
   - Integration tests for full flow

**Effort**: 8-10 hours
- Pass separation: 4h
- Core orchestrator refactoring: 2h
- Tests: 2h
- Documentation: 2h

**Impact**:
- ✅ Clearer 4-pass architecture
- ✅ Easier JoinIR integration for Pattern 4/5
- ✅ Better testability
- ✅ Reduces cognitive load 4/5 → 2/5

---

### File 3: `src/runner/modes/common_util/resolve/strip.rs` (1,081 lines)

**Priority**: 🟠 **HIGH** - Stage-2 Namespace Evolution Needs This

**Purpose**: Collect using targets and strip using lines (no inlining)

**Complexity**: 3/5
- Control flow branches: 120 (4 match, 116 if, 0 loop)
- Functions: 11 functions, 0 impl blocks
- Cognitive load: MEDIUM-HIGH

**Activity Level**: MEDIUM (stable, but will need changes for Stage-2 improvements)

**Maintainability Issues**:
- **Single 800+ line function** `collect_using_and_strip()`
- **Mixed concerns**:
  - Path resolution (file vs package vs alias)
  - Duplicate detection
  - Profile policy enforcement (prod vs dev)
  - Error message generation
- **116 if statements**: deeply nested conditionals
- **Error handling scattered**: 15+ error paths with different messages
- **Hard to extend**: Adding new using patterns requires navigating 800 lines

**Refactoring Plan**: "Using Resolution → 3 Modules"

**Strategy**:
1. **Target Resolution Module** (`using_target_resolver.rs`) - 300 lines
   - Path vs alias vs package detection
   - Canonicalization logic
   - Quote stripping

2. **Policy Enforcement Module** (`using_policy.rs`) - 200 lines
   - Prod vs dev mode checks
   - Package-internal vs top-level rules
   - Duplicate detection (paths + aliases)

3. **Error Message Generator** (`using_errors.rs`) - 150 lines
   - Centralized error messages
   - Hint generation
   - Line number tracking

4. **Main Orchestrator** (keep in `strip.rs`) - 400 lines
   - High-level using collection
   - Delegates to resolver + policy + errors
   - Line stripping logic

**Effort**: 6-8 hours
- Target resolution extraction: 2h
- Policy enforcement separation: 2h
- Error message consolidation: 1h
- Orchestrator refactoring: 2h
- Tests: 1h

**Impact**:
- ✅ Easier to add new using patterns (Stage-2)
- ✅ Clearer policy enforcement
- ✅ Better error messages
- ✅ Unit testable components

---

### File 4: `src/mir/join_ir/lowering/generic_case_a.rs` (1,056 lines)

**Priority**: 🟡 **MEDIUM-HIGH** - Will Expand for Pattern 4/5

**Purpose**: Generic Case A LoopForm → JoinIR lowering (minimal_ssa_skip_ws専用)

**Complexity**: 3/5
- Purpose-built for Pattern 1 (skip_ws)
- Clean structure but will need extension for Pattern 4/5

**Activity Level**: HIGH (Phase 188 active development)

**Maintainability Issues**:
- **Already well-structured** (Phase 192 EntryFunctionBuilder cleanup)
- **BUT**: Pattern 4/5 will add 500-800+ lines to this file
- **Opportunity**: Extract pattern-specific lowerers BEFORE adding Pattern 4/5

**Refactoring Plan**: "Pattern-Specific Lowerers → 4 Modules"

**Strategy**:
1. **Pattern 1 Lowerer** (keep in `generic_case_a.rs`) - 400 lines
   - Current skip_ws logic
   - Phase 192 EntryFunctionBuilder

2. **Pattern 2 Lowerer** (`generic_case_b.rs`) - NEW, 300 lines
   - While-with-break pattern

3. **Pattern 4 Lowerer** (`generic_case_d.rs`) - NEW, 400 lines
   - Nested loop with accumulator

4. **Pattern 5 Lowerer** (`generic_case_e.rs`) - NEW, 500 lines
   - Multi-carrier complex PHI

5. **Common Lowerer Utilities** (`lowerer_common.rs`) - NEW, 200 lines
   - EntryFunctionBuilder (move from generic_case_a.rs)
   - ValueId range helpers
   - JoinModule construction helpers

**Effort**: 4-6 hours (before Pattern 4/5 implementation)
- Extract EntryFunctionBuilder: 1h
- Create Pattern 2 module: 2h
- Setup Pattern 4/5 skeletons: 1h
- Tests: 2h

**Impact**:
- ✅ Pattern 4/5 implementation becomes 1-file changes
- ✅ Avoids 2000+ line mega-file
- ✅ Clear pattern separation
- ✅ Easier to test each pattern independently

---

### File 5: `src/boxes/file/handle_box.rs` (1,052 lines)

**Priority**: 🟢 **MEDIUM** - Stable, Can Refactor Later

**Purpose**: FileHandleBox - Handle-based file I/O

**Complexity**: 2/5
- Well-organized with macros (Phase 115)
- Mostly boilerplate (ny_wrap_* macros)
- Low cognitive load

**Activity Level**: LOW (stable)

**Maintainability Issues**:
- **Already improved** with Phase 115 macro-based method unification
- **Large but not complex**: 1,052 lines, but mostly repetitive wrapper methods
- **Could be further reduced** with trait-based approach

**Refactoring Plan**: "Trait-Based Wrapper Generation"

**Strategy**:
1. **Extract Core Operations** (`file_io_core.rs`) - 300 lines
   - Raw file I/O operations (open/read/write/close)
   - Error handling primitives

2. **Wrapper Trait** (`file_io_wrapper.rs`) - 150 lines
   - Generic wrapper trait for Nyash method generation
   - Macro consolidation

3. **Handle Box** (keep in `handle_box.rs`) - 400 lines
   - Public Nyash API
   - Uses wrapper trait
   - Reduced from 1,052 → 400 lines (60% reduction)

**Effort**: 4-5 hours
- Core extraction: 2h
- Trait design: 1h
- Integration: 1h
- Tests: 1h

**Impact**:
- ✅ 60% line reduction
- ✅ Easier to add new file operations
- ✅ Better code reuse
- ⚠️ Lower priority (stable, not blocking Phase 188)

---

### File 6: `src/mir/builder.rs` (1,029 lines)

**Priority**: 🟠 **HIGH** - Most Modified File (322 commits in 2025)

**Purpose**: Main MIR builder orchestration

**Complexity**: 4/5
- 322 commits in 2025 (most modified file!)
- Central orchestrator for AST → MIR conversion

**Activity Level**: VERY HIGH (continuous development)

**Maintainability Issues**:
- **Already partially modularized**:
  - 25+ submodules (calls, context, exprs, stmts, etc.)
  - Good separation of concerns
- **BUT**: Core orchestrator still 1,029 lines
- **Root cause**: Central struct with many responsibilities
  - Module state (`current_module`, `current_function`, `current_block`)
  - ID generation (`value_gen`, `block_gen`)
  - Context management (`compilation_context`)
  - Variable mapping (`variable_map`)
  - Type tracking (`value_types`, `value_origin_newbox`)
- **322 commits = high churn**: indicates ongoing architectural evolution

**Refactoring Plan**: "MirBuilder State → Smaller Context Objects"

**Strategy**:
1. **Function Context** (`function_context.rs`) - 200 lines
   - `current_function`, `current_block`
   - Block management helpers
   - Function-scoped state

2. **ID Generators** (`id_generators.rs`) - 100 lines
   - `value_gen`, `block_gen`
   - ID allocation strategies
   - Region-scoped generation

3. **Variable Context** (`variable_context.rs`) - 150 lines
   - `variable_map`, `variable_origins`
   - SSA variable tracking
   - Scope management

4. **Type Context** (`type_context.rs`) - 150 lines
   - `value_types`, `value_origin_newbox`
   - Type inference state
   - Box origin tracking

5. **Core MirBuilder** (keep in `builder.rs`) - 400 lines
   - High-level orchestration
   - Delegates to context objects
   - Public API

**Effort**: 10-12 hours
- Context extraction: 5h
- API refactoring: 3h
- Migration of existing code: 2h
- Tests: 2h

**Impact**:
- ✅ Reduces churn (isolates changes to specific contexts)
- ✅ Better testability (mock contexts)
- ✅ Clearer responsibility boundaries
- ✅ Easier onboarding for new developers

---

### File 7: `src/runner/mir_json_emit.rs` (960 lines)

**Priority**: 🟢 **MEDIUM** - Can Refactor Later

**Purpose**: Emit MIR JSON for Python harness/PyVM

**Complexity**: 2/5
- Well-structured v0/v1 format support
- Mostly serialization code

**Activity Level**: LOW (stable, Phase 15.5 complete)

**Refactoring Plan**: "v0/v1 Format Separation"

**Effort**: 3-4 hours (Quick Win)

**Impact**: Clean separation of legacy v0 and modern v1 formats

---

### File 8: `src/config/env.rs` (948 lines)

**Priority**: 🟡 **MEDIUM** - Config Management Improvements Needed

**Purpose**: Global environment configuration aggregator

**Complexity**: 3/5
- 184 control flow branches
- 111 functions (mostly small accessors)

**Activity Level**: MEDIUM (126 commits in 2025)

**Refactoring Plan**: "Feature-Based Config Modules"

**Strategy**:
1. **JoinIR Config** (`config_joinir.rs`)
2. **Parser Config** (`config_parser.rs`)
3. **VM Config** (`config_vm.rs`)
4. **Core Env** (keep in `env.rs`)

**Effort**: 5-6 hours

**Impact**: Clearer feature boundaries, easier to find config options

---

## High Priority Files (Should Refactor Soon)

### File 9: `src/mir/join_ir_runner.rs` (866 lines)

**Priority**: 🟡 **MEDIUM-HIGH** - JoinIR Execution Infrastructure

**Purpose**: JoinIR lowering orchestration and execution

**Complexity**: 2/5
- Clean structure
- Will grow with Pattern 4/5

**Refactoring Plan**: Pattern-specific execution handlers

**Effort**: 4-5 hours

---

### File 10: `src/backend/wasm/codegen.rs` (851 lines)

**Priority**: 🟢 **LOW** - WASM Backend (Stable)

**Purpose**: WASM code generation

**Complexity**: 3/5
- Stable implementation
- Low activity

**Refactoring Plan**: Instruction-type modules (can wait)

**Effort**: 6-8 hours

---

## Medium Priority Files (Can Wait)

*Listing 10 more notable files 600-800 lines:*

| File | Lines | Purpose | Priority | Effort |
|------|-------|---------|----------|--------|
| `src/mir/instruction_kinds/mod.rs` | 803 | Instruction type definitions | 🟢 LOW | 4h |
| `src/macro/mod.rs` | 789 | Macro system | 🟢 LOW | 5h |
| `src/macro/macro_box_ny.rs` | 765 | Nyash macro box | 🟢 LOW | 4h |
| `src/runner/json_v1_bridge.rs` | 764 | JSON v1 bridge | 🟢 LOW | 3h |
| `src/mir/join_ir_vm_bridge/joinir_block_converter.rs` | 758 | JoinIR→VM bridge | 🟡 MED | 4h |
| `src/box_factory/mod.rs` | 724 | Box factory | 🟡 MED | 4h |
| `src/backend/mir_interpreter/handlers/extern_provider.rs` | 722 | Extern call provider | 🟢 LOW | 3h |
| `src/boxes/p2p_box.rs` | 713 | P2P networking | 🟢 LOW | 4h |
| `src/runner/pipeline.rs` | 694 | Runner pipeline | 🟡 MED | 5h |
| `src/mir/phi_core/phi_builder_box.rs` | 660 | PHI builder | 🟡 MED | 4h |

---

## Prioritization Matrix

| Rank | File | Lines | Complexity | Activity | Blocks Phase | Effort | Priority |
|------|------|-------|-----------|----------|-------------|--------|----------|
| 1 | `control_flow.rs` | 1,632 | 5/5 | HIGH | ✅ YES (188+) | 12.5h | 🔴 CRITICAL |
| 2 | `loopform_builder.rs` | 1,166 | 4/5 | MED | ✅ YES (188+) | 8h | 🟠 HIGH |
| 3 | `strip.rs` | 1,081 | 3/5 | MED | ⚠️ Maybe (Stage-2) | 6h | 🟠 HIGH |
| 4 | `generic_case_a.rs` | 1,056 | 3/5 | HIGH | ✅ YES (188+) | 4h | 🟡 MED-HIGH |
| 5 | `builder.rs` | 1,029 | 4/5 | VERY HIGH | ⚠️ Indirectly | 10h | 🟠 HIGH |
| 6 | `mir_json_emit.rs` | 960 | 2/5 | LOW | ❌ NO | 3h | 🟢 MED |
| 7 | `env.rs` | 948 | 3/5 | MED | ❌ NO | 5h | 🟡 MED |
| 8 | `handle_box.rs` | 1,052 | 2/5 | LOW | ❌ NO | 4h | 🟢 MED |
| 9 | `join_ir_runner.rs` | 866 | 2/5 | MED | ⚠️ Maybe (188+) | 4h | 🟡 MED-HIGH |
| 10 | `wasm/codegen.rs` | 851 | 3/5 | LOW | ❌ NO | 6h | 🟢 LOW |

**Total Critical Path**: 40.5 hours (Files 1-4: control_flow + loopform + strip + generic_case_a)

---

## Quick Wins (< 3 hours each)

### 1. `src/runner/mir_json_emit.rs` (960 lines) → 3h
**Strategy**: Split v0/v1 format serialization
- `mir_json_emit_v0.rs` - 400 lines (legacy)
- `mir_json_emit_v1.rs` - 400 lines (modern)
- `mir_json_emit.rs` - 150 lines (dispatcher)

**Impact**: Clean format separation, easier to deprecate v0 later

---

### 2. `src/mir/join_ir/lowering/generic_case_a.rs` (1,056 lines) → 3h (preparation)
**Strategy**: Extract EntryFunctionBuilder before Pattern 4/5
- `lowerer_common.rs` - 200 lines (NEW)
- `generic_case_a.rs` - 850 lines (reduced)

**Impact**: Pattern 4/5 implementation becomes easier

---

### 3. `src/config/env.rs` (948 lines) → 3h (partial)
**Strategy**: Extract JoinIR config module
- `config_joinir.rs` - 200 lines (NEW)
- `env.rs` - 750 lines (reduced)

**Impact**: JoinIR config isolation, clearer Phase 188 config management

---

### 4. `src/mir/join_ir_runner.rs` (866 lines) → 3h
**Strategy**: Extract pattern-specific handlers
- `pattern_handlers.rs` - 200 lines (NEW)
- `join_ir_runner.rs` - 660 lines (reduced)

**Impact**: Easier Pattern 4/5 execution handlers

---

### 5. `src/box_factory/mod.rs` (724 lines) → 2h
**Strategy**: Split factory types
- `factory_policy.rs` - 150 lines (NEW)
- `factory_builtin.rs` - 200 lines (NEW)
- `mod.rs` - 370 lines (reduced)

**Impact**: Clearer factory policy management

**Quick Wins Total**: ~14 hours, 5 files improved

---

## Major Efforts (> 8 hours)

### 1. `control_flow.rs` Refactoring: 12.5h
**ROI**: ⭐⭐⭐⭐⭐ (CRITICAL for Phase 188+)
- Unblocks Pattern 4/5 implementation
- Reduces cognitive load 5/5 → 2/5
- Enables unit testing

---

### 2. `builder.rs` Context Extraction: 10h
**ROI**: ⭐⭐⭐⭐ (High churn file)
- Reduces architectural churn
- Better testability
- Clearer boundaries

---

### 3. `loopform_builder.rs` 4-Pass Split: 8h
**ROI**: ⭐⭐⭐⭐ (Needed for Pattern 4/5)
- Clearer architecture
- Easier JoinIR integration
- Better maintainability

**Major Efforts Total**: ~30.5 hours, 3 files

---

## Recommendations

### Phase 1: Critical Path (Before Pattern 4 Implementation)
**Timeline**: 2-3 days focused work

1. **Day 1**: `control_flow.rs` refactoring (12.5h)
   - Extract pattern detection
   - Separate routing policy
   - Create clean orchestrator
   - **Blocks**: Pattern 4/5 implementation

2. **Day 2**: `generic_case_a.rs` + `join_ir_runner.rs` prep (6h)
   - Extract EntryFunctionBuilder
   - Setup pattern handler structure
   - **Enables**: Quick Pattern 4/5 implementation

3. **Day 3**: Quick wins (3-4 files, 8h)
   - `mir_json_emit.rs` v0/v1 split
   - `config_joinir.rs` extraction
   - `box_factory.rs` policy split

**Total Phase 1**: ~26.5 hours
**Impact**: Pattern 4/5 implementation becomes 2-3 hours each (vs 8-12 hours without refactoring)

---

### Phase 2: High-Impact Improvements (After Pattern 4/5 Complete)
**Timeline**: 1-2 weeks

1. `builder.rs` context extraction (10h)
2. `loopform_builder.rs` 4-pass split (8h)
3. `strip.rs` using resolution modules (6h)
4. `env.rs` feature-based config (5h)

**Total Phase 2**: ~29 hours

---

### Phase 3: Code Health (Ongoing Improvements)
**Timeline**: As needed

- Remaining 600-800 line files (4h each)
- Test coverage improvements
- Documentation updates

**Total Phase 3**: ~40-50 hours

---

## Total Effort Summary

| Phase | Scope | Hours | Priority | When |
|-------|-------|-------|----------|------|
| Phase 1 | Critical Path | 26.5h | 🔴 CRITICAL | Before Pattern 4 |
| Phase 2 | High-Impact | 29h | 🟠 HIGH | After Pattern 4/5 |
| Phase 3 | Code Health | 40-50h | 🟢 MEDIUM | Ongoing |
| **Total** | **All Refactorings** | **95-105h** | - | - |

**Recommended Focus**: Phase 1 only (26.5h) before Pattern 4 implementation
**ROI**: Saves 15-20 hours on Pattern 4/5 implementation + future pattern additions

---

## Files Worse Than `control_flow.rs`?

**Answer**: ❌ **NO**

`control_flow.rs` is the worst file by all metrics:
- Longest (1,632 lines)
- Highest complexity (168 control flow branches)
- Highest cognitive load (714-line function)
- Blocks critical Phase 188+ work

**Second worst**: `loopform_builder.rs` (1,166 lines, 67 branches)
**Third worst**: `strip.rs` (1,081 lines, 120 branches)

---

## Known Issues / Architectural Debt

### Issue 1: `control_flow.rs` 714-line function
**Impact**: CRITICAL - Blocks Pattern 4/5 implementation
**Solution**: Phase 1 refactoring (Day 1)

### Issue 2: Pattern-specific lowerers will exceed 2000 lines
**Impact**: HIGH - Maintainability nightmare
**Solution**: Extract lowerer modules NOW (before Pattern 4/5)

### Issue 3: MirBuilder context churn (322 commits)
**Impact**: MEDIUM - High maintenance cost
**Solution**: Phase 2 context extraction

### Issue 4: Multiple 600-800 line files
**Impact**: LOW - Code health, not blocking
**Solution**: Phase 3 gradual improvements

---

## Appendix: Full File List (110 files > 400 lines)

*See raw data in initial analysis output*

**Total lines in files > 400 lines**: 72,936
**Percentage of total codebase**: ~48% (estimate)

---

## Conclusion

### Should We Do This?

**Phase 1 (Critical Path)**: ✅ **YES** - ROI is clear
- 26.5 hours investment
- Saves 15-20 hours on Pattern 4/5
- Unblocks future pattern development
- Reduces cognitive load dramatically

**Phase 2 (High-Impact)**: ⚠️ **MAYBE** - Depends on development velocity
- 29 hours investment
- Improves code health significantly
- Not blocking immediate work
- Consider after Phase 188 complete

**Phase 3 (Code Health)**: 🟢 **OPTIONAL** - Ongoing maintenance
- 40-50 hours investment
- General code quality
- Can be done incrementally
- Low priority

---

### Next Steps

1. **Review this document** with project team
2. **Approve Phase 1 refactoring** (26.5h before Pattern 4)
3. **Create tracking issues** for each Phase 1 file
4. **Start with `control_flow.rs`** (highest priority)
5. **Re-evaluate** after Phase 1 complete

---

**Document Version**: 1.0
**Last Updated**: 2025-12-05
**Author**: Claude Code Analysis System
