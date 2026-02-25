# Phase 194: JsonParser Loop Inventory

**Date**: 2025-12-09
**File**: `tools/hako_shared/json_parser.hako`
**Total Loops**: 10

---

## JoinIR Target Loops (Phase 194)

Loops that CAN be handled by existing P1/P2/P5 patterns.

| Loop Name | Pattern | Line | Carrier | Update | Status |
|-----------|---------|------|---------|--------|--------|
| `_skip_whitespace` | P2 | 312-319 | p | p+1 | ✅ Whitelisted |
| `_trim` (leading) | P5 | 330-337 | start | start+1 | ✅ Whitelisted |
| `_trim` (trailing) | P5 | 340-347 | end | end-1 | ✅ Whitelisted |
| `_match_literal` | P2 | 357-362 | i | i+1 | ✅ Whitelisted |

### Pattern Details

#### `_skip_whitespace` (Line 312-319)
```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        p = p + 1
    } else {
        break
    }
}
```
- **Pattern**: P2 (break condition)
- **Carrier**: p (IntegerBox)
- **Update**: p = p + 1
- **Routing**: Already whitelisted as `JsonParserBox._skip_whitespace/2`

#### `_trim` Leading (Line 330-337)
```nyash
loop(start < end) {
    local ch = s.substring(start, start+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        start = start + 1
    } else {
        break
    }
}
```
- **Pattern**: P5 (Trim specialized)
- **Carrier**: start (IntegerBox)
- **Update**: start = start + 1
- **Routing**: Already whitelisted as `JsonParserBox._trim/1`

#### `_trim` Trailing (Line 340-347)
```nyash
loop(end > start) {
    local ch = s.substring(end-1, end)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        end = end - 1
    } else {
        break
    }
}
```
- **Pattern**: P5 (Trim specialized)
- **Carrier**: end (IntegerBox)
- **Update**: end = end - 1
- **Routing**: Already whitelisted as `JsonParserBox._trim/1`

#### `_match_literal` (Line 357-362)
```nyash
loop(i < len) {
    if s.substring(pos + i, pos + i + 1) != literal.substring(i, i + 1) {
        return 0
    }
    i = i + 1
}
```
- **Pattern**: P2 (break via return)
- **Carrier**: i (IntegerBox)
- **Update**: i = i + 1
- **Routing**: Already whitelisted as `JsonParserBox._match_literal/3`

---

## Deferred Loops (Phase 200+)

Loops that CANNOT be handled by current P1/P2/P5 patterns.

| Loop Name | Line | Deferral Reason | Target Phase |
|-----------|------|-----------------|--------------|
| `_parse_number` | 121-133 | `digits.indexOf()` - ConditionEnv constraint | Phase 200+ |
| `_atoi` | 453-460 | `digits.indexOf()` - ConditionEnv constraint | Phase 200+ |
| `_parse_string` | 150-178 | Complex carriers (escaped flag via continue) | Phase 195+ |
| `_unescape_string` | 373-431 | Complex carriers (is_escape, has_next, process_escape) | Phase 195+ |
| `_parse_array` | 203-231 | Multiple MethodCalls (`_parse_value`, nested) | Phase 195+ |
| `_parse_object` | 256-304 | Multiple MethodCalls (`_parse_string`, `_parse_value`) | Phase 195+ |

### Deferral Details

#### `_parse_number` (Line 121-133) - **ConditionEnv Constraint**
```nyash
local digits = "0123456789"  // ← External local variable
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)  // ← Uses external 'digits'

    if digit_pos < 0 {
        break
    }

    num_str = num_str + ch
    p = p + 1
}
```
- **Reason**: `digits` is an external local variable not in ConditionEnv
- **Constraint**: Phase 193 ConditionEnv only includes: function params, loop carriers, body-local vars
- **Workaround**: None (fundamental ConditionEnv limitation)
- **Solution**: Phase 200+ ConditionEnv expansion OR .hako rewrite to inline digits

#### `_atoi` (Line 453-460) - **ConditionEnv Constraint**
```nyash
local digits = "0123456789"  // ← External local variable
loop(i < n) {
    local ch = s.substring(i, i+1)
    if ch < "0" || ch > "9" { break }
    local pos = digits.indexOf(ch)  // ← Uses external 'digits'
    if pos < 0 { break }
    v = v * 10 + pos
    i = i + 1
}
```
- **Reason**: Same as `_parse_number` - `digits.indexOf()` dependency
- **Solution**: Phase 200+ ConditionEnv expansion

#### `_parse_string` (Line 150-178) - **Complex Carriers**
```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)

    if ch == '"' {
        // ... MapBox construction ...
        return result
    }

    if ch == "\\" {
        local has_next = 0
        if p + 1 < s.length() { has_next = 1 }

        if has_next == 0 { return null }

        str = str + ch
        p = p + 1
        str = str + s.substring(p, p+1)
        p = p + 1
        continue  // ← Escape handling via continue
    }

    str = str + ch
    p = p + 1
}
```
- **Reason**: Escape handling logic uses `continue` + conditional flag (`has_next`)
- **Pattern Fit**: P2 or P3 with complex control flow
- **Solution**: Phase 195+ Pattern 3 extension for if-in-loop with continue

#### `_unescape_string` (Line 373-431) - **Complex Carriers**
```nyash
loop(i < s.length()) {
    local ch = s.substring(i, i+1)

    local is_escape = 0
    local has_next = 0

    if ch == "\\" { is_escape = 1 }
    if i + 1 < s.length() { has_next = 1 }

    local process_escape = 0
    if is_escape == 1 {
        if has_next == 1 {
            process_escape = 1
        }
    }

    if process_escape == 1 {
        // ... multiple escape type checks ...
        continue
    }

    result = result + ch
    i = i + 1
}
```
- **Reason**: Multiple body-local flags (`is_escape`, `has_next`, `process_escape`) + nested conditions
- **Pattern Fit**: P3 with complex flatten pattern
- **Solution**: Phase 195+ Pattern 3 extension for multi-flag carriers

#### `_parse_array` / `_parse_object` - **Multiple MethodCalls**
```nyash
// _parse_array
loop(p < s.length()) {
    local elem_result = me._parse_value(s, p)  // ← MethodCall 1
    if elem_result == null { return null }

    local elem = elem_result.get("value")  // ← MethodCall 2
    arr.push(elem)  // ← MethodCall 3

    p = elem_result.get("pos")  // ← MethodCall 4
    // ...
}

// _parse_object
loop(p < s.length()) {
    local key_result = me._parse_string(s, p)  // ← MethodCall 1
    // ...
    local value_result = me._parse_value(s, p)  // ← MethodCall 2
    // ...
    obj.set(key, value)  // ← MethodCall 3
    // ...
}
```
- **Reason**: Multiple MethodCalls per iteration (4+ calls)
- **Constraint**: Phase 193 supports 1 MethodCall in init, not multiple in body
- **Solution**: Phase 195+ MethodCall extension for loop body

---

## Statistics

### Coverage
- **JoinIR Target**: 4/10 loops (40%)
- **Deferred**: 6/10 loops (60%)

### Pattern Distribution (Target Loops)
- **P2 (Break)**: 2 loops (`_skip_whitespace`, `_match_literal`)
- **P5 (Trim)**: 2 loops (`_trim` leading, `_trim` trailing)

### Deferral Reasons
- **ConditionEnv Constraint**: 2 loops (`_parse_number`, `_atoi`)
- **Complex Carriers**: 2 loops (`_parse_string`, `_unescape_string`)
- **Multiple MethodCalls**: 2 loops (`_parse_array`, `_parse_object`)

---

## Implementation Strategy (Phase 194)

### ✅ Already Whitelisted
All 4 target loops are ALREADY in the routing whitelist:
- `JsonParserBox._skip_whitespace/2` (Line 88)
- `JsonParserBox._trim/1` (Line 87)
- `JsonParserBox._match_literal/3` (Line 89)

### 🎯 E2E Validation
**Goal**: Verify these loops actually run on JoinIR route without fallback.

```bash
# Test 1: Basic execution
NYASH_JOINIR_CORE=1 ./target/release/hakorune tools/hako_shared/json_parser.hako

# Test 2: Trace verification
NYASH_JOINIR_CORE=1 NYASH_JOINIR_DEBUG=1 ./target/release/hakorune tools/hako_shared/json_parser.hako 2>&1 | grep "\[trace:joinir\]"

# Expected: NO [joinir/freeze] messages (freeze = fallback to legacy)
```

### ⚠️ No New Whitelist Additions Needed
Phase 194 is about **validating existing infrastructure**, not adding new functions.

### 📊 Success Criteria
- [ ] All 4 target loops run on JoinIR route (no freeze)
- [ ] JsonParser parses basic JSON successfully
- [ ] No regressions in Phase 190-193 tests
- [ ] Clear documentation of deferred loops (this file)

---

## Next Phase Priorities (Based on Deferral Analysis)

### Phase 200+: ConditionEnv Expansion (High Value)
**Impact**: 2 loops (`_parse_number`, `_atoi`)
**Solution**: Expand ConditionEnv to include function-scoped locals OR .hako rewrite

### Phase 195+: Pattern 3 Extension (Medium Value)
**Impact**: 2 loops (`_parse_string`, `_unescape_string`)
**Solution**: Support complex carriers with multiple body-local flags

### Phase 195+: MethodCall Extension (Low Priority)
**Impact**: 2 loops (`_parse_array`, `_parse_object`)
**Solution**: Support multiple MethodCalls in loop body
**Note**: These are complex parsers, may be better handled by future optimization passes

---

## Recommendations

### Short-term (Phase 194)
1. ✅ E2E test with existing 4 whitelisted loops
2. ✅ Verify no fallback via trace
3. ✅ Document findings (this file)

### Medium-term (Phase 195)
1. Focus on Pattern 3 extension (if-in-loop + multiple carriers)
2. Target `_parse_string` as representative case
3. Defer `_unescape_string` complexity until Pattern 3 is stable

### Long-term (Phase 200+)
1. ConditionEnv expansion design document
2. Evaluate .hako rewrite vs. compiler extension
3. Consider `digits.indexOf()` as representative case for all external local dependencies

---

## Conclusion

**Phase 194 is validation-focused**: 4 target loops are ALREADY whitelisted and should work with existing P1/P2/P5 patterns. The 6 deferred loops have clear constraints that require future infrastructure:

- **ConditionEnv**: Needs Phase 200+ design
- **Complex Carriers**: Needs Phase 195 Pattern 3 extension
- **Multiple MethodCalls**: Needs Phase 195+ design

This inventory provides a clear roadmap for future JoinIR expansion based on real-world loop patterns.
Status: Historical
