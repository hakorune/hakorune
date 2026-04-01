# Phase 132: loop(true) break-once with Minimal Post Computation (dev-only)

**Date**: 2025-12-18
**Status**: DONE ✅
**Scope**: loop(true) break-once + post-loop minimal computation (Normalized shadow)

---

## Previous Phase 132 Work (2025-12-15) ✅ Done

### Phase 132-P1 to P3: Exit Values Parity and LLVM Python Fixes

- **P1**: FunctionLowerContext Box for state isolation
- **P2**: Exit PHI ValueId collision fix (Case C)
- **P3**: Exit PHI collision early detection (debug-only)

All previous Phase 132 work is complete and documented in commit history.

---

## Phase 132-P4: loop(true) break-once with Post Computation (NEW)

**Goal**: Extend Phase 131 `loop(true){...; break}` to support minimal post-loop computation.

Related:
- Entry: `docs/development/current/main/10-Now.md`
- Phase 131 (loop baseline): `docs/development/current/main/phases/phase-131/README.md`
- Phase 130 (post_k pattern): `docs/development/current/main/phases/archive/phase-130/README.md`
- ControlTree SSOT: `docs/development/current/main/design/control-tree.md`

### Goal

Extend Phase 131 `loop(true){...; break}` to support minimal post-loop computation:

- Enable `x = x + 2` after loop exit, then return
- Keep **PHI禁止**: merge via env + continuations only
- Keep **dev-only** and **既定挙動不変**: unmatched shapes fall back (strict can Fail-Fast)

### Non-Goals

- No general loop conditions (only `true` literal)
- No continue support (only break at end of body)
- No nested control flow inside loop body
- No multiple breaks or conditional breaks
- No complex post-loop computation (one assignment + return only)

### Accepted Forms

#### ✅ Supported

```nyash
// Form: loop(true) with break + minimal post computation
local x
x = 0
loop(true) {
    x = 1
    break
}
x = x + 2
return x  // Expected: 3 (1 + 2)
```

#### ❌ Not Supported

```nyash
// Multiple post-loop statements
loop(true) { x = 1; break }
x = x + 2
y = x + 3
return y

// Continue (not break)
loop(true) { x = 1; continue }

// Nested control flow
loop(true) { if (y == 1) { x = 2 }; break }

// General loop conditions
loop(x < 10) { x = x + 1; break }
```

### SSOT Contracts

#### EnvLayout (Phase 126)

Same as Phase 131:
- `writes`: Variables assigned in the fragment
- `inputs`: Variables read from outer scope
- `env_fields()`: `writes ++ inputs` (SSOT for parameter order)

#### Loop + Post Structure Contract

For `loop(true) { body ; break }; <post>; return`:

1. **Condition**: Must be Bool literal `true` (cond_ast check)
2. **Body**: Must be a Block ending with Break statement
3. **Post-loop**: One assignment statement (`x = x + <int literal>`) followed by one return statement
4. **No exits**: No return/continue in body (only break at end)
5. **Assignments**: Body contains only Assign(int literal/var/add) and LocalDecl

#### Generated JoinModule Structure

```
main(env) → TailCall(loop_step, env)

loop_step(env) →
    TailCall(loop_body, env)  // condition is always true, no branch

loop_body(env) →
    <assign statements update env>
    TailCall(k_exit, env)

k_exit(env) →
    TailCall(post_k, env)  // Phase 132-P4: NEW (was Ret in Phase 131)

post_k(env) →
    <post assignment: x = env[x] + 2>
    Ret(env[x])
```

**Key Differences from Phase 131**:
- Phase 131: `k_exit(env) → Ret(env[x])`
- Phase 132-P4: `k_exit(env) → TailCall(post_k, env)`, `post_k(env) → <assign>; Ret(env[x])`

**Invariants**:
- No PHI instructions (all merging via env parameters)
- All continuations take full env as parameters
- Post-loop assignment updates env before return

#### Reconnection Mode

Phase 132-P4 uses **DirectValue mode** (same as Phase 131):
- No exit PHI generation
- Final env values from `post_k` are reconnected directly to host `variable_map`
- ExitMeta carries `exit_values: Vec<(String, ValueId)>` for reconnection

### VM + LLVM Parity

Both backends must produce identical results:
- VM: Rust VM backend (`--backend vm`)
- LLVM: LLVM EXE backend via llvm_exe_runner.sh

Expected contract for fixture: exit code `3` (1 + 2)

### Work Items

#### Step 0: Documentation ✅

- ✅ Create Phase 132-P4 documentation in `README.md`
- TODO: Update `docs/development/current/main/10-Now.md`

#### Step 1: Fixtures + Smokes

- New fixture: `apps/tests/phase132_loop_true_break_once_post_add_min.hako`
  - Expected exit code: `3` (1 + 2)
  - Form: x=0; loop(true) { x=1; break }; x=x+2; return x
- VM smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase132_loop_true_break_once_post_add_vm.sh`
- LLVM EXE smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase132_loop_true_break_once_post_add_llvm_exe.sh`

#### Step 2: Implementation

- Extend: `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`
  - Box: `LoopTrueBreakOnceBuilderBox` (existing)
  - Accept: `loop(true) { body ; break }; <post>; return` pattern
  - Reject (Ok(None)): Non-matching patterns
  - Generate: main → loop_step → loop_body → k_exit → post_k (no PHI)
  - Reuse: Phase 130's `lower_assign_stmt` for post-loop assignment

#### Step 3: Verification

```bash
cargo test --lib
cargo build --release -p nyash-rust --features llvm

# Phase 132-P4 smokes (NEW)
bash tools/smokes/v2/profiles/integration/apps/archive/phase132_loop_true_break_once_post_add_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase132_loop_true_break_once_post_add_llvm_exe.sh

# Regressions (Phase 131)
bash tools/smokes/v2/profiles/integration/apps/archive/phase131_loop_true_break_once_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase131_loop_true_break_once_llvm_exe.sh

# Regressions (Phase 97 - stable baseline)
bash tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh
```

#### Step 4: Commits

1. `docs: Phase 132-P4 structure documentation`
2. `test(joinir): Phase 132-P4 loop(true) break-once + post fixture + VM/LLVM smokes`
3. `feat(control_tree): Phase 132-P4 loop(true) break-once with minimal post computation (dev-only)`
4. `docs: Phase 132-P4 DONE`

### Implementation Notes

#### Design Principles

1. **Box-First**: Post-loop lowering is a separate concern from loop body lowering
   - Consider extracting to `PostLoopLowererBox` if complexity grows
2. **Reuse Infrastructure**: Leverage Phase 130's post_k pattern and `lower_assign_stmt`
3. **Fail-Fast**: Contract violations in supported patterns → `freeze_with_hint`
4. **Ok(None) Fallback**: Out of scope patterns → return `Ok(None)` for legacy fallback

#### Reconnection Strategy

Phase 132-P4 continues using **DirectValue mode**:
- `ExitMeta.exit_values` must point to the **final** ValueIds after post_k execution
- Not the k_exit parameters, not the loop_body values, but post_k's updated env values
- This is the SSOT for variable_map reconnection

#### Post-Loop Detection

Check StepNode structure:
```rust
let (prefix_nodes, loop_node, post_nodes) = extract_loop_true_pattern(&step_tree.root);

// Phase 131: post_nodes.is_empty() or post_nodes == [Return(Variable)]
// Phase 132-P4: post_nodes == [Assign(Add), Return(Variable)]
```

#### Reusing Phase 130's lower_assign_stmt

Phase 130 already supports:
- `x = y` (Variable)
- `x = x + <int literal>` (Add)

Phase 132-P4 can directly reuse this for post-loop assignment lowering.

### Acceptance Criteria

- ✅ cargo test --lib PASS
- ✅ cargo build --release -p nyash-rust --features llvm PASS
- ✅ Phase 132-P4 new smokes: 2/2 PASS (VM + LLVM EXE)
- ✅ Phase 131 regression smokes: 2/2 PASS
- ✅ Phase 97 regression smoke: 1/1 PASS
- ✅ Dev toggle OFF → no impact (Ok(None) fallback)

### Box-First Feedback Points

#### Modularization Opportunities

1. **PostLoopLowererBox**: If post-loop logic grows beyond one assignment + return
   - Separate from loop body lowering
   - Single responsibility: lower post-loop statements into post_k
2. **ContinuationBuilderBox**: Common pattern for building continuation functions
   - Reusable across loop_true, if-only, etc.
   - SSOT for env parameter allocation and env map building

#### Legacy Cleanup

1. **Direct env map updates**: No legacy "host side PHI merging" in Normalized path
   - All updates via env parameters and continuations
   - DirectValue reconnection is the clean modern approach
2. **Fail-Fast discipline**: Phase 132-P4 should maintain Phase 131's strict error handling
   - Use `freeze_with_hint` for contract violations
   - Clear hints guide users to fix patterns

#### SSOT Maintenance

1. **ExitMeta.exit_values**: Must reference post_k's final env values
   - Not loop_body values, not k_exit parameters
   - This is critical for DirectValue reconnection correctness
2. **EnvLayout.env_fields()**: SSOT for all env parameter lists
   - Used consistently in main, loop_step, loop_body, k_exit, post_k
   - Any deviation is a contract violation

### Current Status

Phase 132 - DONE ✅ (2025-12-18)

#### P0: post_k Generation ✅

- Extended `loop_true_break_once.rs` to generate post_k continuation
- Reused Phase 130's `lower_assign_stmt` for post-loop statements
- ExitMeta uses DirectValue mode (PHI-free)
- Fixture: `apps/tests/phase132_loop_true_break_once_post_add_min.hako`
- Expected exit code: 3

#### P0.5: Suffix Router for StepTree ✅

- **Root cause**: routing.rs created StepTree from Loop node only, losing post statements
- **Solution**: New `normalized_shadow_suffix_router_box.rs`
  - Detects block suffix: Loop + Assign* + Return
  - Creates StepTree from entire suffix (Block([Loop, Assign, Return]))
- Modified `build_block()` to call suffix router (dev-only)
- StepTree unchanged: Block is SSOT for statement order
- No data duplication: Loop doesn't hold post_nodes

#### P1: k_exit Continuation Fix ✅

- **Problem**: k_exit with TailCall(post_k) was classified as skippable continuation
- **Root cause**: Classification was name-based, didn't examine function body
- **Solution**: Check function body structure before classifying as skippable
- Non-skippable pattern: continuation with TailCall to another function
- VM/LLVM EXE parity achieved (exit code 3)

#### R0: Refactoring Infrastructure ✅

**Task 1: Continuation SSOT 一本化**
- Added `JoinInlineBoundary::default_continuations()`
- Replaced all `BTreeSet::from([JoinFuncId::new(2)])` hardcoding (7 locations)
- Single source of truth for continuation function IDs

**Task 2: merge 契約 docs SSOT 化**
- New: `src/mir/builder/control_flow/joinir/merge/README.md`
- Documented continuation contracts, skip conditions, forbidden behaviors
- Prohibited by-name/by-id classification

**Task 3: テスト配置正規化**
- New: `src/mir/builder/control_flow/joinir/merge/tests/continuation_contract.rs`
- Moved tests from instruction_rewriter.rs to dedicated test file
- Added 4 test cases (Case A-D)

**Task 4: legacy 導線隔離**
- New: `src/mir/builder/control_flow/joinir/legacy/`
- Moved `routing_legacy_binding.rs` to `legacy/routing_legacy_binding.rs`
- Added `legacy/README.md` with removal conditions
- No cfg(feature="legacy") (docs-only isolation for now)

**Task 5: ノイズ除去**
- Removed unused imports (ConstValue, MirInstruction)
- Reduced warnings in touched files

#### Test Results ✅

```bash
cargo test --lib: 1176 PASS
Phase 132 VM: PASS (exit code 3)
Phase 132 LLVM EXE: PASS (exit code 3)
Phase 131 regression: PASS
Phase 97 regression: PASS
```

#### Key Achievements

- ✅ loop(true) + post-loop works in both VM and LLVM EXE
- ✅ Continuation contracts SSOT化
- ✅ merge が by-name 推測禁止
- ✅ Legacy code path isolated for future removal
- ✅ Code quality improved (warnings reduced)

#### Documentation

- Entry: `docs/development/current/main/10-Now.md`
- SSOT: `JoinInlineBoundary::default_continuations()`
- SSOT: `src/mir/builder/control_flow/joinir/merge/README.md` (merge contracts)
- Tests: `src/mir/builder/control_flow/joinir/merge/tests/continuation_contract.rs`
- Legacy: `src/mir/builder/control_flow/joinir/legacy/README.md` (removal conditions)
