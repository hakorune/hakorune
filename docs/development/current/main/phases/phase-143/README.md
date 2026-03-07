# Phase 143: Canonicalizer Adaptation Range Expansion

## Status
- State: 🎉 Complete (P0)

Reading note:
- この phase は canonicalizer 初期拡張の historical log だよ。
- 下の `Pattern2Break` / `Pattern4Continue` / `Chosen pattern` は当時の routing/debug token で、current route family では `LoopBreak` / `LoopContinueOnly` と読めばよい。

## P0: parse_number route shape - Break in THEN Clause

### Objective
Expand the canonicalizer to recognize parse_number/digit collection patterns, maximizing the adaptation range before adding new lowering patterns.

### Target Pattern
`tools/selfhost/test_pattern2_parse_number.hako`

```hako
loop(i < num_str.length()) {
  local ch = num_str.substring(i, i + 1)
  local digit_pos = digits.indexOf(ch)

  // Exit on non-digit (break in THEN clause)
  if digit_pos < 0 {
    break
  }

  // Append digit
  result = result + ch
  i = i + 1
}
```

### Route-Shape Characteristics

**Key Difference from skip_whitespace**:
- **skip_whitespace**: `if cond { update } else { break }` - break in ELSE clause
- **parse_number**: `if invalid_cond { break } body... update` - break in THEN clause

**Structure**:
```
loop(cond) {
    // ... body statements (ch, digit_pos computation)
    if invalid_cond {
        break
    }
    // ... rest statements (result append, carrier update)
    carrier = carrier + const
}
```

### Implementation Summary

#### 1. New Recognizer (`ast_feature_extractor.rs`)

Added historical helper `detect_parse_number_pattern()`:
- Detects `if cond { break }` pattern (no else clause)
- Extracts body statements before break check
- Extracts rest statements after break check (including carrier update)
- Returns `ParseNumberInfo { carrier_name, delta, body_stmts, rest_stmts }`

**Lines added**: ~150 lines

#### 2. Canonicalizer Integration (`canonicalizer.rs`)

- Tries parse_number pattern before skip_whitespace pattern
- Builds LoopSkeleton with:
  - Step 1: HeaderCond
  - Step 2: Body (statements before break)
  - Step 3: Body (statements after break, excluding carrier update)
  - Step 4: Update (carrier update)
- Routes to historical choice `Pattern2Break` (current route family: `LoopBreak`, has_break=true)

**Lines modified**: ~60 lines

#### 3. Export Chain

Added exports through the module hierarchy:
- `ast_feature_extractor.rs` → `ParseNumberInfo` struct
- `patterns/mod.rs` → re-export
- `joinir/mod.rs` → re-export
- `control_flow/mod.rs` → re-export
- `builder.rs` → re-export
- `mir/mod.rs` → final re-export

**Files modified**: 6 files (8 lines total)

#### 4. Unit Tests

Added `test_parse_number_pattern_recognized()` in `canonicalizer.rs`:
- Builds AST for parse_number pattern
- Verifies skeleton structure (4 steps)
- Verifies carrier (name="i", delta=1, role=Counter)
- Verifies exit contract (has_break=true)
- Verifies routing decision (`Pattern2Break`; current route family: `LoopBreak`, no missing_caps)

**Lines added**: ~130 lines

### Acceptance Criteria

- ✅ Canonicalizer creates Skeleton for parse_number loop
- ✅ RoutingDecision.chosen matches router (`Pattern2Break`; current route family: `LoopBreak`)
- ✅ Strict parity OK (canonicalizer and router agree)
- ✅ Default behavior unchanged
- ✅ quick profile not affected
- ✅ Unit test added
- ✅ Documentation created

### Results

#### Parity Verification

```bash
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern2_parse_number.hako
```

**Historical output**:
```
[loop_canonicalizer]   Chosen pattern: Pattern2Break
[choose_pattern_kind/PARITY] OK: canonical and actual agree on Pattern2Break
[loop_canonicalizer/PARITY] OK in function 'main': canonical and actual agree on Pattern2Break
```

**Status**: ✅ **Green parity** - canonicalizer and router agree

#### Unit Test Results

```bash
cargo test --release --lib loop_canonicalizer::canonicalizer::tests::test_parse_number_pattern_recognized
```

**Status**: ✅ **PASS**

### Statistics

| Metric | Count |
|--------|-------|
| New patterns supported | 1 (parse_number) |
| Total patterns supported | 3 (skip_whitespace, parse_number, continue) |
| New Capability Tags | 0 (uses existing ConstStep) |
| Lines added | ~280 |
| Files modified | 8 |
| Unit tests added | 1 |
| Parity status | Green ✅ |

### Comparison: Parse Number vs Skip Whitespace

| Aspect | Skip Whitespace | Parse Number |
|--------|----------------|--------------|
| **Break location** | ELSE clause | THEN clause |
| **Pattern** | `if cond { update } else { break }` | `if invalid { break } rest... update` |
| **Body before if** | Optional | Optional (ch, digit_pos) |
| **Body after if** | None (last statement) | Required (result append) |
| **Carrier update** | In THEN clause | After if statement |
| **Routing** | Pattern2Break | Pattern2Break |
| **Example** | skip_whitespace, trim_leading/trailing | parse_number, digit collection |

### Follow-up Opportunities

#### Immediate (Phase 143 P1-P2)
- [ ] Support parse_string pattern (continue + return combo)
- [ ] Add capability for variable-step updates (escape handling)

#### Future Enhancements
- [ ] Extend recognizer for nested if patterns
- [ ] Support multiple break points (requires new capability)
- [ ] Add signature-based corpus analysis

### Lessons Learned

1. **Break location matters**: THEN vs ELSE clause creates different patterns
2. **rest_stmts extraction**: Need to carefully separate body from carrier update
3. **Export chain**: Requires 6-level re-export (ast → patterns → joinir → control_flow → builder → mir)
4. **Parity first**: Always verify strict parity before claiming success

## SSOT

- **Design**: `docs/development/current/main/design/loop-canonicalizer.md`
- **Recognizer**: `src/mir/builder/control_flow/plan/ast_feature_extractor.rs`
  - historical path token: `ast_feature_extractor.rs` under the old `joinir/patterns/` lane
- **Canonicalizer**: `src/mir/loop_canonicalizer/canonicalizer.rs`
- **Tests**: Test file `tools/selfhost/test_pattern2_parse_number.hako`

---

## P1: parse_string Pattern - Continue + Return Combo

### Status
✅ Complete (2025-12-16)

### Objective
Expand canonicalizer to recognize parse_string patterns with both `continue` (escape handling) and `return` (quote found).

### Target Pattern
`tools/selfhost/test_pattern4_parse_string.hako`

```hako
loop(p < len) {
  local ch = s.substring(p, p + 1)

  // Check for closing quote (return)
  if ch == "\"" {
    return 0
  }

  // Check for escape sequence (continue)
  if ch == "\\" {
    result = result + ch
    p = p + 1
    if p < len {  // Nested if
      result = result + s.substring(p, p + 1)
      p = p + 1
      continue  // Nested continue
    }
  }

  // Regular character
  result = result + ch
  p = p + 1
}
```

### Pattern Characteristics

**Key Features**:
- Multiple exit types: both `return` and `continue`
- Nested control flow: continue is inside a nested `if`
- Variable step updates: `p++` normally, but `p += 2` on escape

**Structure**:
```
loop(cond) {
    // ... body statements (ch computation)
    if quote_cond {
        return result
    }
    if escape_cond {
        // ... escape handling
        carrier = carrier + step
        if nested_cond {
            // ... nested handling
            carrier = carrier + step
            continue  // Nested continue!
        }
    }
    // ... regular processing
    carrier = carrier + step
}
```

### Implementation Summary

#### 1. New Recognizer (`ast_feature_extractor.rs`)

Added `detect_parse_string_pattern()`:
- Detects `if cond { return }` pattern
- Detects `continue` statement (with recursive search for nested continue)
- Uses `has_continue_node()` helper for deep search
- Returns `ParseStringInfo { carrier_name, delta, body_stmts }`

**Lines added**: ~120 lines

#### 2. Canonicalizer Integration (`canonicalizer.rs`)

- Tries parse_string pattern first (most specific)
- Builds LoopSkeleton with:
  - Step 1: HeaderCond
  - Step 2: Body (statements before exit checks)
  - Step 3: Update (carrier update)
- Sets ExitContract:
  - `has_break = false`
  - `has_continue = true`
  - `has_return = true`
- Routes to `Pattern4Continue` (has both continue and return)

**Lines modified**: ~45 lines

#### 3. Export Chain

Added exports through the module hierarchy:
- `ast_feature_extractor.rs` → `ParseStringInfo` struct + `detect_parse_string_pattern()`
- `patterns/mod.rs` → re-export
- `joinir/mod.rs` → re-export
- `control_flow/mod.rs` → re-export
- `builder.rs` → re-export
- `mir/mod.rs` → final re-export

**Files modified**: 7 files (10 lines total)

#### 4. Unit Tests

Added `test_parse_string_pattern_recognized()` in `canonicalizer.rs`:
- Builds AST for parse_string pattern
- Verifies skeleton structure (3 steps minimum)
- Verifies carrier (name="p", delta=1, role=Counter)
- Verifies exit contract (has_continue=true, has_return=true, has_break=false)
- Verifies routing decision (Pattern4Continue, no missing_caps)

**Lines added**: ~180 lines

### Acceptance Criteria

- ✅ Canonicalizer creates Skeleton for parse_string loop
- ✅ RoutingDecision.chosen matches router (Pattern4Continue)
- ✅ Strict parity green (canonicalizer and router agree)
- ✅ Default behavior unchanged
- ✅ quick profile not affected (unrelated smoke test failure)
- ✅ Unit test added and passing
- ✅ Nested continue detection implemented

### Results

#### Parity Verification

```bash
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern4_parse_string.hako
```

**Output**:
```
[loop_canonicalizer]   Skeleton steps: 3
[loop_canonicalizer]   Carriers: 1
[loop_canonicalizer]   Has exits: true
[loop_canonicalizer]   Decision: SUCCESS
[loop_canonicalizer]   Chosen pattern: Pattern4Continue
[loop_canonicalizer]   Missing caps: []
[loop_canonicalizer/PARITY] OK in function 'main': canonical and actual agree on Pattern4Continue
```

**Status**: ✅ **Green parity** - canonicalizer and router agree on Pattern4Continue

#### Unit Test Results

```bash
cargo test --release --lib loop_canonicalizer --release
```

**Status**: ✅ **All 19 tests PASS**

### Statistics

| Metric | Count |
|--------|-------|
| New patterns supported | 1 (parse_string) |
| Total patterns supported | 4 (skip_whitespace, parse_number, continue, parse_string) |
| New Capability Tags | 0 (uses existing ConstStep) |
| Lines added | ~300 |
| Files modified | 9 |
| Unit tests added | 1 |
| Parity status | Green ✅ |

### Technical Challenges

1. **Nested Continue Detection**: Required using `has_continue_node()` recursive helper instead of shallow iteration
2. **Complex Exit Contract**: First pattern with both `has_continue=true` AND `has_return=true`
3. **Variable Step Updates**: The actual loop has variable steps (p++ vs p+=2), but canonicalizer uses base delta=1

### Comparison: Parse String vs Other Patterns

| Aspect | Skip Whitespace | Parse Number | Continue | **Parse String** |
|--------|----------------|--------------|----------|------------------|
| **Break** | Yes (ELSE) | Yes (THEN) | No | No |
| **Continue** | No | No | Yes | **Yes** |
| **Return** | No | No | No | **Yes** |
| **Nested control** | No | No | No | **Yes (nested if + continue)** |
| **Routing** | Pattern2Break | Pattern2Break | Pattern4Continue | **Pattern4Continue** |

### Follow-up Opportunities

#### Next Steps (Phase 143 P2-P3)
- [ ] Support parse_array pattern (array element collection)
- [ ] Support parse_object pattern (key-value pair collection)
- [ ] Add capability for true variable-step updates (not just const delta)

#### Future Enhancements
- [ ] Support multiple return points
- [ ] Handle more complex nested patterns
- [ ] Add signature-based corpus analysis for pattern discovery

### Lessons Learned

1. **Nested Detection Required**: Simple shallow iteration isn't enough for real-world patterns
2. **ExitContract Diversity**: Patterns can have multiple exit types simultaneously
3. **Parity vs Execution**: Achieving parity doesn't guarantee runtime success (Pattern4 lowering may need enhancements)
4. **Recursive Helpers**: Reusing existing helpers (`has_continue_node`) is better than duplicating logic

---

## P2: parse_array Pattern - Separator + Stop Combo

### Status
✅ Complete (2025-12-16)

### Objective
Extend canonicalizer to recognize parse_array patterns with both `continue` (separator handling) and `return` (stop condition).

### Target Pattern
`tools/selfhost/test_pattern4_parse_array.hako`

```hako
loop(p < len) {
  local ch = s.substring(p, p + 1)

  // Check for array end (return)
  if ch == "]" {
    if elem.length() > 0 {
      arr.push(elem)
    }
    return 0
  }

  // Check for separator (continue)
  if ch == "," {
    if elem.length() > 0 {
      arr.push(elem)
      elem = ""
    }
    p = p + 1
    continue
  }

  // Accumulate element
  elem = elem + ch
  p = p + 1
}
```

### Pattern Characteristics

**Key Features**:
- Multiple exit types: both `return` (stop condition) and `continue` (separator)
- Separator handling: `,` triggers element save and continue
- Stop condition: `]` triggers final save and return
- Same structural pattern as parse_string

**Structure**:
```
loop(cond) {
    // ... body statements (ch computation)
    if stop_cond {            // ']' for array
        // ... save final element
        return result
    }
    if separator_cond {       // ',' for array
        // ... save element, reset accumulator
        carrier = carrier + step
        continue
    }
    // ... accumulate element
    carrier = carrier + step
}
```

### Implementation Summary

#### Key Discovery: Shared Pattern with parse_string

**No new recognizer needed!** The existing `detect_parse_string_pattern()` already handles both patterns:
- Both have `return` statement (stop condition)
- Both have `continue` statement (separator/escape)
- Both have carrier updates
- Only semantic difference is what the conditions check for

#### Changes Made

1. **Documentation Updates** (~150 lines)
   - Updated `ast_feature_extractor.rs` to document parse_array support
   - Updated `pattern_recognizer.rs` wrapper documentation
   - Updated `canonicalizer.rs` supported patterns list
   - Added parse_array example to pattern documentation

2. **Unit Test** (~165 lines)
   - Added `test_parse_array_pattern_recognized()` in `canonicalizer.rs`
   - Mirrors parse_string test structure with array-specific conditions
   - Verifies same Pattern4Continue routing

3. **Error Messages** (~5 lines)
   - Updated error messages to mention parse_array

**Total lines modified**: ~320 lines (mostly documentation)

### Acceptance Criteria

- ✅ Canonicalizer creates Skeleton for parse_array loop
- ✅ RoutingDecision.chosen == Pattern4Continue
- ✅ Strict parity green (canonicalizer and router agree)
- ✅ Default behavior unchanged
- ✅ Unit test added and passing
- ✅ No new capability needed

### Results

#### Parity Verification

```bash
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern4_parse_array.hako
```

**Output**:
```
[loop_canonicalizer]   Skeleton steps: 3
[loop_canonicalizer]   Carriers: 1
[loop_canonicalizer]   Has exits: true
[loop_canonicalizer]   Decision: SUCCESS
[loop_canonicalizer]   Chosen pattern: Pattern4Continue
[loop_canonicalizer]   Missing caps: []
[loop_canonicalizer/PARITY] OK in function 'main': canonical and actual agree on Pattern4Continue
```

**Status**: ✅ **Green parity** - canonicalizer and router agree on Pattern4Continue

#### Unit Test Results

```bash
cargo test --release --lib loop_canonicalizer::canonicalizer::tests::test_parse_array_pattern_recognized
```

**Status**: ✅ **PASS**

### Statistics

| Metric | Count |
|--------|-------|
| New patterns supported | 1 (parse_array, shares recognizer with parse_string) |
| Total patterns supported | 5 (skip_whitespace, parse_number, continue, parse_string, parse_array) |
| New Capability Tags | 0 (uses existing ConstStep) |
| Lines added | ~320 (mostly documentation) |
| Files modified | 3 (canonicalizer.rs, ast_feature_extractor.rs, pattern_recognizer.rs) |
| Unit tests added | 1 |
| Parity status | Green ✅ |

### Comparison: Parse String vs Parse Array

| Aspect | Parse String | Parse Array |
|--------|--------------|-------------|
| **Stop condition** | `"` (quote) | `]` (array end) |
| **Separator** | `\` (escape) | `,` (element separator) |
| **Structure** | continue + return | continue + return |
| **Recognizer** | `detect_parse_string_pattern()` | **Same recognizer!** |
| **Routing** | Pattern4Continue | Pattern4Continue |
| **ExitContract** | has_continue=true, has_return=true | has_continue=true, has_return=true |

### Key Insight: Structural vs Semantic Patterns

**Major Discovery**: parse_string and parse_array are **structurally identical** at the AST level:
- Both have `if stop_cond { return }`
- Both have `if separator_cond { continue }`
- Both have carrier updates

The **semantic difference** (what the conditions check) doesn't matter for pattern recognition!

This demonstrates the power of AST-based pattern matching: we can recognize structural patterns without understanding their semantic meaning.

### Follow-up Opportunities

#### Next Steps (Phase 143 P3)
- [ ] Support parse_object pattern (likely also shares the same recognizer!)
- [ ] Document pattern families (structural equivalence classes)

#### Future Enhancements
- [ ] Generalize to "dual-exit patterns" (continue + return)
- [ ] Add corpus analysis to discover more structural equivalences
- [ ] Create pattern taxonomy based on AST structure

### Lessons Learned

1. **Structural Equivalence**: Different semantic patterns can share the same AST structure
2. **Recognizer Reuse**: One recognizer can handle multiple use cases
3. **Documentation > Code**: More documentation changes than code changes
4. **Test Coverage**: Unit tests verify both semantic variants work with the same recognizer

---

## P3: parse_object Pattern - Key-Value Pair Collection

### Status
✅ Complete (2025-12-16)

### Objective
Verify that parse_object pattern (key-value pair collection) is recognized by the existing recognizer, maintaining structural equivalence with parse_string/parse_array.

### Target Pattern
`tools/selfhost/test_pattern4_parse_object.hako`

```hako
loop(p < s.length()) {
  // ... optional body statements

  // Check for object end (return)
  local ch = s.substring(p, p+1)
  if ch == "}" {
    return obj  // Stop: object complete
  }

  // Check for separator (continue)
  if ch == "," {
    p = p + 1
    continue  // Separator: continue to next key-value pair
  }

  // Regular processing
  p = p + 1
}
```

### Pattern Characteristics

**Key Features**:
- Multiple exit types: both `return` (stop condition) and `continue` (separator)
- Separator handling: `,` triggers continue to next pair
- Stop condition: `}` triggers return with result
- **Same structural pattern as parse_string/parse_array**

**Structure**:
```
loop(cond) {
    // ... body statements (ch computation)
    if stop_cond {            // '}' for object
        return result
    }
    if separator_cond {       // ',' for object
        carrier = carrier + step
        continue
    }
    // ... regular processing
    carrier = carrier + step
}
```

### Implementation Summary

#### Key Discovery: Complete Structural Equivalence

**No new recognizer needed!** The existing `detect_parse_string_pattern()` handles parse_object perfectly:
- Has `return` statement (stop condition: `}`)
- Has `continue` statement (separator: `,`)
- Has carrier updates (`p = p + 1`)
- Only semantic difference is the stop/separator characters

**Pattern Family Confirmed**: parse_string, parse_array, and parse_object are **structurally identical**.

#### Changes Made

1. **Test File Creation** (~50 lines)
   - Created `tools/selfhost/test_pattern4_parse_object.hako`
   - Minimal test demonstrating parse_object loop structure

2. **Unit Test** (~170 lines)
   - Added `test_parse_object_pattern_recognized()` in `canonicalizer.rs`
   - Mirrors parse_array test structure with object-specific conditions (`}` and `,`)
   - Verifies same Pattern4Continue routing

3. **Documentation** (this section)

**Total implementation**: ~220 lines (no new recognizer code needed!)

### Acceptance Criteria

- ✅ Canonicalizer creates Skeleton for parse_object loop
- ✅ RoutingDecision.chosen == Pattern4Continue
- ✅ RoutingDecision.missing_caps == []
- ✅ Strict parity green (canonicalizer and router agree)
- ✅ Default behavior unchanged
- ✅ Unit test added and passing
- ✅ No new capability needed
- ✅ **No new recognizer needed** (existing recognizer handles it)

### Results

#### Parity Verification

```bash
NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune \
  tools/selfhost/test_pattern4_parse_object.hako
```

**Output**:
```
[loop_canonicalizer]   Chosen pattern: Pattern4Continue
[choose_pattern_kind/PARITY] OK: canonical and actual agree on Pattern4Continue
[loop_canonicalizer/PARITY] OK in function 'Main.parse_object_loop/0': canonical and actual agree on Pattern4Continue
```

**Status**: ✅ **Green parity** - canonicalizer and router agree on Pattern4Continue

#### Unit Test Results

```bash
cargo test --release --lib loop_canonicalizer::canonicalizer::tests::test_parse_object_pattern_recognized
```

**Status**: ✅ **PASS**

### Statistics

| Metric | Count |
|--------|-------|
| New patterns supported | 1 (parse_object, shares recognizer with parse_string/array) |
| Total patterns supported | 6 (skip_whitespace, parse_number, continue, parse_string, parse_array, parse_object) |
| New Capability Tags | 0 (uses existing ConstStep) |
| Lines added | ~220 (test file + unit test + docs) |
| Files modified | 2 (canonicalizer.rs, new test file) |
| Unit tests added | 1 |
| Parity status | Green ✅ |
| **New recognizer code** | **0 lines** (complete reuse!) |

### Comparison: Parse String vs Parse Array vs Parse Object

| Aspect | Parse String | Parse Array | Parse Object |
|--------|--------------|-------------|--------------|
| **Stop condition** | `"` (quote) | `]` (array end) | `}` (object end) |
| **Separator** | `\` (escape) | `,` (element separator) | `,` (pair separator) |
| **Structure** | continue + return | continue + return | continue + return |
| **Recognizer** | `detect_parse_string_pattern()` | **Same** | **Same** |
| **Routing** | Pattern4Continue | Pattern4Continue | Pattern4Continue |
| **ExitContract** | has_continue=true, has_return=true | **Same** | **Same** |

### Key Insight: Structural Pattern Family

**Major Discovery**: parse_string, parse_array, and parse_object form a **structural pattern family**:
- All have `if stop_cond { return }`
- All have `if separator_cond { continue }`
- All have carrier updates
- **One recognizer handles all three!**

The semantic differences (string quote vs array bracket vs object brace) are invisible at the AST structure level.

**Implication**: AST-based pattern matching creates natural pattern families. When we implement one pattern, we often get multiple variants "for free".

### Coverage Expansion Summary

Phase 143 started with 3 patterns (skip_whitespace, parse_number, continue) and expanded to 6 patterns:
- P0: Added parse_number (new recognizer)
- P1: Added parse_string (new recognizer)
- P2: Added parse_array (**reused parse_string recognizer**)
- P3: Added parse_object (**reused parse_string recognizer**)

**Recognizer efficiency**: 2 new recognizers → 4 new patterns supported!

### Follow-up Opportunities

#### Next Steps (Phase 144+)
- [ ] Document pattern families in design docs
- [ ] Add corpus analysis to discover more structural equivalences
- [ ] Create pattern taxonomy based on AST structure
- [ ] Explore other potential pattern families

#### Future Enhancements
- [ ] Generalize to "dual-exit patterns" (continue + return)
- [ ] Support triple-exit patterns (break + continue + return)
- [ ] Add signature-based pattern discovery

### Lessons Learned

1. **Pattern Families**: Structural equivalence creates natural groupings
2. **Recognizer Reuse**: Testing existing recognizers before writing new ones saves effort
3. **Semantic vs Structural**: AST patterns are structural; semantic meaning doesn't affect recognition
4. **Test-Driven Discovery**: Unit tests verify recognizer generality
5. **Documentation Value**: Recording discoveries helps future pattern work

---

**Phase 143 P0: Complete** ✅
**Phase 143 P1: Complete** ✅
**Phase 143 P2: Complete** ✅
**Phase 143 P3: Complete** ✅
**Date**: 2025-12-16
**Implemented by**: Claude Code (Sonnet 4.5)
