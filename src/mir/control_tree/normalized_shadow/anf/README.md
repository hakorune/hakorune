# Phase 145: ANF (A-Normal Form) Module

**Status**: P1/P2 ANF paths active behind ANF gates
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
- **P0**: Diagnostic/plan contract baseline
- **P1+**: `hoist_targets` and `parent_kind` are active in planner/executor routing

**Design Pattern**: Enum discrimination (prevents if-branch explosion)

### plan_box.rs - AST pattern detection

**Responsibility**:
- Walk AST expression to detect impure subexpressions (Call/MethodCall)
- Build AnfPlan indicating what transformation is needed
- Does NOT perform transformation (separation of concerns)

**Phase Scope**:
- **P0**: Basic impure detection (Call/MethodCall presence)
- **P1+**: Whitelist check (e.g., String.length()), parent_kind detection

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
- **P1**: String.length() hoist (whitelist 1 intrinsic)
- **P2**: Recursive compound expression ANF
- **Out-of-scope**: returns `Ok(None)` as a route decline

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
- `Ok(None)`: Transformation not attempted or out-of-scope route decline
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
            Ok(None) => { /* route decline, continue */ }
        },
        Ok(None) => { /* out-of-scope, continue */ }
        Err(reason) => { /* out-of-scope, continue */ }
    }
}
```

**Environment Variable**:
- `HAKO_ANF_DEV=1`: Enable ANF routing
- `HAKO_ANF_ALLOW_PURE=1`: Allow ANF routing from PureOnly lowering scope
- Default: ANF routing disabled (0 impact on existing behavior)

---

## Phase Scope Summary

### Current Implemented Surface

**Goal**: Keep ANF routing explicit and gated while preserving default behavior.

**Implemented**:
- ✅ contract.rs: 3 enums (AnfDiagnosticTag, AnfOutOfScopeReason, AnfPlan)
- ✅ plan_box.rs: AST walk with basic impure detection
- ✅ execute_box.rs: String.length/compare hoist and recursive compound expression ANF
- ✅ Integration: expr_lowerer_box.rs routing (dev-only, no impact)
- ✅ Tests: representative unit tests for contract, plan, and execute paths

**Acceptance Criteria**:
- ✅ cargo check/build passes
- ✅ representative unit tests pass
- ✅ 0 regression (existing tests unchanged)
- ✅ HAKO_ANF_DEV=1 routing remains gated

### P1 (String.length() hoist)

**Goal**: ANF transformation for 1 known intrinsic (String.length()).

**Pattern**:
```hako
x + s.length()
  ↓ ANF
t = s.length()
result = x + t
```

**Implementation**:
- contract.rs: `hoist_targets` in AnfPlan
- plan_box.rs: Whitelist check + BinaryOp pattern detection
- execute_box.rs: hoist + rebuild AST + lower

**Acceptance Criteria**:
- Fixture: `phase145_p1_anf_length_min.hako` (exit code 12)
- VM + LLVM EXE parity
- Whitelist enforcement (other methods → Ok(None))

### P2 (Compound expression ANF)

**Goal**: Recursive ANF for compound expressions (multiple MethodCalls).

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

### Unit Tests

**contract.rs (2 tests)**:
- `test_anf_plan_pure`: AnfPlan::pure() construction
- `test_anf_plan_impure`: AnfPlan::impure(n) construction

**plan_box.rs (4 tests)**:
- `test_plan_pure_variable`: Variable → pure plan
- `test_plan_pure_literal`: Literal → pure plan
- `test_plan_pure_binop`: BinaryOp (pure operands) → pure plan
- `test_plan_call_out_of_scope`: Call → Err(ContainsCall)

**execute_box.rs (1 test)**:
- `test_execute_route_declines_without_targets`: pure/no-target plan returns `Ok(None)`

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
- 2026-04-27: Status wording synced to active P1/P2 executor paths
