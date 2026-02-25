# Phase 20.39 Test Suite - String Scanner Fixes

## Overview
Test suite for string scanner improvements: single-quote support and complete escape sequences.

## Tests

### 1. `parser_escape_sequences_canary.sh`
**Purpose**: Verify all escape sequences work in double-quoted strings

**Escapes tested**:
- `\"` - double-quote
- `\\` - backslash
- `\/` - forward slash (JSON compatibility)
- `\n` - newline (LF)
- `\r` - carriage return (CR) - **FIXED**: was incorrectly `\n`
- `\t` - tab
- `\b` - backspace (MVP: empty string)
- `\f` - form feed (MVP: empty string)

**Expected**: Parser accepts all escapes without error

---

### 2. `parser_single_quote_canary.sh`
**Purpose**: Verify single-quoted strings work in Stage-3 mode

**Test cases**:
- `'hello'` - basic single-quote string
- `'it\'s working'` - single-quote with escape

**Requirements**:
- `NYASH_FEATURES=stage3`
- `HAKO_PARSER_STAGE3=1`

**Expected**: Parser accepts single-quotes in Stage-3

---

### 3. `parser_embedded_json_canary.sh`
**Purpose**: Verify JSON from `jq -Rs .` parses correctly

**Test case**:
```bash
echo '{"key": "value with \"quotes\" and \n newline"}' | jq -Rs .
# Produces: "{\"key\": \"value with \\\"quotes\\\" and \\n newline\"}\n"
```

**Expected**: Parser handles complex escape sequences from jq

---

## Running Tests

### Individual test:
```bash
bash tools/smokes/v2/profiles/quick/core/phase2039/parser_escape_sequences_canary.sh
```

### All phase2039 tests:
```bash
tools/smokes/v2/run.sh --profile quick --filter "phase2039/*"
```

### All quick tests:
```bash
tools/smokes/v2/run.sh --profile quick
```

---

## Implementation Details

**Modified files**:
- `lang/src/compiler/parser/scan/parser_string_scan_box.hako` - Added `scan_with_quote`
- `lang/src/compiler/parser/parser_box.hako` - Updated `read_string_lit`

**Documentation**:
- `docs/updates/phase2039-string-scanner-fix.md` - Complete implementation details

---

## Status
- ✅ Implementation complete
- ✅ Tests created
- ⏳ Integration testing pending

---

## Notes
- Tests use Hako compiler pipeline to verify parser acceptance
- MVP: `\b` and `\f` approximated as empty string
- `\uXXXX`: Concatenated as-is (6 chars), decoding deferred to future phase
