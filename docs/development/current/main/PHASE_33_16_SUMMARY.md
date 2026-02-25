# Phase 33-16 Implementation Summary

**Date**: 2025-12-07  
**Status**: ✅ Complete detailed design with concrete implementation steps  
**Files Saved**: 4 comprehensive guides in `docs/development/current/main/`

---

## What You Asked

You wanted to understand the exact flow for implementing **Phase 33-16: Loop Header PHI SSOT**, specifically:

1. Where exactly should LoopHeaderPhiBuilder::build() be called?
2. How do I get the header_block_id?
3. How do I get the loop variable's initial value?
4. Where should instruction_rewriter record latch_incoming?
5. Should Phase 33-15 skip logic be removed or modified?

---

## What You Got

### 4 Comprehensive Implementation Guides

1. **phase33-16-implementation-plan.md** (18 KB)
   - Executive summary of architecture change
   - Problem analysis and solution
   - 6 concrete implementation steps with exact line numbers
   - Testing strategy
   - Risk analysis
   - Checklist and dependencies

2. **phase33-16-qa.md** (11 KB)
   - Direct answers to all 5 of your questions
   - Exact code snippets ready to copy-paste
   - "What you DON'T need" guidance
   - Complete flow summary

3. **phase33-16-visual-guide.md** (22 KB)
   - Architecture flow diagram showing all 7 phases
   - Code change map with file locations
   - 7 complete code changes (copy-paste ready)
   - Complete flow checklist
   - Testing commands
   - Summary table

4. **This document** - Quick reference

---

## TL;DR: Quick Answers

### Q1: Where to call build()?
**Answer**: Line 107 in `merge/mod.rs`, **after** `remap_values()`
- Phase 3.5 (new phase between remap and instruction_rewriter)
- Pass mutable reference to instruction_rewriter

### Q2: How to get header_block_id?
**Answer**: 
```rust
let entry_block_id = remapper
    .get_block(entry_func_name, entry_func.entry_block)?;
let header_block_id = entry_block_id; // For Pattern 2
```

### Q3: How to get loop_var_init?
**Answer**:
```rust
let loop_var_init = remapper
    .get_value(ValueId(0))?;  // JoinIR param slot is always 0
```

### Q4: Where to record latch_incoming?
**Answer**: In tail call section (line ~300 in instruction_rewriter.rs), after parameter bindings:
```rust
loop_header_phi_info.set_latch_incoming(loop_var_name, target_block, latch_value);
```

### Q5: Should skip logic be removed?
**Answer**: **NO**. Modify with fallback mechanism:
- Use header PHI dst when available (Phase 33-16)
- Fall back to parameter for backward compatibility
- Explicit "Using PHI" vs "Fallback" logs

---

## Architecture Evolution

### Phase 33-15 (Current - Stop-gap)
```
Loop Parameter → (undefined SSA) → SSA-undef error
```

### Phase 33-16 (Your implementation)
```
Loop Parameter → Header PHI (allocated in Phase 3.5)
                    ↓
               Exit values use PHI dst (Phase 4/5)
                    ↓
               SSA-correct (PHI dst is defined!)
```

---

## Implementation Scope

**Files to modify**: 2 files, 7 locations
- `src/mir/builder/control_flow/joinir/merge/mod.rs` (2 locations)
- `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` (5 locations)

**Lines added**: ~100 lines  
**Lines modified**: ~100 lines  
**Lines removed**: 0 (backward compatible)

**Compilation time**: Should be clean, no new dependencies

---

## Key Architectural Insight

**Loop Header PHI as Single Source of Truth (SSOT)**

The brilliant design moves from "skip and hope" to "allocate early, finalize late":

1. **Phase 3.5**: Allocate PHI dsts BEFORE processing instructions
   - Why: So instruction_rewriter knows what values to use
   - Cost: One extra pass through carrier info

2. **Phase 4**: instruction_rewriter sets latch incoming from tail calls
   - Why: Only found during instruction processing
   - Benefit: No need to analyze loop structure separately

3. **Phase 4.5**: Finalize emits PHIs into blocks
   - Why: All incoming edges must be set first
   - Benefit: Validation that all latch incoming are set

4. **Phase 5**: exit_phi_builder gets carrier_phis from header PHIs
   - Why: Header PHI dsts are guaranteed SSA-defined
   - Benefit: No more undefined parameter references!

---

## Testing Roadmap

### Before You Start
```bash
cargo build --release  # Baseline clean build
```

### After Each Implementation Step
```bash
# Compile after step 1 (Phase 3.5)
cargo build --release

# Compile after step 2 (signature update)
cargo build --release

# Debug output verification (all steps)
NYASH_JOINIR_DEBUG=1 ./target/release/nyash --dump-mir \
  apps/tests/joinir_min_loop.hako 2>&1 | grep "Phase 33-16"
```

### Final Integration Test
```bash
# Expected MIR structure
./target/release/nyash --emit-mir-json mir.json apps/tests/joinir_min_loop.hako
jq '.functions[0].blocks[0].instructions[0:3]' mir.json
# First 3 instructions should include PHI nodes!
```

---

## Implementation Strategy

### Recommended Order
1. ✅ **Step 1**: Add Phase 3.5 (build call in merge/mod.rs)
2. ✅ **Step 2**: Update instruction_rewriter signature
3. ✅ **Step 3**: Add latch tracking in tail call section
4. ✅ **Step 4**: Replace exit_phi_inputs skip logic
5. ✅ **Step 5**: Replace carrier_inputs skip logic
6. ✅ **Step 6**: Add Phase 4.5 (finalize call)
7. ✅ **Step 7**: Update module documentation

**Compile after steps 2, 3, 4, 5, 6 to catch errors early**

### Fallback Strategy
If something breaks:
1. Check if loop_var_name is being set (pattern2_with_break.rs line 200)
2. Verify remapper has both block AND value mappings (Phase 3)
3. Check that instruction_rewriter receives mutable reference
4. Verify finalize() is called with all latch_incoming set

---

## What Gets Fixed

### SSA-Undef Errors (Phase 33-15)
```
[mir] Error: SSA-undef: ValueId(X) referenced but not defined
      Context: exit_phi_inputs [(exit_block, ValueId(X))]
      Cause: X is JoinIR parameter (i_param), not SSA-defined
```

**Fixed in Phase 33-16**:
- Use header PHI dst (ValueId(101)) instead of parameter (ValueId(X))
- Header PHI dst IS SSA-defined (allocated + finalized)

### Exit Value Propagation
```
Before: variable_map["i"] = pre-loop value (initial value)
After:  variable_map["i"] = header PHI dst (current iteration value)
```

---

## Files Provided

### Documentation Directory
```
docs/development/current/main/
├── phase33-16-implementation-plan.md    ← Full implementation roadmap
├── phase33-16-qa.md                     ← Your questions answered
├── phase33-16-visual-guide.md           ← Diagrams + copy-paste code
└── PHASE_33_16_SUMMARY.md               ← This file
```

### How to Use Them

1. **Start here**: phase33-16-qa.md
   - Read the 5 Q&A sections
   - Understand the core concepts

2. **For detailed context**: phase33-16-implementation-plan.md
   - Problem analysis
   - Risk assessment
   - Testing strategy

3. **For implementation**: phase33-16-visual-guide.md
   - Architecture diagrams
   - Complete code changes (copy-paste ready)
   - File locations with line numbers

---

## Next Steps After Implementation

### Phase 33-16 Complete
1. Verify MIR has header PHI instructions
2. Verify exit values reference header PHI dsts
3. Run joinir_min_loop.hako test
4. Check variable_map is correctly updated

### Phase 33-16+
1. Extract multiple carriers from exit_bindings
2. Handle Pattern 3+ with multiple PHIs
3. Optimize constant carriers
4. Update other pattern lowerers

### Phase 34+
1. Pattern 4 (continue statement)
2. Trim patterns with complex carriers
3. Loop-as-expression integration
4. Performance optimizations

---

## Key Takeaway

**Phase 33-16 is about moving from "undefined parameters" to "SSA-defined PHI destinations".**

The architecture is elegant because it:
- ✅ Allocates early (Phase 3.5) for instruction rewriter to use
- ✅ Tracks during processing (Phase 4) for accurate values
- ✅ Finalizes late (Phase 4.5) for validation
- ✅ Uses in exit phase (Phase 6) with guaranteed SSA definitions

No magic, no hacks, just clear responsibility separation and SSA-correct design!

---

## Questions?

If you have questions while implementing:
1. Check **phase33-16-qa.md** for direct answers
2. Check **phase33-16-visual-guide.md** for code locations
3. Check **phase33-16-implementation-plan.md** for design rationale

All documents saved to `docs/development/current/main/`

Good luck with the implementation! 🚀
