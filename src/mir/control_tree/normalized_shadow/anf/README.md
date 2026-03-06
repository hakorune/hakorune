# Phase 145 P0: ANF (A-Normal Form) Module

**Status**: Skeleton implemented (P0 complete)
**Date**: 2025-12-19
**Purpose**: Deterministic evaluation order for impure expressions (Call/MethodCall)

---

## Module Architecture (Box-First, 3-layer separation)

### contract.rs - Diagnostic tags, out-of-scope reasons, plan structure (SSOT)

**Responsibility**:
- Defines `AnfDiagnosticTag` enum (OrderViolation, PureRequired, HoistFailed)
- Defines `AnfOutOfScopeReason` enum (ContainsCall, ContainsMethodCall, ...)
- Defines `AnfPlan` struct (requires_anf, impure_count)

**Phase Scope**:
- **P0**: Enum definitions only (not yet used in execute_box)
- **P1+**: Add hoist_targets, parent_kind to AnfPlan

**Design Pattern**: Enum discrimination (prevents if-branch explosion)

### plan_box.rs - AST pattern detection

**Responsibility**:
- Walk AST expression to detect impure subexpressions (Call/MethodCall)
- Build AnfPlan indicating what transformation is needed
- Does NOT perform transformation (separation of concerns)

**Phase Scope**:
- **P0**: Basic impure detection (Call/MethodCall presence)
- **P1+**: Add whitelist check (e.g., String.length()), parent_kind detection

**API**:
```rust
pub fn plan_expr(
    ast: &ASTNode,
    env: &BTreeMap<String, ValueId>,
) -> Result<Option<AnfPlan>, AnfOutOfScopeReason>
```

**Returns**:
- `Ok(Some(plan))`: Expression is in scope (plan.requires_anf indicates if ANF needed)
- `Ok(None)`: Expression is out-of-scope (unknown AST node type)
- `Err(reason)`: Expression is explicitly out-of-scope (e.g., nested impure)

### execute_box.rs - ANF transformation execution

**Responsibility**:
- Execute ANF transformation for expressions that require it (per AnfPlan)
- Hoist impure subexpressions to temporary variables
- Emit transformed JoinInsts

**Phase Scope**:
- **P0**: Stub only (always returns Ok(None), existing behavior unchanged)
- **P1**: Implement String.length() hoist (whitelist 1 intrinsic)
- **P2**: Implement recursive compound expression ANF

**API**:
```rust
pub fn try_execute(
    plan: &AnfPlan,
    ast: &ASTNode,
    env: &mut BTreeMap<String, ValueId>,
    body: &mut Vec<JoinInst>,
    next_value_id: &mut u32,
) -> Result<Option<ValueId>, String>
```

**Returns**:
- `Ok(Some(vid))`: ANF transformation succeeded, result is ValueId (P1+)
- `Ok(None)`: Transformation not attempted (P0 stub) or out-of-scope
- `Err(msg)`: Internal error (strict mode only, P1+)

---

## Integration with expr_lowerer_box.rs

**Location**: `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`

**Routing (Line 54-57)**:
```rust
if crate::config::env::anf_dev_enabled() {
    match AnfPlanBox::plan_expr(ast, env)? {
        Ok(Some(plan)) => match AnfExecuteBox::try_execute(plan, ast, env, body, next_value_id)? {
            Ok(Some(vid)) => return Ok(Some(vid)),  // P1+: ANF succeeded
            Ok(None) => { /* fallback to legacy */ }  // P0: stub returns None
        },
        Ok(None) => { /* out-of-scope, continue */ }
        Err(reason) => { /* out-of-scope, continue */ }
    }
}
```

**Environment Variable**:
- `HAKO_ANF_DEV=1`: Enable ANF routing (P0: debug logging only)
- Default: ANF routing disabled (0 impact on existing behavior)

---

## Phase Scope Summary

### P0 (Skeleton) - Current Status

**Goal**: Establish 3-layer architecture without changing existing behavior.

**Implemented**:
- ✅ contract.rs: 3 enums (AnfDiagnosticTag, AnfOutOfScopeReason, AnfPlan)
- ✅ plan_box.rs: AST walk with basic impure detection
- ✅ execute_box.rs: Stub (always returns Ok(None))
- ✅ Integration: expr_lowerer_box.rs routing (dev-only, no impact)
- ✅ Tests: 7 unit tests (contract: 2, plan: 4, execute: 1)

**Acceptance Criteria**:
- ✅ cargo build --release passes
- ✅ 7 unit tests pass
- ✅ 0 regression (existing tests unchanged)
- ✅ HAKO_ANF_DEV=1 debug logging works

### P1 (String.length() hoist) - Next Phase

**Goal**: Implement ANF transformation for 1 known intrinsic (String.length()).

**Pattern**:
```hako
x + s.length()
  ↓ ANF
t = s.length()
result = x + t
```

**Implementation**:
- contract.rs: Add hoist_targets to AnfPlan
- plan_box.rs: Whitelist check + BinaryOp pattern detection
- execute_box.rs: Stub → implementation (hoist + rebuild AST + lower)

**Acceptance Criteria**:
- Fixture: `phase145_p1_anf_length_min.hako` (exit code 12)
- VM + LLVM EXE parity
- Whitelist enforcement (other methods → Ok(None))

### P2 (Compound expression ANF) - Future Phase

**Goal**: Implement recursive ANF for compound expressions (multiple MethodCalls).

**Examples**:
```hako
// Example A: x + s.length() + z
//   → t1 = s.length(); t2 = x + t1; result = t2 + z

// Example B: s1.length() + s2.length()
//   → t1 = s1.length(); t2 = s2.length(); result = t1 + t2
```

**Implementation**:
- execute_box.rs: Recursive processing (left-to-right, depth-first)
- Diagnostic tags: error_tags.rs integration

---

## Testing Strategy

### Unit Tests (7 total)

**contract.rs (2 tests)**:
- `test_anf_plan_pure`: AnfPlan::pure() construction
- `test_anf_plan_impure`: AnfPlan::impure(n) construction

**plan_box.rs (4 tests)**:
- `test_plan_pure_variable`: Variable → pure plan
- `test_plan_pure_literal`: Literal → pure plan
- `test_plan_pure_binop`: BinaryOp (pure operands) → pure plan
- `test_plan_call_out_of_scope`: Call → Err(ContainsCall)

**execute_box.rs (1 test)**:
- `test_execute_stub_returns_none`: P0 stub always returns Ok(None)

### Integration Tests (P1+)

**Fixtures**:
- `apps/tests/phase145_p1_anf_length_min.hako` (P1)
- `apps/tests/phase145_p2_compound_expr_binop_min.hako` (P2)
- `apps/tests/phase145_p2_compound_expr_double_intrinsic_min.hako` (P2)

**Smoke Tests**:
- VM: `tools/smokes/.../phase145_p*_vm.sh`
- LLVM EXE: `tools/smokes/.../phase145_p*_llvm_exe.sh`

---

## References

**Design SSOT**:
- `docs/development/current/main/phases/phase-144-anf/INSTRUCTIONS.md` - ANF contract definition
- `docs/development/current/main/design/normalized-expr-lowering.md` - ExprLowererBox SSOT

**Related Phases**:
- Phase 140: NormalizedExprLowererBox (pure expression lowering)
- Phase 143: LoopIfExitContract pattern (3-layer separation inspiration)
- Phase 144: ANF docs-only specification

**Implementation SSOT**:
- `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs` - Integration point
- `src/config/env/joinir_dev.rs` - Environment variable helpers

---

**Revision History**:
- 2025-12-19: Phase 145 P0 skeleton implemented (contract/plan/execute separation)
