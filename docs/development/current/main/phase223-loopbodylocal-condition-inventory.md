# Phase 223-1: LoopBodyLocal in Condition - Comprehensive Inventory

---
**Phase 26-45 Completion**: このフェーズで設計した機能は Phase 43/245B で実装完了。最終状態は [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md) を参照。
---

Status: Historical（Phase 26-H 以降の Normalized / DigitPos 導入で一部内容が古くなっています）
Note: LoopBodyLocal が原因で Fail-Fast していたループの在庫を Phase 223 時点で一覧化したメモだよ。DigitPos 系などの一部ループはその後の Phase 224/26-H/34 系で解消済みなので、最新の対応状況は `joinir-architecture-overview.md` と Phase 42 の P2 インベントリを合わせて参照してね。

## Purpose

This document inventories all loops that are currently **blocked** by the LoopConditionScopeBox Fail-Fast mechanism because they have `LoopBodyLocal` variables appearing in loop conditions (header, break, or continue).

The goal is to:
1. Identify **safe patterns** (Trim/JsonParser-style) that can be promoted to carriers
2. Identify **complex patterns** that should continue to Fail-Fast
3. Provide design input for Phase 223-2 carrier promotion system

## Detection Methodology

### Rust-side Detection

**Fail-Fast Location**: `src/mir/join_ir/lowering/loop_with_break_minimal.rs`

```rust
if loop_cond_scope.has_loop_body_local() {
    let body_local_names = extract_body_local_names(&loop_cond_scope.vars);
    return Err(format_unsupported_condition_error("pattern2", &body_local_names));
}
```

**Also checked in**:
- Pattern 4 (with continue): `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`
- TrimLoopLowering: `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs` (tries promotion first)

### .hako File Patterns

Search patterns used:
```bash
# Pattern 1: local variable followed by loop
rg "local\s+\w+.*loop\(" apps/tests tools/hako_shared

# Pattern 2: substring/indexOf assignments (common in parsers)
rg "local\s+\w+\s*=.*substring|local\s+\w+\s*=.*indexOf" apps/tests tools/hako_shared

# Pattern 3: Loop conditions with character comparison
grep -r "loop.*ch.*==" apps/tests tools/hako_shared
```

---

## Category A: Safe Trim/JsonParser Patterns (昇格候補)

These patterns are **safe for carrier promotion** because:
- LoopBodyLocal is a simple value extraction (substring/indexOf)
- Condition is a simple boolean expression (equality/comparison)
- No complex control flow in the extraction
- Carrier update is straightforward

### Pattern A-1: Trim Leading Whitespace

**Example**: `tools/hako_shared/json_parser.hako` (line 330-336)

```hako
loop(start < end) {
    local ch = s.substring(start, start+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        start = start + 1
    } else {
        break
    }
}
```

**LoopBodyLocal**: `ch` (String)
**Definition**: `local ch = s.substring(start, start+1)`
**Condition Usage**: Break condition uses `ch` in OR chain: `ch == " " || ch == "\t" || ch == "\n" || ch == "\r"`
**Promotion Target**: `is_whitespace` (bool carrier)
**Status**: ✅ **Already handled by TrimLoopHelper** (Phase 171-C)
**Carrier Initialization**: `is_whitespace = (ch == " " || ch == "\t" || ch == "\n" || ch == "\r")`
**Carrier Update**: Same as initialization (at end of loop body)

---

### Pattern A-2: Trim Trailing Whitespace

**Example**: `tools/hako_shared/json_parser.hako` (line 340-346)

```hako
loop(end > start) {
    local ch = s.substring(end-1, end)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        end = end - 1
    } else {
        break
    }
}
```

**LoopBodyLocal**: `ch` (String)
**Definition**: `local ch = s.substring(end-1, end)` (note: backward indexing)
**Condition Usage**: Break condition uses `ch` in OR chain
**Promotion Target**: `is_whitespace` (bool carrier)
**Status**: ✅ **Already handled by TrimLoopHelper** (Phase 171-C)

---

### Pattern A-3: Skip Whitespace (Parser Pattern)

**Example**: `apps/tests/parser_box_minimal.hako` (line 30-41)

```hako
loop(i < n) {
    local ch = src.substring(i, i + 1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        i = i + 1
        continue
    }
    break
}
```

**LoopBodyLocal**: `ch` (String)
**Definition**: `local ch = src.substring(i, i + 1)`
**Condition Usage**: Continue condition uses `ch` in OR chain
**Promotion Target**: `is_whitespace` (bool carrier)
**Status**: ⚠️ **Pattern 4 (with continue)** - needs promotion support
**Notes**: Uses `continue` instead of `break` - currently blocked in Pattern 4

---

### Pattern A-4: JsonParser Number Parsing (Digit Detection)

**Example**: `tools/hako_shared/json_parser.hako` (line 121-133)

```hako
local digits = "0123456789"
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)

    // Exit condition: non-digit character found
    if digit_pos < 0 {
        break
    }

    // Continue parsing: digit found
    num_str = num_str + ch
    p = p + 1
}
```

**LoopBodyLocal**: `digit_pos` (Integer)
**Definition**: `local digit_pos = digits.indexOf(ch)` (depends on another LoopBodyLocal `ch`)
**Condition Usage**: Break condition `if digit_pos < 0`
**Promotion Target**: `is_digit` (bool carrier)
**Status**: ⚠️ **More complex** - depends on TWO LoopBodyLocal variables (`ch` and `digit_pos`)
**Notes**: This is the **cascading LoopBodyLocal pattern** - needs special handling

---

### Pattern A-5: Simple Character Comparison Loop

**Example**: `apps/tests/phase182_p1_match_literal.hako` (line 356-364)

```hako
loop(i < len) {
    local ch_s = s.substring(pos + i, pos + i + 1)
    local ch_lit = literal.substring(i, i + 1)
    if ch_s != ch_lit {
        print("Result: NOMATCH")
        return 0
    }
    i = i + 1
}
```

**LoopBodyLocal**: `ch_s` and `ch_lit` (both String)
**Definition**: Two substring extractions
**Condition Usage**: Break condition `if ch_s != ch_lit`
**Promotion Target**: `chars_match` (bool carrier)
**Status**: ⚠️ **Multiple LoopBodyLocal** - needs multi-variable promotion
**Notes**: This is a **string comparison pattern** - safe but needs multi-var support

---

### Pattern A-6: Parser atoi with Range Check

**Example**: `apps/tests/parser_box_minimal.hako` (line 20-25) and `tools/hako_shared/json_parser.hako` (line 453-460)

```hako
loop(i < n) {
    local ch = s.substring(i, i+1)
    if ch < "0" || ch > "9" { break }
    local pos = digits.indexOf(ch)
    if pos < 0 { break }
    v = v * 10 + pos
    i = i + 1
}
```

**LoopBodyLocal**: `ch` (String), `pos` (Integer)
**Definition**:
- `local ch = s.substring(i, i+1)`
- `local pos = digits.indexOf(ch)`
**Condition Usage**:
- Break condition 1: `ch < "0" || ch > "9"` (range check)
- Break condition 2: `pos < 0` (indexOf check)
**Promotion Target**: `is_digit` (bool carrier)
**Status**: ⚠️ **Cascading + Multiple Break Conditions** - complex but safe
**Notes**: Two break conditions using different LoopBodyLocal variables

---

## Category B: Complex Patterns (Fail-Fast 維持)

These patterns should **continue to Fail-Fast** because:
- Multiple complex LoopBodyLocal dependencies
- Nested method calls in conditions
- Complex control flow that cannot be safely promoted

### Pattern B-1: Nested If with LoopBodyLocal

**Example**: `apps/tests/minimal_ssa_bug_loop.hako`

```hako
loop(i < n) {
    local line = src.substring(i, i + 10)

    // Problem pattern: nested conditions with reassignment + immediate use
    if line.length() > 0 {
        line = line.substring(0, 5)     // reassign line
        if line.length() > 0 && line.substring(0, 1) == "u" {  // immediate use
            // ...
        }
    }
    i = i + 1
}
```

**LoopBodyLocal**: `line` (String)
**Why Fail-Fast**:
- LoopBodyLocal is **reassigned** inside the loop body
- **Nested conditions** with method calls (`line.length()`, `line.substring()`)
- Cannot create a simple bool carrier - would need complex state tracking

---

### Pattern B-2: Method Call Chain in Condition

**Example**: Hypothetical (not found in current codebase)

```hako
loop(i < n) {
    local item = array.get(i)
    if item.process().isValid() {
        // ...
    }
    i = i + 1
}
```

**Why Fail-Fast**:
- Method call chain makes carrier initialization complex
- Side effects in `process()` cannot be promoted

---

## Category C: Body-Only LoopBodyLocal (不要 - Already Handled)

These patterns have LoopBodyLocal variables that **only appear in the loop body**, NOT in conditions. They **do not trigger Fail-Fast**.

### Pattern C-1: Body-Only Computation

**Example**: `apps/tests/phase183_body_only_loopbodylocal.hako`

```hako
loop(i < 5) {
    // Body-only LoopBodyLocal: temp is computed but never appears in any condition
    local temp = i * 2

    // Break condition doesn't use temp - only uses outer variable i
    if i == 3 {
        break
    }

    result = result + temp
    i = i + 1
}
```

**LoopBodyLocal**: `temp` (Integer)
**Condition Usage**: **NONE** - only used in body expression `result = result + temp`
**Status**: ✅ **No Fail-Fast** - these are already handled correctly

---

### Pattern C-2: Body-Local Update Variable

**Example**: `apps/tests/phase184_body_local_with_break.hako`

```hako
loop(i < 10) {
    local temp = i * 3  // Body-local variable
    sum = sum + temp    // Use body-local in update expression

    if (sum >= 15) {    // Break condition uses 'sum', NOT 'temp'
        break
    }

    i = i + 1
}
```

**LoopBodyLocal**: `temp` (Integer)
**Condition Usage**: **NONE** - break condition uses `sum`, not `temp`
**Status**: ✅ **No Fail-Fast** - already handled correctly

---

## Summary Statistics

| Category | Count | Description |
|----------|-------|-------------|
| **Category A** (Safe for Promotion) | 6 patterns | Trim/JsonParser-style, simple boolean extraction |
| **Category B** (Fail-Fast Maintained) | 1 pattern | Complex nested conditions, reassignment |
| **Category C** (Body-Only, Not Blocked) | 2 patterns | LoopBodyLocal only in body, not in conditions |

### Category A Breakdown

| Pattern | Status | Complexity | Priority |
|---------|--------|-----------|----------|
| A-1: Trim Leading | ✅ **Handled** (TrimLoopHelper) | Simple | - |
| A-2: Trim Trailing | ✅ **Handled** (TrimLoopHelper) | Simple | - |
| A-3: Skip Whitespace (Pattern 4) | ⚠️ **Needs Pattern 4 Support** | Simple | **P0** |
| A-4: Digit Detection (Cascading) | ⚠️ **Cascading LoopBodyLocal** | Medium | **P1** |
| A-5: String Comparison | ⚠️ **Multi-Variable** | Medium | P2 |
| A-6: atoi Range Check | ⚠️ **Cascading + Multi-Break** | High | P2 |

---

## Key Insights for Phase 223-2 Design

### 1. **Simple Trim Pattern is Solved** ✅

Phase 171-C's TrimLoopHelper already handles patterns A-1 and A-2 successfully. This is the foundation to build on.

### 2. **Pattern 4 (with continue) Needs Promotion Support** ⚠️ **P0**

Pattern A-3 (skip_whitespace) is a **critical blocker** for JsonParser. It's the same as Trim pattern but uses `continue` instead of `break`.

**Action Required**: Extend LoopBodyCarrierPromoter to support Pattern 4 (Phase 223-2-P0).

### 3. **Cascading LoopBodyLocal is Common** ⚠️ **P1**

Pattern A-4 shows a **cascading dependency**:
```hako
local ch = s.substring(p, p+1)       // First LoopBodyLocal
local digit_pos = digits.indexOf(ch) // Second LoopBodyLocal (depends on ch)
```

**Design Question**:
- Promote both to carriers? (`ch_carrier`, `is_digit_carrier`)
- Or only promote the "leaf" variable (`is_digit_carrier`)?

**Recommendation**: Promote only the **leaf** variable that appears in conditions. In Pattern A-4, only `digit_pos` appears in the break condition (`if digit_pos < 0`), so promote that.

### 4. **Multi-Variable Patterns Need Special Handling** (P2)

Pattern A-5 (string comparison) uses TWO LoopBodyLocal variables in the same condition. This is less common but should be supported eventually.

### 5. **Fail-Fast for Complex Patterns is Correct** ✅

Pattern B-1 (nested if with reassignment) correctly Fail-Fasts. These patterns are too complex for safe carrier promotion.

---

## Next Steps (Phase 223-2)

### Phase 223-2-P0: Pattern 4 Promotion (Critical)

**Goal**: Enable `skip_whitespace` pattern (A-3) by supporting carrier promotion in Pattern 4 (with continue).

**Files to Modify**:
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` (add promotion logic)
- `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs` (extend to handle continue)

**Test Case**: `apps/tests/parser_box_minimal.hako` (skip_ws method)

---

### Phase 223-2-P1: Cascading LoopBodyLocal (High Priority)

**Goal**: Enable JsonParser number parsing (Pattern A-4) by promoting leaf variables in cascading dependencies.

**Design**:
1. Detect cascading pattern: `local ch = ...; local digit_pos = indexOf(ch)`
2. Identify leaf variable: `digit_pos` (appears in condition)
3. Promote leaf variable to carrier: `is_digit` (bool)
4. Initialize carrier: `is_digit = (digit_pos >= 0)`
5. Update carrier: Same as initialization (at end of loop)

**Test Case**: `tools/hako_shared/json_parser.hako` (_parse_number method)

---

### Phase 223-2-P2: Multi-Variable Patterns (Lower Priority)

**Goal**: Enable string comparison pattern (A-5) by promoting multiple variables.

**Design**: TBD (after P0/P1 experience)

---

## Appendix: Full File Locations

### Trim Patterns (Already Handled)

- ✅ `tools/hako_shared/json_parser.hako:330-336` (_trim leading)
- ✅ `tools/hako_shared/json_parser.hako:340-346` (_trim trailing)

### Pattern 4 (Needs P0)

- ⚠️ `apps/tests/parser_box_minimal.hako:30-41` (skip_ws with continue)
- ⚠️ `tools/hako_shared/json_parser.hako:310-321` (_skip_whitespace)

### Cascading LoopBodyLocal (Needs P1)

- ⚠️ `tools/hako_shared/json_parser.hako:121-133` (_parse_number digit detection)
- ⚠️ `tools/hako_shared/json_parser.hako:453-460` (_atoi digit parsing)
- ⚠️ `apps/tests/parser_box_minimal.hako:20-25` (to_int)

### Multi-Variable (Needs P2)

- ⚠️ `apps/tests/phase182_p1_match_literal.hako:356-364` (_match_literal)

### Complex (Fail-Fast Maintained)

- ✅ `apps/tests/minimal_ssa_bug_loop.hako` (nested if with reassignment)

### Body-Only (Not Blocked)

- ✅ `apps/tests/phase183_body_only_loopbodylocal.hako` (temp variable)
- ✅ `apps/tests/phase184_body_local_with_break.hako` (temp in update)

---

## Revision History

- **2025-12-10**: Phase 223-1 initial inventory created
Status: Active  
Scope: LoopBodyLocal condition 在庫（JoinIR/ExprLowerer ライン）
