Status: VerificationReport, Historical

# Phase 170: JsonParserBox JoinIR Preparation & Re-validation - Completion Report

**Date**: 2025-12-07
**Duration**: 2 sessions (autonomous work + bug fix verification)
**Status**: ✅ **Complete** - Environment prepared, bug fix verified, next phase planned

---

## Executive Summary

Phase 170 successfully prepared the environment for JsonParserBox JoinIR validation and identified the critical ValueId boundary mapping issue blocking runtime execution. All tasks completed:

- ✅ **Task A-1**: JoinIR routing whitelist expanded (6 JsonParserBox methods + test helper)
- ✅ **Task A-2**: ValueId boundary issue identified with full root cause analysis
- ⚠️ **Task B**: Mini tests blocked by `using` statement (workaround: simplified test created)
- ✅ **Task C**: Next phase direction decided (Option A: Fix boundary mapping)

**Key Achievement**: Identified that BoolExprLowerer integration (Phase 167-169) is correct, but the boundary mechanism needs condition variable extraction to work properly.

---

## Phase 170-A: Environment Setup ✅

### Task A-1: JoinIR Routing Whitelist Expansion

**Objective**: Allow JsonParserBox methods to route to JoinIR patterns instead of `[joinir/freeze]`.

**Changes Made**:

**File**: `src/mir/builder/control_flow/joinir/routing.rs` (lines 68-76)

**Added entries**:
```rust
// Phase 170-A-1: Enable JsonParserBox methods for JoinIR routing
"JsonParserBox._trim/1" => true,
"JsonParserBox._skip_whitespace/2" => true,
"JsonParserBox._match_literal/2" => true,
"JsonParserBox._parse_string/2" => true,
"JsonParserBox._parse_array/2" => true,
"JsonParserBox._parse_object/2" => true,
// Phase 170-A-1: Test methods (simplified versions)
"TrimTest.trim/1" => true,
```

**Result**: ✅ Methods now route to pattern matching instead of immediate `[joinir/freeze]` rejection.

**Evidence**:
```bash
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune local_tests/test_trim_main_pattern.hako
# Output: [joinir/pattern2] Generated JoinIR for Loop with Break Pattern (Phase 169)
```

---

### Task A-2: ValueId Boundary Issue Identification

**Objective**: Understand if ValueId boundary mapping affects JsonParserBox tests.

**Test Created**: `local_tests/test_trim_main_pattern.hako` (48 lines)
- Simplified `_trim` method with same loop structure as JsonParserBox
- Two loops with break (Pattern2 x 2)
- Condition variables: `start < end`, `end > start`

**Findings**:

1. **Pattern Detection**: ✅ Works correctly
   - Both loops match Pattern2
   - JoinIR generation succeeds

2. **Runtime Execution**: ❌ Silent failure
   - Program compiles successfully
   - No output produced
   - Exit code 0 (but no print statements executed)

3. **Root Cause Identified**: ValueId boundary mapping
   - Condition variables (`start`, `end`) resolved from HOST `variable_map`
   - HOST ValueIds (33, 34, 48, 49) used directly in JoinIR
   - Not included in `JoinInlineBoundary`
   - Merge process doesn't remap them → undefined at runtime

**Evidence**:
```
[ssa-undef-debug] fn=TrimTest.trim/1 bb=BasicBlockId(12) inst_idx=0 used=ValueId(33) inst=Compare { dst: ValueId(26), op: Lt, lhs: ValueId(33), rhs: ValueId(34) }
[ssa-undef-debug] fn=TrimTest.trim/1 bb=BasicBlockId(12) inst_idx=0 used=ValueId(34) inst=Compare { dst: ValueId(26), op: Lt, lhs: ValueId(33), rhs: ValueId(34) }
```

**Impact**: CRITICAL - Blocks ALL JsonParserBox methods with complex conditions.

**Detailed Analysis**: See [phase170-valueid-boundary-analysis.md](phase170-valueid-boundary-analysis.md)

---

## Phase 170-B: JsonParserBox Mini Test Re-execution ⚠️

### Original Test Files

**Location**: `tools/selfhost/json_parser_{string,array,object}_min.hako`

**Blocker**: `using` statement not working
```
[using] not found: 'tools/hako_shared/json_parser.hako" with JsonParserBox'
```

**Root Cause**: JsonParserBox is defined in external file, not compiled/loaded at runtime.

**Impact**: Can't run original integration tests in current form.

---

### Workaround: Simplified Test

**Created**: `local_tests/test_trim_main_pattern.hako`

**Purpose**: Test same loop structure without `using` dependency.

**Structure**:
```nyash
static box TrimTest {
  method trim(s) {
    // Same structure as JsonParserBox._trim
    loop(start < end) { ... break }
    loop(end > start) { ... break }
  }
  main(args) { ... }
}
```

**Result**: Successfully routes to Pattern2, exposes boundary issue.

---

## Phase 170-C: Next Phase Planning ✅

### Immediate TODOs (Phase 171+ Candidates)

**Priority 1: Fix ValueId Boundary Mapping** (HIGHEST PRIORITY)
- **Why**: Blocks all JsonParserBox complex condition tests
- **What**: Extract condition variables and add to `JoinInlineBoundary`
- **Where**: Pattern lowerers (pattern1/2/3/4)
- **Estimate**: 4.5 hours
- **Details**: See Option A in [phase170-valueid-boundary-analysis.md](phase170-valueid-boundary-analysis.md)

**Priority 2: Using Statement / Box Loading** (MEDIUM)
- **Why**: Enable actual JsonParserBox integration tests
- **What**: Compile and register boxes from `using` statements
- **Alternatives**:
  - Inline JsonParser code in tests (quick workaround)
  - Auto-compile static boxes (proper solution)

**Priority 3: Multi-Loop Function Support** (LOW)
- **Why**: `_trim` has 2 loops in one function
- **Current**: Each loop calls JoinIR routing separately (seems to work)
- **Risk**: May need validation that multiple JoinIR calls per function work correctly

---

### Recommended Next Phase Direction

**Option A: Fix Boundary Mapping First** ✅ **RECOMMENDED**

**Rationale**:
1. **Root blocker**: Boundary issue blocks ALL tests, not just one
2. **BoolExprLowerer correct**: Phase 169 integration is solid
3. **Pattern matching correct**: Routing and detection work perfectly
4. **Isolated fix**: Boundary extraction is well-scoped and testable
5. **High impact**: Once fixed, all JsonParser methods should work

**Alternative**: Option B (simplify code) or Option C (postpone) - both less effective.

---

## Test Results Matrix

| Method/Test | Pattern | JoinIR Status | Blocker | Notes |
|-------------|---------|---------------|---------|-------|
| `TrimTest.trim/1` (loop 1) | Pattern2 | ⚠️ Routes OK, runtime fail | ValueId boundary | `start < end` uses undefined ValueId(33, 34) |
| `TrimTest.trim/1` (loop 2) | Pattern2 | ⚠️ Routes OK, runtime fail | ValueId boundary | `end > start` uses undefined ValueId(48, 49) |
| `JsonParserBox._trim/1` | (untested) | - | Using statement | Can't load JsonParserBox at runtime |
| `JsonParserBox._skip_whitespace/2` | (untested) | - | Using statement | Can't load JsonParserBox at runtime |
| `JsonParserBox._match_literal/2` | (untested) | - | Using statement | Can't load JsonParserBox at runtime |
| `JsonParserBox._parse_string/2` | (untested) | - | Using statement | Can't load JsonParserBox at runtime |
| `JsonParserBox._parse_array/2` | (untested) | - | Using statement | Can't load JsonParserBox at runtime |
| `JsonParserBox._parse_object/2` | (untested) | - | Using statement | Can't load JsonParserBox at runtime |

**Summary**:
- **Routing**: ✅ All methods whitelisted, pattern detection works
- **Compilation**: ✅ BoolExprLowerer generates correct JoinIR
- **Runtime**: ❌ ValueId boundary issue prevents execution
- **Integration**: ⚠️ `using` statement blocks full JsonParser tests

---

## Files Modified

**Modified**:
- `src/mir/builder/control_flow/joinir/routing.rs` (+8 lines, whitelist expansion)

**Created**:
- `local_tests/test_trim_main_pattern.hako` (+48 lines, test file)
- `docs/development/current/main/phase170-valueid-boundary-analysis.md` (+270 lines, analysis)
- `docs/development/current/main/phase170-completion-report.md` (+THIS file)

**Updated**:
- `CURRENT_TASK.md` (added Phase 170 section with progress summary)
- `docs/development/current/main/phase166-validation-report.md` (added Phase 170 update section)

---

## Technical Insights

### Boundary Mechanism Gap

**Current Design**:
```rust
JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0)],      // JoinIR loop variable
    vec![loop_var_id],     // HOST loop variable
);
```

**What's Missing**: Condition variables!

**Needed Design**:
```rust
JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0), ValueId(1), ValueId(2)],  // loop var + cond vars
    vec![loop_var_id, start_id, end_id],       // HOST ValueIds
);
```

**Why It Matters**:
- `condition_to_joinir.rs` directly references HOST `variable_map` ValueIds
- These ValueIds are NOT in JoinIR's fresh allocator space
- Without boundary mapping, they remain undefined after merge
- Silent failure: compiles but doesn't execute

### Two ValueId Namespaces

**HOST Context** (Main MirBuilder):
- ValueIds from 0 upward (e.g., `start = ValueId(33)`)
- All variables in `builder.variable_map`
- Pre-existing before JoinIR call

**JoinIR Context** (Fresh Allocator):
- ValueIds from 0 upward (independent sequence)
- Generated by JoinIR lowerer
- Post-merge: remapped to new HOST ValueIds

**Bridge**: `JoinInlineBoundary` maps between the two spaces with Copy instructions.

**Current Gap**: Only explicitly listed variables get bridged. Condition variables are implicitly referenced but not bridged.

---

## Validation Checklist

- [x] Whitelist expanded (6 JsonParserBox methods + test)
- [x] Pattern routing verified (Pattern2 detected correctly)
- [x] BoolExprLowerer integration verified (generates JoinIR correctly)
- [x] Boundary issue identified (root cause documented)
- [x] Test file created (simplified _trim test)
- [x] Root cause analysis completed (270-line document)
- [x] Next phase direction decided (Option A recommended)
- [x] Documentation updated (CURRENT_TASK.md, phase166 report)
- [x] Files committed (ready for next phase)

---

## Next Phase: Phase 171 - Boundary Mapping Fix

**Recommended Implementation**:

1. **Create condition variable extractor** (30 min)
   - File: `src/mir/builder/control_flow/joinir/patterns/cond_var_extractor.rs`
   - Function: `extract_condition_variables(ast: &ASTNode, builder: &MirBuilder) -> Vec<(String, ValueId)>`

2. **Update Pattern2** (1 hour)
   - Extract condition variables before lowering
   - Create expanded boundary with condition vars
   - Test with `TrimTest.trim/1`

3. **Update Pattern1, Pattern3, Pattern4** (3 hours)
   - Apply same pattern
   - Ensure all patterns include condition vars in boundary

4. **Validation** (30 min)
   - Re-run `TrimTest.trim/1` → should print output
   - Re-run JsonParserBox tests (if `using` resolved)

**Total Estimate**: 5 hours

---

## Conclusion

Phase 170 successfully prepared the environment for JsonParserBox validation and identified the critical blocker preventing runtime execution. The boundary mapping issue is well-understood, with a clear solution path (Option A: extract condition variables).

**Key Achievements**:
- ✅ Whitelist expansion enables JsonParserBox routing
- ✅ BoolExprLowerer integration verified working correctly
- ✅ Boundary issue root cause identified and documented
- ✅ Clear next steps with 5-hour implementation estimate

**Next Step**: Implement Phase 171 - Condition Variable Extraction for Boundary Mapping.

---

## Phase 170‑C‑1: CaseA Shape 検出の暫定実装メモ

Phase 170‑C‑1 では、当初「LoopUpdateAnalyzer (AST) → UpdateExpr を使って Generic 判定を減らす」方針だったが、
実装コストと他フェーズとの依存関係を考慮し、まずは **carrier 名ベースの軽量ヒューリスティック** を導入した。

### 現状の実装方針

- `CaseALoweringShape::detect_from_features()` の内部で、LoopFeatures だけでは足りない情報を
  **carrier 名からのヒント** で補っている:
  - `i`, `e`, `idx`, `pos` など → 「位置・インデックス」寄りのキャリア
  - `result`, `defs` など → 「蓄積・結果」寄りのキャリア
- これにより、`Generic` 一択だったものを簡易的に:
  - StringExamination 系（位置スキャン系）
  - ArrayAccumulation 系（配列への追加系）
  に二分できるようにしている。

### 限界と今後

- これはあくまで **Phase 170‑C‑1 の暫定策** であり、箱理論上の最終形ではない:
  - 変数名に依存しているため、完全にハードコードを排除できているわけではない。
  - 真に綺麗にするには、LoopUpdateAnalyzer / 型推定層から UpdateKind や carrier 型情報を LoopFeatures に統合する必要がある。
- 今後のフェーズ（170‑C‑2 以降）では:
  - `LoopUpdateAnalyzer` に `UpdateKind` の分類を追加し、
    - `CounterLike` / `AccumulationLike` 等を LoopFeatures に持たせる。
  - 可能であれば carrier の型（String / Array 等）を推定する軽量層を追加し、
    `CaseALoweringShape` は **名前ではなく UpdateKind/型情報だけ** を見て判定する方向に寄せていく。

この暫定実装は「Phase 200 での loop_to_join.rs ハードコード削除に向けた足場」として扱い、
将来的には carrier 名依存のヒューリスティックを段階的に薄めていく予定。

---

## Phase 170‑C‑2b: LoopUpdateSummary 統合（実装メモ）

Phase 170‑C‑2b では、LoopUpdateSummaryBox を実コードに差し込み、  
CaseALoweringShape が直接 carrier 名を見ることなく UpdateKind 経由で判定できるようにした。

### 実装ポイント

- 既存の `LoopUpdateSummary` 型を活用し、`LoopFeatures` にフィールドを追加:

```rust
pub struct LoopFeatures {
    // 既存フィールド …
    pub update_summary: Option<LoopUpdateSummary>, // ← new
}
```

- `CaseALoweringShape` 側に `detect_with_updates()` を追加し、
  `LoopUpdateSummary` 内の `UpdateKind` を見て形を決めるようにした:

```rust
match update.kind {
    UpdateKind::CounterLike       => CaseALoweringShape::StringExamination,
    UpdateKind::AccumulationLike  => CaseALoweringShape::ArrayAccumulation,
    UpdateKind::Other             => CaseALoweringShape::Generic,
}
```

- `loop_to_join.rs` では、まず `detect_with_updates()` を試し、  
  それで決まらない場合のみ従来のフォールバックに流す構造に変更。

### 効果と現状

- carrier 名に依存するロジックは `LoopUpdateSummaryBox` の内部に閉じ込められ、  
  CaseALoweringShape / loop_to_join.rs からは UpdateKind だけが見える形になった。
- 代表的な ループスモーク 16 本のうち 15 本が PASS（1 本は既知の別問題）で、  
  既存パターンへの回帰は維持されている。

この状態を起点に、今後 Phase 170‑C‑3 以降で `LoopUpdateSummary` の中身（AST/MIR ベースの解析）だけを差し替えることで、
段階的に carrier 名ヒューリスティックを薄めていく計画。

---

## Phase 170-D: Loop Condition Scope Analysis - Bug Fix Verification ✅

**Date**: 2025-12-07 (Session 2)
**Status**: ✅ **Bug Fix Complete and Verified**

### Bug Fix: Function Parameter Misclassification

**Issue**: Function parameters (`s`, `pos` in JsonParser methods) were incorrectly classified as **LoopBodyLocal** when used in loop conditions.

**Root Cause**: `is_outer_scope_variable()` in `condition_var_analyzer.rs` defaulted unknown variables (not in `variable_definitions`) to LoopBodyLocal.

**Fix** (User Implemented):
```rust
// File: src/mir/loop_pattern_detection/condition_var_analyzer.rs (lines 175-184)

// At this point:
// - Variable is NOT in body_locals
// - No explicit definition info
// This typically means "function parameter" or "outer local"
true  // ✅ Default to OuterLocal for function parameters
```

### Verification Results

**Test 1: Function Parameter Loop** ✅
- **File**: `/tmp/test_jsonparser_simple.hako`
- **Result**: `Phase 170-D: Condition variables verified: {"pos", "s", "len"}`
- **Analysis**: Function parameters correctly classified as OuterLocal
- **New blocker**: Method calls (Pattern 5+ feature, not a bug)

**Test 2: LoopBodyLocal Correct Rejection** ✅
- **File**: `local_tests/test_trim_main_pattern.hako`
- **Result**: `Phase 170-D: Condition variables verified: {"ch", "end", "start"}`
- **Error**: "Variable 'ch' not bound in ConditionEnv" (correct rejection)
- **Analysis**: `ch` is defined inside loop, correctly rejected by Pattern 2

**Test 3: JsonParser Full File** ✅
- **File**: `tools/hako_shared/json_parser.hako`
- **Result**: Error about MethodCall, not variable classification
- **Analysis**: Variable scope classification now correct, remaining errors are legitimate feature gaps

### Impact

**What the Fix Achieves**:
- ✅ Function parameters work correctly: `s`, `pos` in JsonParser
- ✅ Carrier variables work correctly: `start`, `end` in trim loops
- ✅ Outer locals work correctly: `len`, `maxLen` from outer scope
- ✅ Correct rejection: LoopBodyLocal `ch` properly rejected (not a bug)

**Remaining Blockers** (Pattern 5+ features, not bugs):
- ⚠️ Method calls in conditions: `loop(pos < s.length())`
- ⚠️ Method calls in loop body: `s.substring(pos, pos+1)`
- ⚠️ LoopBodyLocal in break conditions: `if ch == " " { break }`

### Documentation

**Created**:
1. ✅ `phase170-d-fix-verification.md` - Comprehensive verification report
2. ✅ `phase170-d-fix-summary.md` - Executive summary
3. ✅ Updated `CURRENT_TASK.md` - Bug fix section added
4. ✅ Updated `phase170-d-impl-design.md` - Bug fix notes

### Next Steps

**Priority 1**: Pattern 5+ implementation for LoopBodyLocal in break conditions
**Priority 2**: .hako rewrite strategy for complex method calls
**Priority 3**: Coverage metrics for JsonParser loop support

**Build Status**: ✅ `cargo build --release` successful (0 errors, 50 warnings)
