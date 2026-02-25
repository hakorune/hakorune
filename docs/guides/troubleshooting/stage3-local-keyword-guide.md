# Stage-3 Local Keyword Troubleshooting Guide

## Problem Description

When using Stage-B (self-hosted) code that contains `local` keyword, you may encounter:
```
❌ MIR compilation error: Undefined variable: local
```

## Root Cause

The `local` keyword is a **Stage-3 keyword** that requires explicit enablement via environment variables. Without these ENV variables:
1. The tokenizer downgrades `local` from `TokenType::LOCAL` to `TokenType::IDENTIFIER`
2. The MIR builder then treats it as an undefined variable

## Quick Fix

### For AotPrep Verification (Recommended)
Use the provided script which automatically sets all required ENV variables:

```bash
tools/hakorune_emit_mir.sh input.hako output.json
```

This script automatically enables:
- `NYASH_PARSER_STAGE3=1`
- `HAKO_PARSER_STAGE3=1`
- `NYASH_PARSER_ALLOW_SEMICOLON=1`

### For Manual Execution

```bash
NYASH_PARSER_STAGE3=1 \
HAKO_PARSER_STAGE3=1 \
NYASH_PARSER_ALLOW_SEMICOLON=1 \
./target/release/hakorune --backend vm your_file.hako
```

### For AotPrep with CollectionsHot

```bash
NYASH_SKIP_TOML_ENV=1 \
NYASH_DISABLE_PLUGINS=1 \
HAKO_APPLY_AOT_PREP=1 \
NYASH_AOT_COLLECTIONS_HOT=1 \
NYASH_LLVM_FAST=1 \
NYASH_MIR_LOOP_HOIST=1 \
NYASH_JSON_ONLY=1 \
tools/hakorune_emit_mir.sh input.hako output.json
```

## Diagnostic Tools

### 1. Improved Error Message (New!)

When you forget to enable Stage-3, you'll now see:

```
❌ MIR compilation error: Undefined variable: local
Hint: 'local' is a Stage-3 keyword. Enable NYASH_PARSER_STAGE3=1 (and HAKO_PARSER_STAGE3=1 for Stage-B).
For AotPrep verification, use tools/hakorune_emit_mir.sh which sets these automatically.
```

### 2. Tokenizer Trace

To see exactly when keywords are being downgraded:

```bash
NYASH_TOK_TRACE=1 ./target/release/hakorune --backend vm your_file.hako
```

Output example:
```
[tok-stage3] Degrading LOCAL to IDENTIFIER (NYASH_PARSER_STAGE3=false)
```

## Stage-3 Keywords

The following keywords require `NYASH_PARSER_STAGE3=1`:
- `local` - Local variable declaration
- `flow` - Flow control (reserved)
- `try` - Exception handling
- `catch` - Exception handling
- `throw` - Exception handling
- `while` - Loop construct
- `for` - Loop construct
- `in` - Loop/iteration operator

## Code Changes Made

### 1. Builder Error Message Enhancement
**File**: `src/mir/builder.rs:382-406`

Added Stage-3 keyword detection with helpful hints when `parser_stage3()` is disabled.

### 2. Documentation Update
**File**: `lang/src/llvm_ir/boxes/aot_prep/README.md`

Added "Stage-3 キーワード要件" section explaining:
- Why Stage-3 ENV variables are needed for AotPrep
- Recommended usage of `tools/hakorune_emit_mir.sh`
- Manual ENV variable requirements
- Diagnostic options

## Related Issues

### CollectionsHot lastIndexOf Error

If you see:
```
❌ VM error: Invalid instruction: lastIndexOf expects 1 arg(s), got 2
```

This is a **separate issue** from Stage-3 keywords. The CollectionsHot pass uses a two-argument `lastIndexOf(needle, start_pos)` which is not yet implemented in the VM's StringBox.

**Workaround**: Disable CollectionsHot until the VM implementation is updated:
```bash
# Omit NYASH_AOT_COLLECTIONS_HOT=1
tools/hakorune_emit_mir.sh input.hako output.json
```

## Testing

### Test 1: Error Message Without ENV
```bash
cat > /tmp/test.hako << 'EOF'
static box Main {
    method main(args) {
        local x
        x = 42
        return 0
    }
}
EOF

env -u NYASH_PARSER_STAGE3 -u HAKO_PARSER_STAGE3 \
./target/release/hakorune --backend vm /tmp/test.hako
```

Expected: Error message with Stage-3 hint

### Test 2: Success With ENV
```bash
NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 NYASH_PARSER_ALLOW_SEMICOLON=1 \
./target/release/hakorune --backend vm /tmp/test.hako
```

Expected: Program executes successfully

### Test 3: Tokenizer Trace
```bash
NYASH_TOK_TRACE=1 env -u NYASH_PARSER_STAGE3 \
./target/release/hakorune --backend vm /tmp/test.hako 2>&1 | grep tok-stage3
```

Expected: `[tok-stage3] Degrading LOCAL to IDENTIFIER (NYASH_PARSER_STAGE3=false)`

## References

- **Tokenizer Stage-3 Gate**: `src/tokenizer/lex_ident.rs:69-89`
- **Parser Stage-3 Check**: `src/config/env.rs:495-504`
- **Builder Error Generation**: `src/mir/builder.rs:382-406`
- **AotPrep Documentation**: `lang/src/llvm_ir/boxes/aot_prep/README.md`
- **Emit MIR Script**: `tools/hakorune_emit_mir.sh`

## Summary

✅ **Stage-3 keyword error resolved** - Improved error messages guide users to the fix
✅ **Documentation updated** - AotPrep README now explains Stage-3 requirements
✅ **Diagnostic tools available** - `NYASH_TOK_TRACE=1` for tokenizer debugging
✅ **Recommended workflow** - Use `tools/hakorune_emit_mir.sh` for hassle-free execution

The original issue was environmental configuration, not a bug in CollectionsHot itself. Once Stage-3 ENV variables are properly set, Stage-B code compiles and executes correctly.
