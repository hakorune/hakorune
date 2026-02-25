# Phase 47 Test Files Inventory

**Date**: 2025-12-12

## P3 (If-Sum) Test Files Available

### 1. phase212_if_sum_min.hako ⭐ PRIMARY TARGET

**Path**: `/home/tomoaki/git/hakorune-selfhost/apps/tests/phase212_if_sum_min.hako`

**Pattern**: Simple if-sum with single carrier
```nyash
local sum = 0
local i = 0
local len = 3

loop(i < len) {
    if (i > 0) {
        sum = sum + 1  // Conditional update
    } else {
        sum = sum + 0  // Else branch (no-op)
    }
    i = i + 1
}
```

**Characteristics**:
- ✅ Single carrier: `sum`
- ✅ Simple condition: `i > 0`
- ✅ One loop param: `i`
- ✅ Explicit else branch
- ✅ Clean structure (no method calls, no body-local)

**Expected output**: `sum = 2`

**Use case**: **Phase 47-A Minimal** (first Normalized P3 implementation)

---

### 2. phase217_if_sum_multi_min.hako

**Path**: `/home/tomoaki/git/hakorune-selfhost/apps/tests/phase217_if_sum_multi_min.hako`

**Pattern**: Multi-carrier if-sum (2 accumulators)
```nyash
local sum = 0
local count = 0
local i = 0
local len = 3

loop(i < len) {
    if (i > 0) {
        sum = sum + 1    // Conditional update 1
        count = count + 1  // Conditional update 2
        print(sum)
        print(count)
    } else {
        print(0)
    }
    i = i + 1
}
```

**Characteristics**:
- ✅ **Two carriers**: `sum`, `count`
- ✅ Side effects: `print()` calls
- ✅ Multiple updates in then branch
- ⚠️ Slightly more complex than phase212

**Expected output**: `sum = 2, count = 2`

**Use case**: **Phase 47-A Extended** (multi-carrier test)

---

### 3. phase218_json_if_sum_min.hako

**Path**: `/home/tomoaki/git/hakorune-selfhost/apps/tests/phase218_json_if_sum_min.hako`

**Pattern**: JsonParser-style accumulation
```nyash
local sum = 0
local i = 0
local len = 5

loop(i < len) {
    if (i > 0) {
        sum = sum + i  // Variable accumulation (not constant)
        print(sum)
    } else {
        print(0)
    }
    i = i + 1
}
```

**Characteristics**:
- ✅ Single carrier: `sum`
- ✅ Variable accumulation: `sum + i` (not `sum + 1`)
- ✅ Simulates JsonParser digit accumulation pattern
- ⚠️ More complex arithmetic

**Expected output**: `sum = 10` (1+2+3+4)

**Use case**: **Phase 47-A Extended** (variable accumulation test)

---

## Phase 47-A Implementation Priority

### First Target: phase212_if_sum_min.hako

**Why**:
1. ✅ **Simplest structure** - single carrier, simple condition
2. ✅ **Clean pattern** - no side effects (no print), no method calls
3. ✅ **Direct P2 analogy** - just adds if branching to P2 structure
4. ✅ **Best for proving concept** - minimal complexity

**Normalized components needed**:
- EnvLayout: `{ i: int, sum: int }` (same as P2)
- StepSchedule: `[HeaderCond, IfCond, ThenUpdates, Updates, Tail]`
- JpInst: Reuse existing `If` instruction

### Second Target: phase217_if_sum_multi_min.hako

**Why**:
1. ✅ **Multi-carrier test** - proves P3 handles multiple accumulators
2. ⚠️ Side effects (print) - tests that side effects work in then/else branches
3. ✅ **Still simple condition** - `i > 0` (same as phase212)

**Additional Normalized components**:
- EnvLayout: `{ i: int, sum: int, count: int }` (multi-carrier)
- Side effect handling in then/else branches

### Third Target: phase218_json_if_sum_min.hako

**Why**:
1. ✅ **Variable accumulation** - `sum + i` (not constant)
2. ✅ **JsonParser pattern** - prepares for real JsonParser loops
3. ⚠️ More complex arithmetic expression

**Additional Normalized components**:
- Expression lowering for `sum + i` (already supported by ExprLowerer)

---

## Phase 47-B Target (Future)

**Candidate**: JsonParser array_filter pattern (if exists in codebase)
```nyash
local out = new ArrayBox()
local i = 0

loop(i < arr.length()) {
    local v = arr.get(i)
    if (predicate(v)) {
        out.push(v)
    }
    i = i + 1
}
```

**Complexity**:
- ⚠️ Method calls: `arr.length()`, `arr.get(i)`, `out.push(v)`
- ⚠️ Body-local: `v` (used in if condition)
- ⚠️ External predicate call

**Status**: Not yet verified if this exact pattern exists in codebase

---

## Summary

**Available P3 test files**: 3
1. ✅ phase212_if_sum_min.hako (PRIMARY)
2. ✅ phase217_if_sum_multi_min.hako (SECONDARY)
3. ✅ phase218_json_if_sum_min.hako (TERTIARY)

**Phase 47-A roadmap**:
1. Start with phase212 (single carrier, simple)
2. Extend to phase217 (multi-carrier)
3. Extend to phase218 (variable accumulation)
4. All dev-only (Normalized→MIR direct comparison tests)

**Phase 47-B roadmap** (future):
- Real JsonParser loops with method calls + body-local
- Requires body-local handling in EnvLayout (already exists for P2 DigitPos)

**Phase 47-C roadmap** (future):
- Canonical promotion of P3 minimal (like P2)
- Performance validation

---

## References

- **Phase 47 Design**: [phase47-norm-p3-design.md](./phase47-norm-p3-design.md)
- **P2 Completion**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md)
- **Architecture**: [joinir-architecture-overview.md](./joinir-architecture-overview.md)
