Status: Active  
Scope: Phase 33-16（Loop Header PHI SSOT）に関する現役目次。歴史詳細は archive 側を参照。

# Phase 33-16: Loop Header PHI SSOT - Documentation Index

**Last Updated**: 2025-12-07  
**Status**: ✅ Complete implementation design ready

## Quick Navigation

### For Implementation
**Start here**: [phase33-16-visual-guide.md](phase33-16-visual-guide.md)
- Architecture flow diagram (all 7 phases)
- Code change map with exact line numbers
- 7 complete code changes (copy-paste ready)
- Testing commands

### For Understanding
**Read first**: [phase33-16-qa.md](phase33-16-qa.md)
- Answer to all 5 core questions
- Exact code snippets with explanations
- "What you DON'T need" guidance
- Complete flow summary

### For Context
**Detailed planning**: [phase33-16-implementation-plan.md](phase33-16-implementation-plan.md)
- Executive summary
- Problem analysis
- 6 concrete implementation steps
- Testing strategy
- Risk analysis and mitigation
- Future enhancements

### Quick Summary
**Reference**: [PHASE_33_16_SUMMARY.md](PHASE_33_16_SUMMARY.md)
- TL;DR answers
- Architecture evolution
- Implementation scope
- Key architectural insight
- Testing roadmap

### Original Design
**Background**: [phase33-16-loop-header-phi-design.md](phase33-16-loop-header-phi-design.md)
- Original design document
- Problem statement
- Solution overview

---

## Your 5 Questions - Direct Answers

### Q1: Where exactly should LoopHeaderPhiBuilder::build() be called?
**Location**: `src/mir/builder/control_flow/joinir/merge/mod.rs`, line 107
**When**: Between Phase 3 (remap_values) and Phase 4 (instruction_rewriter)
**Why**: Phase 3.5 allocates PHI dsts before instruction_rewriter needs them
**Details**: [phase33-16-qa.md#q1](phase33-16-qa.md#q1-where-exactly-should-loopheaderphibuilderbuilld-be-called)

### Q2: How do I get the header_block_id (loop_step's entry block after remapping)?
**Code**: `remapper.get_block(entry_func_name, entry_func.entry_block)?`
**Key**: Entry block is the loop header for Pattern 2
**Details**: [phase33-16-qa.md#q2](phase33-16-qa.md#q2-how-do-i-get-the-header_block_id-loops-entry-block-after-remapping)

### Q3: How do I get the loop variable's initial value (host-side)?
**Code**: `remapper.get_value(ValueId(0))?`
**Key**: ValueId(0) is always the loop parameter in JoinIR space
**Details**: [phase33-16-qa.md#q3](phase33-16-qa.md#q3-how-do-i-get-the-loop-variables-initial-value-host-side)

### Q4: Where should instruction_rewriter record latch_incoming?
**Location**: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`, ~line 300
**When**: In tail call section, after parameter bindings
**Code**: `loop_header_phi_info.set_latch_incoming(loop_var_name, target_block, latch_value)`
**Details**: [phase33-16-qa.md#q4](phase33-16-qa.md#q4-where-should-instruction_rewriter-record-latch_incoming)

### Q5: Should the Phase 33-15 skip logic be removed or modified?
**Answer**: **Modify, NOT remove**
**Strategy**: Use header PHI dst when available, fallback to parameter
**Details**: [phase33-16-qa.md#q5](phase33-16-qa.md#q5-should-the-phase-33-15-skip-logic-be-removed-or-modified-to-use-header-phi-dst)

---

## Implementation at a Glance

### Files to Modify
- `src/mir/builder/control_flow/joinir/merge/mod.rs` (2 locations: Phase 3.5, Phase 4.5)
- `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` (5 locations: signature, latch tracking, exit logic)

### Changes Summary
| Phase | What | Where | Type |
|-------|------|-------|------|
| 3.5 | Build header PHIs | merge/mod.rs | New |
| 4 | Update signature | instruction_rewriter.rs | Signature |
| 4 | Track latch | instruction_rewriter.rs | New code |
| 4 | Use PHI for exit | instruction_rewriter.rs | Modified |
| 4 | Use PHI for carriers | instruction_rewriter.rs | Modified |
| 4.5 | Finalize PHIs | merge/mod.rs | New |

### Scope
- **Lines added**: ~100
- **Lines modified**: ~100
- **Lines removed**: 0 (backward compatible)
- **New files**: 0 (uses existing LoopHeaderPhiBuilder)

---

## Architecture Diagram

```
Phase 3: remap_values()
  ↓ (ValueIds remapped: 0→100, 0→101, etc.)
Phase 3.5: LoopHeaderPhiBuilder::build() ⭐ NEW
  └── Allocate phi_dst(101), entry_incoming(init_value)
  └── Pass loop_header_phi_info to Phase 4
Phase 4: instruction_rewriter::merge_and_rewrite()
  ├── When tail call: set_latch_incoming()
  └── When Return: use phi_dst (not parameter)
Phase 4.5: LoopHeaderPhiBuilder::finalize() ⭐ NEW
  └── Emit PHIs into header block
Phase 5: exit_phi_builder::build_exit_phi()
  └── Return carrier_phis with header PHI dsts
Phase 6: ExitLineOrchestrator::execute()
  └── Update variable_map with carrier_phis
```

---

## Testing Checklist

Before implementation:
- [ ] Review phase33-16-qa.md (answers to your questions)
- [ ] Review phase33-16-visual-guide.md (code locations)
- [ ] Baseline compile: `cargo build --release`

During implementation:
- [ ] Implement Phase 3.5 → compile
- [ ] Update signature → compile
- [ ] Add latch tracking → compile
- [ ] Replace exit_phi_inputs → compile
- [ ] Replace carrier_inputs → compile
- [ ] Add Phase 4.5 → compile
- [ ] Update docs → compile

After implementation:
- [ ] `NYASH_JOINIR_DEBUG=1 ./target/release/nyash --dump-mir test.hako | grep "Phase 33-16"`
- [ ] Verify header block has PHI instructions at start
- [ ] Verify exit values reference header PHI dsts
- [ ] Test: `joinir_min_loop.hako` produces correct MIR
- [ ] Test: Loop variable values correct at exit

---

## Document Quick Links

### Complete Guides (in order of use)

1. **phase33-16-qa.md** (11 KB)
   - Start here for understanding
   - All 5 questions answered
   - Code examples ready to copy-paste

2. **phase33-16-visual-guide.md** (22 KB)
   - Use during implementation
   - Architecture diagrams
   - Complete code changes with line numbers
   - Copy-paste ready code

3. **phase33-16-implementation-plan.md** (18 KB)
   - For detailed planning
   - Risk analysis
   - Testing strategy
   - Future enhancements

4. **PHASE_33_16_SUMMARY.md** (8.2 KB)
   - Quick reference
   - TL;DR answers
   - Key insights
   - Next steps

### Reference Documents

5. **phase33-16-loop-header-phi-design.md** (9.0 KB)
   - Original design document
   - Historical context

---

## Key Concepts

### Loop Header PHI as SSOT (Single Source of Truth)

The core idea: Instead of using undefined loop parameters, use PHI nodes at the loop header to track the "current value" of loop variables.

**Why it works**:
- PHI nodes ARE SSA-defined at the header block
- PHI dsts can safely be referenced in exit values
- Eliminates SSA-undef errors from undefined parameters

**Architecture**:
- **Phase 3.5**: Pre-allocate PHI dsts (before instruction processing)
- **Phase 4**: Set latch incoming during instruction processing
- **Phase 4.5**: Finalize PHIs with all incoming edges
- **Phase 6**: Use PHI dsts in exit line reconnection

### Two-Phase PHI Construction

**Phase 3.5: build()** - Allocate PHI dsts
```rust
let phi_dst = builder.next_value_id();  // Allocate
entry_incoming = (entry_block, init_value);  // Set entry
latch_incoming = None;  // Will be set in Phase 4
```

**Phase 4.5: finalize()** - Emit PHI instructions
```rust
PHI {
  dst: phi_dst,  // Already allocated
  inputs: [
    (entry_block, init_value),  // From Phase 3.5
    (header_block, latch_value),  // From Phase 4
  ]
}
```

---

## Known Limitations & Future Work

### Phase 33-16 (Current)
✅ Minimal scope: Loop variable only, no other carriers
✅ Pattern 2 primary focus (break statements)
✅ Backward compatible with Phase 33-15

### Phase 33-16+ (Future)
- [ ] Extract multiple carriers from exit_bindings
- [ ] Handle Pattern 3+ with multiple PHIs
- [ ] Pattern-specific header block identification
- [ ] Optimize constant carriers

---

## Support & Debugging

### If implementation breaks:

1. **Compilation error**: Check if loop_header_phi_info is properly passed as mutable reference
2. **SSA-undef error**: Check if finalize() is called and all latch_incoming are set
3. **Wrong values**: Check if latch_incoming is set from correct tail call args
4. **No header PHI**: Enable `NYASH_JOINIR_DEBUG=1` and check "Phase 33-16" output

### Debug commands:
```bash
# Check debug output
NYASH_JOINIR_DEBUG=1 ./target/release/nyash --dump-mir test.hako 2>&1 | grep "Phase 33-16"

# Inspect MIR
./target/release/nyash --emit-mir-json mir.json test.hako
jq '.functions[0].blocks[0].instructions[0:3]' mir.json
```

---

## Next Phase: Phase 34+

After Phase 33-16 is complete:
1. Pattern 4 (continue statement) implementation
2. Trim patterns with complex carriers
3. Loop-as-expression full integration
4. Performance optimizations

---

**Ready to implement?** Start with [phase33-16-qa.md](phase33-16-qa.md)!
