# Phase 280: ExitKind+Frag Composition SSOT (A→B→C)

**Status**: ✅ Complete (2025-12-23)
**Phase Number**: 280
**Type**: Design-First (Documentation → Minimal Code)

## Executive Summary

**Goal**: Stop numbered route label enumeration proliferation by establishing Frag composition API as the Single Source of Truth (SSOT) for structured control flow → CFG lowering.

**Strategy**: Three-phase approach (A→B→C):
- **Phase A**: Document SSOT positioning (docs-only, no code) ✅ **Complete**
- **Phase B**: Solidify composition API contract (minimal test-based verification) ✅ **Complete**
- **Phase C**: Prepare ScanWithInit / SplitScan (historical labels: Pattern6/7) for composition API (documentation-only, defer migration to Phase 281) ✅ **Complete**

**Key Insight**: numbered route labels (Pattern1-9+) are **symptom labels** for regression tests, NOT architectural concepts. The architectural SSOT is **Frag composition rules** (`seq`/`if`/`loop`/`cleanup`).

**Phase 280 Goal**: SSOT positioning + 導線固定 (NOT full migration - that's Phase 281)

---

## Purpose

**What this phase achieves**:
1. Establish Frag composition API as THE absorption destination for numbered-route proliferation
2. Document composition SSOT positioning in architecture docs (edgecfg-fragments.md, joinir-architecture-overview.md)
3. Solidify composition API contract through verification and testing
4. Identify ScanWithInit / SplitScan hand-rolled Frag construction locations for future migration
5. Create clear導線 (guidance) for Phase 281+ migration work

**Why this is needed**:
- numbered route labels became architectural decision points (17+ routes across JoinIR/Plan lanes at the time)
- CFG construction logic duplicated across patterns
- Adding new loop shapes required full-stack pattern additions
- Need convergence point to absorb route-specific knowledge

---

## Non-Goals

**What this phase does NOT do**:
1. ❌ Migrate ScanWithInit / SplitScan to use composition API (deferred to Phase 281)
2. ❌ Remove hand-rolled Frag construction (documentation-only in Phase 280 C)
3. ❌ Change router behavior (routing remains unchanged)
4. ❌ Add new composition functions (seq/if/loop already exist)

**Rationale**: Phase 280 is about SSOT positioning, NOT full migration. 行動は最小が正解.

---

## SSOT References

### Primary Documentation

1. **Frag Composition SSOT**: [`docs/development/current/main/design/edgecfg-fragments.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/edgecfg-fragments.md)
   - Status: **Active SSOT** (updated Phase 280 A1)
   - 5 new sections: Composition SSOT, Rules, Laws, Fail-Fast, Ownership

2. **JoinIR Architecture Overview**: [`docs/development/current/main/joinir-architecture-overview.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/joinir-architecture-overview.md)
   - Section 0.2: **Numbered Route Absorption Destination (Phase 280)** (updated Phase 280 A2)
   - Comparison: JoinIR vs Plan (different extraction, same SSOT)

3. **Phase 280 Plan**: [`/home/tomoaki/.claude/plans/elegant-wondering-stroustrup.md`](/home/tomoaki/.claude/plans/elegant-wondering-stroustrup.md)
   - Full implementation plan with A→B→C breakdown

### Related Documentation

- **Phase 273 (Plan Line SSOT)**: ScanWithInit / SplitScan Plan-based routing completed
- **Phase 264 (歴史/別案件)**: BundleResolver loop fix (separate scope)
- **Phase 265-268**: Frag composition API creation and terminator SSOT

---

## Three-Phase Approach (A→B→C)

### Phase A: Design SSOT Solidification (Docs-Only) ✅ **COMPLETE**

**No code changes - documentation only**

#### A1: Update edgecfg-fragments.md to "Composition SSOT" ✅

**Changes Made**:
- Header: `Status: Draft` → `Status: Active SSOT`
- Added 5 new sections (after line 103):
  1. **Composition SSOT (Phase 280)**: Why it's SSOT, input/output contract
  2. **Composition Rules**: seq/if/loop/cleanup with composition laws
  3. **Composition Laws (Invariants)**: Wires/Exits Separation, Terminator Uniqueness
  4. **Fail-Fast Invariants**: Two-level verification (verify_frag_invariants vs strict)
  5. **Ownership (Who Allocates What)**: 3-tier model (Normalizer/Composition/Lowerer)
- Updated "実装入口" section: Phase 280 entry added, Phase 264 separated as history

**File**: [`docs/development/current/main/design/edgecfg-fragments.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/edgecfg-fragments.md)

#### A2: Update joinir-architecture-overview.md with "Numbered Route Absorption Destination" ✅

**Changes Made**:
- Added section 0.2: **Pattern Number Absorption Destination (Phase 280)**
- Content:
  - Problem: numbered route label proliferation
  - Solution: Frag Composition SSOT
  - JoinIR vs Plan comparison table (different extraction, same SSOT)
  - Route absorption status table (ScanWithInit / SplitScan highlighted as Phase 280 targets)
  - Absorption timeline (Phase 280-283+)

**File**: [`docs/development/current/main/joinir-architecture-overview.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/joinir-architecture-overview.md)

#### A3: Create Phase 280 README ✅

**This file** - Full roadmap documentation with:
- Purpose / Non-Goals
- SSOT References
- Three-Phase Approach breakdown
- Execution Order
- Acceptance Criteria
- Risks and Mitigation
- Critical Files

---

### Phase B: Frag Combiner Minimal API Design

**Minimal code changes - documentation + tests only**

#### B1: Document compose.rs Entry Points

**File**: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`

**Changes Required**:
1. Add module-level documentation explaining:
   - Composition SSOT (Phase 280)
   - Entry points: seq(), if_(), loop_(), cleanup()
   - Contract: Input/Output Frag, No Allocation, Pure CFG Transform
   - Usage example

2. Add function-level "Phase 280: Composition SSOT" sections documenting:
   - Constraints (caller allocates X, composition wires Y)
   - Composition laws (input → output transformation)

**Acceptance**:
- [ ] Module-level docs added
- [ ] Each function has "Phase 280" constraint section
- [ ] Composition laws documented

#### B2: Verify Composition API Implements Contract

**File**: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`

**Action**: Documentation-only verification (checklist)

**Verification Checklist**:

**seq() Contract** ✓:
- [x] a.Normal → b.entry (wires): Line 42-51
- [x] Non-Normal propagate: Line 54-58
- [x] Tests exist

**if_() Contract** ✓:
- [x] header → t.entry/e.entry (BranchStub): Line 114-122
- [x] t/e.Normal → join (wires): Line 131-169
- [x] Tests exist

**loop_() Contract** ✓:
- [x] Continue → header (wires): Line 224-233
- [x] Break → after (wires): Line 236-245
- [x] Tests exist

**cleanup() Contract** ⏸:
- [ ] TODO placeholder (Phase 280+ scope)

**Acceptance**:
- [ ] Checklist complete (all seq/if/loop verified)
- [ ] No missing contract implementations discovered

#### B3: Add Tests for Composition Invariants

**File**: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs` (test module)

**Action**: Gap analysis + add tests if needed

**Current Coverage**: 13 tests exist (seq: 2, if: 2, loop: 5, emit: 1, basic: 3)

**Gap Analysis**:
1. **Output Determinism** (BTreeMap ordering)
   - Check: Verify exits/wires maintain ordering
   - Status: Implicit (BTreeMap used, not explicitly tested)
   - Action: Add test if gap confirmed

2. **Wires/Exits Separation**
   - Check: Verify wires target=Some, exits target=None
   - Status: Partially covered
   - Action: Add explicit test if gap confirmed

**Decision**: Check if gaps exist, add tests **only if** gaps confirmed

**Acceptance**:
- [ ] Gap analysis complete
- [ ] Tests added if gaps found
- [ ] `cargo test --lib --release` PASS

---

### Phase C: Prepare ScanWithInit / SplitScan to Use Composition API

**Behavior-preserving refactor OR documentation**

**✅ USER DECISION: Option 2 (Documentation-Only) - Recommended**

**Rationale**:
- Phase 280 goal is **SSOT positioning + 導線固定** (NOT full migration)
- ScanWithInit route: early-return doesn't naturally fit `compose::if_()` model
- SplitScan route: 挙動不変保証が難しい、得られる差分が小さい
- **行動は最小が正解** - Full migration deferred to Phase 281

#### C1: Identify Hand-Rolled Frag Construction Locations

**File**: `src/mir/builder/control_flow/plan/normalizer.rs`

**ScanWithInit route (historical label: Pattern6)**:
- **Function**: `normalize_scan_with_init()`
- **識別コメント**: Search for `// Step 12: Build CoreLoopPlan` or `let branches = vec![...]` near `BranchStub { from: header_bb, cond: cond_loop`
- **Structure**: 5 blocks (preheader, header, body, step, found, after)
- **Hand-rolled**: 2 BranchStub + 2 EdgeStub
- **Composition opportunity**: **Hand-rolled clearer** (early exit breaks if_ model)

**SplitScan route (historical label: Pattern7)**:
- **Function**: `normalize_split_scan()`
- **識別コメント**: Search for `// Build Frag with branches and wires` or `let branches = vec![...]` near `BranchStub { from: header_bb, cond: cond_loop` in split context
- **Structure**: 6 blocks (preheader, header, body, then, else, step, after)
- **Hand-rolled**: 2 BranchStub + 3 EdgeStub
- **Composition opportunity**: **Defer to Phase 281** (挙動不変保証が難しい)

**Acceptance**:
- [ ] ScanWithInit location identified (function name + 識別コメント)
- [ ] SplitScan location identified (function name + 識別コメント)
- [ ] Composition opportunities assessed (both defer to Phase 281)

#### C2: Document Hand-Rolled Construction (Defer Migration to Phase 281)

**Scope**: Add TODO comments showing future refactor path

**Example Documentation** (Option 2 - DEFAULT):

**ScanWithInit route** (`normalize_scan_with_init()`, historical label: Pattern6):
```rust
// Phase 280 TODO: Hand-rolled Frag construction for early exit route
// Reason: `found` is early Return, doesn't fit compose::if_() model
// Future: Consider compose::cleanup() for early exit normalization (Phase 281+)
```

**SplitScan route** (`normalize_split_scan()`, historical label: Pattern7):
```rust
// Phase 280 TODO: Hand-rolled Frag construction for split scan pattern
// Target (Phase 281): compose::if_(body_bb, cond_match, then_frag, else_frag, step_frag)
// Reason deferred: 挙動不変保証が難しい、Phase 280 は SSOT positioning 優先
// Migration: Phase 281+ で compose::if_() への移行を検討
```

**Implementation**:
1. Search for hand-rolled locations using function name + 識別コメント (from C1)
2. Add TODO comments above hand-rolled construction
3. Document: (a) current structure, (b) future compose target, (c) defer reason

**Acceptance**:
- [ ] TODO comments added to both ScanWithInit and SplitScan
- [ ] Comments document: current structure + future target + defer reason
- [ ] No behavior change (documentation-only)

#### C3: Verify No Behavior Change (Smoke Tests - Optional)

**Status**: Optional (Phase C is documentation-only, no code changes)

**Test Strategy**: If desired, run representative smokes to verify baseline

**ScanWithInit smoke (Optional)**:
```bash
bash tools/smokes/v2/profiles/integration/apps/archive/phase258_p0_index_of_string_llvm_exe.sh
```

**SplitScan smoke (Optional)**:
```bash
# Find SplitScan smokes
tools/smokes/v2/run.sh --profile integration --filter "*split*"
```

**Quick Profile (Optional)**:
```bash
tools/smokes/v2/run.sh --profile quick
# Expected: 45/46 PASS (baseline maintained)
```

**Acceptance**:
- [ ] Smoke tests run if desired (optional - docs-only change)
- [ ] Behavior unchanged (guaranteed - no code modifications in Phase C)

---

## Execution Order (Critical!)

**MUST execute in strict A→B→C order**:

1. **Phase A** (Docs-only, no code) ✅ **COMPLETE**:
   - A1: Update edgecfg-fragments.md ✅
   - A2: Update joinir-architecture-overview.md ✅
   - A3: Create phase-280/README.md ✅
   - **Verify**: No code changes made ✅

2. **Phase B** (API solidification):
   - B1: Add compose.rs module docs
   - B2: Verify contract checklist
   - B3: Add missing tests if needed
   - **Verify**: `cargo test --lib --release` PASS

3. **Phase C** (route-family preparation - Documentation-only):
   - C1: Identify hand-rolled locations (function name + 識別コメント)
   - C2: Add TODO comments (Option 2 - default, defer migration to Phase 281)
   - C3: Run smoke tests (optional - no code change, behavior guaranteed)
   - **Verify**: No regression (docs-only, no behavior change)

4. **Final Verification**:
   - All acceptance criteria met
   - Smoke tests PASS
   - No regression

---

## Acceptance Criteria

### Phase A (Docs) ✅ **COMPLETE**

- [x] edgecfg-fragments.md updated (5 sections, Active SSOT)
- [x] joinir-architecture-overview.md updated (absorption section + table)
- [x] phase-280/README.md created
- [x] All docs cross-reference each other
- [x] No code changes

### Phase B (API) ✓ When all checked:

- [ ] compose.rs module-level docs added
- [ ] Function constraints documented
- [ ] Composition contract verified
- [ ] Missing tests added if gaps found
- [ ] `cargo test --lib --release` PASS

### Phase C (route-family prep) ✓ When all checked:

- [ ] Hand-rolled locations identified (function name + 識別コメント)
- [ ] Documentation-only (Option 2) executed (TODO comments added)
- [ ] Smoke tests PASS (optional - no code change expected)
- [ ] Behavior unchanged (guaranteed - docs-only)

### Overall ✓ When all checked:

- [ ] No regression (all tests/smokes PASS)
- [ ] SSOT positioning clear
- [ ] ScanWithInit / SplitScan prepared for Phase 281

---

## Risks and Mitigation

### Risk 1: Composition API Doesn't Match Hand-Rolled Exactly

- **Impact**: Behavior change, regression
- **Likelihood**: Medium
- **Mitigation**: Phase A (docs-only) first, Phase C Option 2 (document) is fallback
- **Phase 280 Decision**: Documentation-only (Option 2), defer refactor to Phase 281

### Risk 2: ScanWithInit / SplitScan Have Hidden Dependencies

- **Impact**: Refactor breaks edge cases
- **Likelihood**: Low (well-tested)
- **Mitigation**: C3 runs both route-family-specific and quick profile smokes
- **Phase 280 Decision**: No refactor, only documentation

### Risk 3: Smoke Tests Miss Edge Cases

- **Impact**: Regression not caught
- **Likelihood**: Low (45+ tests in quick profile)
- **Mitigation**: Run both VM and LLVM backends if needed
- **Phase 280 Status**: Optional (docs-only change)

---

## Critical Files

### Phase A (Documentation) ✅

1. [`docs/development/current/main/design/edgecfg-fragments.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/edgecfg-fragments.md) ✅
2. [`docs/development/current/main/joinir-architecture-overview.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/joinir-architecture-overview.md) ✅
3. [`docs/development/current/main/phases/phase-280/README.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-280/README.md) ✅ (this file)

### Phase B (Code: API)

4. `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`

### Phase C (Code: route-family prep - Documentation-only)

5. `src/mir/builder/control_flow/plan/normalizer.rs`
   - ScanWithInit: `normalize_scan_with_init()` function
   - SplitScan: `normalize_split_scan()` function
   - Action: Add TODO comments (no code changes)

---

## Related Phases

### Predecessor Phases

- **Phase 273 (P0-P4)**: Plan line SSOT for ScanWithInit / SplitScan (DomainPlan → CorePlan → Frag → emit_frag)
- **Phase 265-268**: Frag composition API creation, terminator SSOT (emit_frag)
- **Phase 264**: BundleResolver loop fix (歴史/別案件, separate scope)

### Successor Phases (Planned)

- **Phase 281**: Full ScanWithInit / SplitScan absorption (replace hand-rolled with compose_*)
- **Phase 282**: Router shrinkage (pattern numbers → test labels)
- **Phase 283+**: BoolPredicateScan and beyond (historical numbered label lane continues there)

---

## Status Summary

**Phase A**: ✅ **COMPLETE** (2025-12-23)
- A1: edgecfg-fragments.md updated ✅
- A2: joinir-architecture-overview.md updated ✅
- A3: phase-280/README.md created ✅

**Phase B**: ⏳ **In Progress**
- B1: compose.rs module docs (pending)
- B2: Verify composition contract (pending)
- B3: Add missing tests if needed (pending)

**Phase C**: ⏸ **Pending**
- C1: Identify hand-rolled locations (pending)
- C2: Add TODO comments (pending)
- C3: Run smoke tests (optional)

**Overall**: 33% complete (Phase A done, Phases B+C remaining)

---

**End of Phase 280 README**
