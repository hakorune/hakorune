# Phase 91: JoinIR Coverage Expansion (Selfhost depth-2)

## Status
- ✅ **Analysis Complete**: Loop inventory across selfhost codebase (Step 1)
- ✅ **Planning Complete**: escape route P5b candidate selected (Step 1)
- ✅ **Implementation Complete**: AST recognizer, canonicalizer integration, unit tests (Step 2-A/B/D)
- ✅ **Parity Verified**: Strict mode green in `test_pattern5b_escape_minimal.hako` (legacy selfhost test stem, Step 2-E)
- 📝 **Documentation**: Updated Phase 91 README with completion status

## Executive Summary

**Inventory snapshot**: 47% (16/30 loops in selfhost code)  
この数値は Phase 91 開始時点の棚卸しメモで、Phase 91 自体では「P5b の認識（canonicalizer）まで」を完了した。

| Category | Count | Status | Effort |
|----------|-------|--------|--------|
| LoopSimpleWhile route family (historical label `1`) | 16 | ✅ Ready | None |
| LoopBreak route family (historical label `2`) | 1 | ⚠️ Partial | Low |
| Escape route P5b (legacy label) | ~3 | ✅ Recognized (canonicalizer) | Medium |
| Guard-bounded legacy route P5 | ~2 | ❌ Blocked | High |
| Nested-loop legacy route P6 | ~8 | ❌ Blocked | Very High |

---

## Analysis Results

### Loop Inventory by Component

#### File: `apps/selfhost-vm/boxes/json_cur.hako` (3 loops)
- Lines 9-14: ✅ LoopSimpleWhile route family
- Lines 23-32: ✅ LoopBreak-style bounded loop (historical label `2` note)
- Lines 42-57: ✅ LoopSimpleWhile variant with guard-less `loop(true)`

#### File: `apps/selfhost-vm/json_loader.hako` (3 loops)
- Lines 16-22: ✅ LoopSimpleWhile route family
- **Lines 30-37**: ✅ escape route P5b (canonicalizer recognizes)
- Lines 43-48: ✅ LoopSimpleWhile route family

#### File: `apps/selfhost-vm/boxes/mini_vm_core.hako` (9 loops)
- Lines 208-231: ⚠️ LoopSimpleWhile variant (with continue)
- Lines 239-253: ✅ LoopSimpleWhile route family (with accumulator)
- Lines 388-400, 493-505: ✅ LoopSimpleWhile route family (6 bounded search loops)
- **Lines 541-745**: ❌ guard-bounded legacy route P5 **PRIME CANDIDATE** (204-line collect_prints)

#### File: `apps/selfhost-vm/boxes/seam_inspector.hako` (13 loops)
- Lines 10-26: ✅ LoopSimpleWhile route family
- Lines 38-42, 116-120, 123-127: ✅ LoopSimpleWhile variants
- **Lines 76-107**: ❌ nested-loop legacy route P6 (deeply nested, 7+ levels)
- Remaining: Mix of ⚠️ LoopSimpleWhile variants with nested loops

#### File: `apps/selfhost-vm/boxes/mini_vm_prints.hako` (1 loop)
- Line 118+: ❌ guard-bounded legacy route P5 (multi-case)

---

## Candidate Selection: Priority Order

### 🥇 **IMMEDIATE CANDIDATE: escape route P5b**

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

**Route/shape classification**:
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

### 🥈 **SECOND CANDIDATE: guard-bounded legacy route P5**

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
- ❌ **High complexity** - needs new guard-bounded early-exit variant（historical label `5` lane）
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

### Phase 91 (This Session): escape route P5b planning

**Objective**: Design the escape route P5b (escape sequence handling) with minimal implementation

**Steps**:
1. ✅ **Analysis complete** (done by Explore agent)
2. **Design the P5b route shape** (canonicalizer contract)
3. **Create minimal fixture** (`test_pattern5b_escape_minimal.hako`, legacy selfhost test stem)
4. **Extend Canonicalizer** to recognize escape route shapes
5. **Plan lowering** (defer implementation to next session)
6. **Document the P5b architecture** in loop-canonicalizer.md

**Acceptance Criteria**:
- ✅ Escape route P5b design document complete
- ✅ Minimal escape test fixture created
- ✅ Canonicalizer recognizes escape route shapes (dev-only observation)
- ✅ Parity check passes (strict mode)
- ✅ No lowering changes yet (recognition-only phase)

**Deliverables**:
- `docs/development/current/main/phases/phase-91/README.md` - This document
- `docs/development/current/main/design/pattern-p5b-escape-design.md` - escape route design (new)
- `tools/selfhost/test_pattern5b_escape_minimal.hako` - Test fixture (new; legacy selfhost test stem)
- Updated `docs/development/current/main/design/loop-canonicalizer.md` - Capability tags extended

---

## Design: escape route P5b (Escape Sequence Handling)

escape route P5b の詳細設計は重複を避けるため、設計 SSOT に集約する。

- **設計 SSOT**: `docs/development/current/main/design/pattern-p5b-escape-design.md`
- **Canonicalizer SSOT（語彙/境界）**: `docs/development/current/main/design/loop-canonicalizer.md`

この Phase 91 README は「在庫分析 + 実装完了の記録」に徹し、アルゴリズム本文や疑似コードは上記 SSOT を参照する。

---

## Completion Status

### Phase 91 Step 2: Implementation ✅ COMPLETE
- ✅ Extended `UpdateKind` enum with `ConditionalStep` variant
- ✅ Implemented `detect_escape_skip_shape()` in AST recognizer
- ✅ Updated canonicalizer to recognize escape route P5b
- ✅ Added comprehensive unit test: `test_escape_skip_pattern_recognition`
- ✅ Verified parity in strict mode (canonical vs actual decision routing)

**Key Deliverables**:
- Updated `skeleton_types.rs`: ConditionalStep support
- Updated `ast_feature_extractor.rs`: P5b route-shape detection
- Updated `canonicalizer.rs`: P5b routing to loop_break route + unit test
- Updated `test_pattern5b_escape_minimal.hako`: Fixed syntax errors (legacy selfhost test stem retained)

**Test Results**: 1062/1062 tests PASS (including new P5b unit test)

---

## Next Steps (Future Sessions)

### Phase 92: Lowering
- 進捗は Phase 92 で実施済み（ConditionalStep lowering + body-local 条件式サポート + 最小E2E smoke）。
  - 入口: `docs/development/current/main/phases/phase-92/README.md`

### Phase 93: guard-bounded legacy route P5
- Implement the P5 follow-up for `mini_vm_core.hako:541`
- Consider micro-loop refactoring alternative
- Document guard-counter optimization strategy

### Phase 94+: nested-loop legacy route P6
- Recursive JoinIR composition for `seam_inspector.hako:76`
- Cross-level scope/carrier handoff

---

## SSOT References

- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **Loop Canonicalizer Design**: `docs/development/current/main/design/loop-canonicalizer.md`
- **Capability Tags**: `src/mir/loop_canonicalizer/capability_guard.rs`

---

## Summary

**Phase 91** establishes the next frontier of JoinIR coverage: **escape route P5b (Escape Handling)**.

This pattern unlocks:
- ✅ escape skip を含む “条件付き増分” 系ループの取り込み足場（recognizer + contract）
- ✅ Foundation for guard-bounded legacy route P5
- ✅ Preparation for nested-loop legacy route P6

**Current readiness**: 47% (16/30 loops)
**After Phase 91**: Expected to reach ~60% (18/30 loops)
**Long-term target**: >90% coverage with the P5 / P5b / P6 route families

All acceptance criteria defined. Implementation ready for next session.
