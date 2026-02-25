# MIR Nested-If-in-Loop Bug (Critical)

**Date**: 2025-12-04
**Severity**: Critical
**Status**: Workaround Applied, Root Fix Needed

## 🐛 Problem Summary

**Symptom**: Infinite loop when using nested `if-else` statements inside `loop()` blocks.

**Error Message**:
```
[ERROR] VM error: vm step budget exceeded (max_steps=1000000, steps=1000001)
```

## 📊 Root Cause Analysis

### MIR Lowering Bug

The MIR builder generates incorrect control flow for nested if-else statements inside loops:

```hako
// ❌ Causes infinite loop
loop(condition) {
    if expr1 {
        if expr2 {
            // code
        } else {
            break
        }
    } else {
        break
    }
}
```

### Control Flow Issue

**Generated MIR**:
```
bb4: loop header (PHI)
bb6: unconditional jump to bb4
bb11: unconditional jump to bb6

Jump chain: bb11 → bb6 → bb4 → ... (infinite)
```

**Problem**: The PHI node in bb4 never gets updated because the execution gets stuck in the bb11→bb6→bb4 jump chain.

## 🔬 Reproduction Case

### Minimal Test Case

```hako
static box Main {
  main() {
    local i = 0

    loop(i < 3) {
      local x = 1

      if x == 1 {
        if x == 1 {
          i = i + 1
        } else {
          break
        }
      } else {
        break
      }
    }

    return 0
  }
}
```

**Result**: `vm step budget exceeded` at bb6

### MIR Dump Analysis

```
bb6:
    1: br label bb4

bb11:
    1: br label bb6
```

Infinite unconditional jump chain with no PHI update.

## ✅ Workaround

### Strategy: Flatten Nested Ifs

**Before** (infinite loop):
```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch >= "0" && ch <= "9" {
        num_str = num_str + ch
        p = p + 1
    } else {
        break
    }
}
```

**After** (fixed):
```hako
local parsing_done = 0
loop(p < s.length()) {
    if parsing_done == 1 { break }

    local ch = s.substring(p, p+1)
    local digits = "0123456789"
    local digit_pos = digits.indexOf(ch)

    if digit_pos >= 0 {
        num_str = num_str + ch
        p = p + 1
    } else {
        parsing_done = 1
    }
}
```

### Patterns to Avoid

1. **Nested if-else in loop**:
   ```hako
   loop(cond) {
       if a {
           if b { ... } else { break }
       } else { break }
   }
   ```

2. **`&&` operator in loop condition**:
   ```hako
   loop(cond) {
       if x >= "0" && x <= "9" { ... }
   }
   ```

### Safe Patterns

1. **Flatten with flags**:
   ```hako
   local done = 0
   loop(cond) {
       if done == 1 { break }
       // single-level if statements
   }
   ```

2. **Use indexOf instead of range check**:
   ```hako
   local digits = "0123456789"
   if digits.indexOf(ch) >= 0 { ... }
   ```

## 📋 Affected Code

### Fixed Files

1. **tools/hako_shared/json_parser.hako**:
   - `_parse_number()`: Used `indexOf()` workaround
   - `_parse_string()`: Flattened escape sequence check
   - `_unescape_string()`: Flattened `ch == "\\" && i + 1 < s.length()`

2. **Converted `while` → `loop()`** (10 occurrences):
   - All `while` loops converted to `loop()` syntax per language spec

### Commit

- **Commit**: `608693af`
- **Title**: fix(json_parser): Fix infinite loop by working around MIR nested-if bug
- **Files Changed**: 1 file, +45/-23 lines

## 🎯 Root Fix Needed

### MIR Builder Issue

**Location**: `src/mir/builder/` (control flow lowering)

**Problem**: When lowering nested if-else inside loops, the builder creates:
- Unreachable PHI nodes
- Unconditional jump chains
- Missing latch block updates

**Solution Required**:
1. Fix control flow graph generation for nested conditionals
2. Ensure PHI nodes are properly connected
3. Add test cases for nested if-else in loops

### Test Coverage

Add comprehensive tests for:
- Nested if-else in loops (2+ levels)
- `&&` and `||` operators in loop conditions
- `break` and `continue` in nested contexts

## 📚 Related Issues

- **Phase 173 Task 3**: JsonParserBox bug fix (completed with workaround)
- **CLAUDE.md**: `while` → `loop()` syntax migration
- **Loop Builder**: `src/mir/loop_builder.rs` (potential fix location)

## 🔗 References

- **Test Case**: `test_nested_if_loop.hako` (reproduces bug)
- **JSON Parser**: `tools/hako_shared/json_parser.hako` (workaround applied)
- **CURRENT_TASK.md**: Phase 173 tracking

---

**Status**: Workaround deployed, root fix tracked for future MIR lowering improvements.
