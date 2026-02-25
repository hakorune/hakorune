# Phase 223: LoopBodyLocal Condition Promotion - Summary

---
**Phase 26-45 Completion**: このフェーズで設計した機能は Phase 43/245B で実装完了。最終状態は [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md) を参照。
---

## Overview

Phase 223 addresses the "LoopBodyLocal in condition" constraint that blocks JsonParser loops (discovered in Phase 221). This phase enables Pattern2/Pattern4 to handle loops where loop-body-local variables appear in break/continue conditions.

---

## Phase Breakdown

### Phase 223-1: Comprehensive Inventory ✅ COMPLETE

**Deliverable**: `/docs/development/current/main/phase223-loopbodylocal-condition-inventory.md`

**Key Findings**:
- **Category A** (Safe for Promotion): 6 patterns
  - A-1/A-2: Trim leading/trailing (✅ already handled by TrimLoopHelper)
  - **A-3: Skip whitespace (Pattern 4)** - **P0 target** ⚠️ needs Pattern4 support
  - A-4: Digit detection (cascading LoopBodyLocal) - P1 candidate
  - A-5: String comparison (multi-variable) - P2 candidate
  - A-6: atoi range check (complex) - P2 candidate
- **Category B** (Fail-Fast Maintained): 1 pattern (complex nested conditions)
- **Category C** (Body-Only): 2 patterns (not blocked, already working)

**Priority Classification**:
- **P0**: Category A-3 (skip_whitespace) - critical for JsonParser
- **P1**: Category A-4 (cascading LoopBodyLocal) - high priority
- **P2**: Category A-5/A-6 (multi-variable, complex) - future extension

---

### Phase 223-2: API-Level Design ✅ COMPLETE

**Deliverable**: `/docs/development/current/main/phase223-loopbodylocal-condition-design.md`

**Key Design Decisions**:

1. **New Box: LoopBodyCondPromoter** (thin coordinator)
   - **Location**: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs` (to be created in Phase 223-3)
   - **Role**: Unified API for Pattern2/Pattern4 condition promotion
   - **Delegation**: Reuses existing `LoopBodyCarrierPromoter` for detection logic
   - **Output**: Metadata only (no code generation)

2. **API Signature**:
   ```rust
   pub struct ConditionPromotionRequest<'a> {
       pub loop_param_name: &'a str,
       pub cond_scope: &'a LoopConditionScope,
       pub scope_shape: Option<&'a LoopScopeShape>,
       pub break_cond: Option<&'a ASTNode>,
       pub continue_cond: Option<&'a ASTNode>,
       pub loop_body: &'a [ASTNode],
   }

   pub enum ConditionPromotionResult {
       Promoted { carrier_info, promoted_var, carrier_name },
       CannotPromote { reason, vars },
   }

   impl LoopBodyCondPromoter {
       pub fn try_promote_for_condition(req: ConditionPromotionRequest)
           -> ConditionPromotionResult;
   }
   ```

3. **Pattern4 Integration Strategy**:
   - **Current**: Immediate Fail-Fast when `has_loop_body_local() == true`
   - **Future** (Phase 223-3):
     ```rust
     if loop_cond_scope.has_loop_body_local() {
         match LoopBodyCondPromoter::try_promote_for_condition(req) {
             Promoted { carrier_info, .. } => {
                 // Merge carrier, continue Pattern4 lowering
             }
             CannotPromote { .. } => {
                 // Fail-Fast (same as current)
             }
         }
     }
     ```

4. **P0 Constraints** (strict):
   - Single LoopBodyLocal variable only (e.g., `ch`)
   - Must match existing Trim pattern (substring + equality chain)
   - No cascading dependencies (A-4: `ch` + `digit_pos` → rejected)
   - No multi-variable patterns (A-5: `ch_s` + `ch_lit` → rejected)

5. **Box Role Matrix**:

| Box | Detection | Metadata | Code Gen | Integration |
|-----|-----------|----------|----------|-------------|
| **LoopConditionScopeBox** | ✅ Classify vars | ❌ | ❌ | Pattern2/4 |
| **LoopBodyCarrierPromoter** | ✅ Trim pattern | ✅ TrimPatternInfo | ❌ | TrimLoopLowerer |
| **TrimLoopLowerer** | ❌ (delegates) | ✅ CarrierInfo | ✅ MIR emission | Pattern2 only |
| **LoopBodyCondPromoter** (新) | ❌ (delegates) | ✅ CarrierInfo | ❌ | **Pattern2/4** |

**Design Principle**: Single Responsibility
- **Detection**: LoopBodyCarrierPromoter
- **Metadata**: TrimPatternInfo, CarrierInfo
- **Code Generation**: Pattern-specific lowerers (TrimLoopLowerer for P2, Pattern4 lowerer for P4)
- **Coordination**: LoopBodyCondPromoter (thin wrapper)

---

### Phase 223-3: Implementation (PLANNED)

**Estimated Deliverables**:

1. **LoopBodyCondPromoter implementation** (~50-80 lines)
   - Extract promotion logic from TrimLoopLowerer
   - Add P0 constraint checking (single var, simple pattern)
   - Delegate to LoopBodyCarrierPromoter

2. **Pattern4 integration** (+30-40 lines)
   - Call LoopBodyCondPromoter before Fail-Fast
   - Merge promoted carrier into existing CarrierInfo
   - Continue with Pattern4 lowering if promotion succeeds

3. **Unit tests** (5-7 test cases)
   - P0 promotion success (Category A-3: skip_whitespace)
   - Cascading Fail-Fast (Category A-4: ch + digit_pos)
   - Complex pattern Fail-Fast (Category B-1: nested if)
   - Multi-variable Fail-Fast (Category A-5: ch_s + ch_lit)

4. **E2E test** (1 file)
   - `apps/tests/phase223_p4_skip_whitespace.hako`
   - Pattern: _skip_whitespace with continue
   - Expected: Pattern4 lowering + MIR execution success

**Total Estimated Size**: +150-200 lines (net)

---

## Documentation Updates

### CURRENT_TASK.md ✅ UPDATED

Added Phase 223 summary:
```
- **Phase 223 進行中**: LoopBodyLocal Condition Promotion（条件昇格システム）
  - **Phase 223-1 完了** ✅: 包括的棚卸（Category A: 6 patterns, Category B: 1, Category C: 2）
  - **Phase 223-2 完了** ✅: API レベル設計（LoopBodyCondPromoter Box, P0: Pattern4/_skip_whitespace 向け）
  - **Phase 223-3 予定**: 実装（LoopBodyCondPromoter 抽出, Pattern4 統合, E2E テスト）
```

### joinir-architecture-overview.md ✅ UPDATED

Added LoopBodyCondPromoter to Section 2.2 (条件式ライン):
```
- **LoopBodyCondPromoter（Phase 223-2 設計完了）**
  - ファイル: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs`（Phase 223-3 で実装予定）
  - 責務: ループ条件に出てくる LoopBodyLocal を carrier に昇格する統一 API
  - 設計原則: Thin coordinator, Pattern-agnostic, Fail-Fast
  - 入出力: ConditionPromotionRequest → ConditionPromotionResult
  - 使用元: Pattern4 (promotion-first), Pattern2 (TrimLoopLowerer 経由)
```

---

## Impact Analysis

### Immediate Impact (P0 - Phase 223-3)

**Unblocks**:
- `apps/tests/parser_box_minimal.hako` (skip_ws method)
- `tools/hako_shared/json_parser.hako` (_skip_whitespace)

**Pattern Coverage**:
- Category A-1/A-2: Already working (TrimLoopHelper)
- **Category A-3: WILL WORK** (Phase 223-3 target) ✨
- Category A-4/A-5/A-6: Still blocked (P1/P2 future work)

**JsonParser Loop Coverage**:
- Before Phase 223: 7/13 loops (54%)
- After Phase 223-3 (P0): **8/13 loops (62%)** (+1 loop: _skip_whitespace Pattern4 variant)

---

### Future Extensions (P1/P2)

**P1: Cascading LoopBodyLocal** (Category A-4)
- Pattern: `local ch = ...; local digit_pos = digits.indexOf(ch); if digit_pos < 0 { break }`
- Solution: Promote only leaf variable (`digit_pos` → `is_digit`)
- Requires: Dependency analysis in LoopBodyCarrierPromoter

**P2: Multi-Variable Patterns** (Category A-5, A-6)
- Pattern: `local ch_s = ...; local ch_lit = ...; if ch_s != ch_lit { break }`
- Solution: Promote to single carrier (`chars_match`)
- Requires: Multi-variable carrier initialization

**Non-Goals**:
- Category B (Complex patterns): Continue to Fail-Fast
- Nested if with reassignment
- Method call chains

---

## Test Strategy

### Phase 223-3 Tests

1. **Unit Tests** (5-7 cases):
   - `test_p0_skip_whitespace_promotion()`: Category A-3 success
   - `test_cascading_fail_fast()`: Category A-4 rejection (P0 constraint)
   - `test_multi_variable_fail_fast()`: Category A-5 rejection
   - `test_complex_pattern_fail_fast()`: Category B-1 rejection
   - `test_pattern2_integration()`: Verify existing TrimLoopLowerer still works

2. **E2E Tests** (1 file):
   - `apps/tests/phase223_p4_skip_whitespace.hako`
   - Expected output: Correct whitespace skipping behavior
   - Verification: MIR execution returns expected result

3. **Regression Tests**:
   - All Phase 220-222 tests must continue to pass
   - Existing TrimLoopHelper tests (Phase 171-176) must pass

---

## Success Criteria

### Phase 223-2 ✅ COMPLETE

- [x] Design document created (`phase223-loopbodylocal-condition-design.md`)
- [x] API types defined (ConditionPromotionRequest/Result)
- [x] Box roles clarified (LoopBodyCondPromoter vs existing boxes)
- [x] Pattern4 integration strategy documented
- [x] P0 constraints specified (single var, Trim pattern only)
- [x] CURRENT_TASK.md updated
- [x] joinir-architecture-overview.md updated

### Phase 223-3 (PLANNED)

- [ ] LoopBodyCondPromoter implementation (~50-80 lines)
- [ ] Pattern4 integration (+30-40 lines)
- [ ] Unit tests (5-7 cases) all passing
- [ ] E2E test passing (phase223_p4_skip_whitespace.hako)
- [ ] No regressions in Phase 220-222 tests
- [ ] TrimLoopHelper tests continue to pass

---

## Files Modified/Created

### Phase 223-1
- Created: `docs/development/current/main/phase223-loopbodylocal-condition-inventory.md`

### Phase 223-2
- Created: `docs/development/current/main/phase223-loopbodylocal-condition-design.md`
- Updated: `CURRENT_TASK.md`
- Updated: `docs/development/current/main/joinir-architecture-overview.md`
- Created: `docs/development/current/main/PHASE_223_SUMMARY.md` (this file)

### Phase 223-3 (Planned)
- Create: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs` (~80 lines)
- Modify: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` (+40 lines)
- Create: `apps/tests/phase223_p4_skip_whitespace.hako` (E2E test)
- Create: Unit test module in `loop_body_cond_promoter.rs` (~100 lines tests)

---

## Relation to Overall JoinIR Roadmap

**Phase 223** is part of the **JsonParser実戦投入ライン** (Phase 220-225):

```
Phase 220: ConditionEnv統合 ✅
Phase 221: 実戦投入・制約整理 ✅
Phase 222: If Condition正規化 ✅
Phase 223: LoopBodyLocal Condition Promotion ← 現在地
  ├─ Phase 223-1: Inventory ✅
  ├─ Phase 223-2: Design ✅ (完了)
  └─ Phase 223-3: Implementation (次)
Phase 224: P1 Cascading LoopBodyLocal (計画中)
Phase 225: P2 Multi-Variable Patterns (計画中)
```

**Overall Goal**: Enable all JsonParser loops to use JoinIR Pattern1-4 infrastructure

**Current Coverage**:
- Before Phase 223: 7/13 loops (54%)
- After Phase 223-3 (P0): 8/13 loops (62%)
- After Phase 224 (P1): 10/13 loops (77% estimated)
- After Phase 225 (P2): 11/13 loops (85% estimated)
- Remaining 2 loops: Category B (complex, may remain Fail-Fast)

---

## Revision History

- **2025-12-10**: Phase 223-2 design complete, summary document created
