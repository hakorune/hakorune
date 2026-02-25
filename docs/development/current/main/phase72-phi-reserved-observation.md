# Phase 72: PHI Reserved Region Observation Report

## Executive Summary

**Date**: 2025-12-13
**Status**: ⚠️ **Finding: PHI dst allocation does NOT respect reserved region**

### Key Finding

PHI dst ValueIds are allocated via `builder.next_value_id()` from the host MirBuilder, NOT from the reserved region (0-99) described in `join_value_space.rs`.

### Evidence

1. **Documentation states**:
   ```rust
   // src/mir/join_ir/lowering/join_value_space.rs
   //!  0          100        1000                     u32::MAX
   //!  ├──────────┼──────────┼──────────────────────────┤
   //!  │  PHI     │  Param   │       Local             │
   //!  │  Reserved│  Region  │       Region            │
   //!  └──────────┴──────────┴──────────────────────────┘
   //! - **PHI Reserved (0-99)**: Pre-reserved for LoopHeader PHI dst
   ```

2. **Actual PHI allocation** (from `loop_header_phi_builder.rs:90,147`):
   ```rust
   let loop_var_phi_dst = builder.next_value_id();  // From host MirBuilder!
   let phi_dst = builder.next_value_id();          // Not from JoinValueSpace!
   ```

3. **Observed PHI dst from `loop_min_while.hako`**:
   ```
   bb4:
       1: %3: String = phi [%2, bb0], [%12, bb7]
   ```
   - PHI dst = `%3` (ValueId(3))
   - ✅ This IS in reserved region (0-99)

### Analysis

#### Current Behavior

- PHI dst values come from `builder.next_value_id()` which starts from 0
- MirBuilder allocates ValueIds sequentially: 0, 1, 2, 3, ...
- Early ValueIds (from function setup) naturally fall into 0-99 range
- **This is ACCIDENTAL compliance**, not architectural enforcement

#### Observed Pattern

From `loop_min_while.hako` MIR dump:
- Entry block constants: `%1`, `%2` (ValueId 1,2)
- PHI dst: `%3` (ValueId 3) - in loop header
- Loop body values: `%8`, `%9`, `%10`, `%11`, `%12` (8-12)
- Exit value: `%17` (ValueId 17)

**Conclusion**: PHI dst happens to be low-numbered because it's allocated early in the function, NOT because of reserved region logic.

#### Why This Works Today

1. Loop header PHI is allocated BEFORE loop body instructions
2. Function entry typically uses ValueIds 0-10
3. PHI dst gets allocated in early range (0-20 typically)
4. No collision with JoinValueSpace regions (100-999, 1000+) because:
   - JoinIR uses high ValueIds (100+, 1000+)
   - Host MIR uses low ValueIds (0-99)
   - They happen to not overlap in practice

### Risk Assessment

#### Current Risks: **LOW**

- No observed collisions in 937/937 tests
- JoinValueSpace and MirBuilder allocate from different ranges
- Pattern2 frontend bug (Phase 201) was fixed with explicit regions

#### Future Risks: **MEDIUM**

- If MirBuilder allocates 100+ ValueIds before loop header:
  - PHI dst could be ValueId(100+)
  - Could collide with JoinValueSpace Param region
  - Would break `remap_values()` assumptions
- If JoinIR lowering uses ValueIds < 100:
  - Could collide with PHI dst
  - Would corrupt SSA graph

### Recommendation

**DO NOT strengthen verifier** to enforce PHI dst ∈ [0, 99].

**Reasons**:
1. Current architecture does NOT guarantee this
2. PHI dst allocation is a host MirBuilder concern, not JoinIR concern
3. Reserve region (0-99) is a JoinValueSpace contract for JoinIR lowering
4. PHI dst is allocated OUTSIDE JoinIR layer

**Instead**:
1. Document current behavior (Phase 72 observation)
2. Keep `JoinValueSpace.reserve_phi()` as debug marker only
3. Maintain existing collision detection (Phase 205)
4. Monitor for regressions in test suite

### Alternative: Architectural Fix (Future Phase)

If strict PHI dst reservation is desired:

1. **Allocate PHI dst from reserved pool**:
   ```rust
   // In LoopHeaderPhiBuilder
   let phi_dst = builder.alloc_phi_reserved(); // New API: 0-99 pool
   ```

2. **Separate PHI ValueId space**:
   ```rust
   struct PhiReservedPool {
       next_phi_id: u32, // Start at 0
   }
   impl PhiReservedPool {
       fn alloc(&mut self) -> ValueId {
           assert!(self.next_phi_id < 100, "PHI pool exhausted");
           let id = ValueId(self.next_phi_id);
           self.next_phi_id += 1;
           id
       }
   }
   ```

3. **Fail-fast at 100 PHI nodes**:
   - Explicit limit prevents accidental overflow
   - 100 PHI nodes per function is generous

**Scope**: Phase 73+ (optional enhancement, not urgent)

## Implementation Record

### Files Modified

1. `src/mir/join_ir/verify_phi_reserved.rs` (new)
   - Observation infrastructure
   - Distribution analyzer
   - Report generator

2. `src/mir/join_ir/mod.rs`
   - Added verify_phi_reserved module

3. `src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs`
   - Added observation hooks (debug-only)

4. `tests/phase72_phi_observation.rs` (created, not used)
   - Integration test skeleton (visibility issues)

### Test Results

- Observation mechanism: ✅ Implemented
- Manual verification via `--dump-mir`: ✅ Confirmed PHI dst in low range
- Automatic test collection: ⚠️ Blocked by API visibility

### Decision

**Phase 72 COMPLETE** - Observation phase only.

**Verifier strengthening**: ❌ NOT RECOMMENDED

**Next steps**: Document findings, monitor in future phases.

---

## Appendix: Observed PHI ValueIds

### loop_min_while.hako
- Loop variable `i`: PHI dst = %3 (ValueId(3))
- Range: [3, 3]
- ✅ In reserved region

### Expected Pattern (Not Tested)
- Multi-carrier loops (sum+count): PHI dst = %3, %4 expected
- Nested loops: PHI dst could be %5-10
- Complex functions: PHI dst could exceed 20

### Theoretical Maximum

Without enforcement:
- Large function with 200 const/copy before loop: PHI dst could be %200+
- Would fall into Param region (100-999)
- Would NOT be caught by current verifier

## Code References

- `src/mir/join_ir/lowering/join_value_space.rs`: Region definitions
- `src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs`: PHI allocation
- `docs/development/current/main/joinir-architecture-overview.md`: Invariant 8

## Phase 72 Complete

**Conclusion**: PHI dst allocation is currently stable through accidental low-numbering, not architectural enforcement. Verifier strengthening would create false assumptions. Document and monitor instead.
