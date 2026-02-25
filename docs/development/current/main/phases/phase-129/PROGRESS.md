# Phase 129 Progress Report

## Status: P0/P2/P3 Complete, P1 in Progress (129-B complete)

### Completed Tasks ✅

#### P0: LLVM EXE Smoke Test (Commit: e7ad3d31b)
- ✅ Created `phase128_if_only_partial_assign_normalized_llvm_exe.sh`
- ✅ VM+LLVM parity verified for Phase 128
- ✅ Regression tests pass:
  - phase103_if_only_llvm_exe.sh: PASS
  - phase118_loop_nested_if_merge_llvm_exe.sh: PASS
  - phase128_if_only_partial_assign_normalized_vm.sh: PASS

#### P2: Post-If Return Var Fixture (Commit: 083be9921)
- ✅ Created `apps/tests/phase129_if_only_post_if_return_var_min.hako`
  - Pattern: `x=1; if flag==1 { x=2 }; print(x)`
  - Tests join_k env merge concept
- ✅ Created `phase129_if_only_post_if_return_var_vm.sh`
  - Expected output: 2
  - Currently PASSES via fallback path (non-Normalized)

#### P3: Documentation (Commits: 083be9921, 0d2eafd78)
- ✅ Phase 129 README (`README.md`)
  - Goal, design, scope, acceptance criteria
  - join_k continuation pattern explanation
  - Structural verification requirements
- ✅ P1 Implementation Plan (`P1-implementation-plan.md`)
  - Current state analysis
  - Required changes detailed
  - Implementation options (A vs B)
  - Testing strategy
  - Risk assessment
  - Open questions

### In-Progress Task 🔄

#### P1: Materialize join_k Continuation

**Current State**: Phase 129-B (if-as-last) implemented; Phase 129-C (post-if) remains.

**Key Findings**:
1. Current `lower_if_node` is Phase 123-124 minimal (incomplete)
   - Only processes then branch
   - No join_k generation
   - No branching structure
2. Requires substantial refactoring:
   - Create JoinKBuilder helper module
   - Refactor lower_if_node signature (affects call chain)
   - Add env snapshot/merge logic
   - Process post-if statements in join_k body

**Implementation Options**:

**Option A: In-Place Modification**
- Pros: Direct, fast (1 commit)
- Cons: High risk, harder to test incrementally

**Option B: Phased Approach (Recommended)**
- Phase 129-A: JoinKBuilder foundation + helpers
- Phase 129-B: Branch lowering to join_k
- Phase 129-C: Post-if statement support
- Pros: Incremental testing, clear rollback points
- Cons: More commits (3), slightly slower

**Decision Needed**: None for 129-B; next is 129-C (post-if via post_k continuation).

## Test Results Summary

### Library Tests
```bash
cargo test --lib
```
✅ Result: **1165 passed; 0 failed; 56 ignored**

### Smoke Tests
```bash
# Phase 128 VM
phase128_if_only_partial_assign_normalized_vm.sh
```
✅ Result: **PASS** (output: 2)

```bash
# Phase 128 LLVM EXE
phase128_if_only_partial_assign_normalized_llvm_exe.sh
```
✅ Result: **PASS** (output: 2)

```bash
# Phase 129 VM
phase129_if_only_post_if_return_var_vm.sh
```
✅ Result: **PASS** (output: 2)
⚠️ Note: Currently via fallback path (non-Normalized)

```bash
# Regression
phase103_if_only_llvm_exe.sh
phase118_loop_nested_if_merge_llvm_exe.sh
```
✅ Result: **ALL PASS**

## Feedback (Box-First / Fail-Fast / Legacy)

### Box-First Modularization
- ✅ **Proposed**: JoinKBuilder module for join_k generation
  - Single responsibility: join_k function creation
  - Separate file: `join_k_builder.rs`
  - Testable in isolation
- 📋 **Future**: Consider extracting env management to EnvBuilder

### Fail-Fast Opportunities
- ✅ **Proposed**: `verify_normalized_structure` function
  - Early detection of malformed Normalized modules
  - Checks: join_k exists if If exists, no PHI instructions
  - Returns structured errors with hints
- 📋 **Current**: `verify_branch_is_return_literal` too restrictive
  - Should allow Assign statements (Phase 128 already supports)
  - Should verify tail-call structure (not just Return)

### Legacy Findings
- ⚠️ **Phase 123-124 Legacy**: `lower_if_node` incomplete implementation
  - Only processes then branch (line 463)
  - Verification too restrictive for Phase 128+
  - No branching structure generated
  - **Recommendation**: Mark as Phase 123-124 legacy, replace in Phase 129
- ⚠️ **Placeholder Logic**: lhs_vid uses Const(0) placeholder (line 431-434)
  - **Recommendation**: Phase 129 should use actual variable resolution

## Open Questions for User/ChatGPT

1. **Implementation Strategy**: Choose Option A or B?
   - Option A: Fast, risky (in-place modification)
   - Option B: Safe, incremental (3-phase rollout)

2. **Function ID Allocation**: Where should `next_func_id` come from?
   - Add to `lower_if_only_to_normalized` parameters?
   - Module tracks max ID?

3. **Post-If Statement Detection**: How to parse post-if nodes?
   - StepTree structure provides this?
   - Need lookahead logic?

4. **join_k Naming Convention**:
   - `join_k_<func_id>`?
   - `k_join_<line_number>`?
   - Auto-increment `join_k_0`, `join_k_1`, ...?

5. **env Parameter Passing**: Representation?
   - Vec<ValueId> (ordered by env_layout)?
   - Future: Tuple unpacking?

## Next Steps

### If Option B (Recommended)

**Phase 129-A: Foundation**
1. Create `src/mir/control_tree/normalized_shadow/join_k_builder.rs`
2. Implement JoinKBuilder with:
   - `create_join_k_function`
   - `snapshot_env`
   - `generate_tail_call`
3. Unit tests for JoinKBuilder
4. Commit: "feat(control_tree): Phase 129-A - JoinKBuilder foundation"

**Phase 129-B: Branch Lowering**
1. Refactor `lower_if_node` to use JoinKBuilder
2. Generate then/else tail-calls to join_k
3. Test with simple fixture (no post-if)
4. Commit: "feat(control_tree): Phase 129-B - branch lowering to join_k"

**Phase 129-C: Post-If Support**
1. Process post-if statements in join_k body
2. Test with Phase 129 fixture
3. Add `verify_normalized_structure` checks
4. Commit: "feat(control_tree): Phase 129-C - post-if support + verification"

**Final**
- Update 10-Now.md, INDEX, Backlog
- Commit: "docs: Phase 129 DONE"

### If Option A (Fast Track)

**Single Commit**
1. Implement all changes in one go
2. Extensive testing required before commit
3. Higher rollback risk
4. Commit: "feat(control_tree): Phase 129 - materialize join_k continuation"

## Commits (Current Session)

1. **e7ad3d31b**: test(joinir): Phase 129 P0 - add LLVM EXE smoke for Phase 128
2. **083be9921**: test(joinir): Phase 129 P2 - add post-if return var fixture + VM smoke
3. **0d2eafd78**: docs: Phase 129 P1 implementation plan (join_k materialization)

## Acceptance Status

### P0 ✅ DONE
- [x] Phase 128 LLVM EXE smoke passes
- [x] Regression tests pass

### P1 🔄 AWAITING DECISION
- [ ] JoinKBuilder module created
- [ ] join_k materialized in builder.rs
- [ ] verify_normalized_structure implemented
- [ ] Unit tests pass

### P2 ✅ DONE
- [x] Phase 129 fixture created
- [x] VM smoke test created and passing
- [x] Note: Currently fallback path

### P3 ✅ DONE
- [x] README.md documentation
- [x] P1 implementation plan
- [x] Progress report (this file)

## Summary

**Phase 129 Status**: P0, P2, P3 complete. P1 awaiting user decision on implementation strategy.

**Recommendation**: Choose **Option B (Phased Approach)** for safer, incremental implementation with clear rollback points.

**Key Achievement**: VM+LLVM parity for Phase 128 baseline established. Phase 129 foundation (tests, docs, fixtures) ready for P1 implementation.

**Next Action**: User/ChatGPT decides on Option A vs B, then implement P1 accordingly.
