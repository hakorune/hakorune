# Phase 131-11: Case C 本命タスク - 調査完了レポート

**Date**: 2025-12-14
**Status**: Active - Pattern detection landed; follow-ups tracked

---

## 🎯 Task Summary

**Goal**: Make `loop(true) { ... break ... continue }` compile and run in JoinIR

**Test File**: `apps/tests/llvm_stage3_loop_only.hako`

## 状態アップデート（Phase 131-11 A–C / H）

- Phase 131-11 A–C: `loop(true)` + break/continue を専用パターン（`pattern5_infinite_early_exit.rs`）へルーティングできる状態まで到達（検出/shape guard）。
- Phase 131-11 H: ループキャリアPHIの型が循環で壊れる問題に対して、PHI作成時に entry(init) 側の型のみを seed する修正が入った。
  - 参考（原因レポート）: `docs/development/current/main/phase-131-11-g-phi-type-bug-report.md`
  - PHI/型デバッグ: `docs/reference/environment-variables.md` の `NYASH_PHI_TYPE_DEBUG` / `NYASH_PHI_META_DEBUG`

現状メモ:
- `apps/tests/llvm_stage3_loop_only.hako` については Phase 132-P2 で VM/LLVM parity（`Result: 3`）まで到達した。
  - 調査ログ: `docs/development/current/main/investigations/phase132-case-c-llvm-exe.md`

---

## 🔍 Root Cause (完全解明済み)

### 問題の核心

**Case C fails because**:
1. **Pattern Gap**: `loop(true)` (infinite loop) is NOT recognized by any of Patterns 1-4
2. **Loop Variable Extraction Fails**: `extract_loop_variable_from_condition()` expects binary comparison (`i < 3`), not boolean literal (`true`)
3. **Classification Priority Bug**: `has_continue = true` routes to Pattern 4, but Pattern 4 expects a loop variable

### Failure Flow (5 Steps)

```
1. LoopPatternContext::new()
   └─ has_continue=true, has_break=true (✅ detected correctly)

2. classify(features)
   └─ Returns Pattern4Continue (❌ WRONG - should be Pattern2 or new Pattern5)

3. Pattern4::can_lower()
   └─ Tries extract_loop_variable_from_condition(BoolLiteral(true))

4. extract_loop_variable_from_condition()
   └─ ❌ Error: "Unsupported loop condition pattern"
   └─ Expected: BinaryOp comparison (i < 3)
   └─ Actual: BoolLiteral(true)

5. Pattern falls through
   └─ No pattern matches → freeze() error
```

### 現在のパターン対応表

| Pattern | Condition Type | Break | Continue | Supported? |
|---------|---------------|-------|----------|------------|
| Pattern 1 | Comparison (`i < 3`) | ❌ | ❌ | ✅ |
| Pattern 2 | Comparison (`i < 3`) | ✅ | ❌ | ✅ |
| Pattern 3 | Comparison (`i < 3`) | ❌ | ❌ | ✅ |
| Pattern 4 | Comparison (`i < 3`) | ❌ | ✅ | ✅ |
| **Case C** | **Boolean (`true`)** | ✅ | ✅ | ❌ **GAP!** |

---

## 💡 Recommended Solution: Dedicated “InfiniteEarlyExit” pattern (recommended)

**Approach**: Add `is_infinite_loop` feature + fix classification, then add a dedicated JoinIR loop pattern module for `loop(true)` with early-exit.

### Why this approach?

1. **Correctness-first**: `has_break && has_continue` must not route to Pattern 4 (Pattern 4 assumes a loop variable in the condition).
2. **No naming collision**: Existing “Trim/P5” already exists; avoid calling this “Pattern 5”.
3. **Minimal + Fail-Fast**: Support exactly the Case C skeleton first, reject everything else.
4. **Keeps Pattern2 invariants**: Pattern2 is “break but no continue”; extending it to include continue blurs its contract.

### Implementation Strategy (3 Phases)

#### Phase 131-11-A: Feature Detection (30 min)

**Add `is_infinite_loop` field to LoopFeatures**

```rust
// src/mir/loop_pattern_detection/mod.rs
pub struct LoopFeatures {
    pub has_break: bool,
    pub has_continue: bool,
    pub has_if: bool,
    pub has_if_else_phi: bool,
    pub carrier_count: usize,
    pub break_count: usize,
    pub continue_count: usize,
+   pub is_infinite_loop: bool,  // NEW: true for loop(true)
    pub update_summary: Option<LoopUpdateSummary>,
}
```

**Update extract_features to detect loop(true)**

```rust
// src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs
pub(crate) fn extract_features(
+   condition: &ASTNode,  // NEW: need condition for infinite loop detection
    body: &[ASTNode],
    has_continue: bool,
    has_break: bool
) -> LoopFeatures {
+   let is_infinite_loop = matches!(condition, ASTNode::BoolLiteral { value: true, .. });
    // ... rest
}
```

**Update callers**

```rust
// src/mir/builder/control_flow/joinir/patterns/router.rs (LoopPatternContext::new)
- let features = ast_features::extract_features(body, has_continue, has_break);
+ let features = ast_features::extract_features(condition, body, has_continue, has_break);
```

#### Phase 131-11-B: Classification Fix (15 min)

**Fix classify() so break+continue does not misroute to Pattern 4**

```rust
// src/mir/loop_pattern_detection/mod.rs
pub fn classify(features: &LoopFeatures) -> LoopPatternKind {
+   // NEW: Case C core: infinite loop + break + continue → dedicated pattern kind
+   if features.is_infinite_loop && features.has_break && features.has_continue {
+       return LoopPatternKind::InfiniteEarlyExit;
+   }

    // Pattern 4: Continue (existing)
    if features.has_continue && !features.has_break {
        return LoopPatternKind::Pattern4Continue;
    }

    // ... rest unchanged
}
```

#### Phase 131-11-C: InfiniteEarlyExit lowering (2-3 hours)

**Add a dedicated pattern module and keep Pattern2 untouched**

```rust
// src/mir/builder/control_flow/joinir/patterns/pattern_infinite_early_exit.rs
pub fn can_lower(ctx: &LoopPatternContext) -> bool {
    matches!(ctx.pattern_kind, LoopPatternKind::InfiniteEarlyExit)
}

pub fn lower(builder: &mut MirBuilder, ctx: &LoopPatternContext) -> Result<Option<ValueId>, String> {
    // Shape guard (Fail-Fast):
    // - condition must be `true` literal
    // - exactly one break site and one continue site
    // - no nested loop / nested break/continue
    // Lowering outline:
    // - header: unconditional enter step
    // - step: body core → break-check → continue (tailcall to step) / exit
    unimplemented!()
}
```

---

## 📋 Modified Files Summary

**Total**: 5–6 files modified (+1 new pattern module)

1. **`src/mir/loop_pattern_detection/mod.rs`**
   - Add `is_infinite_loop: bool` field to `LoopFeatures` struct
   - Add `LoopPatternKind::InfiniteEarlyExit`
   - Update `classify()` (infinite+break+continue must not route to Pattern4)

2. **`src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs`**
   - Update `extract_features()` signature (add `condition: &ASTNode` param)
   - Detect `loop(true)` in condition

3. **`src/mir/builder/control_flow/joinir/patterns/router.rs`**
   - Pass condition into `extract_features()`
   - Ensure `LOOP_PATTERNS` table routes `InfiniteEarlyExit`

4. **`src/mir/builder/control_flow/joinir/patterns/mod.rs`**
   - `pub(in crate::mir::builder) mod pattern_infinite_early_exit;` (new module export)

5. **`src/mir/builder/control_flow/joinir/patterns/pattern_infinite_early_exit.rs`** (NEW)
   - Shape guard + lowering implementation for Case C

---

## 🧪 Test Strategy

### Unit Tests

**File**: `src/mir/loop_pattern_detection/mod.rs` (tests module)

```rust
#[test]
fn test_classify_infinite_loop_with_break() {
    let features = LoopFeatures {
        has_break: true,
        has_continue: false,
        is_infinite_loop: true,
        // ... other fields
    };
    assert_eq!(classify(&features), LoopPatternKind::Pattern2Break);
}

#[test]
fn test_classify_infinite_loop_with_continue_unsupported() {
    let features = LoopFeatures {
        has_break: false,
        has_continue: true,
        is_infinite_loop: true,
        // ... other fields
    };
    assert_eq!(classify(&features), LoopPatternKind::Unknown);
}
```

### Integration Tests

**Case C Minimal** (`/tmp/case_c_minimal.hako`):
```nyash
static box Main {
  main() {
    local i = 0
    loop (true) {
      i = i + 1
      if i == 3 { break }
    }
    print(i)
    return 0
  }
}
```
Expected: Prints `3`

**Case C Full** (`apps/tests/llvm_stage3_loop_only.hako`):
```nyash
loop (true) {
  counter = counter + 1
  if counter == 3 { break }
  continue
}
```
Expected: MIR compile error (Fail-Fast - not supported yet)

### End-to-End Test

```bash
# VM test
./target/release/hakorune /tmp/case_c_minimal.hako
# Expected output: 3

# LLVM test (after Phase 131-11 complete)
tools/build_llvm.sh /tmp/case_c_minimal.hako -o /tmp/case_c_exe
/tmp/case_c_exe
# Expected output: 3
echo $?
# Expected exit code: 0
```

---

## ⏱️ Timeline Estimate

**Phase 131-11-A**: 30 minutes (Feature Detection)
**Phase 131-11-B**: 15 minutes (Classification Fix)
**Phase 131-11-C**: 2-3 hours (Pattern 2 Extension)

**Total**: 3-4 hours to complete Phase 131-11

---

## ✅ Success Criteria

**Phase 131-11 Complete** when:
1. ✅ Case C (minimal) compiles to MIR successfully
2. ✅ Case C (minimal) passes VM execution (prints `3`)
3. ✅ Case C (minimal) passes LLVM AOT (EMIT + LINK + RUN)
4. ✅ Case C (with continue) fails with clear error message (Fail-Fast)
5. ✅ No regression in Cases A, B, B2 (all still pass)
6. ✅ Unit tests for classification pass

---

## 📚 References

**Detailed Analysis**: [case-c-infinite-loop-analysis.md](./case-c-infinite-loop-analysis.md) (13KB, complete investigation)

**SSOT**: [phase131-3-llvm-lowering-inventory.md](./phase131-3-llvm-lowering-inventory.md) (updated with root cause)

**Related Files**:
- Pattern Detection: `src/mir/loop_pattern_detection/mod.rs`
- Feature Extraction: `src/mir/builder/control_flow/joinir/patterns/ast_feature_extractor.rs`
- Pattern Router: `src/mir/builder/control_flow/joinir/patterns/router.rs`
- Pattern 2 Lowering: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

---

## 🚦 Next Steps

**For ChatGPT** (Implementation):
1. Start with Phase 131-11-A (Feature Detection) - 30 min
2. Proceed to Phase 131-11-B (Classification Fix) - 15 min
3. Implement Phase 131-11-C (Pattern 2 Extension) - 2-3 hours
4. Run unit tests + integration tests
5. Verify end-to-end LLVM AOT path

**For Claude** (Review):
- Review implementation for Box Theory alignment (Fail-Fast, modular boundaries)
- Verify no regression in existing patterns
- Check for code duplication opportunities (Pattern 2 finite vs infinite)

---

**Last Updated**: 2025-12-14
**Phase**: 131-11 (Case C本命タスク)
**Status**: 🎯 Ready for Implementation
