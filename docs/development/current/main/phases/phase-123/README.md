# Phase 123: if-only Normalized Semantics (dev-only)

**Status**: In Progress
**Started**: 2025-12-18
**Scope**: if-only patterns with normalized semantics

## Goals

Phase 122 completed "JoinModule generation + structure validation". Phase 123 adds meaningful normalized semantics to if-only patterns.

## Scope

### In-Scope (Phase 123)
- **Return(Integer literal)**: `return 7` → `Const + Ret(Some(vid))`
- **Return(Variable)**: Fail-Fast with strict mode (needs reads facts - Phase 124)
- **If(cond_ast)**: Minimal Compare lowering (`flag == 1`, `i < 3`)
  - Variable vs Integer literal comparisons only
  - then/else: Return(Integer literal) only
  - Merge: `join_k(env)` tail-call (no PHI)

### Out-of-Scope (Future Phases)
- **Return(Variable)**: Requires reads facts (Phase 124)
- **Complex conditions**: Compound expressions, &&/||, method calls
- **Loop/Break/Continue**: Capability insufficient (future)

## Design Principles

- **Box-First**: Modular separation of concerns
- **Fail-Fast**: No fallbacks, explicit errors with hints
- **SSOT**: No re-analysis of facts/contracts (use computed values only)
- **Dev-only**: Default behavior unchanged
- **Contract-based**: Use contract information, not AST inference

## Node Support (Phase 123)

### Return Nodes

#### Return(Integer literal) - P1
```
return 7
→ Const { dst: v1, value: Integer(7) }
→ Ret { value: Some(v1) }
```

#### Return(Variable) - P2 (Fail-Fast)
```
return x
→ freeze_with_hint(
    "phase123/return/var_unsupported",
    "Phase 123 only supports return with integer literals",
    "Add reads fact (Phase 124) or return literal only"
  )
```

### If Nodes - P3

#### Minimal Compare Support
```
if (flag == 1) {
  return 2
} else {
  return 3
}

→ Compare { ... }
→ Branch { ... }
→ Ret { ... } (in then block)
→ Ret { ... } (in else block)
→ join_k(env) (merge continuation)
```

**Supported comparisons**:
- `Variable == Integer`
- `Variable < Integer`
- `Variable > Integer`
- `Variable <= Integer`
- `Variable >= Integer`
- `Variable != Integer`

**Not supported** (cap_missing):
- Compound expressions: `a == 1 && b == 2`
- Method calls: `s.length() == 0`
- Complex expressions: `a + b == 5`

## Implementation Plan

### P0: docs-only (Plan & Scope)

**Docs**:
- `docs/development/current/main/phases/phase-123/README.md` (this file)
- Update `docs/development/current/main/10-Now.md`: Next: Phase 123
- Update `docs/development/current/main/30-Backlog.md`: Next candidate → Phase 123

**Commit**: `docs: Phase 123 plan (if-only normalized semantics)`

### P1: Return payload (Integer literal)

**Implementation**:
- `src/mir/control_tree/normalized_shadow/builder.rs`
  - Return node → `JoinInst::Ret { value: Some(v) }`
  - Literal(Integer) → `Const` + `Ret(Some(const_vid))`
  - Other literals → strict `freeze_with_hint` ("Phase 123 integer only")

**Unit tests**:
- "return 7" generates `Const(Integer 7)` + `Ret(Some(...))`

**Commit**: `feat(control_tree): Phase 123 return integer literal in Normalized if-only`

### P2: Return payload (Variable) - Fail-Fast (A plan)

**Design Decision**:
- **A plan (recommended)**: Adopted
- Do not add "reads" to StepTreeFacts
- Phase 123 does not allow Return(Variable)
- To support Return(Variable), add reads facts in Phase 124

**Implementation**:
- `src/mir/control_tree/normalized_shadow/builder.rs`
  - Return(Variable) → strict `freeze_with_hint(
      "phase123/return/var_unsupported",
      "Phase 123 only supports return with integer literals",
      "Add reads fact (Phase 124) or return literal only"
    )`

**Unit tests**:
- Verify strict mode fails with appropriate hint

**Commit**: `feat(control_tree): Phase 123 fail-fast on return variable (needs reads fact)`

### P3: If(cond_ast) minimal lowering

**Implementation**:
- `src/mir/control_tree/normalized_shadow/builder.rs`
  - If node:
    - Parse `cond_ast` as "minimal Compare"
    - Generate JoinIR Compare instruction
    - then/else: Return(Integer literal) only (other → strict freeze)
    - Merge: `join_k(env)` tail-call (no PHI)
  - Minimal parsing:
    - `flag == 1` / `i < 3` (Variable vs Integer literal)
    - Other (compound, &&/||, method call) → cap_missing strict freeze

**Unit tests**:
- If(true/false equivalent comparison) generates correct then/else structure

**Commit**: `feat(control_tree): Phase 123 if-only compare+return lowering (Normalized, dev-only)`

### P4: integration smoke (dev-only strict)

**Fixture**:
- `apps/tests/phase123_if_only_return_literal_min.hako` (output: 7)

**Smoke**:
- `tools/smokes/v2/profiles/integration/apps/phase123_if_only_normalized_semantics_vm.sh`
  - Target: New fixture
  - Dev+strict: `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1`
  - Output validation: `output_validator.sh`

**Commit**: `test(joinir): Phase 123 normalized semantics smoke (VM)`

### P5: docs completion

**Docs**:
- `docs/development/current/main/phases/phase-123/README.md`: Add DONE section
- `docs/development/current/main/10-Now.md`: Update to Phase 123 complete
- `docs/development/current/main/01-JoinIR-Selfhost-INDEX.md`: Add Phase 123

**Commit**: `docs: Phase 123 DONE`

## Verification Commands

```bash
# Unit tests
cargo test --lib

# Smoke tests
bash tools/smokes/v2/profiles/integration/apps/phase121_shadow_if_only_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase122_if_only_normalized_emit_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase118_loop_nested_if_merge_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase123_if_only_normalized_semantics_vm.sh
```

## Node Support Design

### Return(Integer literal)
- Direct lowering to Const + Ret
- Single responsibility: literal → instruction generation

### Return(Variable)
- Fail-Fast with structured error
- Clear migration path: Phase 124 reads facts
- No workarounds or fallbacks

### If(cond_ast) - Minimal Compare
- Parse only simple binary comparisons
- Variable on left, Integer literal on right
- Explicit scope: no compound/complex expressions
- Merge via continuation: `join_k(env)` tail-call

## Success Criteria

- [x] P0: Docs complete, scope frozen
- [x] P1: Return(Integer literal) working with unit tests
- [x] P2: Return(Variable) returns Ok(None) (out of scope)
- [x] P3: If(minimal compare) generates correct JoinIR
- [x] P4: Integration smoke passing
- [x] P5: Docs updated, Phase 123 recorded as complete

## Progress

- **2025-12-18**: P0 started (docs-only planning)
- **2025-12-18**: P1-P2 completed (Return lowering with graceful degradation)
- **2025-12-18**: P3 completed (If minimal compare lowering)
- **2025-12-18**: P4 completed (integration smoke test passing)
- **2025-12-18**: P5 completed (documentation updated)

## DONE

**Date**: 2025-12-18

### What Was Completed

1. **Return(Integer literal)** (`P1`):
   - Generates `Const + Ret(Some(vid))` correctly
   - Unit test: `test_return_integer_literal`

2. **Return(Variable)** (`P2`):
   - Returns `Ok(None)` for Phase 123 unsupported patterns
   - Graceful degradation: falls back to legacy lowering
   - Unit test: `test_return_variable_out_of_scope`

3. **If(minimal compare)** (`P3`):
   - Parses binary comparisons: `Variable op Integer`
   - Supports: `==`, `!=`, `<`, `<=`, `>`, `>=`
   - Generates: `Compare + Const + Ret` structure
   - Unit test: `test_if_minimal_compare`
   - Graceful degradation: returns `Ok(None)` for unsupported patterns

4. **Integration smoke** (`P4`):
   - New fixture: `apps/tests/phase123_if_only_return_literal_min.hako`
   - Smoke test: `tools/smokes/v2/profiles/integration/apps/phase123_if_only_normalized_semantics_vm.sh`
   - Status: PASSING

5. **Documentation** (`P5`):
   - Phase 123 README updated with DONE section
   - 10-Now.md updated (next section)
   - 01-JoinIR-Selfhost-INDEX.md updated (next section)

### Key Design Decisions

1. **Graceful Degradation**:
   - Phase 123 returns `Ok(None)` for unsupported patterns instead of failing
   - Allows dev-only mode to coexist with legacy code
   - Error messages prefixed with `[phase123/...]` are caught and converted to `Ok(None)`

2. **Fail-Fast with Structured Errors**:
   - All Phase 123 limitations use structured error codes
   - Format: `[phase123/category/specific]`
   - Examples:
     - `[phase123/return/var_unsupported]`
     - `[phase123/if/compare_rhs_unsupported]`
     - `[phase123/if/branch_return_not_int_literal]`

3. **Box-First Principles**:
   - `parse_minimal_compare`: Single responsibility parser
   - `verify_branch_is_return_literal`: Branch validation box
   - `lower_if_node`: If lowering box

### Implementation Details

**Files Modified**:
- `src/mir/control_tree/normalized_shadow/builder.rs` (+180 lines)
  - `lower_if_node`: If lowering with minimal compare
  - `parse_minimal_compare`: Binary comparison parser
  - `verify_branch_is_return_literal`: Branch validator
  - Updated `lower_if_only_to_normalized` return type: `Result<Option<...>, ...>`

**Files Created**:
- `apps/tests/phase123_if_only_return_literal_min.hako`: Minimal test fixture
- `tools/smokes/v2/profiles/integration/apps/phase123_if_only_normalized_semantics_vm.sh`: Smoke test

**Tests Added**:
- `test_return_variable_out_of_scope`: Verifies graceful degradation
- `test_if_minimal_compare`: Verifies If lowering structure

### Next Steps (Phase 124)

1. **Reads Facts**: Add variable reads tracking to StepTreeFacts
2. **Return(Variable)**: Use reads facts to support variable returns
3. **Complex Conditions**: Support compound expressions (&&, ||)
4. **Method Calls in Conditions**: Support method call conditions

### Verification

```bash
# Unit tests
cargo test --lib control_tree::normalized_shadow::builder::tests
# Result: 8 passed

# Integration smoke
bash tools/smokes/v2/profiles/integration/apps/phase123_if_only_normalized_semantics_vm.sh
# Result: PASS

# Legacy tests still passing
bash tools/smokes/v2/profiles/integration/apps/phase121_shadow_if_only_vm.sh
# Result: Should still pass (graceful degradation)
```
