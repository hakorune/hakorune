# Phase 243-EX: JoinIR Refactoring & Modularization Opportunities

**Date**: 2025-12-11
**Status**: Investigation Complete (909 tests PASS maintained)
**Goal**: Identify modularization/boxification opportunities after Phase 226-242 implementations

---

## Executive Summary

**Current State**:
- **74 files** in `src/mir/join_ir/lowering/` (23,183 total lines)
- **5 subdirectories**: common, generic_case_a, loop_patterns, loop_scope_shape
- **15 large files** (>500 lines each)
- **9 TODOs** scattered across files
- **Successfully implemented**: CarrierRole, CarrierInit, ExprLowerer, ScopeManager

**Key Findings**:
1. **Condition Lowering Pipeline** is highly fragmented (19 files touch condition logic)
2. **Carrier Management** is partially boxified but spread across 7 files
3. **Exit Line Reconnection** has clean boundary but needs orchestration box
4. **Pattern Detection** logic is scattered (3+ files)
5. **Module Structure** is flat and hard to navigate

**Top Priority**: Condition Lowering Pipeline unification (3 stars)

---

## 1. Module Structure Visualization

### Current Structure (Flat Hierarchy)
```
src/mir/join_ir/lowering/
├── *.rs (52 files in root) ← TOO FLAT, HARD TO NAVIGATE
├── common/              (1 test helper file)
├── generic_case_a/      (5 files, 2.4K lines - Case A implementations)
├── loop_patterns/       (5 files, 1.4K lines - Pattern 1-4 implementations)
└── loop_scope_shape/    (3 files, 1.2K lines - Shape detection)
```

### File Count by Category
| Category | Files | Total Lines | Avg Lines/File |
|----------|-------|-------------|----------------|
| **Root Directory** | 52 | 19,033 | 366 |
| **Subdirectories** | 22 | 4,150 | 188 |
| **Total** | 74 | 23,183 | 313 |

### Large Files (>500 lines)
| File | Lines | Purpose |
|------|-------|---------|
| `carrier_update_emitter` | 956 | Carrier update instruction emission |
| `loop_with_break_minimal.rs` | 868 | Pattern 2 implementation |
| `carrier_info.rs` | 827 | Carrier metadata structures |
| `expr_lowerer.rs` | 796 | Expression lowering (Phase 231) |
| `funcscanner_trim.rs` | 650 | Trim function lowering |
| `method_call_lowerer.rs` | 639 | Method call lowering |
| `inline_boundary.rs` | 576 | JoinIR ↔ Host boundary |
| `loop_update_summary.rs` | 561 | Update pattern analysis |
| `loop_with_continue_minimal.rs` | 551 | Pattern 4 implementation |
| `loop_body_local_init.rs` | 542 | Body-local variable init |
| `condition_lowerer.rs` | 537 | Condition AST → JoinIR |
| `generic_case_a/trim.rs` | 536 | Generic Case A trim |
| `loop_with_if_phi_if_sum.rs` | 535 | Pattern 3 implementation |
| `condition_pattern.rs` | 527 | Condition pattern detection |
| `loop_scope_shape/tests.rs` | 526 | Shape detection tests |

**Analysis**: 15 files account for 9,501 lines (41% of total), indicating room for further decomposition.

---

## 2. Common Code & Duplication Analysis

### 2.1 Condition Processing (19 files)
Files matching `lower.*condition|extract.*condition`:
- `condition_lowerer.rs` (537 lines) - Core lowering logic
- `condition_to_joinir.rs` (154 lines) - Orchestrator
- `condition_env.rs` (237 lines) - Environment management
- `condition_pattern.rs` (527 lines) - Pattern detection
- `condition_var_extractor.rs` (184 lines) - Variable extraction
- `loop_with_if_phi_if_sum.rs` (535 lines) - Pattern 3 uses all above
- + 13 other files

**Problem**: Condition lowering logic is scattered across 19 files with unclear responsibilities.

**Opportunity**: Create unified **ConditionLoweringBox** trait with implementations:
- `SimpleConditionLowerer` (vars + literals)
- `ComplexConditionLowerer` (BinaryOp + MethodCall)
- `LegacyConditionLowerer` (fallback)

### 2.2 Variable/Carrier Processing (13 files)
Files matching `variable_map|carrier_info`:
- `carrier_info.rs` (827 lines) - Metadata structures
- `carrier_update_emitter` (956 lines) - Update emission
- `inline_boundary.rs` (576 lines) - Boundary management
- `inline_boundary_builder.rs` (493 lines) - Builder pattern
- `scope_manager.rs` (354 lines) - Unified lookup (Phase 231)
- + 8 other files

**Status**: Partially boxified (Phase 228 CarrierRole/CarrierInit, Phase 231 ScopeManager).

**Opportunity**: Extend **CarrierManagerBox** to consolidate carrier lifecycle:
- Initialization (CarrierInit)
- Role management (CarrierRole)
- Update tracking
- Exit binding generation

### 2.3 PHI Processing (9 files)
Files matching `loop.*phi|phi.*init|phi.*exit`:
- `inline_boundary.rs` (exit PHI bindings)
- `condition_pattern.rs` (if-phi detection)
- `if_phi_spec.rs` (if-phi specification)
- `if_phi_context.rs` (if-phi context)
- `loop_with_if_phi_if_sum.rs` (Pattern 3 if-sum)
- + 4 other files in loop_patterns/

**Problem**: PHI logic is split between "if-phi" and "loop-phi" with overlapping concerns.

**Opportunity**: Create **PHIGeneratorBox** trait:
- `LoopHeaderPHI` (carrier initialization)
- `ExitPHI` (loop result propagation)
- `IfMergePHI` (if-expression merging)

### 2.4 Lowerer Patterns (7 struct definitions)
```rust
pub struct ExprLowerer<'env, 'builder, S: ScopeManager> { ... }
pub struct BoolExprLowerer<'a> { ... }  // TODO: Consider removal
pub struct LoopBodyLocalInitLowerer<'a> { ... }
pub struct MethodCallLowerer { ... }
pub struct LoopToJoinLowerer { ... }
pub struct IfMergeLowerer { ... }
pub struct IfSelectLowerer { ... }
```

**Analysis**: 81 total functions matching `fn lower_|fn extract_|fn emit_` across 35 files.

**Opportunity**: Standardize on unified **LoweringBox** trait:
```rust
trait LoweringBox {
    fn lower(&mut self, ast: &ASTNode) -> Result<(ValueId, Vec<JoinInst>), LoweringError>;
    fn context(&self) -> LoweringContext;
    fn can_handle(&self, ast: &ASTNode) -> bool;
}
```

---

## 3. Boxification Candidates (Priority Order)

### 3.1 ConditionLoweringBox (⭐⭐⭐ Highest Priority)
**Impact**: High (19 files affected)
**Effort**: Medium (unify existing 5 modules)
**Value**: High (single responsibility, reusable across patterns)

**Current Fragmentation**:
```
condition_lowerer.rs       - Core AST → JoinIR logic
condition_to_joinir.rs     - Orchestrator (thin wrapper)
condition_env.rs           - Environment (variable resolution)
condition_pattern.rs       - Pattern detection (Simple vs Complex)
condition_var_extractor.rs - Variable extraction from AST
```

**Proposed Structure**:
```rust
// Box A: Unified interface
pub trait ConditionLoweringBox {
    fn lower(&mut self, cond: &ASTNode) -> Result<ConditionResult, String>;
    fn pattern(&self) -> ConditionPattern;
    fn extract_variables(&self, cond: &ASTNode) -> Vec<String>;
}

// Box B: Implementation selector
pub struct ConditionLoweringDispatcher {
    simple: SimpleConditionLowerer,
    complex: ComplexConditionLowerer,
    legacy: LegacyConditionLowerer,
}

impl ConditionLoweringDispatcher {
    pub fn lower(&mut self, cond: &ASTNode, env: &ConditionEnv) -> Result<...> {
        match analyze_condition_pattern(cond) {
            ConditionPattern::SimpleComparison => self.simple.lower(cond, env),
            ConditionPattern::Complex => self.complex.lower(cond, env),
        }
    }
}
```

**Benefits**:
- Single entry point for all condition lowering
- Pattern-specific optimizations (Simple vs Complex)
- Easy to add new patterns (e.g., `MethodCallCondition`)
- Testable in isolation

**Risks**:
- Medium refactoring effort (19 files)
- Must maintain backward compatibility (909 tests)

---

### 3.2 CarrierManagerBox (⭐⭐ Medium Priority)
**Impact**: Medium (7 files affected)
**Effort**: Low (extend existing Phase 228 infrastructure)
**Value**: Medium (consolidates carrier lifecycle)

**Current State**:
- Phase 227: `CarrierRole` enum (LoopState vs ConditionOnly) ✅
- Phase 228: `CarrierInit` enum (FromHost vs BoolConst) ✅
- Phase 231: `ScopeManager` trait (variable lookup) ✅

**Missing Pieces**:
- Unified carrier initialization logic
- Carrier update tracking
- Exit binding generation
- Role-based PHI participation

**Proposed Extension**:
```rust
pub struct CarrierManagerBox {
    carrier_info: CarrierInfo,
    scope: Box<dyn ScopeManager>,
    update_emitter: CarrierUpdateEmitter,
}

impl CarrierManagerBox {
    pub fn init_carriers(&mut self, ...) -> Vec<JoinInst> { ... }
    pub fn update_carrier(&mut self, name: &str, new_value: ValueId) -> Result<...> { ... }
    pub fn generate_exit_bindings(&self) -> Vec<LoopExitBinding> { ... }
    pub fn carriers_for_phi(&self, role: Option<CarrierRole>) -> Vec<&CarrierVar> { ... }
}
```

**Benefits**:
- Single source of truth for carrier management
- Easier to add new CarrierRole types
- Consolidates 3 related modules (carrier_info, carrier_update_emitter, inline_boundary)

**Risks**:
- Low risk (builds on existing Phase 228 infrastructure)

---

### 3.3 ExitLineReconnectorBox (⭐ Low Priority)
**Impact**: Low (2 files affected)
**Effort**: Low (already modularized in Phase 33)
**Value**: Low (works well, just needs orchestration)

**Current State** (Phase 33-10):
- `ExitLineReconnector` (exists)
- `ExitMetaCollector` (exists)
- `ExitLineOrchestrator` (exists)

**Status**: Already boxified! ✅

**Opportunity**: Add convenience methods for common patterns.

---

### 3.4 PatternDetectorBox (⭐⭐ Medium Priority)
**Impact**: Medium (4+ files affected)
**Effort**: Medium (consolidate pattern detection logic)
**Value**: Medium (single entry point for pattern classification)

**Current Fragmentation**:
```
condition_pattern.rs        - If condition patterns (Simple vs Complex)
loop_pattern_validator.rs   - Loop structure validation
loop_pattern_router.rs      - Pattern 1-4 routing
loop_update_summary.rs      - Update pattern analysis
```

**Proposed Structure**:
```rust
pub trait PatternDetectorBox {
    fn detect(&self, ast: &ASTNode) -> PatternKind;
    fn validate(&self, ast: &ASTNode) -> Result<(), String>;
}

pub enum PatternKind {
    Loop(LoopPattern),  // Pattern 1-4
    If(IfPattern),      // Simple vs Complex
    Update(UpdatePattern),
}

pub struct UnifiedPatternDetector {
    loop_detector: LoopPatternDetector,
    if_detector: IfPatternDetector,
    update_detector: UpdatePatternDetector,
}
```

**Benefits**:
- Unified pattern detection API
- Easier to add new patterns
- Consolidates 4 related modules

**Risks**:
- Medium effort (pattern detection is spread across multiple phases)

---

## 4. Module Reorganization Proposal

### Current (Flat, Hard to Navigate)
```
src/mir/join_ir/lowering/
├── *.rs (52 files) ← TOO MANY FILES IN ROOT
└── (4 subdirectories)
```

### Proposed (Hierarchical, Clear Responsibilities)
```
src/mir/join_ir/lowering/
├── core/                            # Core lowering boxes (NEW)
│   ├── condition_lowering/          # ConditionLoweringBox (Box A)
│   │   ├── mod.rs                   # Trait + dispatcher
│   │   ├── simple_lowerer.rs        # Simple comparisons
│   │   ├── complex_lowerer.rs       # BinaryOp + MethodCall
│   │   ├── legacy_lowerer.rs        # Fallback
│   │   ├── pattern.rs               # ConditionPattern enum
│   │   └── env.rs                   # ConditionEnv
│   ├── carrier_management/          # CarrierManagerBox (Box B)
│   │   ├── mod.rs                   # Unified manager
│   │   ├── info.rs                  # CarrierInfo structures
│   │   ├── update_emitter.rs        # Update emission
│   │   └── role.rs                  # CarrierRole/CarrierInit
│   └── exit_line/                   # ExitLineReconnectorBox (Box C)
│       ├── mod.rs                   # Orchestrator
│       ├── reconnector.rs           # Reconnection logic
│       └── meta_collector.rs        # Metadata collection
├── infrastructure/                  # Shared utilities (KEEP)
│   ├── expression_lowering.rs       # ExprLowerer (Phase 231)
│   ├── scope_manager.rs             # ScopeManager trait
│   ├── join_value_space.rs          # ValueId allocation
│   ├── inline_boundary.rs           # Boundary structures
│   └── common.rs                    # CFG sanity checks
├── patterns/                        # Pattern-specific lowering (REORGANIZE)
│   ├── detection/                   # PatternDetectorBox (NEW)
│   │   ├── mod.rs                   # Unified detector
│   │   ├── loop_pattern.rs          # Loop patterns 1-4
│   │   ├── if_pattern.rs            # If patterns
│   │   └── update_pattern.rs        # Update patterns
│   ├── loop_patterns/               # Pattern 1-4 implementations (KEEP)
│   │   ├── mod.rs
│   │   ├── simple_while.rs          # Pattern 1
│   │   ├── with_break.rs            # Pattern 2
│   │   ├── with_if_phi.rs           # Pattern 3
│   │   └── with_continue.rs         # Pattern 4
│   ├── if_lowering/                 # If-expression lowering (NEW)
│   │   ├── mod.rs
│   │   ├── select.rs                # If/Select lowering
│   │   ├── merge.rs                 # IfMerge lowering
│   │   └── router.rs                # Routing logic
│   └── routers/                     # Dispatching logic (CONSOLIDATE)
│       ├── loop_pattern_router.rs
│       └── if_lowering_router.rs
├── specialized/                     # Function-specific lowering (KEEP)
│   ├── min_loop.rs
│   ├── skip_ws.rs
│   ├── funcscanner_trim.rs
│   ├── funcscanner_append_defs.rs
│   ├── stage1_using_resolver.rs
│   ├── stageb_body.rs
│   └── stageb_funcscanner.rs
├── generic_case_a/                  # Generic Case A (KEEP)
│   ├── mod.rs
│   ├── skip_ws.rs
│   ├── trim.rs
│   ├── append_defs.rs
│   └── stage1_using_resolver.rs
├── loop_scope_shape/                # Shape detection (KEEP)
│   ├── mod.rs
│   ├── case_a_lowering_shape.rs
│   └── tests.rs
└── mod.rs                           # Top-level re-exports
```

**Benefits**:
- **Clear hierarchy**: 7 top-level modules vs 52 files in root
- **Single responsibility**: Each module has one concern
- **Easy navigation**: Condition lowering → `core/condition_lowering/`
- **Scalability**: Adding new patterns = new file in appropriate directory
- **Backward compatibility**: Re-export from `mod.rs` maintains existing API

**Metrics**:
- **Before**: 52 files in root (74 total)
- **After**: 7 directories in root (74 total, reorganized)
- **Reduction**: 86% fewer files in root directory

---

## 5. Dependency Graph Analysis

### 5.1 High-Dependency Modules (Used by 10+ files)
1. **condition_lowerer.rs** (used by 10 files)
   - `expr_lowerer.rs`
   - `condition_to_joinir.rs`
   - `loop_with_if_phi_if_sum.rs`
   - All 4 pattern implementations
   - + 3 other files

2. **carrier_info.rs** (used by 7 files)
   - `inline_boundary.rs`
   - `inline_boundary_builder.rs`
   - `carrier_update_emitter`
   - `loop_update_analyzer.rs`
   - All pattern implementations

3. **scope_manager.rs** (used by 1 file, NEW in Phase 231)
   - `expr_lowerer.rs` (only user so far)

**Analysis**: `condition_lowerer.rs` is a critical dependency. Boxifying it will require careful coordination.

### 5.2 Cross-Module Dependencies
```
ExprLowerer (Phase 231)
  ↓ depends on
ScopeManager (Phase 231)
  ↓ depends on
ConditionEnv (Phase 171)
  ↓ depends on
CarrierInfo (Phase 196)
  ↓ depends on
InlineBoundary (Phase 188)
```

**Observation**: Clean dependency flow from top (ExprLowerer) to bottom (InlineBoundary).

---

## 6. Implementation Sketches (Top 3 Priorities)

### 6.1 ConditionLoweringBox (⭐⭐⭐)

**Files to Consolidate**:
- `condition_lowerer.rs` (537 lines)
- `condition_to_joinir.rs` (154 lines)
- `condition_env.rs` (237 lines)
- `condition_pattern.rs` (527 lines)
- `condition_var_extractor.rs` (184 lines)
- **Total**: 1,639 lines → Split into 5 focused modules

**Implementation Sketch**:
```rust
// src/mir/join_ir/lowering/core/condition_lowering/mod.rs

pub trait ConditionLoweringBox {
    /// Lower condition AST to JoinIR ValueId + instructions
    fn lower(&mut self, cond: &ASTNode, env: &ConditionEnv)
        -> Result<(ValueId, Vec<JoinInst>), String>;

    /// Detect pattern (Simple vs Complex)
    fn pattern(&self) -> ConditionPattern;

    /// Extract variables used in condition
    fn extract_variables(&self, cond: &ASTNode) -> Vec<String>;
}

pub struct ConditionLoweringDispatcher {
    simple: SimpleConditionLowerer,
    complex: ComplexConditionLowerer,
}

impl ConditionLoweringDispatcher {
    pub fn new() -> Self {
        Self {
            simple: SimpleConditionLowerer,
            complex: ComplexConditionLowerer,
        }
    }

    pub fn lower(&mut self, cond: &ASTNode, env: &ConditionEnv, alloc: &mut impl FnMut() -> ValueId)
        -> Result<(ValueId, Vec<JoinInst>), String>
    {
        match analyze_condition_pattern(cond) {
            ConditionPattern::SimpleComparison => {
                self.simple.lower(cond, env, alloc)
            }
            ConditionPattern::Complex => {
                self.complex.lower(cond, env, alloc)
            }
        }
    }
}

// src/mir/join_ir/lowering/core/condition_lowering/simple_lowerer.rs
pub struct SimpleConditionLowerer;

impl SimpleConditionLowerer {
    pub fn lower(&mut self, cond: &ASTNode, env: &ConditionEnv, alloc: &mut impl FnMut() -> ValueId)
        -> Result<(ValueId, Vec<JoinInst>), String>
    {
        // Handles: var CmpOp literal, var CmpOp var
        // Reuses existing condition_lowerer.rs logic for simple cases
        // ...
    }
}

// src/mir/join_ir/lowering/core/condition_lowering/complex_lowerer.rs
pub struct ComplexConditionLowerer;

impl ComplexConditionLowerer {
    pub fn lower(&mut self, cond: &ASTNode, env: &ConditionEnv, alloc: &mut impl FnMut() -> ValueId)
        -> Result<(ValueId, Vec<JoinInst>), String>
    {
        // Handles: BinaryOp CmpOp literal, MethodCall, etc.
        // Uses lower_value_expression for LHS/RHS
        // Phase 242-EX-A already supports this!
        // ...
    }
}
```

**Migration Strategy**:
1. Create new directory: `core/condition_lowering/`
2. Copy existing code to new structure (no logic changes)
3. Add trait definition + dispatcher
4. Update 19 files to use dispatcher instead of direct calls
5. Run all 909 tests (expect 100% pass)
6. Delete old files

**Risk Mitigation**:
- **No logic changes** in Step 2-3 (pure reorganization)
- **Backward compatibility shim** in old file locations (re-export)
- **Gradual migration** (update callers one by one)

---

### 6.2 CarrierManagerBox Extension (⭐⭐)

**Files to Extend**:
- `carrier_info.rs` (827 lines) - Already has CarrierRole/CarrierInit
- `carrier_update_emitter` (956 lines) - Update emission logic
- `inline_boundary.rs` (576 lines) - Exit binding generation

**Implementation Sketch**:
```rust
// src/mir/join_ir/lowering/core/carrier_management/mod.rs

pub struct CarrierManagerBox {
    carrier_info: CarrierInfo,
    scope: Box<dyn ScopeManager>,
    update_emitter: CarrierUpdateEmitter,
}

impl CarrierManagerBox {
    /// Initialize all carriers with their initial values
    pub fn init_carriers(&mut self, join_value_space: &mut JoinValueSpace)
        -> Vec<JoinInst>
    {
        let mut insts = Vec::new();
        for carrier in self.carrier_info.carriers.iter_mut() {
            match carrier.init {
                CarrierInit::FromHost => {
                    // Use existing host_id value (no instruction needed)
                }
                CarrierInit::BoolConst(value) => {
                    // Emit Const instruction
                    let vid = join_value_space.alloc_local();
                    carrier.join_id = Some(vid);
                    insts.push(JoinInst::MirLike(MirLikeInst::Const {
                        dst: vid,
                        value: ConstValue::Bool(value),
                    }));
                }
            }
        }
        insts
    }

    /// Generate exit PHI bindings (only for LoopState carriers)
    pub fn generate_exit_bindings(&self) -> Vec<LoopExitBinding> {
        self.carrier_info.carriers.iter()
            .filter(|c| c.role == CarrierRole::LoopState)
            .filter_map(|c| c.join_id.map(|jid| {
                LoopExitBinding {
                    carrier_name: c.name.clone(),
                    join_exit_value: jid,
                    host_slot: c.host_id,
                    role: c.role,
                }
            }))
            .collect()
    }

    /// Get carriers participating in PHI (filter by role)
    pub fn carriers_for_phi(&self, role: Option<CarrierRole>) -> Vec<&CarrierVar> {
        match role {
            Some(r) => self.carrier_info.carriers.iter().filter(|c| c.role == r).collect(),
            None => self.carrier_info.carriers.iter().collect(),
        }
    }
}
```

**Benefits**:
- **Single API** for carrier initialization, updates, and exit bindings
- **Role-based filtering** (LoopState vs ConditionOnly)
- **Encapsulation** of 3 related modules

**Migration Strategy**:
1. Create `CarrierManagerBox` struct (wraps existing CarrierInfo)
2. Move `init_carriers()` logic from `loop_with_break_minimal.rs` (repeated 3x)
3. Move `generate_exit_bindings()` from `inline_boundary_builder.rs`
4. Update 3 pattern implementations to use manager
5. Run tests

**Risk**: Low (extends existing Phase 228 infrastructure, no breaking changes)

---

### 6.3 PatternDetectorBox (⭐⭐)

**Files to Consolidate**:
- `condition_pattern.rs` (527 lines) - If pattern detection
- `loop_pattern_validator.rs` (212 lines) - Loop validation
- `loop_update_summary.rs` (561 lines) - Update pattern analysis

**Implementation Sketch**:
```rust
// src/mir/join_ir/lowering/patterns/detection/mod.rs

pub trait PatternDetectorBox {
    fn detect(&self, ast: &ASTNode) -> PatternKind;
    fn validate(&self, ast: &ASTNode) -> Result<(), String>;
}

pub enum PatternKind {
    Loop(LoopPattern),
    If(IfPattern),
    Update(UpdatePattern),
}

pub enum LoopPattern {
    SimpleWhile,      // Pattern 1
    WithBreak,        // Pattern 2
    WithIfPhi,        // Pattern 3
    WithContinue,     // Pattern 4
    Generic,          // Generic Case A
}

pub enum IfPattern {
    SimpleComparison, // var CmpOp literal
    Complex,          // BinaryOp, MethodCall
}

pub enum UpdatePattern {
    SimpleIncrement,  // i = i + 1
    CarrierUpdate,    // sum = sum + val
    Complex,          // Other
}

pub struct UnifiedPatternDetector {
    loop_detector: LoopPatternDetector,
    if_detector: IfPatternDetector,
    update_detector: UpdatePatternDetector,
}

impl UnifiedPatternDetector {
    pub fn detect(&self, ast: &ASTNode) -> PatternKind {
        if self.loop_detector.is_loop(ast) {
            PatternKind::Loop(self.loop_detector.detect(ast))
        } else if self.if_detector.is_if(ast) {
            PatternKind::If(self.if_detector.detect(ast))
        } else if self.update_detector.is_update(ast) {
            PatternKind::Update(self.update_detector.detect(ast))
        } else {
            panic!("Unknown pattern: {:?}", ast)
        }
    }
}
```

**Benefits**:
- **Single entry point** for all pattern detection
- **Consistent API** across loop/if/update patterns
- **Easy to extend** with new patterns

**Migration Strategy**:
1. Create new directory: `patterns/detection/`
2. Move `analyze_condition_pattern()` → `if_detector.rs`
3. Move loop validation logic → `loop_detector.rs`
4. Move update analysis → `update_detector.rs`
5. Create unified API in `mod.rs`
6. Update callers (loop_pattern_router.rs, if_lowering_router.rs)

**Risk**: Medium (pattern detection is spread across 3+ modules)

---

## 7. Risk Assessment

### 7.1 Test Impact Analysis
- **Current**: 909 tests PASS ✅
- **Risk Level by Candidate**:

| Candidate | Tests Affected | Risk | Mitigation |
|-----------|----------------|------|------------|
| ConditionLoweringBox | 50+ (all condition tests) | **Medium** | Gradual migration, backward compat shim |
| CarrierManagerBox | 20+ (Pattern 2-4 tests) | **Low** | Extends existing, no breaking changes |
| ExitLineReconnectorBox | 5 (already boxified) | **None** | Already done in Phase 33 |
| PatternDetectorBox | 30+ (pattern detection tests) | **Medium** | API unification requires coordination |

### 7.2 Breaking Changes Risk
- **ConditionLoweringBox**: Medium risk (19 files depend on condition_lowerer.rs)
- **CarrierManagerBox**: Low risk (extends existing API)
- **PatternDetectorBox**: Medium risk (changes pattern detection API)

**Mitigation**:
1. **Backward compatibility shims** in old file locations
2. **Gradual migration** (update callers one by one)
3. **No logic changes** in initial refactoring (pure reorganization)
4. **Test-driven** (run all 909 tests after each step)

---

## 8. Priority Scorecard

| Candidate | Impact | Effort | Value | Priority | Score |
|-----------|--------|--------|-------|----------|-------|
| **ConditionLoweringBox** | High (19 files) | Medium (1,639 lines) | High (single API, reusable) | ⭐⭐⭐ | 9 |
| **CarrierManagerBox** | Medium (7 files) | Low (extends Phase 228) | Medium (consolidates lifecycle) | ⭐⭐ | 6 |
| **PatternDetectorBox** | Medium (4 files) | Medium (3 modules) | Medium (unified detection) | ⭐⭐ | 6 |
| **Module Reorganization** | Medium (navigation) | Large (74 files) | Medium (clarity) | ⭐⭐ | 5 |
| **ExitLineReconnectorBox** | Low (already done) | None | Low | ⭐ | 1 |

**Scoring**: Impact × Value - Effort (normalized 1-10)

---

## 9. Recommended Roadmap (Next 3-5 Phases)

### Phase 244: ConditionLoweringBox Unification (⭐⭐⭐)
**Goal**: Consolidate 5 condition-related modules into unified box.

**Steps**:
1. Create `core/condition_lowering/` directory structure
2. Define `ConditionLoweringBox` trait + dispatcher
3. Move existing code to new modules (no logic changes)
4. Add backward compatibility shims
5. Update 19 callers to use dispatcher
6. Run all 909 tests (expect 100% pass)
7. Document new API in CLAUDE.md

**Estimated Effort**: 1-2 days
**Risk**: Medium (19 files affected)
**Value**: High (single API for all condition lowering)

---

### Phase 245: CarrierManagerBox Extension (⭐⭐)
**Goal**: Extend Phase 228 CarrierRole/CarrierInit with unified lifecycle management.

**Steps**:
1. Create `CarrierManagerBox` struct (wraps CarrierInfo)
2. Move `init_carriers()` logic from 3 pattern implementations
3. Move `generate_exit_bindings()` from inline_boundary_builder
4. Add `carriers_for_phi()` convenience method
5. Update Pattern 2-4 to use manager
6. Run tests

**Estimated Effort**: 0.5-1 day
**Risk**: Low (extends existing API)
**Value**: Medium (consolidates 3 modules)

---

### Phase 246: Module Reorganization (⭐⭐)
**Goal**: Reorganize flat 52-file root into 7 hierarchical directories.

**Steps**:
1. Create new directory structure (core/, infrastructure/, patterns/, specialized/)
2. Move files to appropriate directories
3. Update `mod.rs` with re-exports
4. Update import paths in all files
5. Run all 909 tests (expect 100% pass)
6. Update documentation

**Estimated Effort**: 1-2 days
**Risk**: Low (pure reorganization, no logic changes)
**Value**: Medium (navigation + clarity)

---

### Phase 247: PatternDetectorBox Unification (⭐⭐)
**Goal**: Consolidate pattern detection logic into single API.

**Steps**:
1. Create `patterns/detection/` directory
2. Define `PatternDetectorBox` trait
3. Move if pattern detection → `if_detector.rs`
4. Move loop validation → `loop_detector.rs`
5. Move update analysis → `update_detector.rs`
6. Create unified API in `mod.rs`
7. Update loop_pattern_router + if_lowering_router

**Estimated Effort**: 1 day
**Risk**: Medium (3 modules affected)
**Value**: Medium (unified detection API)

---

### Phase 248: Legacy Cleanup (⭐)
**Goal**: Remove backward compatibility shims from Phase 244-247.

**Steps**:
1. Remove old condition_lowerer.rs (after all callers migrated)
2. Remove backward compatibility re-exports
3. Update documentation
4. Run final test sweep

**Estimated Effort**: 0.5 day
**Risk**: Low
**Value**: High (clean codebase)

---

## 10. Open Questions & Future Work

### 10.1 TODO Items (9 total)
1. `loop_with_continue_minimal.rs:499` - Return mechanism for exit values
2. `bool_expr_lowerer.rs:39` - **Consider removal** (unused module)
3. `bool_expr_lowerer.rs:213` - Update tests for new MirBuilder API
4. `loop_with_if_phi_if_sum.rs:139` - Get init value from AST
5. `simple_while.rs:61` - Implement proper detection
6. `simple_while.rs:86` - Phase 188-4 implementation
7. `stage1_using_resolver.rs:338` - Stricter CFG pattern matching
8. `if_phi_spec.rs:61` - Reverse lookup variable_name from dst
9. `if_phi_spec.rs:66` - Reverse lookup variable_name from merge_pair.dst

**Recommendation**: Address #2 (bool_expr_lowerer removal) in Phase 244 or 248.

### 10.2 Unused Module Candidate
- **`bool_expr_lowerer.rs` (446 lines)**: TODO comment says "Consider removal or unification".
  - All tests are commented out
  - Appears to be superseded by ExprLowerer (Phase 231)
  - **Action**: Remove in Phase 248 (Legacy Cleanup)

### 10.3 Future Opportunities (Beyond Phase 248)
1. **Generic Case A Unification**: Consolidate 5 specialized lowerers into generic pipeline
2. **Method Call Lowering Extension**: Support more methods (currently limited whitelist)
3. **Loop Body Local Promotion**: Automate ConditionOnly carrier detection
4. **PHI Generation Unification**: Single API for loop/if/exit PHI
5. **ValueId Allocation**: Unify JoinValueSpace usage across all lowerers

---

## 11. Conclusion

**Summary**:
- **74 files** in JoinIR lowering (23,183 lines) - ripe for modularization
- **5 major opportunities** identified (ConditionLowering, CarrierManager, PatternDetector, Module Reorg, Legacy Cleanup)
- **Recommended priority**: ConditionLoweringBox (⭐⭐⭐), CarrierManagerBox (⭐⭐), Module Reorg (⭐⭐)
- **Estimated timeline**: 4-6 days for Phases 244-248

**Key Principles**:
1. **Box-First**: Consolidate related logic into unified boxes
2. **Single Responsibility**: Each module has one clear purpose
3. **Backward Compatibility**: Gradual migration with shims
4. **Test-Driven**: All 909 tests must pass at each step
5. **Fail-Safe**: No logic changes in initial refactoring (pure reorganization)

**Next Steps**:
1. Review this report with stakeholders
2. Approve Phase 244 (ConditionLoweringBox) implementation plan
3. Begin implementation following recommended roadmap

**Status**: Ready for Phase 244 implementation! 🚀

---

**End of Phase 243-EX Investigation Report**
