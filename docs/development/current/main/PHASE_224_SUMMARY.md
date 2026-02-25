# Phase 224: A-4 DigitPos Promoter - Implementation Summary

**Date**: 2025-12-10
**Status**: Core Implementation Complete（Phase 224-D まで反映済み、init MethodCall は別 Phase）
**Branch**: main
**Commits**: TBD

---

## Executive Summary

Phase 224 successfully implemented the **DigitPosPromoter** Box for A-4 pattern (cascading indexOf) promotion, achieving:

✅ **Complete**: DigitPosPromoter implementation with full unit test coverage (6/6 tests passing)
✅ **Complete**: Integration into LoopBodyCondPromoter orchestrator
✅ **Complete**: Two-tier promotion strategy (A-3 Trim → A-4 DigitPos fallback)
✅ **Verified**: Promotion detection working correctly in Pattern2/4 pipeline
✅ **Complete** (Phase 224-D): ConditionEnv alias bridge（`digit_pos` → `is_digit_pos`）実装
⚠️ **Partial**: Full E2E flowは body-local init の MethodCall 制約で一部ブロック中

---

## Accomplishments

### 1. Design Document (224-2) ✅

**File**: `docs/development/current/main/phase224-digitpos-promoter-design.md`

**Key Design Decisions**:
- **One Box, One Question**: DigitPosPromoter handles ONLY A-4 pattern (indexOf-based)
- **Separation of Concerns**: Trim patterns remain in LoopBodyCarrierPromoter
- **Orchestrator Pattern**: LoopBodyCondPromoter delegates to specialized promoters
- **Bool Carrier**: Promote to `is_digit_pos` (bool) for consistency with A-3 Trim

### 2. DigitPosPromoter Implementation (224-3) ✅

**File**: `src/mir/loop_pattern_detection/loop_body_digitpos_promoter.rs` (467 lines)

**Features**:
- **Pattern Detection**: Identifies cascading substring() → indexOf() → comparison
- **Comparison Operators**: Supports `<`, `>`, `<=`, `>=`, `!=` (not equality)
- **Dependency Validation**: Verifies indexOf() depends on another LoopBodyLocal
- **Comprehensive Tests**: 6 unit tests covering normal/edge cases

**Test Results**:
```bash
cargo test --release --lib digitpos
# running 6 tests
# test result: ok. 6 passed; 0 failed; 0 ignored
```

### 3. LoopBodyCondPromoter Integration (224-4) ✅

**File**: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs`

**Two-Tier Strategy**:
```
Step 1: Try A-3 Trim promotion (LoopBodyCarrierPromoter)
        ↓ (if fails)
Step 2: Try A-4 DigitPos promotion (DigitPosPromoter)
        ↓ (if fails)
Step 3: Fail-Fast with clear error message
```

**Logs Verify Success（昇格フェーズ）**:
```
[cond_promoter] A-3 Trim promotion failed: No promotable Trim pattern detected
[cond_promoter] Trying A-4 DigitPos promotion...
[digitpos_promoter] Phase 224: Found 1 LoopBodyLocal variables: ["digit_pos"]
[digitpos_promoter] A-4 DigitPos pattern promoted: digit_pos → is_digit_pos
[cond_promoter] A-4 DigitPos pattern promoted: 'digit_pos' → carrier 'is_digit_pos'
```

---

## Current Limitation: Lowerer Integration Gap

### Problem Statement（当初） / Phase 224-D での一部解消

**Symptom（224 実装直後）**: E2E test fails despite successful promotion  
**Root Cause**: `lower_loop_with_break_minimal` performs independent LoopBodyLocal check  
**Result**: Promoted variables are detected as "unsupported" by the lowerer

### Error Flow

```
Phase 223.5 (Pattern2) → LoopBodyCondPromoter.try_promote() → SUCCESS ✅
                       ↓
Phase 180-3 (Pattern2) → TrimLoopLowerer.try_lower() → SKIP (not Trim)
                       ↓
Pattern2 Lowerer → lower_loop_with_break_minimal() → Analyze break condition
                 ↓
LoopConditionScopeBox.analyze() → Detects "digit_pos" as LoopBodyLocal
                 ↓
ERROR ❌: "Unsupported condition: uses loop-body-local variables: [\"digit_pos\"]"
```

### Why A-3 Trim Patterns Work

For A-3 Trim patterns, TrimLoopLowerer **rewrites the break condition** to remove LoopBodyLocal references before passing to lower_loop_with_break_minimal:

```rust
// TrimLoopLowerer returns trim_result.condition (rewritten)
let effective_break_condition = trim_result.condition;  // No LoopBodyLocal!
```

But for A-4 DigitPos (Phase 223.5), we:
- Successfully promote to carrier: `digit_pos` → `is_digit_pos` ✅
- Merge carrier into CarrierInfo ✅
- **BUT**: Break condition AST still contains `digit_pos` ❌

### Root Cause Analysis（Phase 224 時点）

The break condition is an **AST node** containing:
```nyash
if digit_pos < 0 { break }
```

After promotion（224 時点）:
- CarrierInfo knows about `is_digit_pos` carrier ✅
- LoopBodyCondPromoter recorded the promotion ✅
- **But**: AST node still says `digit_pos`, not `is_digit_pos` → ConditionEnv から `digit_pos` が見えない ❌

Phase 224-D では AST を直接書き換えるのではなく、
**ConditionAlias（old_name → carrier_name）を CarrierInfo/ConditionEnv に導入する**ことで
「`digit_pos` という名前で条件式から参照された場合も、内部的には `is_digit_pos` carrier を読む」
というブリッジを追加している。

これにより：
- LoopBodyLocal 昇格後に `digit_pos < 0` のような条件があっても、
  ConditionEnvBuilder が ConditionAlias を介して `is_digit_pos` の ValueId に解決できるようになった。
- 「LoopBodyLocal が条件にあることによる not‑bound エラー」は解消され、
  **現時点の Blocker は body‑local init MethodCall（substring など）の lowering 制約だけ**になった。

---

## Solution Options (Phase 224-continuation 時点の整理)

### Option A: AST Rewriting (Comprehensive)

**Approach**: Rewrite break condition AST to replace promoted variables with carrier references

**Implementation**:
1. After LoopBodyCondPromoter.try_promote() succeeds
2. Create ASTRewriter to traverse break_condition_node
3. Replace Variable("digit_pos") → Variable("is_digit_pos")
4. Pass rewritten condition to lower_loop_with_break_minimal

**Pros**: Clean, consistent with Trim pattern flow
**Cons**: AST rewriting is complex, error-prone
**Effort**: ~2-3 hours

### Option B: Promoted Variable Tracking + ConditionAlias（採用済み）

**Approach（決定案）**:  
LoopBodyLocal を carrier に昇格した事実を **CarrierInfo（promoted_loopbodylocals + ConditionAlias）** に
メタデータとして記録し、ConditionEnvBuilder 側で「元の変数名 → carrier 名」のエイリアスを解決する。

実装（Phase 224-cont / 224-D）:
1. `CarrierInfo` に `promoted_loopbodylocals: Vec<String>` と `condition_aliases: Vec<ConditionAlias>` を追加。
2. LoopBodyCondPromoter（Trim/DigitPos の両方）で昇格成功時に
   `promoted_loopbodylocals.push("digit_pos")` と  
   `condition_aliases.push(ConditionAlias { old_name: "digit_pos", carrier_name: "is_digit_pos" })` を記録。
3. Pattern2/4 lowerer は ConditionEnvBuilder v2 を呼ぶ際に `&carrier_info.condition_aliases` を渡す。
4. ConditionEnvBuilder は `var_name == "digit_pos"` のような未解決変数を見つけた場合、
   ConditionAlias を使って `carrier_name == "is_digit_pos"` に解決し、その ValueId を Condition 役の Param としてバインド。

効果:
- LoopBodyLocal 条件パターン（A-3 Trim/A-4 DigitPos）は、
  **AST 書き換えなしで「条件式から見える名前」と「carrier 実体」を橋渡しできる**。
- Pattern2/4 や LoopConditionScopeBox は「昇格済み LoopBodyLocal」とそれ以外を区別できるようになり、
  不要な Fail‑Fast を避けつつ、未昇格の LoopBodyLocal には引き続き厳格に対応できる。

### Option C: DigitPosLoopHelper Metadata (Consistent)

**Approach**: Create DigitPosLoopHelper similar to TrimLoopHelper

**Implementation**:
1. Create `loop_body_digitpos_helper.rs` (similar to trim_loop_helper.rs)
2. Attach DigitPosLoopHelper to CarrierInfo in DigitPosPromoter
3. In Phase 223.5, check for digitpos_helper (like trim_helper check at line 321)
4. In lower_loop_with_break_minimal, check CarrierInfo for helpers before error

**Pros**: Consistent with Trim pattern architecture
**Cons**: More boilerplate
**Effort**: ~2-3 hours

---

## Recommended Next Steps

### Immediate (P0 - Phase 224 Completion)

**Goal**: Unblock `phase2235_p2_digit_pos_min.hako` test

**Approach**: Implement **Option B** (Promoted Variable Tracking)

**Tasks**:
1. Add `promoted_loopbodylocals` field to CarrierInfo
2. Record promoted_var in Phase 223.5 (pattern2_with_break.rs:303)
3. Pass promoted list to lower_loop_with_break_minimal
4. Modify LoopConditionScopeBox to exclude promoted vars
5. Verify E2E test passes

**Estimated Time**: 1-2 hours
**Risk**: Low (surgical change)

### Short-term (P1 - Phase 225)

**Goal**: Extend to Pattern4 (with continue)

**Tasks**:
1. Apply same promoted variable tracking to Pattern4
2. Test with `parser_box_minimal.hako` (skip_ws pattern)
3. Verify no regression in existing Trim patterns

**Estimated Time**: 1 hour
**Risk**: Low (reuse Option B infrastructure)

### Medium-term (P2 - Phase 226)

**Goal**: A-5/A-6 patterns (multi-variable, cascading)

**Tasks**:
1. Extend DigitPosPromoter to handle multiple indexOf() calls
2. Support range check patterns (A-6: `ch < "0" || ch > "9"`)
3. Add multi-variable promotion support

**Estimated Time**: 3-4 hours
**Risk**: Medium (more complex patterns)

---

## Files Modified

### New Files (3)
- `docs/development/current/main/phase224-digitpos-promoter-design.md` (design doc)
- `src/mir/loop_pattern_detection/loop_body_digitpos_promoter.rs` (promoter implementation)
- `docs/development/current/main/PHASE_224_SUMMARY.md` (this file)

### Modified Files (2)
- `src/mir/loop_pattern_detection/mod.rs` (add digitpos_promoter module)
- `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs` (two-tier integration)

### Test Coverage
- Unit tests: 6/6 passing (100%)
- E2E test: 0/1 passing (blocked by lowerer integration)

---

## Build & Test Status

### Build
```bash
cargo build --release
# Finished `release` profile [optimized] target(s) in 1m 11s
# 7 warnings (snake_case naming, visibility)
```

### Unit Tests
```bash
cargo test --release --lib digitpos
# running 6 tests
# test result: ok. 6 passed; 0 failed; 0 ignored
```

### E2E Test (Current State)
```bash
./target/release/hakorune apps/tests/phase2235_p2_digit_pos_min.hako
# [digitpos_promoter] A-4 DigitPos pattern promoted: digit_pos → is_digit_pos ✅
# [cond_promoter] A-4 DigitPos pattern promoted ✅
# ERROR: Unsupported condition: uses loop-body-local variables: ["digit_pos"] ❌
```

---

## Technical Debt & Future Work

### Code Quality Improvements
1. Fix snake_case warnings in digitpos_promoter.rs (is_indexOf → is_index_of)
2. Add LoopScopeShape visibility annotation (pub(crate) or pub)
3. Extract common AST traversal logic (find_definition_in_body) to shared module

### Documentation
1. Add DigitPosPromoter to `joinir-architecture-overview.md` box catalog
2. Update `phase223-loopbodylocal-condition-inventory.md` with Phase 224 status
3. Create integration guide for future pattern promoters

### Testing
1. Add integration tests for Pattern2+DigitPos combination
2. Add regression tests for Trim patterns (ensure no impact)
3. Add performance benchmarks for promotion detection

---

## Appendix: Key Design Insights

### Why Two Promoters Instead of One?

**Question**: Why not extend LoopBodyCarrierPromoter to handle A-4 patterns?

**Answer**: **Separation of Concerns**
- LoopBodyCarrierPromoter: Equality-based patterns (A-3 Trim)
  - `ch == " " || ch == "\t"` (OR chain)
  - Single LoopBodyLocal
  - Well-tested, stable

- DigitPosPromoter: Comparison-based patterns (A-4 DigitPos)
  - `digit_pos < 0` (comparison)
  - Cascading dependencies (ch → digit_pos)
  - New, experimental

**Box-First Principle**: "One Box = One Question"

### Why Bool Carrier Instead of Int?

**Question**: Why not preserve `digit_pos` as int carrier?

**Answer**: **Consistency with Existing Architecture**
- A-3 Trim patterns use bool carriers (`is_whitespace`)
- Pattern2/Pattern4 lowerers expect bool carriers in conditions
- Bool carrier simplifies condition rewriting (just a flag)

**Future**: Int carrier variant can be added for downstream use (Phase 226+)

### Why Not Rewrite AST Immediately?

**Question**: Why defer AST rewriting to Phase 224-continuation?

**Answer**: **Incremental Development**
- AST rewriting is complex and error-prone
- Surgical fix (Option B) unblocks test faster
- Learn from A-3 Trim pattern experience first
- Can refactor to AST rewriting later if needed

---

## Conclusion

Phase 224 successfully implemented the **core promotion logic** for A-4 DigitPos patterns, achieving:

- ✅ Comprehensive design document
- ✅ Robust promoter implementation with full test coverage
- ✅ Clean integration into orchestrator pattern
- ✅ Verified promotion detection in Pattern2 pipeline

**Remaining Work**: Lowerer integration (1-2 hours, Option B approach)

**Next Session**: Implement Option B (promoted variable tracking) to complete Phase 224 and unblock `phase2235_p2_digit_pos_min.hako` test.

---

## References

- [Phase 223 Inventory](phase223-loopbodylocal-condition-inventory.md) - A-4 pattern specification
- [Phase 224 Design](phase224-digitpos-promoter-design.md) - Detailed design document
- [JoinIR Architecture](joinir-architecture-overview.md) - Overall system architecture
- [Test File](../../../apps/tests/phase2235_p2_digit_pos_min.hako) - Minimal A-4 test case
