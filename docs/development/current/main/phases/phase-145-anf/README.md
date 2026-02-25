# Phase 145 P0: ANF (A-Normal Form) Skeleton Implementation

**Status**: Complete
**Date**: 2025-12-19
**Purpose**: Establish 3-layer ANF architecture (contract/plan/execute) without changing existing behavior

---

## Executive Summary

Phase 145 P0 implements the skeleton for ANF (A-Normal Form) transformation in Normalized JoinIR, following the Phase 143 pattern of 3-layer separation (contract/plan/execute). **Existing behavior is unchanged** (P0 is non-invasive).

**Key Constraint**: execute_box always returns `Ok(None)` (stub), ensuring 0 regression.

**Next Steps**: P1 (String.length() hoist), P2 (compound expression ANF).

---

## Implementation Summary

### Files Created (5 + 1 doc)

**New Module** (`src/mir/control_tree/normalized_shadow/anf/`):
1. `mod.rs` (~30 lines) - Module entry point + re-exports
2. `contract.rs` (~200 lines) - 3 enums + 2 tests
   - `AnfDiagnosticTag` (OrderViolation, PureRequired, HoistFailed)
   - `AnfOutOfScopeReason` (ContainsCall, ContainsMethodCall, ...)
   - `AnfPlan` (requires_anf, impure_count)
3. `plan_box.rs` (~200 lines) - AST walk + 4 tests
   - `plan_expr()`: Detect impure subexpressions (Call/MethodCall)
   - `is_pure()`: Helper for quick pure/impure discrimination
4. `execute_box.rs` (~80 lines) - Stub + 1 test
   - `try_execute()`: Always returns `Ok(None)` (P0 stub)
5. `README.md` (~100 lines) - Module architecture documentation

**Documentation**:
6. `docs/development/current/main/phases/phase-145-anf/README.md` (this file)

### Files Modified (3)

1. `src/mir/control_tree/normalized_shadow/mod.rs` (+1 line)
   - Added `pub mod anf;`

2. `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs` (+23 lines)
   - Added ANF routing at Line 54-76 (before out_of_scope_reason check)
   - Dev-only (`HAKO_ANF_DEV=1`)
   - Fallback to legacy when execute_box returns None

3. `src/config/env/joinir_dev.rs` (+26 lines)
   - Added `anf_dev_enabled()` function
   - Environment variable: `HAKO_ANF_DEV=1`

---

## Architecture (Box-First, 3-layer separation)

### Layer 1: contract.rs - Diagnostic tags & plan structure (SSOT)

**Responsibility**:
- Define `AnfDiagnosticTag` enum (future error categorization)
- Define `AnfOutOfScopeReason` enum (graceful Ok(None) fallback)
- Define `AnfPlan` struct (requires_anf, impure_count)

**Design Pattern**: Enum discrimination (prevents if-branch explosion)

### Layer 2: plan_box.rs - AST pattern detection

**Responsibility**:
- Walk AST to detect impure subexpressions (Call/MethodCall)
- Build `AnfPlan` indicating what transformation is needed
- Does NOT perform transformation (separation of concerns)

**API**:
```rust
pub fn plan_expr(
    ast: &ASTNode,
    env: &BTreeMap<String, ValueId>,
) -> Result<Option<AnfPlan>, AnfOutOfScopeReason>
```

**Returns**:
- `Ok(Some(plan))`: Expression in scope (plan.requires_anf indicates if ANF needed)
- `Ok(None)`: Expression out-of-scope (unknown AST node type)
- `Err(reason)`: Expression explicitly out-of-scope (ContainsCall/ContainsMethodCall)

### Layer 3: execute_box.rs - ANF transformation execution (P0: stub)

**Responsibility**:
- Execute ANF transformation for expressions that require it (per AnfPlan)
- P0: Always returns `Ok(None)` (existing behavior unchanged)
- P1+: Implement hoist + rebuild AST + lower

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
- `Ok(Some(vid))`: ANF transformation succeeded (P1+)
- `Ok(None)`: Transformation not attempted (P0 stub)
- `Err(msg)`: Internal error (strict mode only, P1+)

---

## Integration with expr_lowerer_box.rs

**Location**: `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`

**Routing (Line 54-76)**:
```rust
// Phase 145 P0: ANF routing (dev-only)
if crate::config::env::anf_dev_enabled() {
    use super::super::anf::{AnfPlanBox, AnfExecuteBox};
    match AnfPlanBox::plan_expr(ast, env) {
        Ok(Some(plan)) => {
            match AnfExecuteBox::try_execute(&plan, ast, &mut env.clone(), body, next_value_id)? {
                Some(vid) => return Ok(Some(vid)),  // P1+: ANF succeeded
                None => {
                    // P0: stub returns None, fallback to legacy
                    if crate::config::env::joinir_dev_enabled() {
                        eprintln!("[phase145/debug] ANF plan found but execute returned None (P0 stub)");
                    }
                }
            }
        }
        Ok(None) => { /* out-of-scope, continue */ }
        Err(_reason) => { /* out-of-scope, continue */ }
    }
}
```

**Environment Variable**:
- `HAKO_ANF_DEV=1`: Enable ANF routing
- Default: ANF routing disabled (0 impact)

**Debug Logging**:
- `[phase145/debug] ANF plan found but execute returned None (P0 stub)`
- `[phase145/debug] ANF execute called (P0 stub, returning Ok(None))`

---

## Testing

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

**Regression Tests**:
- All existing tests pass (0 regression)
- Phase 97/131/143 smoke tests unchanged

---

## Acceptance Criteria

- [x] 5 new files created (anf/ module)
- [x] 3 existing files modified (mod.rs, expr_lowerer_box.rs, joinir_dev.rs)
- [x] 7 unit tests pass
- [x] cargo build --release passes
- [x] 0 regression (existing tests unchanged)
- [x] Debug log with `HAKO_ANF_DEV=1`

---

## Next Steps

### Phase 145 P1: String.length() hoist (最小成功例)

**Goal**: Implement ANF transformation for 1 known intrinsic (String.length()).

**Pattern**:
```hako
x + s.length()
  ↓ ANF
t = s.length()
result = x + t
```

**Implementation**:
- contract.rs: Add hoist_targets to AnfPlan (~50 lines)
- plan_box.rs: Whitelist check + BinaryOp pattern detection (~100 lines)
- execute_box.rs: Stub → implementation (~150 lines)

**Fixtures**:
- `apps/tests/phase145_p1_anf_length_min.hako` (exit code 12)
- `tools/smokes/.../phase145_p1_anf_length_vm.sh`
- `tools/smokes/.../phase145_p1_anf_length_llvm_exe.sh`

**Acceptance Criteria**:
- Exit code 12 (VM + LLVM EXE parity)
- String.length() hoisted (JoinInst::MethodCall emitted first)
- BinaryOp uses temp variable (not direct MethodCall)
- Whitelist enforcement (other methods → Ok(None))

### Phase 145 P2: Compound expression ANF (再帰的線形化)

**Goal**: Implement recursive ANF for compound expressions (multiple MethodCalls).

**Patterns**:
```hako
// Pattern 1: x + s.length() + z
//   → t1 = s.length(); t2 = x + t1; result = t2 + z

// Pattern 2: s1.length() + s2.length()
//   → t1 = s1.length(); t2 = s2.length(); result = t1 + t2
```

**Implementation**:
- execute_box.rs: Recursive processing (left-to-right, depth-first) (~80 lines)
- Diagnostic tags: error_tags.rs integration (~30 lines)

**Acceptance Criteria**:
- 2 fixtures pass (exit codes 18, 5)
- Left-to-right order preserved
- Recursive ANF documented

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
- `src/mir/control_tree/normalized_shadow/anf/README.md` - Module architecture
- `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs` - Integration point
- `src/config/env/joinir_dev.rs` - Environment variable helpers

---

**Revision History**:
- 2025-12-19: Phase 145 P0 skeleton implemented (contract/plan/execute separation)
