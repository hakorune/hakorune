# Phase 243-EX: JoinIR Lowering Dependency Graph

**Companion to**: [phase243-ex-refactoring-opportunities.md](phase243-ex-refactoring-opportunities.md)

---

## 1. High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    JoinIR Lowering System                   │
│                                                             │
│  ┌─────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │   Pattern   │───▶│  Condition   │───▶│   Carrier    │  │
│  │  Detection  │    │   Lowering   │    │  Management  │  │
│  └─────────────┘    └──────────────┘    └──────────────┘  │
│         │                   │                    │          │
│         ▼                   ▼                    ▼          │
│  ┌─────────────────────────────────────────────────────┐   │
│  │          JoinIR Module Generation                   │   │
│  │  (Pattern 1-4 implementations)                      │   │
│  └─────────────────────────────────────────────────────┘   │
│         │                                                   │
│         ▼                                                   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │    Inline Boundary & Exit Line Reconnection        │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Core Module Dependencies (Critical Path)

### 2.1 Condition Lowering Pipeline (19 files)

```
┌──────────────────────────────────────────────────────────────┐
│              Condition Lowering Pipeline                     │
│  (Currently fragmented across 19 files)                     │
└──────────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
┌───────────────┐  ┌──────────────┐  ┌──────────────────┐
│ Pattern       │  │ Condition    │  │ Variable         │
│ Detection     │  │ Lowering     │  │ Extraction       │
│ (527 lines)   │  │ (537 lines)  │  │ (184 lines)      │
└───────────────┘  └──────────────┘  └──────────────────┘
condition_pattern.rs condition_lowerer.rs condition_var_extractor.rs
        │                  │                  │
        └──────────────────┼──────────────────┘
                           ▼
                  ┌──────────────────┐
                  │ Condition Env    │
                  │ (237 lines)      │
                  └──────────────────┘
                  condition_env.rs
                           │
                           ▼
                  ┌──────────────────┐
                  │ Orchestrator     │
                  │ (154 lines)      │
                  └──────────────────┘
                  condition_to_joinir.rs

Used by (10 files):
- expr_lowerer.rs
- loop_with_if_phi_if_sum.rs (Pattern 3)
- loop_with_break_minimal.rs (Pattern 2)
- loop_with_continue_minimal.rs (Pattern 4)
- simple_while_minimal.rs (Pattern 1)
- loop_patterns/with_if_phi.rs
- loop_patterns/with_break.rs
- loop_patterns/with_continue.rs
- loop_patterns/simple_while.rs
- condition_to_joinir.rs
```

**Boxification Opportunity**: Create `ConditionLoweringBox` to unify these 5 modules.

---

### 2.2 Carrier Management (7 files)

```
┌──────────────────────────────────────────────────────────────┐
│              Carrier Management Pipeline                     │
│  (Partially boxified: CarrierRole, CarrierInit)            │
└──────────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
┌───────────────┐  ┌──────────────┐  ┌──────────────────┐
│ Carrier Info  │  │ Update       │  │ Boundary         │
│ (827 lines)   │  │ Emitter      │  │ Builder          │
│               │  │ (956 lines)  │  │ (493 lines)      │
└───────────────┘  └──────────────┘  └──────────────────┘
carrier_info.rs  carrier_update_emitter  inline_boundary_builder.rs
        │                  │                  │
        │                  │                  │
        │                  ▼                  │
        │         ┌──────────────┐            │
        │         │ Scope        │            │
        │         │ Manager      │            │
        │         │ (354 lines)  │            │
        │         └──────────────┘            │
        │         scope_manager.rs            │
        │                  │                  │
        └──────────────────┼──────────────────┘
                           ▼
                  ┌──────────────────┐
                  │ Inline Boundary  │
                  │ (576 lines)      │
                  └──────────────────┘
                  inline_boundary.rs

Phase 227-228 Infrastructure:
- CarrierRole (LoopState vs ConditionOnly)
- CarrierInit (FromHost vs BoolConst)
- Phase 231: ScopeManager trait

Used by (7 files):
- inline_boundary.rs
- inline_boundary_builder.rs
- carrier_update_emitter
- loop_update_analyzer.rs
- loop_with_break_minimal.rs
- loop_with_continue_minimal.rs
- loop_with_if_phi_if_sum.rs
```

**Boxification Opportunity**: Create `CarrierManagerBox` to consolidate lifecycle management.

---

### 2.3 Expression Lowering (Phase 231 Infrastructure)

```
┌──────────────────────────────────────────────────────────────┐
│              Expression Lowering (Phase 231)                 │
│  (Pilot implementation - Condition context only)            │
└──────────────────────────────────────────────────────────────┘
                           │
                           ▼
                  ┌──────────────────┐
                  │ ExprLowerer      │
                  │ (796 lines)      │
                  └──────────────────┘
                  expr_lowerer.rs
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
┌───────────────┐  ┌──────────────┐  ┌──────────────────┐
│ Scope         │  │ Condition    │  │ Method Call      │
│ Manager       │  │ Lowerer      │  │ Lowerer          │
│ (354 lines)   │  │ (537 lines)  │  │ (639 lines)      │
└───────────────┘  └──────────────┘  └──────────────────┘
scope_manager.rs  condition_lowerer.rs method_call_lowerer.rs

Design:
- trait ScopeManager { lookup(&str) -> Option<ValueId> }
- enum ExprContext { Condition, General }
- enum ExprLoweringError { UnsupportedNode, VariableNotFound, ... }

Used by (1 file so far):
- loop_with_if_phi_if_sum.rs (Pattern 3 if-sum mode)

Future Expansion:
- Phase 240-EX: Integrated into if-sum mode ✅
- Phase 242-EX-A: Supports complex conditions (BinaryOp LHS) ✅
- Future: General expression context (method calls, etc.)
```

**Status**: Successfully piloted in Pattern 3, ready for expansion.

---

## 3. Pattern Implementation Dependencies

### 3.1 Pattern 1: Simple While

```
simple_while_minimal.rs (239 lines)
         │
         ├─▶ condition_lowerer.rs
         ├─▶ condition_env.rs
         ├─▶ carrier_info.rs
         ├─▶ inline_boundary.rs
         └─▶ join_value_space.rs

Characteristics:
- No break/continue
- Simple loop condition
- Straightforward carrier management
```

---

### 3.2 Pattern 2: With Break

```
loop_with_break_minimal.rs (868 lines)
         │
         ├─▶ condition_lowerer.rs       (condition lowering)
         ├─▶ carrier_info.rs            (carrier management)
         ├─▶ carrier_update_emitter  (update emission)
         ├─▶ inline_boundary.rs         (exit bindings)
         ├─▶ inline_boundary_builder.rs (boundary construction)
         ├─▶ join_value_space.rs        (ValueId allocation)
         └─▶ scope_manager.rs           (variable lookup)

Characteristics:
- Two exit paths (natural + break)
- Break condition lowering
- Multiple carrier updates
- Exit PHI generation

Complexity: 868 lines (largest pattern implementation)
```

---

### 3.3 Pattern 3: With If-Phi (If-Sum Mode)

```
loop_with_if_phi_if_sum.rs (535 lines)
         │
         ├─▶ expr_lowerer.rs           ✅ Phase 240-EX integration
         ├─▶ condition_lowerer.rs      (loop/if conditions)
         ├─▶ condition_pattern.rs      (pattern detection)
         ├─▶ condition_env.rs          (variable resolution)
         ├─▶ carrier_info.rs           (carrier metadata)
         ├─▶ join_value_space.rs       (ValueId allocation)
         └─▶ method_call_lowerer.rs    (method calls in conditions)

Characteristics:
- If-expression inside loop
- Conditional carrier updates
- Complex condition support (Phase 242-EX-A) ✅
- AST-driven lowering

Innovation:
- Phase 213: AST-based if-sum pattern
- Phase 220-D: Variable resolution via ConditionEnv
- Phase 240-EX: ExprLowerer integration
- Phase 242-EX-A: Complex LHS support (BinaryOp in condition)
```

---

### 3.4 Pattern 4: With Continue

```
loop_with_continue_minimal.rs (551 lines)
         │
         ├─▶ condition_lowerer.rs
         ├─▶ carrier_info.rs
         ├─▶ inline_boundary.rs
         ├─▶ join_value_space.rs
         └─▶ scope_manager.rs

Characteristics:
- Continue statement support
- Jump to loop_step on continue
- Multiple carrier updates
- Exit PHI generation

Status: Phase 195 implementation complete
```

---

## 4. Shared Infrastructure Dependencies

### 4.1 JoinValueSpace (ValueId Allocation)

```
join_value_space.rs (431 lines)
         │
         │ Used by ALL pattern implementations
         │
         ├─▶ Pattern 1 (simple_while_minimal.rs)
         ├─▶ Pattern 2 (loop_with_break_minimal.rs)
         ├─▶ Pattern 3 (loop_with_if_phi_if_sum.rs)
         ├─▶ Pattern 4 (loop_with_continue_minimal.rs)
         ├─▶ condition_lowerer.rs
         ├─▶ expr_lowerer.rs
         └─▶ carrier_update_emitter

API:
- alloc_local() -> ValueId
- alloc_param() -> ValueId
- local_allocator() -> impl FnMut() -> ValueId
- param_allocator() -> impl FnMut() -> ValueId

Design: Phase 201 unified allocation system
```

---

### 4.2 InlineBoundary (Exit Reconnection)

```
inline_boundary.rs (576 lines)
         │
         ├─▶ carrier_info.rs (CarrierRole, CarrierInit)
         ├─▶ LoopExitBinding struct
         └─▶ JoinInlineBoundary struct

Used by:
- inline_boundary_builder.rs (Phase 200-2 builder pattern)
- All pattern implementations (exit PHI generation)

Phase 33-10 Infrastructure:
- ExitLineReconnector ✅
- ExitMetaCollector ✅
- ExitLineOrchestrator ✅

Status: Already boxified!
```

---

## 5. Cross-Cutting Concerns

### 5.1 Pattern Detection & Routing

```
┌──────────────────────────────────────────────────────────────┐
│              Pattern Detection & Routing                     │
└──────────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
┌───────────────┐  ┌──────────────┐  ┌──────────────────┐
│ Loop Pattern  │  │ If Pattern   │  │ Update Pattern   │
│ Router        │  │ Router       │  │ Analyzer         │
│ (193 lines)   │  │ (207 lines)  │  │ (561 lines)      │
└───────────────┘  └──────────────┘  └──────────────────┘
loop_pattern_router.rs if_lowering_router.rs loop_update_summary.rs
        │                  │                  │
        └──────────────────┼──────────────────┘
                           ▼
                  ┌──────────────────┐
                  │ Pattern          │
                  │ Validator        │
                  │ (212 lines)      │
                  └──────────────────┘
                  loop_pattern_validator.rs

Routing Logic:
- Loop patterns: Pattern 1-4 dispatcher
- If patterns: Select vs IfMerge dispatcher
- Update patterns: Carrier update detection

Used by:
- control_flow/joinir/patterns/ (Pattern 1-4 dispatch)
- control_flow/if_form.rs (If-expression dispatch)
```

**Boxification Opportunity**: Create `PatternDetectorBox` to unify detection logic.

---

### 5.2 Method Call Lowering (Phase 224-B)

```
method_call_lowerer.rs (639 lines)
         │
         │ Whitelist-based metadata-driven lowering
         │
         ├─▶ allowed_in_condition() → Vec<&str>
         ├─▶ allowed_in_init() → Vec<&str>
         ├─▶ lower_for_condition()
         └─▶ lower_for_init()

Whitelisted Methods (Condition):
- substring(offset, end)
- indexOf(search)
- length()

Whitelisted Methods (Init):
- substring(offset, end)
- indexOf(search)
- length()
- + more permissive

Used by:
- condition_lowerer.rs (method calls in conditions)
- loop_body_local_init.rs (method calls in init expressions)
- expr_lowerer.rs (Phase 231 integration)

Future Work:
- Expand whitelist (more StringBox/ArrayBox methods)
- Generic method lowering (eliminate whitelist)
```

---

## 6. Dependency Metrics

### 6.1 High-Dependency Modules (Used by 10+ files)

| Module | Dependents | Category |
|--------|-----------|----------|
| `condition_lowerer.rs` | 10 | Critical (condition lowering) |
| `carrier_info.rs` | 7 | Critical (carrier management) |
| `join_value_space.rs` | 20+ | Infrastructure (all patterns) |
| `inline_boundary.rs` | 5 | Infrastructure (exit reconnection) |

---

### 6.2 Dependency Depth (Longest Chains)

```
Pattern 3 Implementation
         │
         ▼
ExprLowerer (Phase 231)
         │
         ▼
ScopeManager (Phase 231)
         │
         ▼
ConditionEnv (Phase 171)
         │
         ▼
CarrierInfo (Phase 196)
         │
         ▼
InlineBoundary (Phase 188)

Depth: 6 levels
```

**Analysis**: Clean dependency flow from high-level (Pattern 3) to low-level (InlineBoundary).

---

## 7. Circular Dependencies (None Detected!)

**Good News**: No circular dependencies found in the module graph.

All dependencies flow in one direction:
```
Pattern Implementations
         ↓
Condition/Carrier/Expression Lowering
         ↓
Infrastructure (ValueSpace, Boundary, Env)
         ↓
Core Structures (ValueId, JoinInst)
```

---

## 8. Proposed Boxification Dependencies

### 8.1 ConditionLoweringBox (Phase 244)

```
ConditionLoweringBox (NEW)
         │
         ├─▶ SimpleConditionLowerer (NEW)
         ├─▶ ComplexConditionLowerer (NEW)
         ├─▶ LegacyConditionLowerer (NEW)
         └─▶ ConditionLoweringDispatcher (NEW)
                  │
                  ├─▶ condition_pattern.rs (existing)
                  ├─▶ condition_env.rs (existing)
                  └─▶ condition_var_extractor.rs (existing)

Replaces 5 modules with unified API:
- condition_lowerer.rs
- condition_to_joinir.rs
- condition_env.rs
- condition_pattern.rs
- condition_var_extractor.rs
```

---

### 8.2 CarrierManagerBox (Phase 245)

```
CarrierManagerBox (NEW)
         │
         ├─▶ carrier_info.rs (existing)
         ├─▶ carrier_update_emitter (existing)
         ├─▶ inline_boundary.rs (existing)
         └─▶ scope_manager.rs (existing, Phase 231)

API:
- init_carriers() -> Vec<JoinInst>
- update_carrier(name, value) -> Result<...>
- generate_exit_bindings() -> Vec<LoopExitBinding>
- carriers_for_phi(role) -> Vec<&CarrierVar>

Consolidates 3 modules with unified lifecycle API
```

---

### 8.3 PatternDetectorBox (Phase 247)

```
PatternDetectorBox (NEW)
         │
         ├─▶ LoopPatternDetector (NEW)
         ├─▶ IfPatternDetector (NEW)
         ├─▶ UpdatePatternDetector (NEW)
         └─▶ UnifiedPatternDetector (NEW)
                  │
                  ├─▶ condition_pattern.rs (existing)
                  ├─▶ loop_pattern_validator.rs (existing)
                  └─▶ loop_update_summary.rs (existing)

API:
- detect(ast) -> PatternKind
- validate(ast) -> Result<(), String>

Consolidates 3 modules with unified detection API
```

---

## 9. Impact Analysis by Phase

### Phase 244: ConditionLoweringBox

**Files Modified**: 19
**Files Created**: 5
**Tests Affected**: 50+
**Risk**: Medium
**Benefit**: High (single API for all condition lowering)

**Dependency Changes**:
- 10 files currently depend on `condition_lowerer.rs`
- After Phase 244: 10 files depend on `ConditionLoweringBox`
- Backward compatibility shim maintains existing API during migration

---

### Phase 245: CarrierManagerBox

**Files Modified**: 7
**Files Created**: 1
**Tests Affected**: 20+
**Risk**: Low
**Benefit**: Medium (consolidates lifecycle management)

**Dependency Changes**:
- 7 files currently depend on `carrier_info.rs`
- After Phase 245: 7 files depend on `CarrierManagerBox`
- No breaking changes (extends existing API)

---

### Phase 246: Module Reorganization

**Files Modified**: 74 (all files)
**Directories Created**: 7
**Tests Affected**: 0 (pure reorganization)
**Risk**: Low
**Benefit**: Medium (navigation + clarity)

**Dependency Changes**:
- Import paths updated (e.g., `use lowering::condition_lowerer` → `use lowering::core::condition_lowering`)
- Re-exports in `mod.rs` maintain backward compatibility

---

## 10. Conclusion

**Dependency Health**: ✅ Good
- No circular dependencies
- Clean dependency flow (top → bottom)
- Well-defined module boundaries

**Refactoring Readiness**: ✅ Ready
- Clear boxification targets (Condition, Carrier, Pattern)
- Existing infrastructure (Phase 227-231) provides solid foundation
- Low risk of breaking changes (backward compatibility shims)

**Recommended Action**: Proceed with Phase 244 (ConditionLoweringBox) implementation.

---

**End of Dependency Graph Analysis**
