# Phase 73: BindingId-Based Scope Manager - Index

**Status**: ✅ Complete (Design Phase)
**Date**: 2025-12-13

---

## Quick Links

### 📋 Core Documents
1. **[Design Document (SSOT)](phase73-scope-manager-design.md)** - Complete design specification
2. **[Completion Summary](phase73-completion-summary.md)** - Phase 73 deliverables and next steps

### 💻 Code
- **PoC Implementation**: `src/mir/join_ir/lowering/scope_manager_bindingid_poc/mod.rs`
  - Feature-gated: `#[cfg(feature = "normalized_dev")]`
  - Tests: 6/6 passing ✅

---

## What is Phase 73?

**Purpose**: Design a BindingId-based scope management system for JoinIR lowering to align with MIR's lexical scope model.

**Problem**:
- MIR builder uses **BindingId** for shadowing (Phase 68-69)
- JoinIR lowering uses **name-based** lookup (fragile, string matching)
- Mismatch creates future bug risk

**Solution**:
- Introduce **BindingId** into JoinIR's ScopeManager
- Gradual migration (Phases 74-77)
- Eliminate naming convention hacks (`is_digit_pos`, `is_ch_match`)

---

## Phase 73 Deliverables

### ✅ Design Document
**File**: [phase73-scope-manager-design.md](phase73-scope-manager-design.md)

**Contents** (34 sections, ~700 lines):
- Current state analysis (MIR + JoinIR scope systems)
- Problem identification (shadowing, naming brittleness)
- Proposed architecture (Option A: Parallel BindingId Layer)
- Integration with MirBuilder (binding_map additions)
- Migration roadmap (Phases 74-77)
- Example scenarios (shadowing, promoted variables)

---

### ✅ Proof-of-Concept
**File**: `src/mir/join_ir/lowering/scope_manager_bindingid_poc/mod.rs`

**Highlights**:
- `BindingId` type wrapper
- `ConditionEnvV2` (parallel name + BindingId lookup)
- `CarrierInfoV2` (BindingId-based promotion)
- `ScopeManagerV2` trait + implementation

**Test Results**:
```
running 6 tests
test test_condition_env_v2_basic ... ok
test test_shadowing_simulation ... ok
test test_promoted_binding_resolution ... ok
test test_scope_manager_v2_binding_lookup ... ok
test test_scope_manager_v2_promoted_lookup ... ok
test test_unified_lookup_fallback ... ok
```

---

## Migration Roadmap

### Phase 74: Infrastructure (2-3 hours)
- Add `binding_map` to `MirBuilder`
- Add `binding_to_join` to `ConditionEnv`
- BindingId allocator

### Phase 75: Pattern 1 Pilot (1-2 hours)
- Migrate simplest pattern (no carriers)
- Prove BindingId integration works

### Phase 76: Pattern 2 Promotion (2-3 hours)
- Eliminate naming convention hacks
- BindingId-based carrier promotion

### Phase 77: Pattern 3-4 + Cleanup (2-3 hours)
- Complete migration
- Remove legacy name-based code

**Total Estimated Effort**: 8-12 hours

---

## Key Design Decisions

### 1. Gradual Migration (Option A)
**Why**: Low risk, backward compatible, easy rollback

**Alternative Rejected**: Full replacement (Option B) - too risky for Phase 73

---

### 2. Parallel Lookup Strategy
```rust
// Phase 74-76 (transition)
fn lookup(&self, name: &str) -> Option<ValueId> {
    // 1. Try BindingId lookup (new code)
    if let Some(binding) = self.name_to_binding.get(name) {
        if let Some(value) = self.binding_to_join.get(binding) {
            return Some(value);
        }
    }
    // 2. Fallback to name lookup (legacy code)
    self.name_to_join.get(name).copied()
}
```

---

### 3. Per-Function BindingId Scope
**Decision**: Each function has independent BindingId allocation

**Reasoning**:
- Like ValueId (proven model)
- No global state needed
- Simpler implementation

**Alternative**: Global BindingId pool (for Phase 63 integration) - deferred

---

## No Production Impact

**Guarantee**:
- ✅ No changes to production code (except 1 line mod.rs)
- ✅ PoC is feature-gated (`normalized_dev`)
- ✅ All existing tests pass (1049 tests)
- ✅ Normal build unaffected

**Modified Files** (3 total):
1. Design doc (new)
2. Completion summary (new)
3. PoC module (new, dev-only)
4. mod.rs (1 line for PoC)

---

## Success Metrics

### Design Quality ✅
- [x] SSOT document (34 sections)
- [x] Clear problem statement
- [x] Proposed architecture with examples
- [x] Integration points identified
- [x] Migration path defined

### PoC Validation ✅
- [x] Compiles under `normalized_dev`
- [x] All 6 tests passing
- [x] Demonstrates key concepts:
  - Parallel lookup (BindingId + name)
  - Shadowing simulation
  - Promoted variable resolution

### Risk Mitigation ✅
- [x] Feature-gated (no prod impact)
- [x] Gradual migration plan
- [x] Backward compatibility preserved
- [x] Clear rollback strategy

---

## Open Questions (Phase 74+)

### Q1: Performance
**Concern**: Dual maps double memory usage

**Mitigation**: Remove legacy maps after Phase 77, profile during Phase 74

---

### Q2: Captured Variables
**Question**: How to add BindingId to CapturedVar?

**Answer**: Phase 76 task (update function_scope_capture module)

---

### Q3: Phase 63 Integration
**Question**: Use global BindingId for ownership analysis?

**Answer**: Phase 78+ (future enhancement)

---

## Related Work

### Completed Phases
- **Phase 68-69**: MIR lexical scope + shadowing
- **Phase 231**: ScopeManager trait (current impl)
- **Phase 224**: Promoted LoopBodyLocal (naming convention)

### Future Phases
- **Phase 74**: BindingId infrastructure
- **Phase 75**: Pattern 1 migration
- **Phase 76**: Pattern 2 migration (carrier promotion)
- **Phase 77**: Pattern 3-4 migration + cleanup

---

## References

### Design Documents
- [phase73-scope-manager-design.md](phase73-scope-manager-design.md) - **SSOT**
- [phase73-completion-summary.md](phase73-completion-summary.md) - Deliverables
- [phase238-exprlowerer-scope-boundaries.md](phase238-exprlowerer-scope-boundaries.md) - Scope boundaries (related)

### Code Files
- `src/mir/builder/vars/lexical_scope.rs` - MIR lexical scope (existing)
- `src/mir/join_ir/lowering/scope_manager.rs` - Current ScopeManager
- `src/mir/join_ir/lowering/carrier_info.rs` - Current CarrierInfo
- `src/mir/join_ir/lowering/scope_manager_bindingid_poc/mod.rs` - PoC (Phase 73)

---

## Recommended Reading Order

### For Implementation (Phase 74+)
1. **[Design Document](phase73-scope-manager-design.md)** - Full context
2. **PoC Code** - Concrete examples
3. **[Completion Summary](phase73-completion-summary.md)** - Migration checklist

### For Review
1. **This Index** - Quick overview
2. **[Completion Summary](phase73-completion-summary.md)** - What was delivered
3. **[Design Document](phase73-scope-manager-design.md)** - Deep dive (if needed)

---

## Contact / Questions

**Phase 73 Design**: Complete, ready for user review
**Next Steps**: User approval → Phase 74 implementation

**Estimated Timeline**:
- Phase 74: 1 week (infrastructure)
- Phase 75: 2-3 days (Pattern 1)
- Phase 76: 3-4 days (Pattern 2)
- Phase 77: 3-4 days (Pattern 3-4 + cleanup)
- **Total**: 2-3 weeks (leisurely pace)

---

**Status**: ✅ Phase 73 Complete - Ready for Phase 74
