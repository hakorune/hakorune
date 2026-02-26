# Phase 143-loopvocab: Loop Vocabulary Extension

Status: ✅ **P0 Complete**
Date: 2025-12-19

---

## 目的（Why）

Phase 131で `loop(true) break` を実装したが、Phase 143では「条件付きbreak」パターンを吸収する。

**Phase 143-loopvocab** では、`loop(true) { if(cond_pure) break }` を **Normalized shadow lowering** で段階的に拡張するための語彙追加を行う。

### パターン進化

**Phase 131**:
```nyash
loop(true) { break }  // 無条件break
```

**Phase 143 P0**:
```nyash
loop(true) { if(cond_pure) break }  // 条件付きbreak
```

---

## P0: Minimal Vocabulary Extension (COMPLETE ✅)

### Summary

`loop(true) { if(cond_pure) break }` パターンを Normalized shadow で実装完了。

### Scope (P0 - Conservative)

**Supported**:
- Loop condition: `true` literal only
- Loop body: Single `if` statement only
- If then branch: `break` only (no continue, no nested if)
- Condition expression: Pure only (variables, literals, arith, compare)

**Out of Scope** (Ok(None) fallback):
- Loop condition not `true`: `loop(x > 0) { ... }`
- Multiple statements in loop body
- If/else statements (no else branch)
- Impure conditions: `loop(true) { if(s.length() > 0) break }`
- Nested structures within if body

### Design: 6-Function JoinModule

```
main(env) → loop_step(env) → loop_cond_check(env)
                              [condition lowering + Jump]
                              → k_exit(env) on break [Ret]
                              → loop_step(env) on continue [Call]

k_then(unused in P0)
k_else(unused in P0)
```

**Jump Semantics**:
- `Jump { cont: k_exit, args: [...], cond: Some(vid) }`
- If `vid` is true: jump to k_exit (break)
- If `vid` is false: fall through to Call(loop_step)

### Implementation Steps (Complete ✅)

#### Step 1-2: Pattern Detection + Routing
- Extract loop(true) + if + break pattern
- Validate loop condition = Bool(true) literal
- Validate if body = single Break statement
- Add module declaration and routing in builder.rs

#### Step 3: Condition Lowering Integration
- Validate condition with NormalizedExprLowererBox
- Use ExprLoweringScope::PureOnly
- Return Ok(None) for impure conditions (graceful fallback)

#### Step 4: Branch Instruction Emission
- Allocate 6 JoinFuncIds (stable IDs 0-5)
- Build env parameter passing (deterministic BTreeMap)
- Emit Jump instruction in loop_cond_check
- Create Call fallthrough for loop_step

#### Step 5: Exit Action Discrimination
- Complete k_exit() function
- Build exit values from env_layout.writes
- Extract final ValueIds from environment

#### Step 6: ExitMeta Construction
- Create ExitMeta::multiple() from exit values
- Use JoinFragmentMeta::carrier_only()
- Return Ok(Some((module, meta)))

#### Step 7: Routing Integration
- Already integrated in builder.rs (Step 1)
- Phase 131 (simpler) → Phase 143 (conditional) priority

#### Step 8: Fixtures + Smoke Tests
**Fixture**: `apps/tests/phase143_loop_true_if_break_min.hako`

**Tests**:
- `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_break_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_break_llvm_exe.sh`

Expected exit code: 7

#### Step 9: Documentation (THIS FILE)

### Acceptance Criteria (All Met ✅)

- [x] Pattern detection correctly identifies `loop(true) { if(cond) break }`
- [x] Condition lowering validates pure scope
- [x] Out-of-scope patterns always return `Ok(None)`
- [x] JoinModule built with 6 functions
- [x] Jump instruction with conditional emission
- [x] k_exit returns exit values
- [x] ExitMeta constructed correctly
- [x] Fixture executes correctly (VM)
- [x] LLVM EXE parity test ready
- [x] No regressions in Phase 131-142
- [x] cargo check passes (no errors)
- [x] All commits created

### Verification Results

**Compilation**: `cargo check -p nyash-rust --lib` ✅ (0 errors)

**Commits**:
1. `f55f6cc65` - Step 3: Condition lowering integration
2. `434c891a1` - Step 4: Branch instruction emission
3. `d1d59dc82` - Steps 5-6: Exit handling + ExitMeta
4. `e28d59101` - Step 8: Fixtures + Smoke tests

---

## P1: Continue Vocabulary (COMPLETE ✅)

### Summary

`loop(true) { if(cond_pure) continue }` を語彙として受理し、Normalized shadow 経路で「コンパイル/実行が落ちない」ことを固定する。

### Important note (P1 scope)

P1 の fixture は **意図的に non-terminating**（break/else/state update を含まない）である。

- 条件式は `ExprLoweringScope::PureOnly` で lowering できることを検証する（スコープ境界の固定）。
- ただし P1 では continue を「条件分岐で skip する」語彙までに留め、bridge 側の Jump(early-return) と混線させないため、
  実際の JoinIR emission は `loop_step` への tail call を基本にする。

### Fixture / smokes

- Fixture: `apps/tests/phase143_loop_true_if_continue_min.hako`
- VM smoke (timeout contract):
  - `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_continue_vm.sh`
  - Contract: `HAKO_VM_MAX_STEPS=0` + `timeout ${SMOKES_P143_CONTINUE_TIMEOUT_SECS:-1}` → exit code `124`
- LLVM EXE smoke (timeout contract):
  - `tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_continue_llvm_exe.sh`
  - Contract: `RUN_TIMEOUT_SECS=${SMOKES_P143_CONTINUE_TIMEOUT_SECS:-1}` → exit code `124`

---

## Design Principles

### Out-of-Scope Handling

All out-of-scope patterns return `Ok(None)` for graceful fallback to legacy routing.

### Fail-Fast Policy

- **Out-of-scope**: Always `Ok(None)` (no change to existing behavior)
- **In-scope errors**: Return `Err(msg)` (internal errors, strict mode freeze)

---

## Files

### Core Implementation

1. **src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs** (~400 lines)

### Fixtures + Tests

2. **apps/tests/phase143_loop_true_if_break_min.hako**
3. **tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_break_vm.sh**
4. **tools/smokes/v2/profiles/integration/apps/archive/phase143_loop_true_if_break_llvm_exe.sh**

---

## Related Documentation

- [Phase 131 (loop break-once)](../phase-131/README.md)
- [Phase 142 (statement-level normalization)](../phase-142-loopstmt/README.md)
- [JoinIR architecture](../../joinir-architecture-overview.md)

---

## Commits

1. `f55f6cc65` - feat(phase143): Step 3 - Condition lowering integration
2. `434c891a1` - feat(phase143): Step 4 - Branch instruction emission
3. `d1d59dc82` - feat(phase143): Steps 5-6 - Exit handling + ExitMeta
4. `e28d59101` - feat(phase143): Step 8 - Fixtures + Smoke tests
