# Phase 91: JoinIR Coverage Expansion (Selfhost depth-2)

## Status
- ✅ **Analysis Complete**: Loop inventory across selfhost codebase (Step 1)
- ✅ **Planning Complete**: Pattern P5b (Escape Handling) candidate selected (Step 1)
- ✅ **Implementation Complete**: AST recognizer, canonicalizer integration, unit tests (Step 2-A/B/D)
- ✅ **Parity Verified**: Strict mode green in `test_pattern5b_escape_minimal.hako` (Step 2-E)
- 📝 **Documentation**: Updated Phase 91 README with completion status

## Executive Summary

**Inventory snapshot**: 47% (16/30 loops in selfhost code)  
この数値は Phase 91 開始時点の棚卸しメモで、Phase 91 自体では「P5b の認識（canonicalizer）まで」を完了した。

| Category | Count | Status | Effort |
|----------|-------|--------|--------|
| Pattern 1 (simple bounded) | 16 | ✅ Ready | None |
| Pattern 2 (with break) | 1 | ⚠️ Partial | Low |
| Pattern P5b (escape handling) | ~3 | ✅ Recognized (canonicalizer) | Medium |
| Pattern P5 (guard-bounded) | ~2 | ❌ Blocked | High |
| Pattern P6 (nested loops) | ~8 | ❌ Blocked | Very High |

---

## Analysis Results

### Loop Inventory by Component

#### File: `apps/selfhost-vm/boxes/json_cur.hako` (3 loops)
- Lines 9-14: ✅ Pattern 1 (simple bounded)
- Lines 23-32: ✅ Pattern 1 variant with break
- Lines 42-57: ✅ Pattern 1 with guard-less loop(true)

#### File: `apps/selfhost-vm/json_loader.hako` (3 loops)
- Lines 16-22: ✅ Pattern 1 (simple bounded)
- **Lines 30-37**: ✅ Pattern P5b (escape skip; canonicalizer recognizes)
- Lines 43-48: ✅ Pattern 1 (simple bounded)

#### File: `apps/selfhost-vm/boxes/mini_vm_core.hako` (9 loops)
- Lines 208-231: ⚠️ Pattern 1 variant (with continue)
- Lines 239-253: ✅ Pattern 1 (with accumulator)
- Lines 388-400, 493-505: ✅ Pattern 1 (6 bounded search loops)
- **Lines 541-745**: ❌ Pattern P5 **PRIME CANDIDATE** (guard-bounded, 204-line collect_prints)

#### File: `apps/selfhost-vm/boxes/seam_inspector.hako` (13 loops)
- Lines 10-26: ✅ Pattern 1
- Lines 38-42, 116-120, 123-127: ✅ Pattern 1 variants
- **Lines 76-107**: ❌ Pattern P6 (deeply nested, 7+ levels)
- Remaining: Mix of ⚠️ Pattern 1 variants with nested loops

#### File: `apps/selfhost-vm/boxes/mini_vm_prints.hako` (1 loop)
- Line 118+: ❌ Pattern P5 (guard-bounded multi-case)

---

## Candidate Selection: Priority Order

### 🥇 **IMMEDIATE CANDIDATE: Pattern P5b (Escape Handling)**

**Target**: `json_loader.hako:30` - `read_digits_from()`

**Scope**: 8-line loop

**Current Structure**:
```nyash
loop(i < n) {
  local ch = s.substring(i, i+1)
  if ch == "\"" { break }
  if ch == "\\" {
    i = i + 1
    ch = s.substring(i, i+1)
  }
  out = out + ch
  i = i + 1
}
```

**Pattern Classification**:
- **Header**: `loop(i < n)`
- **Escape Check**: `if ch == "\\" { i = i + 2 instead of i + 1 }`
- **Body**: Append character
- **Carriers**: `i` (position), `out` (buffer)
- **Challenge**: Variable increment (sometimes +1, sometimes +2)

**Why This Candidate**:
- ✅ **Small scope** (8 lines) - good for initial implementation
- ✅ **High reuse potential** - same pattern appears in multiple parser locations
- ✅ **Moderate complexity** - requires conditional step extension (not fully generic)
- ✅ **Clear benefit** - would unlock escape sequence handling across all string parsers
- ❌ **Scope limitation** - conditional increment not yet in Canonicalizer

**Effort Estimate**: 2-3 days
- Canonicalizer extension: 4-6 hours
- Pattern recognizer: 2-3 hours
- Lowering implementation: 4-6 hours
- Testing + verification: 2-3 hours

---

### 🥈 **SECOND CANDIDATE: Pattern P5 (Guard-Bounded)**

**Target**: `mini_vm_core.hako:541` - `collect_prints()`

**Scope**: 204-line loop (monolithic)

**Current Structure**:
```nyash
loop(true) {
  guard = guard + 1
  if guard > 200 { break }

  local p = index_of_from(json, k_print, pos)
  if p < 0 { break }

  // 5 different cases based on JSON type
  if is_binary_op { ... pos = ... out.push(...) }
  if is_compare { ... pos = ... out.push(...) }
  if is_literal { ... pos = ... out.push(...) }
  if is_function_call { ... pos = ... out.push(...) }
  if is_nested { ... pos = ... out.push(...) }

  pos = obj_end + 1
}
```

**Pattern Classification**:
- **Header**: `loop(true)` (unconditional)
- **Guard**: `guard > LIMIT` with increment each iteration
- **Body**: Multiple case-based mutations
- **Carriers**: `pos`, `printed`, `guard`, `out` (ArrayBox)
- **Exit conditions**: Guard exhaustion OR search failure

**Why This Candidate**:
- ✅ **Monolithic optimization opportunity** - 204 lines of complex control flow
- ✅ **Real-world JSON parsing** - demonstrates practical JoinIR application
- ✅ **High performance impact** - guard counter could be eliminated via SSA
- ❌ **High complexity** - needs new Pattern5 guard-handling variant
- ❌ **Large scope** - would benefit from split into micro-loops first

**Effort Estimate**: 1-2 weeks
- Design: 2-3 days (pattern definition, contract)
- Implementation: 5-7 days
- Testing + verification: 2-3 days

**Alternative Strategy**: Could split into 5 micro-loops per case:
```nyash
// Instead of one 204-line loop with 5 cases:
// Create 5 functions, each handling one case:
loop_binary_op() { ... }
loop_compare() { ... }
loop_literal() { ... }
loop_function_call() { ... }
loop_nested() { ... }

// Then main loop dispatches:
loop(true) {
  guard = guard + 1
  if guard > limit { break }
  if type == BINARY_OP { loop_binary_op(...) }
  ...
}
```

This would make each sub-loop Pattern 1-compatible immediately.

---

### 🥉 **THIRD CANDIDATE: Pattern P6 (Nested Loops)**

**Target**: `seam_inspector.hako:76` - `_scan_boxes()`

**Scope**: Multi-level nested (7+ nesting levels)

**Current Structure**: 37-line outer loop containing 6 nested loops

**Pattern Classification**:
- **Nesting levels**: 7+
- **Carriers**: Multiple per level (`i`, `j`, `k`, `name`, `pos`, etc.)
- **Exit conditions**: Varied per level (bounds, break, continue)
- **Scope handoff**: Complex state passing between levels

**Why This Candidate**:
- ✅ **Demonstrates nested composition** - needed for production parsers
- ✅ **Realistic code** - actual box/function scanner
- ❌ **Highest complexity** - requires recursive JoinIR composition
- ❌ **Long-term project** - 2-3 weeks minimum

**Effort Estimate**: 2-3 weeks
- Design recursive composition: 3-5 days
- Per-level implementation: 7-10 days
- Testing nested composition: 3-5 days

---

## Recommended Immediate Action

### Phase 91 (This Session): Pattern P5b Planning

**Objective**: Design Pattern P5b (escape sequence handling) with minimal implementation

**Steps**:
1. ✅ **Analysis complete** (done by Explore agent)
2. **Design P5b pattern** (canonicalizer contract)
3. **Create minimal fixture** (`test_pattern5b_escape_minimal.hako`)
4. **Extend Canonicalizer** to recognize escape patterns
5. **Plan lowering** (defer implementation to next session)
6. **Document P5b architecture** in loop-canonicalizer.md

**Acceptance Criteria**:
- ✅ Pattern P5b design document complete
- ✅ Minimal escape test fixture created
- ✅ Canonicalizer recognizes escape patterns (dev-only observation)
- ✅ Parity check passes (strict mode)
- ✅ No lowering changes yet (recognition-only phase)

**Deliverables**:
- `docs/development/current/main/phases/phase-91/README.md` - This document
- `docs/development/current/main/design/pattern-p5b-escape-design.md` - Pattern design (new)
- `tools/selfhost/test_pattern5b_escape_minimal.hako` - Test fixture (new)
- Updated `docs/development/current/main/design/loop-canonicalizer.md` - Capability tags extended

---

## Design: Pattern P5b (Escape Sequence Handling)

Pattern P5b の詳細設計は重複を避けるため、設計 SSOT に集約する。

- **設計 SSOT**: `docs/development/current/main/design/pattern-p5b-escape-design.md`
- **Canonicalizer SSOT（語彙/境界）**: `docs/development/current/main/design/loop-canonicalizer.md`

この Phase 91 README は「在庫分析 + 実装完了の記録」に徹し、アルゴリズム本文や疑似コードは上記 SSOT を参照する。

---

## Completion Status

### Phase 91 Step 2: Implementation ✅ COMPLETE
- ✅ Extended `UpdateKind` enum with `ConditionalStep` variant
- ✅ Implemented `detect_escape_skip_pattern()` in AST recognizer
- ✅ Updated canonicalizer to recognize P5b patterns
- ✅ Added comprehensive unit test: `test_escape_skip_pattern_recognition`
- ✅ Verified parity in strict mode (canonical vs actual decision routing)

**Key Deliverables**:
- Updated `skeleton_types.rs`: ConditionalStep support
- Updated `ast_feature_extractor.rs`: P5b pattern detection
- Updated `canonicalizer.rs`: P5b routing to Pattern2Break + unit test
- Updated `test_pattern5b_escape_minimal.hako`: Fixed syntax errors

**Test Results**: 1062/1062 tests PASS (including new P5b unit test)

---

## Next Steps (Future Sessions)

### Phase 92: Lowering
- 進捗は Phase 92 で実施済み（ConditionalStep lowering + body-local 条件式サポート + 最小E2E smoke）。
  - 入口: `docs/development/current/main/phases/phase-92/README.md`

### Phase 93: Pattern P5 (Guard-Bounded)
- Implement Pattern5 for `mini_vm_core.hako:541`
- Consider micro-loop refactoring alternative
- Document guard-counter optimization strategy

### Phase 94+: Pattern P6 (Nested Loops)
- Recursive JoinIR composition for `seam_inspector.hako:76`
- Cross-level scope/carrier handoff

---

## SSOT References

- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **Loop Canonicalizer Design**: `docs/development/current/main/design/loop-canonicalizer.md`
- **Capability Tags**: `src/mir/loop_canonicalizer/capability_guard.rs`

---

## Summary

**Phase 91** establishes the next frontier of JoinIR coverage: **Pattern P5b (Escape Handling)**.

This pattern unlocks:
- ✅ escape skip を含む “条件付き増分” 系ループの取り込み足場（recognizer + contract）
- ✅ Foundation for Pattern P5 (guard-bounded)
- ✅ Preparation for Pattern P6 (nested loops)

**Current readiness**: 47% (16/30 loops)
**After Phase 91**: Expected to reach ~60% (18/30 loops)
**Long-term target**: >90% coverage with P5, P5b, P6 patterns

All acceptance criteria defined. Implementation ready for next session.
